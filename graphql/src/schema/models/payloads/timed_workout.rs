#[derive(juniper::GraphQLObject)]
/// The finished time for the [Exercise]
pub struct TimedWorkout {
    pub duration: i32,
}
