pub enum FilterOp {
    Eq(String),
    ILike(String),
}

pub trait Listable {
    const TABLE: &'static str;
    const COLUMNS: &'static [&'static str];

    /// columna, operación
    fn filters(&self) -> Vec<(&'static str, FilterOp)>;

    fn page(&self) -> Option<u32> {
        None
    }

    fn per_page(&self) -> Option<u32> {
        None
    }
}