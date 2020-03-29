use super::{Exercise, Tag};
use async_trait::async_trait;
use coi::Inject;

#[derive(Inject)]
#[coi(provides pub dyn ExerciseServiceTrait with ExerciseService)]
pub struct ExerciseService;

#[async_trait]
impl ExerciseServiceTrait for ExerciseService {
    async fn get_exercise(&self, id: i32) -> Exercise {
        Exercise {
            id: id,
            name: "Example Exercise".to_string(),
        }
    }

    async fn get_exercises_by_tags(&self, _tags: &[Tag]) -> Vec<Exercise> {
        vec![
            Exercise {
                id: 0,
                name: "Exercise 1".to_string(),
            },
            Exercise {
                id: 1,
                name: "Exercise 2".to_string(),
            },
        ]
    }
}

#[async_trait]
pub trait ExerciseServiceTrait: Inject {
    async fn get_exercise(&self, id: i32) -> Exercise;
    async fn get_exercises_by_tags(&self, types: &[Tag]) -> Vec<Exercise>;
}
