use crate::error::Error;
use async_trait::async_trait;
use coi::Inject;
use data_access::TagRepositoryTrait;
use std::sync::Arc;

#[derive(Inject)]
#[coi(provides pub dyn TagServiceTrait with TagService::new(tag_repository))]
pub struct TagService {
    #[coi(inject)]
    tag_repository: Arc<dyn TagRepositoryTrait>,
}

#[async_trait]
impl TagServiceTrait for TagService {
    async fn get_tag(&self, id: i32) -> Tag {
        Tag {
            id: id,
            name: "A Tag".to_string(),
        }
    }

    async fn get_tags(&self) -> Result<Vec<Tag>, Error> {
        self.tag_repository
            .get_tags()
            .await
            .map(|x| x.into_iter().map(Into::into).collect())
            .map_err(|e| (Box::new(e) as Box<dyn std::error::Error + Send + Sync>).into())
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
    async fn get_tags(&self) -> Result<Vec<Tag>, Error>;
    async fn get_tags_by_exercise(&self, exercise_id: i32) -> Vec<Tag>;
}

impl TagService {
    fn new(tag_repository: Arc<dyn TagRepositoryTrait>) -> Self {
        Self { tag_repository }
    }
}

pub struct Tag {
    pub id: i32,
    pub name: String,
}

impl From<data_access::DbTag> for Tag {
    fn from(db_tag: data_access::DbTag) -> Self {
        Tag {
            id: db_tag.id,
            name: db_tag.name,
        }
    }
}
