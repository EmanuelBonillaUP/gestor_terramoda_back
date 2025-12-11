use std::sync::Arc;

/// Resolver trait for resolving instances of type T
pub trait Resolver<T: ?Sized> {
    /// Resolve an instance of type T
    fn resolve(&self) -> Arc<T>;
}
