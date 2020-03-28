#[derive(juniper::GraphQLObject)]
/// The finished time for the [Exercise]
pub struct TimedWorkout {
    pub id: juniper::ID,
    pub duration: i32,
}
