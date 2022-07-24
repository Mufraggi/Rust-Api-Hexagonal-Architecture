use crate::domain::entities::{BirthdayDate, CityName, FirstName, LastName};
use crate::repository::user::{DbUser, InsertError, PostgresRepository, Repository};
use chrono::NaiveDate;
use futures::future::ok;
use futures::FutureExt;
use std::borrow::Borrow;
use std::convert::Infallible;
use std::future::Future;
use std::sync::Arc;
use actix_web::web::Data;
use uuid::Uuid;

#[derive(Debug)]
pub struct Request {
    pub first_name: String,
    pub last_name: String,
    pub birthday_date: String,
    pub city: String,
}

#[derive(Debug)]
pub struct Response {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub birthday_date: NaiveDate,
    pub city: String,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    BadRequest,
    Conflict,
    Unknown,
}

pub async fn execute(repo: Data<PostgresRepository>, req: Request) -> Result<Response, Error> {
    match (
        FirstName::try_from(req.first_name),
        LastName::try_from(req.last_name),
        BirthdayDate::try_from(req.birthday_date),
        CityName::try_from(req.city),
    ) {
        (Ok(firstName), Ok(lastName), Ok(birthdayDate), Ok(cityName)) => {
            let res = repo
                .insert(DbUser {
                    id: Uuid::new_v4().to_string(),
                    first_name: String::from(firstName),
                    last_name: String::from(lastName),
                    birthday_date: NaiveDate::from(birthdayDate),
                    city: String::from(cityName),
                })
                .await;
            match res {
                Ok(DbUser {
                       id,
                       first_name,
                       last_name,
                       birthday_date,
                       city,
                   }) => Ok(Response {
                    id,
                    first_name,
                    last_name,
                    birthday_date,
                    city,
                }),
                Err(InsertError::Conflict) => Err(Error::Conflict),
                Err(InsertError::Unknown) => Err(Error::Unknown),
            }
        }
        _ => Err(Error::BadRequest),
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    use futures::executor::block_on;
    use super::*;
    use crate::domain::entities::{BirthdayDate, CityName, FirstName, LastName};
    use crate::PostgresRepository;
    use crate::repository::user::{DeleteError, FetchAllError, FetchOneError};
    use async_trait::async_trait;

    struct RepoMock {}

    impl RepoMock {
        pub fn new() -> Result<RepoMock, ()> {
            Ok(Self{})
        }
    }

    #[async_trait]
    impl Repository for RepoMock {
        async fn insert(&self, user: DbUser) -> anyhow::Result<DbUser, InsertError> {
            Err(InsertError::Conflict)
        }

        async fn fetch_all(&self) -> anyhow::Result<Vec<DbUser>, FetchAllError> {
            todo!()
        }

        async fn get(&self, id: String) -> anyhow::Result<DbUser, FetchOneError> {
            todo!()
        }

        async fn update(&self, id: String, new_db_user: DbUser) -> anyhow::Result<DbUser, FetchAllError> {
            todo!()
        }

        async fn delete(&self, number: u32) -> anyhow::Result<(), DeleteError> {
            todo!()
        }
    }

    #[tokio::test]
    async fn create_domain_works() {
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let repository = PostgresRepository::new_pool(url).await.unwrap();
        let repo = Data::new(repository);
        let request = Request {
            first_name: String::from(FirstName::name()),
            last_name: String::from(LastName::name()),
            birthday_date: BirthdayDate::date_string().parse().unwrap(),
            city: String::from(CityName::name()),
        };
        let res = execute(repo, request).await.unwrap();
        assert_eq!(res.last_name, String::from(LastName::name()));
        assert_eq!(res.first_name, String::from(FirstName::name()));
        assert_eq!(res.birthday_date, BirthdayDate::date_native());
        assert_eq!(res.city, String::from(CityName::name()));
    }

    #[tokio::test]
    async fn create_domain_fail_bad_request() {
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let repository = PostgresRepository::new_pool(url).await.unwrap();
        let repo = Data::new(repository);
        let request = Request {
            first_name: String::from(FirstName::bad()),
            last_name: String::from(LastName::name()),
            birthday_date: BirthdayDate::date_string().parse().unwrap(),
            city: String::from(CityName::name()),
        };
        let res = execute(repo, request).await;
        assert_eq!(res.err().unwrap(), Error::BadRequest)
    }

    /*#[tokio::test]
    async fn create_domain_fail_conflict() {
        let repository = RepoMock::new().unwrap();
        let repository = PostgresRepository::new_pool(url).await.unwrap();
        let repo = Data::new(repository);
        let request = Request {
            first_name: String::from(FirstName::name()),
            last_name: String::from(LastName::name()),
            birthday_date: BirthdayDate::date_string().parse().unwrap(),
            city: String::from(CityName::name()),
        };
        let res = execute(repo, request).await;
        assert_eq!(res.err().unwrap(), Error::Conflict)
    }*/

}
