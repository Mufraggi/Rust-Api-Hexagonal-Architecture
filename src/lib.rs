use chrono::{DateTime, NaiveDate};
use sqlx::postgres::PgPool;
use sqlx::{Pool, Postgres, query};
use uuid::Uuid;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};


pub enum InsertError {
    Conflict,
    Unknown,
}

pub enum FetchAllError {
    Unknown,
}

pub enum FetchOneError {
    NotFound,
    Unknown,
}

pub enum DeleteError {
    NotFound,
    Unknown,
}
//todo matez ca https://stackoverflow.com/questions/30389043/how-are-you-able-to-create-partially-initialised-structs
pub struct PostgresRepository {
    db_pool: Pool<Postgres>,
}

#[derive(Deserialize, Serialize)]
pub struct DbUser {
    id: Uuid,
    first_name: String,
    last_name: String,
    birthday_date: NaiveDate,
    city: String,
}

impl PostgresRepository {
    pub async  fn new(mut self, url_db: String) -> Result<(), String> {
        let tmp = PgPool::connect(&url_db).await;
        match tmp {
            Ok(value) => {
                self.db_pool = value;
                Ok(())
            }
            Err(err) => {
                Err("blabla".to_string())
            }
        }
    }
}

#[async_trait]
pub trait Repository {
    async fn insert(
        &self,
        user: DbUser,
    ) -> anyhow::Result<DbUser, InsertError>;
    async fn fetch_all(&self) -> anyhow::Result<Vec<DbUser>, FetchAllError>;

    async fn fetch_one(&self, number: u32) -> anyhow::Result<DbUser, FetchOneError>;
    async fn update(&self, id: String, new_db_user: DbUser) -> anyhow::Result<DbUser, FetchAllError>;
    async fn delete(&self, number: u32) -> anyhow::Result<(), DeleteError>;
}

#[async_trait]
impl Repository for PostgresRepository {
    async fn insert(&self, db_user: DbUser) -> anyhow::Result<DbUser, InsertError> {
        let rec = query!(
        r#"
INSERT INTO  users (id, first_name, last_name, birthday_date, city)
        VALUES ( $1, $2, $3, $4, $5)
        "#,
            db_user.id.to_string(),
        db_user.first_name,
            db_user.last_name,
                db_user.birthday_date,
                db_user.city
    )
            .fetch_one(&self.db_pool)
            .await;
        Ok(db_user)
    }

    async fn fetch_all(&self) -> anyhow::Result<Vec<DbUser>, FetchAllError> {
        todo!()
    }

    async fn fetch_one(&self, number: u32) -> anyhow::Result<DbUser, FetchOneError> {
        todo!()
    }

    async fn update(&self, id: String, new_db_user: DbUser) -> anyhow::Result<DbUser, FetchAllError> {
        todo!()
    }

    async fn delete(&self, number: u32) -> anyhow::Result<(), DeleteError> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use uuid::Uuid;
    use crate::{DbUser, PostgresRepository};
    use random_string::generate;

    #[test]
    fn create_works() {
        let charset = "abcdefghijkl";
        let user = DbUser{
            id: Uuid::new_v4(),
            last_name: generate(6, charset),
            first_name:generate(6, charset),
            city: generate(6, charset),
            birthday_date: NaiveDate::from_ymd(2015, 3, 14),
        };
       //let repo = PostgresRepository::new("");

    }
}
