#[derive(juniper::GraphQLObject)]
/// The finished sets completed for an [Exercise]
pub struct RepitionWorkout {
    pub id: juniper::ID,
    pub repitions: i32,
    pub sets: i32,
}