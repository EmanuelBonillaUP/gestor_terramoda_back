use crate::domain::repositories::CustomerRepository;
use crate::domain::value_objects::{Email, Phone};
use crate::shared::SharedError;
use crate::shared::input_handler::{Input, InputHandler};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct EditCustomerCommand {
    pub customer_id: u32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub direction: Option<String>,
}
impl Input for EditCustomerCommand {
    type Output = ();
}
pub struct EditCustomerCommandHandler {
    pub customer_repository: Arc<dyn CustomerRepository + Send + Sync>,
}
#[async_trait::async_trait]
impl InputHandler<EditCustomerCommand> for EditCustomerCommandHandler {
    async fn handle(
        &self,
        input: Arc<EditCustomerCommand>,
    ) -> Result<<EditCustomerCommand as Input>::Output, SharedError> {
        let mut customer = match self.customer_repository.get_by_id(input.customer_id).await {
            Some(cust) => cust,
            None => {
                return Err(SharedError::new(
                    format!("Customer with ID {} not found", input.customer_id).as_str(),
                    404,
                ));
            }
        };
        if let Some(new_name) = &input.name {
            customer.set_name(new_name.clone());
        }
        if let Some(new_email) = &input.email {
            let email_vo = Email::new(new_email.clone())?;
            customer.set_email(email_vo);
        }
        if let Some(new_phone) = &input.phone {
            customer.set_phone(Some(Phone::new(new_phone.clone()).unwrap()));
        }
        if let Some(new_direction) = &input.direction {
            customer.set_direction(Some(new_direction.clone()));
        }
        let result = self.customer_repository.save(&customer).await;
        result
    }
}
