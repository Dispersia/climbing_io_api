use super::context::Context;
use crate::error::GraphQLError;
use juniper::FieldResult;
use services::{ExerciseServiceTrait, TagServiceTrait};

use super::models::{Exercise, Tag};

pub struct Query;

#[juniper::graphql_object(
    Context = Context,
)]
impl Query {
    async fn tags(context: &Context) -> Result<Vec<Tag>, GraphQLError> {
        let tag_service = context.resolve::<dyn TagServiceTrait>("tag_service");

        let tags: Vec<Tag> = tag_service
            .get_tags()
            .await
            .map(|x| x.into_iter().map(Into::into).collect())
            .map_err(|e| {
                log::error!("{}", e);
                GraphQLError::GenericFailure
            })?;

        Ok(tags)
    }

    async fn exercise(context: &Context, id: juniper::ID) -> FieldResult<Exercise> {
        let exercise_service = context.resolve::<dyn ExerciseServiceTrait>("exercise_service");
        let id = id.parse::<i32>()?;

        let exercise = exercise_service.get_exercise(id).await.into();
        Ok(exercise)
    }
}
