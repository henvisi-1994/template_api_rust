use sqlx::{Postgres, postgres::PgArguments};

pub trait Updatable {
    /// Tabla
    const TABLE: &'static str;

    /// Columnas a actualizar (SIN id)
    const COLUMNS: &'static [&'static str];

    /// Columna WHERE (ej: id)
    const ID_COLUMN: &'static str;

    /// Binds de columnas + id (AL FINAL)
    fn bind_update<'q>(
        self,
        query: sqlx::query::QueryAs<'q, Postgres, Self, PgArguments>,
    ) -> sqlx::query::QueryAs<'q, Postgres, Self, PgArguments>
    where
        Self: Sized;
}
