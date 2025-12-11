mod mysql_repositories;
mod services;

use actix_web::web::{Data, ServiceConfig};
use std::{
    env,
    sync::{Arc, LazyLock},
};

use crate::{
    application::{
        commands, queries,
        services::{CredentialsValidator, LogLevel, Logger},
    },
    domain::repositories::{CustomerRepository, ProductRepository, SaleRepository},
    shared::{Resolver, input_handler::InputHandler, input_handler::Sender},
};

pub struct DependenciesResolver;

static RESOLVER: LazyLock<Arc<DependenciesResolver>> =
    LazyLock::new(|| Arc::new(DependenciesResolver));

impl Resolver<dyn Logger + Send + Sync> for DependenciesResolver {
    fn resolve(&self) -> Arc<dyn Logger + Send + Sync> {
        let logger_config = services::LoggerConfig {
            min_level: LogLevel::Debug,
            format: "{timestamp}|{level}|{message}".to_string(),
        };
        Arc::new(services::LoggerPrinter::new(logger_config))
    }
}

static POOL_DB: LazyLock<Arc<mysql_repositories::GuardPool>> =
    LazyLock::new(|| Arc::new(mysql_repositories::GuardPool::new()));

impl Resolver<mysql_repositories::GuardPool> for DependenciesResolver {
    fn resolve(&self) -> Arc<mysql_repositories::GuardPool> {
        POOL_DB.clone()
    }
}

impl Resolver<sqlx::MySqlPool> for DependenciesResolver {
    fn resolve(&self) -> Arc<sqlx::MySqlPool> {
        POOL_DB.get()
    }
}

static CUSTOMER_REPOSITORY: LazyLock<Arc<dyn CustomerRepository + Send + Sync>> =
    LazyLock::new(|| {
        Arc::new(mysql_repositories::MysqlCustomerRepository {
            pool: RESOLVER.resolve(),
            logger: RESOLVER.resolve(),
        })
    });

impl Resolver<dyn CustomerRepository + Send + Sync> for DependenciesResolver {
    fn resolve(&self) -> Arc<dyn CustomerRepository + Send + Sync> {
        CUSTOMER_REPOSITORY.clone()
    }
}

static PRODUCT_REPOSITORY: LazyLock<Arc<dyn ProductRepository + Send + Sync>> =
    LazyLock::new(|| {
        Arc::new(mysql_repositories::MySQLProductRepository {
            pool: RESOLVER.resolve(),
            logger: RESOLVER.resolve(),
        })
    });

impl Resolver<dyn ProductRepository + Send + Sync> for DependenciesResolver {
    fn resolve(&self) -> Arc<dyn ProductRepository + Send + Sync> {
        PRODUCT_REPOSITORY.clone()
    }
}

static SALE_REPOSITORY: LazyLock<Arc<dyn SaleRepository + Send + Sync>> = LazyLock::new(|| {
    Arc::new(mysql_repositories::MySqlSaleRepository {
        pool: RESOLVER.resolve(),
        logger: RESOLVER.resolve(),
        customer_repository: RESOLVER.resolve(),
        product_repository: RESOLVER.resolve(),
    })
});
impl Resolver<dyn SaleRepository + Send + Sync> for DependenciesResolver {
    fn resolve(&self) -> Arc<dyn SaleRepository + Send + Sync> {
        SALE_REPOSITORY.clone()
    }
}
static CREDENTIALS_VALIDATOR: LazyLock<Arc<dyn CredentialsValidator + Send + Sync>> =
    LazyLock::new(|| {
        let config = services::CredentialsValidatorOneUserConfig {
            key: env::var("API_SECRET").unwrap_or("SUPER_SECRET_XD".to_string()),
            user: env::var("API_USER").unwrap_or("username".to_string()),
            pass: env::var("API_PASS").unwrap_or("password".to_string()),
        };
        Arc::new(services::CredentialsValidatorOneUser { config: config })
    });

