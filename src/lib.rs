use async_trait::async_trait;
use chrono::NaiveDate;
use futures::future::err;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgSeverity::Log;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::{query, query_as, Error, FromRow, Pool, Postgres, Row};
use std::any::TypeId;
use std::fmt::Debug;
use uuid::{uuid, Uuid};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum InsertError {
    Conflict,
    Unknown,
}

pub enum FetchAllError {
    Unknown,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
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
    id: String,
    first_name: String,
    last_name: String,
    birthday_date: NaiveDate,
    city: String,
}

impl<'r> FromRow<'r, PgRow> for DbUser {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id = row.try_get("id")?;
        let first_name = row.try_get("first_name")?;
        let last_name = row.try_get("last_name")?;
        let birthday_date = row.try_get("birthday_date")?;
        let city = row.try_get("city")?;
        Ok(DbUser {
            id,
            first_name,
            last_name,
            birthday_date,
            city,
        })
    }
}

impl PostgresRepository {
    pub async fn new_pool(url_db: &str) -> Result<PostgresRepository, ()> {
        let tmp = PgPool::connect(&url_db).await;
        match tmp {
            Ok(value) => Ok(Self {
                db_pool: Some(value),
            }),
            Err(err) => Err(()),
        }
    }
}

#[async_trait]
pub trait Repository {
    async fn insert(self, user: DbUser) -> anyhow::Result<DbUser, InsertError>;
    async fn fetch_all(&self) -> anyhow::Result<Vec<DbUser>, FetchAllError>;

    async fn get(&self, id: String) -> anyhow::Result<DbUser, FetchOneError>;
    async fn update(
        &self,
        id: String,
        new_db_user: DbUser,
    ) -> anyhow::Result<DbUser, FetchAllError>;
    async fn delete(&self, number: u32) -> anyhow::Result<(), DeleteError>;
}

#[async_trait]
impl Repository for PostgresRepository {
    async fn insert(self, db_user: DbUser) -> anyhow::Result<DbUser, InsertError> {
        let db_pool = self.db_pool.as_ref().unwrap();
        let rec = query!(
            r#"
INSERT INTO  users (id, first_name, last_name, birthday_date, city)
        VALUES ( $1, $2, $3, $4, $5) returning id
        "#,
            db_user.id.to_string(),
            db_user.first_name,
            db_user.last_name,
            db_user.birthday_date,
            db_user.city
        )
        .fetch_one(db_pool)
        .await;
        match rec {
            Ok(value) => Ok(db_user),
            Err(_) => Err(InsertError::Conflict),
        }
    }

    async fn fetch_all(&self) -> anyhow::Result<Vec<DbUser>, FetchAllError> {
        todo!()
    }

    async fn get(&self, id: String) -> anyhow::Result<DbUser, FetchOneError> {
        let db_pool = self.db_pool.as_ref().unwrap();
        let rec = query_as::<_, DbUser>("SELECT id, first_name, last_name, birthday_date, city FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(db_pool)
            .await;
        match rec {
            Ok(value) => {
                let res: DbUser = DbUser {
                    id: value.id,
                    last_name: value.last_name,
                    first_name: value.first_name,
                    city: value.city,
                    birthday_date: value.birthday_date,
                };
                Ok(res)
            }
            Err(_) => Err(FetchOneError::NotFound),
        }
    }

    async fn update(
        &self,
        id: String,
        new_db_user: DbUser,
    ) -> anyhow::Result<DbUser, FetchAllError> {
        todo!()
    }

    async fn delete(&self, number: u32) -> anyhow::Result<(), DeleteError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{DbUser, FetchOneError, InsertError, PostgresRepository, Repository};
    use chrono::NaiveDate;
    use random_string::generate;
    use std::borrow::Borrow;
    use uuid::{uuid, Uuid};

    #[tokio::test]
    async fn create_works() {
        let charset = "abcdefghijkl";
        let user = DbUser {
            id: Uuid::new_v4().to_string(),
            last_name: generate(6, charset),
            first_name: generate(6, charset),
            city: generate(6, charset),
            birthday_date: NaiveDate::from_ymd(2015, 3, 14),
        };
        let user_res = DbUser {
            id: user.id.clone(),
            last_name: user.last_name.clone(),
            first_name: user.first_name.clone(),
            city: user.city.clone(),
            birthday_date: NaiveDate::from_ymd(2015, 3, 14),
        };
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let repo = PostgresRepository::new_pool(url).await.unwrap();
        let res = repo.insert(user).await;
        let user_create = res.unwrap();
        println!("{}", Uuid::new_v4().to_string());
        assert_eq!(user_create.eq(&user_res), true)
    }

    #[tokio::test]
    async fn create_fail() {
        let charset = "abcdefghijkl";
        let user = DbUser {
            id: uuid!("0a708f88-bedb-4dad-a2f2-65dd4e8c132a").to_string(),
            last_name: generate(6, charset),
            first_name: generate(6, charset),
            city: generate(6, charset),
            birthday_date: NaiveDate::from_ymd(2015, 3, 14),
        };
        let user_res = DbUser {
            id: user.id.clone(),
            last_name: user.last_name.clone(),
            first_name: user.first_name.clone(),
            city: user.city.clone(),
            birthday_date: NaiveDate::from_ymd(2015, 3, 14),
        };
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let repo = PostgresRepository::new_pool(url).await.unwrap();
        let res = repo.insert(user).await;
        let user2 = DbUser {
            id: uuid!("0a708f88-bedb-4dad-a2f2-65dd4e8c132a").to_string(),
            last_name: generate(6, charset),
            first_name: generate(6, charset),
            city: generate(6, charset),
            birthday_date: NaiveDate::from_ymd(2015, 3, 14),
        };
        let repo2 = PostgresRepository::new_pool(url).await.unwrap();
        let res = repo2.insert(user2).await;
        assert_eq!(res.err().unwrap(), InsertError::Conflict)
    }

    #[tokio::test]
    async fn get_work() {
        let charset = "abcdefghijkl";
        let id = Uuid::new_v4().to_string();
        let user = DbUser {
            id: id.clone(),
            last_name: generate(6, charset),
            first_name: generate(6, charset),
            city: generate(6, charset),
            birthday_date: NaiveDate::from_ymd(2015, 3, 14),
        };
        let user_res = DbUser {
            id: user.id.clone(),
            last_name: user.last_name.clone(),
            first_name: user.first_name.clone(),
            city: user.city.clone(),
            birthday_date: NaiveDate::from_ymd(2015, 3, 14),
        };
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let mut repo = PostgresRepository::new_pool(url).await.unwrap();
        repo.insert(user).await;
        let repo2 = PostgresRepository::new_pool(url).await.unwrap();
        let res1 = repo2.get(id).await;
        let tmp = res1.unwrap();
        assert_eq!(user_res.eq(&tmp), true)
    }

    #[tokio::test]
    async fn get_notFound() {
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let charset = "abcdefghijkl";
        let id = Uuid::new_v4().to_string();
        let repo2 = PostgresRepository::new_pool(url).await.unwrap();
        let res1 = repo2.get(id).await;
        let user_response = res1.err().unwrap();
        assert_eq!(user_response, FetchOneError::NotFound)
    }
}

//
/*let rec = query_as!(
            Dbuser,
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
        match rec {
            Ok(elem) => {
                match  DbUser::from_row(&elem) {
                    Ok(user) => Ok(user),
                    Err(_) => Err(InsertError::Conflict)
                }
            }
            Err(e) => {
                println!("{}", e);
                Err(InsertError::Conflict)
            }
        }
    }*/
