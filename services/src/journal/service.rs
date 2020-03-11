use async_trait::async_trait;
use coi::Inject;

#[async_trait]
pub trait JournalServiceTrait: Inject {
    async fn test_get(&self) -> String;
}

#[derive(Inject)]
#[coi(provides pub dyn JournalServiceTrait with JournalService)]
pub struct JournalService;

#[async_trait]
impl JournalServiceTrait for JournalService {
    async fn test_get(&self) -> String {
        "Aaron Housh - Async".to_string()
    }
}
