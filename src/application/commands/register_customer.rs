use crate::domain::value_objects::{CC, Email, Phone};
use crate::shared::input_handler::{Input, InputHandler};
use crate::shared::SharedError;
use crate::domain::repositories::CustomerRepository;
use std::sync::Arc;

pub struct RegisterCustomerCommand {
    pub cc: String,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub direction: Option<String>,
}
pub struct RegisterCustomerOutput {
    pub id: u32,
}
impl Input for RegisterCustomerCommand {
    type Output = RegisterCustomerOutput;
}
pub struct RegisterCustomerCommandHandler {
    pub customer_repository: Arc<dyn CustomerRepository + Send + Sync>,
}
#[async_trait::async_trait]
impl InputHandler<RegisterCustomerCommand> for RegisterCustomerCommandHandler {
    async fn handle(
        &self,
        input: Arc<RegisterCustomerCommand>,
    ) -> Result<<RegisterCustomerCommand as Input>::Output, SharedError> {
        let cc = CC::new(input.cc.clone())?;
        let email = Email::new(input.email.clone())?;
        let phone = match &input.phone {
            Some(p) => Some(Phone::new(p.clone())?),
            None => None,
        };
        let existing_customer = self.customer_repository.get_by_cc(&cc).await;
        if let Some(_) = existing_customer {
            return Err(SharedError::new(
                format!("Customer with CC {} already exists", input.cc).as_str(),
                400,
            ));
        }
        let customer = self
            .customer_repository
            .create(&cc, &input.name, &email, phone.as_ref(), input.direction.as_deref())
            .await?;
        Ok(RegisterCustomerOutput{
            id: customer.id(),
        })
    }
}

