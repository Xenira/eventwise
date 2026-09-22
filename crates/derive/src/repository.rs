use convert_case::{Case, Casing};
use darling::{
    util::{Override, PathList},
    FromDeriveInput, FromMeta,
};
use itertools::Itertools as _;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Ident, ItemStruct, Path, Type, Variant};

use crate::prelude::*;

#[derive(FromMeta)]
pub struct EventArgs {
    #[darling(default)]
    bootstrap: bool,
    variant: Path,
}

#[derive(FromDeriveInput)]
#[darling(
    attributes(aggregate),
    supports(struct_named),
    // forward_attrs(allow, doc, cfg)
)]
pub struct AggregateArgs {
    ident: Ident,
    // #[darling(multiple)]
    events: PathList,
    // #[darling(multiple)]
    bootstrap: PathList,
    id_type: Type,
    error_type: Type,
    // events:
}

pub(crate) fn derive_aggregate(input: DeriveInput) -> Result<TokenStream> {
    let input = AggregateArgs::from_derive_input(&input)?;
    ensure!(
        !input.events.is_empty(),
        input.ident => "Repository must have at least one event"
    );
    ensure!(
        !input.bootstrap.is_empty(),
        input.ident => "Repository must have at least one bootstrap event"
    );

    let impl_aggregate = impl_aggregate(&input)?;
    let event_enum = event_enum(&input)?;

    Ok(quote! {
        //hello
        #impl_aggregate
        #event_enum
    })
}

fn event_enum(input: &AggregateArgs) -> Result<TokenStream> {
    let event_enum_name = format_ident!("{}Event", input.ident);
    let event_variants: Vec<TokenStream> = input
        .events
        .iter()
        .chain(input.bootstrap.iter())
        .map(|event| {
            let event_variant_name = format_ident!("{}", event.get_ident().unwrap());
            quote! {
                #event_variant_name(#event)
            }
        })
        .collect();
    let event_variant_names: Vec<Ident> = input
        .events
        .iter()
        .chain(input.bootstrap.iter())
        .map(|event| format_ident!("{}", event.get_ident().unwrap()))
        .collect();

    let variant_kinds: Vec<String> = input
        .events
        .iter()
        .chain(input.bootstrap.iter())
        .map(|event| {
            format_ident!("{}", &event.get_ident().unwrap())
                .to_string()
                .to_case(Case::Snake)
        })
        .collect();

    Ok(quote! {
        // TODO: Remove serde
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
        pub enum #event_enum_name {
            #(#event_variants),*
        }

        impl eventwise::event::Event for #event_enum_name {
            fn kind(&self) -> &'static str {
                match self {
                    #(#event_enum_name::#event_variant_names(_) => #variant_kinds),*
                }
            }
        }
    })
}

fn impl_aggregate(input: &AggregateArgs) -> Result<TokenStream> {
    let repo_name = &input.ident;
    let kind = repo_name.to_string().to_case(Case::Snake);
    let event_enum_name = format_ident!("{}Event", repo_name);
    let id_type = &input.id_type;
    let error_type = &input.error_type;

    let create_fn = create_fn(input)?;
    let evolve_fn = evolve_fn(input)?;

    Ok(quote! {
        impl eventwise::aggregate::Aggregate for #repo_name {
            const KIND: &'static str = #kind;
            type Event = #event_enum_name;
            type Id = #id_type;
            type Error = #error_type;

            #create_fn
            #evolve_fn
        }
    })
}

fn create_fn(input: &AggregateArgs) -> Result<TokenStream> {
    let event_enum_name = format_ident!("{}Event", input.ident);
    let event_variants: Vec<Ident> = input
        .bootstrap
        .iter()
        .map(|event| format_ident!("{}", event.get_ident().unwrap()))
        .collect();

    Ok(quote! {
        fn create(event: Self::Event) -> Result<Self, eventwise::aggregate::BootstrapError<Self::Event>> {
            match event {
                #(#event_enum_name::#event_variants(e) => Ok(Self::bootstrap(e))),*,
                _ => Err(eventwise::aggregate::BootstrapError(event.clone())),
            }
        }
    })
}

fn evolve_fn(input: &AggregateArgs) -> Result<TokenStream> {
    let event_enum_name = format_ident!("{}Event", input.ident);
    let event_variants: Vec<Ident> = input
        .events
        .iter()
        .map(|event| format_ident!("{}", event.get_ident().unwrap()))
        .collect();

    Ok(quote! {
        fn evolve(&mut self, event: Self::Event) -> Result<(), Self::Error> {
            match event {
                #(#event_enum_name::#event_variants(e) => Ok(self.apply(e))),*,
                _ => Ok(()),
            }
        }
    })
}
