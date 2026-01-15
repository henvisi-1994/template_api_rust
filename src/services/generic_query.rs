use crate::repositories::deletable::Deletable;
use crate::repositories::insertable::Insertable;
use crate::repositories::listable::Listable;
use crate::repositories::listable::FilterOp;
use crate::repositories::updatable::Updatable;
use sqlx::FromRow;
use sqlx::{postgres::PgRow, Error, Pool, Postgres};

pub async fn generic_list<T, F>(
    filter: F,
    order_by: Option<(&'static str, &'static str)>,
    pool: &Pool<Postgres>,
) -> Result<Vec<T>, Error>
where
    T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
    F: Listable,
{
    let columns = F::COLUMNS.join(", ");
    let mut query = format!("SELECT {} FROM {}", columns, F::TABLE);

    let filters = filter.filters();
    let mut bind_index = 1;
    let mut binds = Vec::new();

    if !filters.is_empty() {
        query.push_str(" WHERE ");

        for (i, (col, op)) in filters.iter().enumerate() {
            if i > 0 {
                query.push_str(" AND ");
            }

            match op {
                FilterOp::Eq(val) => {
                    query.push_str(&format!("{} = ${}", col, bind_index));
                    binds.push(val.clone());
                }
                FilterOp::ILike(val) => {
                    query.push_str(&format!("{} ILIKE ${}", col, bind_index));
                    binds.push(format!("%{}%", val));
                }
            }

            bind_index += 1;
        }
    }

    if let Some((col, dir)) = order_by {
        let d = if dir.eq_ignore_ascii_case("DESC") {
            "DESC"
        } else {
            "ASC"
        };
        query.push_str(&format!(" ORDER BY {} {}", col, d));
    }

    // PAGINACIÓN
    if let (Some(page), Some(per_page)) = (filter.page(), filter.per_page()) {
        let offset = (page.saturating_sub(1) * per_page) as i64;

        query.push_str(&format!(
            " LIMIT {} OFFSET {}",
            per_page, offset
        ));
    }

    let mut q = sqlx::query_as::<_, T>(&query).persistent(false);

    for v in binds {
        q = q.bind(v);
    }

    q.fetch_all(pool).await
}

pub async fn generic_insert<T>(value: T, pool: &Pool<Postgres>) -> Result<T, Error>
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

pub async fn generic_update<T>(value: T, pool: &Pool<Postgres>) -> Result<T, Error>
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
pub async fn generic_delete<T, ID>(id: ID, pool: &Pool<Postgres>) -> Result<T, Error>
where
    T: Deletable + for<'r> FromRow<'r, PgRow> + Send + Unpin,
    for<'q> ID: Send + Sync + sqlx::Encode<'q, Postgres> + sqlx::Type<Postgres>,
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
