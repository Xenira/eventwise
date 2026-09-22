#[macro_use]
extern crate eventwise_derive;
use eventwise_derive::Aggregate;
#[repo(events(Foo, Baz))]
#[repo(bootstrap(Bar, Qux))]
#[repo(id_type = MyId, error_type = MyError)]
struct MyAggregate {
    foo: String,
}
impl eventwise::aggregate::Aggregate for MyAggregate {
    const KIND: &'static str = "my_aggregate";
    type Event = MyAggregateEvent;
    type Id = MyId;
    type Error = MyError;
    fn create(
        event: &Self::Event,
    ) -> Result<Self, eventwise::aggregate::BootstrapError<Self::Event>> {
        match event {
            MyAggregateEvent::Bar(e) => Ok(Self::bootstrap(e)),
            MyAggregateEvent::Qux(e) => Ok(Self::bootstrap(e)),
            _ => Err(eventwise::aggregate::BootstrapError(event.clone())),
        }
    }
    fn evolve(&mut self, event: &Self::Event) -> Result<(), Self::Error> {
        match event {
            MyAggregateEvent::Foo(e) => Ok(self.apply(e)),
            MyAggregateEvent::Baz(e) => Ok(self.apply(e)),
            _ => Ok(()),
        }
    }
}
pub enum MyAggregateEvent {
    Foo(Foo),
    Baz(Baz),
    Bar(Bar),
    Qux(Qux),
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for MyAggregateEvent {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private229::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                MyAggregateEvent::Foo(ref __field0) => {
                    _serde::Serializer::serialize_newtype_variant(
                        __serializer,
                        "MyAggregateEvent",
                        0u32,
                        "Foo",
                        __field0,
                    )
                }
                MyAggregateEvent::Baz(ref __field0) => {
                    _serde::Serializer::serialize_newtype_variant(
                        __serializer,
                        "MyAggregateEvent",
                        1u32,
                        "Baz",
                        __field0,
                    )
                }
                MyAggregateEvent::Bar(ref __field0) => {
                    _serde::Serializer::serialize_newtype_variant(
                        __serializer,
                        "MyAggregateEvent",
                        2u32,
                        "Bar",
                        __field0,
                    )
                }
                MyAggregateEvent::Qux(ref __field0) => {
                    _serde::Serializer::serialize_newtype_variant(
                        __serializer,
                        "MyAggregateEvent",
                        3u32,
                        "Qux",
                        __field0,
                    )
                }
            }
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for MyAggregateEvent {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private229::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private229::Formatter,
                ) -> _serde::__private229::fmt::Result {
                    _serde::__private229::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private229::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private229::Ok(__Field::__field0),
                        1u64 => _serde::__private229::Ok(__Field::__field1),
                        2u64 => _serde::__private229::Ok(__Field::__field2),
                        3u64 => _serde::__private229::Ok(__Field::__field3),
                        _ => {
                            _serde::__private229::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 4",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private229::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "Foo" => _serde::__private229::Ok(__Field::__field0),
                        "Baz" => _serde::__private229::Ok(__Field::__field1),
                        "Bar" => _serde::__private229::Ok(__Field::__field2),
                        "Qux" => _serde::__private229::Ok(__Field::__field3),
                        _ => {
                            _serde::__private229::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private229::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"Foo" => _serde::__private229::Ok(__Field::__field0),
                        b"Baz" => _serde::__private229::Ok(__Field::__field1),
                        b"Bar" => _serde::__private229::Ok(__Field::__field2),
                        b"Qux" => _serde::__private229::Ok(__Field::__field3),
                        _ => {
                            let __value = &_serde::__private229::from_utf8_lossy(
                                __value,
                            );
                            _serde::__private229::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private229::PhantomData<MyAggregateEvent>,
                lifetime: _serde::__private229::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = MyAggregateEvent;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private229::Formatter,
                ) -> _serde::__private229::fmt::Result {
                    _serde::__private229::Formatter::write_str(
                        __formatter,
                        "enum MyAggregateEvent",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private229::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data) {
                        _serde::__private229::Ok((__Field::__field0, __variant)) => {
                            _serde::__private229::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<
                                    Foo,
                                >(__variant),
                                MyAggregateEvent::Foo,
                            )
                        }
                        _serde::__private229::Ok((__Field::__field1, __variant)) => {
                            _serde::__private229::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<
                                    Baz,
                                >(__variant),
                                MyAggregateEvent::Baz,
                            )
                        }
                        _serde::__private229::Ok((__Field::__field2, __variant)) => {
                            _serde::__private229::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<
                                    Bar,
                                >(__variant),
                                MyAggregateEvent::Bar,
                            )
                        }
                        _serde::__private229::Ok((__Field::__field3, __variant)) => {
                            _serde::__private229::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<
                                    Qux,
                                >(__variant),
                                MyAggregateEvent::Qux,
                            )
                        }
                        _serde::__private229::Err(__err) => {
                            _serde::__private229::Err(__err)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &["Foo", "Baz", "Bar", "Qux"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "MyAggregateEvent",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private229::PhantomData::<MyAggregateEvent>,
                    lifetime: _serde::__private229::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::clone::Clone for MyAggregateEvent {
    #[inline]
    fn clone(&self) -> MyAggregateEvent {
        match self {
            MyAggregateEvent::Foo(__self_0) => {
                MyAggregateEvent::Foo(::core::clone::Clone::clone(__self_0))
            }
            MyAggregateEvent::Baz(__self_0) => {
                MyAggregateEvent::Baz(::core::clone::Clone::clone(__self_0))
            }
            MyAggregateEvent::Bar(__self_0) => {
                MyAggregateEvent::Bar(::core::clone::Clone::clone(__self_0))
            }
            MyAggregateEvent::Qux(__self_0) => {
                MyAggregateEvent::Qux(::core::clone::Clone::clone(__self_0))
            }
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for MyAggregateEvent {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            MyAggregateEvent::Foo(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Foo", &__self_0)
            }
            MyAggregateEvent::Baz(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Baz", &__self_0)
            }
            MyAggregateEvent::Bar(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Bar", &__self_0)
            }
            MyAggregateEvent::Qux(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Qux", &__self_0)
            }
        }
    }
}
impl eventwise::event::Event for MyAggregateEvent {
    fn kind(&self) -> &'static str {
        match self {
            MyAggregateEvent::Foo(_) => "foo",
            MyAggregateEvent::Baz(_) => "baz",
            MyAggregateEvent::Bar(_) => "bar",
            MyAggregateEvent::Qux(_) => "qux",
        }
    }
}
