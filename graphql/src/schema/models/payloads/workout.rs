#[derive(juniper::GraphQLObject)]
/// A completed workout
pub struct Workout {
    pub id: i32,
    pub time_completed: i32
}