impl Resolver<dyn CredentialsValidator + Send + Sync> for DependenciesResolver {
    fn resolve(&self) -> Arc<dyn CredentialsValidator + Send + Sync> {
        CREDENTIALS_VALIDATOR.clone()
    }
}

pub struct Mediator;

impl Sender<commands::RegisterCustomerCommand> for Mediator {
    fn get_input_handler(
        &self,
    ) -> Arc<dyn InputHandler<commands::RegisterCustomerCommand> + Send + Sync> {
        Arc::new(commands::RegisterCustomerCommandHandler {
            customer_repository: RESOLVER.resolve(),
        })
    }
}

impl Sender<commands::RegisterSaleCommand> for Mediator {
    fn get_input_handler(
        &self,
    ) -> Arc<dyn InputHandler<commands::RegisterSaleCommand> + Send + Sync> {
        Arc::new(commands::RegisterSaleCommandHandler {
            customer_repository: RESOLVER.resolve(),
            product_repository: RESOLVER.resolve(),
            sale_repository: RESOLVER.resolve(),
        })
    }
}
impl Sender<commands::RegisterProductCommand> for Mediator {
    fn get_input_handler(
        &self,
    ) -> Arc<dyn InputHandler<commands::RegisterProductCommand> + Send + Sync> {
        Arc::new(commands::RegisterProductCommandHandler {
            product_repository: RESOLVER.resolve(),
        })
    }
}

impl Sender<commands::EditCustomerCommand> for Mediator {
    fn get_input_handler(
        &self,
    ) -> Arc<dyn InputHandler<commands::EditCustomerCommand> + Send + Sync> {
        Arc::new(commands::EditCustomerCommandHandler {
            customer_repository: RESOLVER.resolve(),
        })
    }
}

impl Sender<commands::EditProductCommand> for Mediator {
    fn get_input_handler(
        &self,
    ) -> Arc<dyn InputHandler<commands::EditProductCommand> + Send + Sync> {
        Arc::new(commands::EditProductCommandHandler {
            product_repository: RESOLVER.resolve(),
        })
    }
}

impl Sender<queries::GenerateCsvReportQuery> for Mediator {
    fn get_input_handler(
        &self,
    ) -> Arc<dyn InputHandler<queries::GenerateCsvReportQuery> + Send + Sync> {
        Arc::new(queries::GenerateCsvReportQueryHandler {
            sale_repository: RESOLVER.resolve(),
        })
    }
}

impl Sender<queries::GetCustomersQuery> for Mediator {
    fn get_input_handler(&self) -> Arc<dyn InputHandler<queries::GetCustomersQuery> + Send + Sync> {
        Arc::new(queries::GetCustomersQueryHandler {
            customer_repository: RESOLVER.resolve(),
        })
    }
}
impl Sender<queries::GetProductsQuery> for Mediator {
    fn get_input_handler(&self) -> Arc<dyn InputHandler<queries::GetProductsQuery> + Send + Sync> {
        Arc::new(queries::GetProductsQueryHandler {
            product_repository: RESOLVER.resolve(),
        })
    }
}
impl Sender<queries::GetSalesQuery> for Mediator {
    fn get_input_handler(&self) -> Arc<dyn InputHandler<queries::GetSalesQuery> + Send + Sync> {
        Arc::new(queries::GetSalesQueryHandler {
            sale_repository: RESOLVER.resolve(),
        })
    }
}
impl Sender<queries::LoginQuery> for Mediator {
    fn get_input_handler(&self) -> Arc<dyn InputHandler<queries::LoginQuery> + Send + Sync> {
        Arc::new(queries::LoginQueryHandler {
            credentials_validator: RESOLVER.resolve(),
        })
    }
}

pub async fn init() {
    POOL_DB.init().await;
    sqlx::migrate!("./migrations")
        .run(POOL_DB.get().as_ref())
        .await
        .expect("Failed to run database migrations");
}

pub fn configure(config: &mut ServiceConfig) {
    let mediator = Arc::new(Mediator {});
    let resolver = RESOLVER.clone();
    config.app_data(Data::new(mediator));
    config.app_data(Data::new(resolver));
}
