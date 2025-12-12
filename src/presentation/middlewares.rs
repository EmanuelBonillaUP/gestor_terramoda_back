use std::sync::Arc;

use actix_web::{
    Error, HttpResponse,
    body::{BoxBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web::Data,
};

use crate::{
    application::services::CredentialsValidator, infrastructure::DependenciesResolver,
    shared::Resolver,
};

pub async fn loggin_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let method = req.method().to_string();
    let path = req.path().to_string();
    let url = req.uri().to_string();
    let logger: Arc<dyn crate::application::services::Logger + Send + Sync> = req
        .app_data::<Data<Arc<DependenciesResolver>>>()
        .map(|d| d.as_ref())
        .unwrap()
        .resolve();
    logger.debug(format!("method: {}, path: {}, url: {}", method, path, url).as_str());
    let result = next.call(req).await;
    if let Ok(r) = &result {
        logger.debug(
            format!(
                "request(method: {}, path: {}), response(status: {})",
                method,
                path,
                r.status().as_u16()
            )
            .as_str(),
        );
    }
    result
}

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let key = match req.headers().get("API_KEY") {
        Some(k) => match k.to_str() {
            Ok(s) => s,
            Err(_) => return Ok(req.into_response(HttpResponse::Unauthorized())),
        },
        None => return Ok(req.into_response(HttpResponse::Unauthorized())),
    };
    let credentials_validator: Arc<dyn CredentialsValidator + Send + Sync> = req
        .app_data::<Data<Arc<DependenciesResolver>>>()
        .map(|d| d.as_ref())
        .unwrap()
        .resolve();
    if let Err(e) = credentials_validator.has_access(&key.to_string()) {
        return Ok(req.into_response(HttpResponse::from(e)));
    }
    next.call(req).await
}
