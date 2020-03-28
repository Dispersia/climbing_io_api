mod context;
mod models;
mod mutation;
mod query;

pub use context::Context;

use mutation::Mutation;
use query::Query;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}
