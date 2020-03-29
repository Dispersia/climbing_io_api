#[derive(juniper::GraphQLObject)]
/// The finished sets completed for an [Exercise]
pub struct RepititionWorkout {
    pub repitions: i32,
    pub sets: i32,
}
