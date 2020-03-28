use super::context::Context;
use juniper::FieldResult;
use std::ops::Deref;

use super::models::{Exercise, Tag};

pub struct Query;

#[juniper::graphql_object(
    Context = Context,
)]
impl Query {
    async fn tags(_context: &Context) -> FieldResult<Tag> {
        Ok(Tag {
            id: juniper::ID::new("1"),
            name: "Test".to_string(),
        })
    }

    async fn exercise(_context: &Context, id: juniper::ID) -> FieldResult<Exercise> {
        Ok(Exercise::new(id.deref(), "Some Exercise"))
    }
}
