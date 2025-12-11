use crate::{
    application::services::CredentialsValidator,
    shared::{
        SharedError,
        input_handler::{Input, InputHandler},
    },
};
use std::sync::Arc;

pub struct LoginQuery {
    pub user: String,
    pub pass: String,
}
pub struct LoginOutput {
    pub key: String,
}
impl Input for LoginQuery {
    type Output = LoginOutput;
}

pub struct LoginQueryHandler {
    pub credentials_validator: Arc<dyn CredentialsValidator + Send + Sync>,
}
#[async_trait::async_trait]
impl InputHandler<LoginQuery> for LoginQueryHandler {
    async fn handle(
        &self,
        query: Arc<LoginQuery>,
    ) -> Result<<LoginQuery as Input>::Output, SharedError> {
        return self
            .credentials_validator
            .validate(&query.user, &query.pass)
            .map(|r| LoginOutput { key: r });
    }
}
