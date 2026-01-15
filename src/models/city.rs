use sqlx::FromRow;
use sqlx::{Postgres, postgres::PgArguments};

use crate::repositories::insertable::Insertable;
use crate::repositories::updatable::Updatable;
use crate::repositories::deletable::Deletable;
use crate::repositories::listable::Listable;
use crate::repositories::listable::FilterOp;
use crate::resources::city_resource::CityResource;

#[derive(Debug, FromRow, serde::Serialize)]
pub struct City {
    pub id: i64,
    pub name: String,
    pub region: String
}
impl Listable for CityResource {
    const TABLE: &'static str = "cities";
    const COLUMNS: &'static [&'static str] = &["id", "name", "region"];

    fn filters(&self) -> Vec<(&'static str, FilterOp)> {
        let mut v = Vec::new();

        if let Some(name) = &self.name {
            v.push(("name", FilterOp::ILike(name.clone())));
        }

        if let Some(region) = &self.region {
            v.push(("region", FilterOp::Eq(region.clone())));
        }

        v
    }

    fn page(&self) -> Option<u32> {
        self.page
    }

    fn per_page(&self) -> Option<u32> {
        self.per_page
    }
}


impl Insertable for City {
    const TABLE: &'static str = "cities";
    const COLUMNS: &'static [&'static str] = &["name", "region"];

    fn bind<'q>(
        self,
        query: sqlx::query::QueryAs<'q, Postgres, Self, PgArguments>,
    ) -> sqlx::query::QueryAs<'q, Postgres, Self, PgArguments> {
        query
            .bind(self.name)
            .bind(self.region)
    }
}
impl Updatable for City {
    const TABLE: &'static str = "cities";
    const COLUMNS: &'static [&'static str] = &["name", "region"];
    const ID_COLUMN: &'static str = "id";

    fn bind_update<'q>(
        self,
        query: sqlx::query::QueryAs<'q, Postgres, Self, PgArguments>,
    ) -> sqlx::query::QueryAs<'q, Postgres, Self, PgArguments> {
        query
            .bind(self.name)
            .bind(self.region)
            .bind(self.id) // WHERE id = $n
    }
}

impl Deletable for City {
    const TABLE: &'static str = "cities";
    const ID_COLUMN: &'static str = "id";
}