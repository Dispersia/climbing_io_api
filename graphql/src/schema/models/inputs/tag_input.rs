#[derive(juniper::GraphQLInputObject)]
/// Tags denoting the categories for the [Exercise]
pub struct TagInput {
    pub name: String,
}