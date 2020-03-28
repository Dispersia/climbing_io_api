#[derive(juniper::GraphQLObject)]
/// The type of the exercise, such as a Repition based, Time based, etc
pub struct ExerciseType {
    pub id: juniper::ID,
    pub name: String,
}