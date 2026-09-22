use eventwise::{
    aggregate::{Apply, Create},
    event::StreamId,
};
use eventwise_derive::Aggregate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Foo;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bar;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Baz;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Qux;

#[derive(Clone)]
pub struct MyId(uuid::Uuid);
impl StreamId for MyId {
    fn key(&self) -> uuid::Uuid {
        self.0
    }
}

impl From<uuid::Uuid> for MyId {
    fn from(value: uuid::Uuid) -> Self {
        MyId(value)
    }
}

impl Apply<Foo> for MyAggregate {
    fn apply(&mut self, event: Foo) {
        // Implement the logic to evolve the aggregate based on the Foo event
    }
}

impl Apply<Baz> for MyAggregate {
    fn apply(&mut self, event: Baz) {
        // Implement the logic to evolve the aggregate based on the Bar event
    }
}

impl Create<Qux> for MyAggregate {
    fn bootstrap(event: Qux) -> Self {
        MyAggregate {
            foo: "Bootstrapped".to_string(),
        }
    }
}

impl Create<Bar> for MyAggregate {
    fn bootstrap(event: Bar) -> Self {
        MyAggregate {
            foo: "Bootstrapped Bar".to_string(),
        }
    }
}

#[derive(Error, Debug)]
#[error("MyError occurred")]
pub struct MyError;

#[derive(Aggregate, Serialize, Deserialize)]
#[aggregate(events(Foo, Baz))]
#[aggregate(bootstrap(Bar, Qux))]
#[aggregate(id_type = MyId, error_type = MyError)]
struct MyAggregate {
    foo: String,
}

pub fn main() {
    let aggregate = MyAggregate {
        foo: "Hello".to_string(),
    };
    // println!("Aggregate: {:?}", aggregate);
}
