pub trait Deletable {
    /// Tabla
    const TABLE: &'static str;

    /// Columna identificadora (normalmente id)
    const ID_COLUMN: &'static str;
}