use sqlx::{Postgres, postgres::PgArguments};

pub trait Insertable {
    const TABLE: &'static str;
    const COLUMNS: &'static [&'static str];

    fn bind<'q>(
        self,
        query: sqlx::query::QueryAs<'q, Postgres, Self, PgArguments>,
    ) -> sqlx::query::QueryAs<'q, Postgres, Self, PgArguments>
    where
        Self: Sized;
}
