#[macro_use]
extern crate eventwise_derive;
use eventwise_derive::Aggregate;

#[derive(Aggregate)]
#[repo(events(Foo, Baz))]
#[repo(bootstrap(Bar, Qux))]
#[repo(id_type = MyId, error_type = MyError)]
struct MyAggregate {
    foo: String,
}
