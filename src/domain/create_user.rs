use crate::domain::entities::{BirthdayDate, CityName, FirstName, LastName};
use crate::repository::user::{DbUser, InsertError, PostgresRepository, Repository};
use chrono::NaiveDate;
use futures::future::ok;
use futures::FutureExt;
use std::borrow::Borrow;
use std::convert::Infallible;
use std::future::Future;
use std::sync::Arc;
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

#[derive(Debug)]
pub enum Error {
    BadRequest,
    Conflict,
    Unknown,
}
pub async fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Response, Error> {
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
    use futures::executor::block_on;
    use super::*;
    use crate::domain::entities::{BirthdayDate, CityName, FirstName, LastName};
    use crate::PostgresRepository;


    #[tokio::test]
    async fn create_doamain_works()  {
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let repository = PostgresRepository::new_pool(url).await.unwrap();
        let repo = Arc::new(repository);
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


}
