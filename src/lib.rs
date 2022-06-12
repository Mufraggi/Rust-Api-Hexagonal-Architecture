use chrono::DateTime;
use sqlx::postgres::PgPool;
use sqlx::{Error, Pool, Postgres};

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
    db_pool: Pool<Postgres>,
}

pub struct DbUser {
    id:String,
    first_name: String,
    last_name : String,
    birthday_date: DateTime<chrono::Utc>,
    city: String
}

impl PostgresRepository {
    async fn new(mut self, url_db: String) -> Result<(),String> {
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

pub trait Repository: Send + Sync {
    fn insert(
        &self,
        number: u32,
        name: String,
    ) -> anyhow::Result<DbUser, InsertError>;
    fn fetch_all(&self) -> anyhow::Result<Vec<DbUser>, FetchAllError>;

    fn fetch_one(&self, number: u32) -> anyhow::Result<DbUser, FetchOneError>;
    fn update(&self, id: String, new_db_user: DbUser) -> anyhow::Result<DbUser, FetchAllError>;
    fn delete(&self, number: u32) -> anyhow::Result<(), DeleteError>;
}

impl Repository for PostgresRepository {
    fn insert(&self, number: u32, name: String) -> anyhow::Result<DbUser, InsertError> {
        todo!()
    }

    fn fetch_all(&self) -> anyhow::Result<Vec<DbUser>, FetchAllError> {
        todo!()
    }

    fn fetch_one(&self, number: u32) -> anyhow::Result<DbUser, FetchOneError> {
        todo!()
    }

    fn update(&self, id: String, new_db_user: DbUser) -> anyhow::Result<DbUser, FetchAllError> {
        todo!()
    }

    fn delete(&self, number: u32) -> anyhow::Result<(), DeleteError> {
        todo!()
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
