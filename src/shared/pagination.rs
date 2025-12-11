#[derive(Debug, Clone, Copy)]
pub struct Pagination {
    pub page: u32,
    pub per_page: u32,
}

pub struct PaginationResult<T> {
    pub items: Vec<T>,
    pub items_count: u32,
    pub total_items: u32,
    pub current_page: u32,
    pub per_page: u32,
}

impl<T> From<(&Pagination, u32)> for PaginationResult<T> {
    fn from(pagination: (&Pagination, u32)) -> Self {
        let (pagination, total_items) = pagination;
        PaginationResult {
            items: vec![],
            items_count: 0,
            total_items: total_items,
            current_page: pagination.page,
            per_page: pagination.per_page,
        }
    }
}
impl PaginationResult<()> {
    pub fn with_data<T>(&self, items: Vec<T>) -> PaginationResult<T> {
        let items_count = items.len() as u32;
        PaginationResult {
            items,
            items_count,
            total_items: self.total_items,
            current_page: self.current_page,
            per_page: self.per_page,
        }
    }

    pub fn from_other<TSelf: From<TOther>, TOther: Clone>(
        other: &PaginationResult<TOther>,
    ) -> PaginationResult<TSelf> {
        PaginationResult {
            items: other.items.iter().map(|i| TSelf::from(i.clone())).collect(),
            items_count: other.items_count,
            total_items: other.total_items,
            current_page: other.current_page,
            per_page: other.per_page,
        }
    }
}
