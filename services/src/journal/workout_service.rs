use async_trait::async_trait;
use coi::Inject;

#[derive(Inject)]
#[coi(provides pub dyn WorkoutServiceTrait with WorkoutService)]
pub struct WorkoutService;

#[async_trait]
impl WorkoutServiceTrait for WorkoutService {
    async fn test_get(&self) -> String {
        "Aaron Housh - Async".to_string()
    }
}

#[async_trait]
pub trait WorkoutServiceTrait: Inject {
    async fn test_get(&self) -> String;
}
