use crate::{
    application::dtos::CustomerDTO,
    domain::value_objects::CC,
    shared::{
        SharedError,
        input_handler::{Input, InputHandler},
    },
};
use std::sync::Arc;

use crate::domain::repositories::CustomerRepository;

pub struct GetCustomerByCcQuery {
    pub cc: String,
}
impl Input for GetCustomerByCcQuery {
    type Output = CustomerDTO;
}
pub struct GetCustomerByCcQueryHandler {
    pub customer_repository: Arc<dyn CustomerRepository + Send + Sync>,
}
#[async_trait::async_trait]
impl InputHandler<GetCustomerByCcQuery> for GetCustomerByCcQueryHandler {
    async fn handle(
        &self,
        input: Arc<GetCustomerByCcQuery>,
    ) -> Result<<GetCustomerByCcQuery as Input>::Output, SharedError> {
        let cc = CC::new(input.cc.clone())?;
        let customer_option = self.customer_repository.get_by_cc(&cc).await;
        match customer_option {
            Some(customer) => Ok(CustomerDTO::from(customer)),
            None => Err(SharedError::new(
                &format!("Customer with CC {} not found", input.cc),
                404,
            )),
        }
    }
}
