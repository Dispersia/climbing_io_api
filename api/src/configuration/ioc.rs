use coi::{container, Container};
use data_access::{PostgresPoolProvider, TagRepositoryProvider};
use services::{ExerciseServiceProvider, TagServiceProvider, WorkoutServiceProvider};
use shared::SettingsProvider;

pub fn create_container() -> Container {
    container! {
        settings => SettingsProvider; singleton,
        database_pool => PostgresPoolProvider; singleton,

        tag_repository => TagRepositoryProvider; singleton,
        
        tag_service => TagServiceProvider; singleton,
        workout_service => WorkoutServiceProvider; singleton,
        exercise_service => ExerciseServiceProvider; singleton
    }
}
