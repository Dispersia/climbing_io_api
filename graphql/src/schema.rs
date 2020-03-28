mod context;
mod models;
mod mutation;
mod query;

pub use context::Context;

use juniper::EmptySubscription;
use mutation::Mutation;
use query::Query;

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::<Context>::new())
}
