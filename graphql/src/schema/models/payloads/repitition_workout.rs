#[derive(juniper::GraphQLObject)]
/// The finished sets completed for an [Exercise]
pub struct RepititionWorkout {
    pub id: juniper::ID,
    pub repitions: i32,
    pub sets: i32,
}
