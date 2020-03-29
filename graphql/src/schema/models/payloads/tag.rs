#[derive(juniper::GraphQLObject)]
/// Tags denoting the categories for the [Exercise]
pub struct Tag {
    pub id: juniper::ID,
    pub name: String,
}

impl From<services::Tag> for Tag {
    fn from(tag: services::Tag) -> Tag {
        Tag {
            id: juniper::ID::new(tag.id.to_string()),
            name: tag.name,
        }
    }
}
