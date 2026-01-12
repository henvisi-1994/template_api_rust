use sqlx::{Pool, Postgres, Error, postgres::PgRow};
use sqlx::FromRow;
use crate::repositories::insertable::Insertable;
use crate::repositories::updatable::Updatable;
use crate::repositories::deletable::Deletable;
pub async fn generic_insert<T>(
    value: T,
    pool: &Pool<Postgres>,
) -> Result<T, Error>
where
    T: Insertable + for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
{
    let columns = T::COLUMNS.join(", ");

    let placeholders = (1..=T::COLUMNS.len())
        .map(|i| format!("${}", i))
        .collect::<Vec<_>>()
        .join(", ");

    let query = format!(
        "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
        T::TABLE,
        columns,
        placeholders
    );

    let q = sqlx::query_as::<_, T>(&query);

    let q = value.bind(q);

    q.fetch_one(pool).await
}

pub async fn generic_list<'a, T>(
    table: &str,
    columns: &[&str],
    order_by: Option<(&str, &str)>,
    pool: &'a Pool<Postgres>,
) -> Result<Vec<T>, Error>
where
    T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
{
    // Construimos la lista de columnas manualmente (sin dependencias externas)
    let mut columns_str = String::new();
    for (i, &col) in columns.iter().enumerate() {
        if i > 0 {
            columns_str.push_str(", ");
        }
        columns_str.push_str(col);
    }

    // Query base
    let mut query = format!("SELECT {} FROM {}", columns_str, table);

    // ORDER BY opcional
    if let Some((column, direction)) = order_by {
        let dir = if direction.eq_ignore_ascii_case("DESC") {
            "DESC"
        } else {
            "ASC"
        };
        query.push_str(&format!(" ORDER BY {} {}", column, dir));
    }

    // Ejecutamos
sqlx::query_as::<_, T>(&query)
    .persistent(false)
    .fetch_all(pool)
    .await
}
pub async fn generic_update<T>(
    value: T,
    pool: &Pool<Postgres>,
) -> Result<T, Error>
where
    T: Updatable + for<'r> FromRow<'r, PgRow> + Send + Unpin,
{
    let set_clause = T::COLUMNS
        .iter()
        .enumerate()
        .map(|(i, col)| format!("{} = ${}", col, i + 1))
        .collect::<Vec<_>>()
        .join(", ");

    let where_index = T::COLUMNS.len() + 1;

    let query = format!(
        "UPDATE {} SET {} WHERE {} = ${} RETURNING *",
        T::TABLE,
        set_clause,
        T::ID_COLUMN,
        where_index
    );

    let q = sqlx::query_as::<_, T>(&query);
    let q = value.bind_update(q);

    q.fetch_one(pool).await
}
pub async fn generic_delete<T, ID>(
    id: ID,
    pool: &Pool<Postgres>,
) -> Result<T, Error>
where
    T: Deletable + for<'r> FromRow<'r, PgRow> + Send + Unpin,
    for<'q> ID: Send
        + Sync
        + sqlx::Encode<'q, Postgres>
        + sqlx::Type<Postgres>,
{
    let query = format!(
        "DELETE FROM {} WHERE {} = $1 RETURNING *",
        T::TABLE,
        T::ID_COLUMN
    );

    sqlx::query_as::<_, T>(query.as_str())
        .bind(id)
        .fetch_one(pool)
        .await
}
