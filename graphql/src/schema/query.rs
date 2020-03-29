use super::context::Context;
use juniper::FieldResult;
use services::{ExerciseServiceTrait, TagServiceTrait};

use super::models::{Exercise, Tag};

pub struct Query;

#[juniper::graphql_object(
    Context = Context,
)]
impl Query {
    async fn tags(context: &Context) -> FieldResult<Vec<Tag>> {
        let tag_service = context.resolve::<dyn TagServiceTrait>("tag_service");

        let tags: Vec<Tag> = tag_service
            .get_tags()
            .await
            .into_iter()
            .map(|x| x.into())
            .collect();

        Ok(tags)
    }

    async fn exercise(context: &Context, id: juniper::ID) -> FieldResult<Exercise> {
        let exercise_service = context.resolve::<dyn ExerciseServiceTrait>("exercise_service");
        let id = id.parse::<i32>()?;

        let exercise = exercise_service.get_exercise(id).await.into();
        Ok(exercise)
    }
}
