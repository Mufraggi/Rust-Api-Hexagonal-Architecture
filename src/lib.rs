use std::fmt;
use std::fmt::Debug;
use std::future::Future;
use std::str::FromStr;
use chrono::{DateTime, NaiveDate};
use sqlx::postgres::PgPool;
use sqlx::{Pool, Postgres, query};
use uuid::Uuid;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
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

pub struct PostgresRepository {
    db_pool: Option<Pool<Postgres>>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct DbUser {
    id: Uuid,
    first_name: String,
    last_name: String,
    birthday_date: NaiveDate,
    city: String,
}

impl PostgresRepository {
    pub async  fn new_pool(url_db: &str) -> Result<PostgresRepository, ()> {
        let tmp = PgPool::connect(&url_db).await;
        match tmp {
            Ok(value) => {
                Ok(Self{db_pool:Some(value)})
            }
            Err(err) => {
                Err(())
            }
        }
    }
}

#[async_trait]
pub trait Repository {
    async fn insert(
        self,
        user: DbUser,
    ) -> anyhow::Result<DbUser, InsertError>;
    async fn fetch_all(&self) -> anyhow::Result<Vec<DbUser>, FetchAllError>;

    async fn fetch_one(&self, number: u32) -> anyhow::Result<DbUser, FetchOneError>;
    async fn update(&self, id: String, new_db_user: DbUser) -> anyhow::Result<DbUser, FetchAllError>;
    async fn delete(&self, number: u32) -> anyhow::Result<(), DeleteError>;
}

#[async_trait]
impl Repository for PostgresRepository {
    async fn insert(self, db_user: DbUser) -> anyhow::Result<DbUser, InsertError>
    {
        let db_pool = self.db_pool.as_ref().unwrap();
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
            .fetch_one(db_pool)
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
    use std::borrow::Borrow;
    use std::fmt::Debug;
    use std::str::FromStr;
    use chrono::NaiveDate;
    use uuid::Uuid;
    use crate::{DbUser, PostgresRepository, Repository};
    use random_string::generate;

    #[tokio::test]
    async fn create_works()
       {
        let charset = "abcdefghijkl";
        let user = DbUser{
            id: Uuid::new_v4(),
            last_name: generate(6, charset),
            first_name:generate(6, charset),
            city: generate(6, charset),
            birthday_date: NaiveDate::from_ymd(2015, 3, 14),
        };
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let repo = PostgresRepository::new_pool(url).await.unwrap();
        let res = repo.insert(user).await;
        let user_create = res.unwrap();
           let userRes = DbUser{
               id: Uuid::new_v4(),
               last_name: generate(6, charset),
               first_name:generate(6, charset),
               city: generate(6, charset),
               birthday_date: NaiveDate::from_ymd(2015, 3, 14),
           };
        assert_eq!(user_create.eq(&userRes), true)

    }
}
