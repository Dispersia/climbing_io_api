use coi::{container, Container};
use data_access::{create_pool_provider, TagRepositoryProvider};
use services::{ExerciseServiceProvider, TagServiceProvider, WorkoutServiceProvider};
use shared::ConfigProvider;

pub fn create_container() -> Container {
    container! {
        config => ConfigProvider; singleton,
        database_pool => create_pool_provider(); singleton,
        tag_repository => TagRepositoryProvider; singleton,
        tag_service => TagServiceProvider; singleton,
        workout_service => WorkoutServiceProvider; singleton,
        exercise_service => ExerciseServiceProvider; singleton
    }
}
