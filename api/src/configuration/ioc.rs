use coi::{container, Container};
use services::{ExerciseServiceProvider, TagServiceProvider, WorkoutServiceProvider};
use shared::ConfigProvider;

pub fn create_container() -> Container {
    container! {
        config => ConfigProvider; singleton,
        tag_service => TagServiceProvider; singleton,
        workout_service => WorkoutServiceProvider; singleton,
        exercise_service => ExerciseServiceProvider; singleton
    }
}
