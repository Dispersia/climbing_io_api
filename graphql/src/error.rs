use juniper::graphql_value;

pub enum GraphQLError {
    GenericFailure,
}

impl juniper::IntoFieldError for GraphQLError {
    fn into_field_error(self) -> juniper::FieldError {
        match self {
            GraphQLError::GenericFailure => juniper::FieldError::new(
                "Unable to successfully retrieve tags",
                graphql_value!({
                    "internal_error": "FAILURE_PROCESSING"
                }),
            ),
        }
    }
}
