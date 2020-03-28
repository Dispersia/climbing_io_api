use super::context::Context;
use super::models::{Tag, TagInput};
use juniper::FieldResult;

pub struct Mutation;

#[juniper::graphql_object(
    Context = Context,
)]
impl Mutation {
    fn create_tag(_context: &Context, new_tag: TagInput) -> FieldResult<Tag> {
        Ok(Tag {
            id: juniper::ID::new("123"),
            name: new_tag.name.to_string(),
        })
    }
}
