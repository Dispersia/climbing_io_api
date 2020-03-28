#[derive(juniper::GraphQLObject)]
/// Tags denoting the categories for the [Exercise]
pub struct Tag {
    pub id: juniper::ID,
    pub name: String,
}
