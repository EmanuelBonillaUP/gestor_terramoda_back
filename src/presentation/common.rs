use actix_web::{HttpResponse, http::StatusCode};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct GenericError {
    message: String,
}

impl From<crate::shared::SharedError> for HttpResponse {
    fn from(value: crate::shared::SharedError) -> Self {
        let status = StatusCode::from_u16(value.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        HttpResponse::build(status).json(GenericError {
            message: value.message,
        })
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Pagination {
    pub page: u32,
    pub per_page: u32,
}

impl Into<crate::shared::Pagination> for Pagination {
    fn into(self) -> crate::shared::Pagination {
        crate::shared::Pagination {
            per_page: self.per_page,
            page: self.page,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct PaginationResult<T: Serialize + Clone> {
    pub items: Vec<T>,
    pub items_count: u32,
    pub total_items: u32,
    pub current_page: u32,
    pub per_page: u32,
}

impl<TSelf: Serialize + Clone + From<TOther>, TOther: Clone>
    From<crate::shared::PaginationResult<TOther>> for PaginationResult<TSelf>
{
    fn from(value: crate::shared::PaginationResult<TOther>) -> Self {
        PaginationResult {
            per_page: value.per_page,
            total_items: value.total_items,
            current_page: value.current_page,
            items_count: value.items_count,
            items: value.items.iter().map(|i| TSelf::from(i.clone())).collect(),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct CustomerResponse {
    pub id: u32,
    pub cc: String,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub direction: Option<String>,
}
impl From<crate::application::dtos::CustomerDTO> for CustomerResponse {
    fn from(value: crate::application::dtos::CustomerDTO) -> Self {
        CustomerResponse {
            id: value.id,
            cc: value.cc,
            name: value.name,
            email: value.email,
            phone: value.phone,
            direction: value.direction,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct ProductResponse {
    pub id: u32,
    pub sku: String,
    pub name: String,
    pub price: f64,
    pub stock: u32,
    pub flags: Vec<String>,
    pub img_url: Option<String>,
    pub description: Option<String>,
}
impl From<crate::application::dtos::ProductDTO> for ProductResponse {
    fn from(value: crate::application::dtos::ProductDTO) -> Self {
        ProductResponse {
            id: value.id,
            sku: value.sku,
            name: value.name,
            price: value.price,
            stock: value.stock,
            flags: value.flags,
            img_url: value.img_url,
            description: value.description,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct ProductSaleResponse {
    pub product: ProductResponse,
    pub quantity: u32,
}

#[derive(Serialize, Debug, Clone)]
pub struct SaleResponse {
    pub id: u32,
    pub customer: CustomerResponse,
    pub products: Vec<ProductSaleResponse>,
    pub total_amount: f64,
    pub generated_at: String,
}
impl From<crate::application::dtos::SaleDTO> for SaleResponse {
    fn from(value: crate::application::dtos::SaleDTO) -> Self {
        SaleResponse {
            id: value.id,
            customer: CustomerResponse::from(value.customer),
            products: value
                .products
                .into_iter()
                .map(|(prod, qty)| ProductSaleResponse {
                    product: ProductResponse::from(prod),
                    quantity: qty,
                })
                .collect(),
            total_amount: value.total_amount,
            generated_at: value.generated_at,
        }
    }
}
