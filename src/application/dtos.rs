use crate::domain::{
    entities::{Customer, Product, Sale},
    value_objects::ValueObject,
};

#[derive(Debug, Clone)]
pub struct CustomerDTO {
    pub id: u32,
    pub cc: String,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub direction: Option<String>,
}

impl From<Customer> for CustomerDTO {
    fn from(customer: Customer) -> Self {
        CustomerDTO {
            id: customer.id(),
            cc: customer.cc().value().clone(),
            name: customer.name().clone(),
            email: customer.email().value().clone(),
            phone: customer.phone().as_ref().map(|p| p.value().clone()),
            direction: customer.direction().clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProductDTO {
    pub id: u32,
    pub sku: String,
    pub name: String,
    pub price: f64,
    pub stock: u32,
    pub flags: Vec<String>,
    pub img_url: Option<String>,
    pub description: Option<String>,
}

impl From<Product> for ProductDTO {
    fn from(product: Product) -> Self {
        ProductDTO {
            id: product.id(),
            sku: product.sku().clone(),
            name: product.name().clone(),
            price: product.price(),
            stock: product.stock(),
            flags: product.flags().clone(),
            img_url: product.img_url().as_ref().map(|url| url.value().clone()),
            description: product.description().clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SaleDTO {
    pub id: u32,
    pub customer: CustomerDTO,
    pub products: Vec<(ProductDTO, u32)>,
    pub total_amount: f64,
    pub generated_at: String,
}

impl From<Sale> for SaleDTO {
    fn from(sale: Sale) -> Self {
        SaleDTO {
            id: sale.id(),
            customer: CustomerDTO::from(sale.customer().clone()),
            products: sale
                .products_sale()
                .iter()
                .map(|(product, quantity)| (ProductDTO::from(product.clone()), *quantity as u32))
                .collect(),
            total_amount: sale.total_amount(),
            generated_at: sale.generated_at().to_rfc3339(),
        }
    }
}
