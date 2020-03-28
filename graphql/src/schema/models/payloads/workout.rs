use super::{RepititionWorkout, TimedWorkout};

#[derive(juniper::GraphQLObject)]
/// A completed workout
pub struct Workout {
    pub id: i32,
    pub time_completed: i32,
    pub repitition: RepititionWorkout,
    pub timed: TimedWorkout,
}
