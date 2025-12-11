use std::sync::Arc;
use super::SharedError;

/// Input trait for inputs producing output of type Output
pub trait Input {
    type Output;
}


/// Handler of an Input type T, producing an output of type T::Output
#[async_trait::async_trait]
pub trait InputHandler<T: Input + Send + Sync> {
    async fn handle(&self, input: Arc<T>) -> Result<T::Output, SharedError>;
}


/// Sender for Input type I
#[async_trait::async_trait]
pub trait Sender<I: Input + Send + Sync + 'static> {

    /// Get the input handler for the input type I
    fn get_input_handler(&self) -> Arc<dyn InputHandler<I> + Send + Sync>;

    /// Send input and get output
    async fn send(&self, input: I) -> Result<I::Output, SharedError>{
        let input = Arc::new(input);
        let handler = self.get_input_handler();
        handler.handle(input).await
    }
}
