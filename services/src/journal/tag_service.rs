use super::Tag;
use async_trait::async_trait;
use coi::Inject;
#[derive(Inject)]
#[coi(provides pub dyn TagServiceTrait with TagService)]
pub struct TagService;

#[async_trait]
impl TagServiceTrait for TagService {
    async fn get_tag(&self, id: i32) -> Tag {
        Tag {
            id: id,
            name: "A Tag".to_string(),
        }
    }

    async fn get_tags(&self) -> Vec<Tag> {
        vec![
            Tag {
                id: 0,
                name: "Tag 1".to_string(),
            },
            Tag {
                id: 1,
                name: "Tag 2".to_string(),
            },
        ]
    }

    async fn get_tags_by_exercise(&self, exercise_id: i32) -> Vec<Tag> {
        vec![
            Tag {
                id: exercise_id * 10 + 1,
                name: "Tag1".to_string(),
            },
            Tag {
                id: exercise_id * 10 + 2,
                name: "Tag2".to_string(),
            },
        ]
    }
}

#[async_trait]
pub trait TagServiceTrait: Inject {
    async fn get_tag(&self, id: i32) -> Tag;
    async fn get_tags(&self) -> Vec<Tag>;
    async fn get_tags_by_exercise(&self, exercise_id: i32) -> Vec<Tag>;
}
