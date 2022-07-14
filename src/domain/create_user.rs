use std::borrow::Borrow;
use std::convert::Infallible;
use std::future::Future;
use std::sync::Arc;
use chrono::NaiveDate;
use futures::future::ok;
use futures::FutureExt;
use uuid::Uuid;
use crate::domain::entities::{BirthdayDate, CityName, FirstName, LastName};
use crate::repository::user::{DbUser, InsertError, PostgresRepository, Repository};

pub struct Request {
    pub first_name: String,
    pub last_name: String,
    pub birthday_date: String,
    pub city: String,
}

pub struct Response {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub birthday_date: NaiveDate,
    pub city: String,
}

pub enum Error {
    BadRequest,
    Conflict,
    Unknown,
}

pub async fn execute(
    repo: Arc<dyn Repository>,
    req: Request
) -> Result<Response, Error> {
    match (FirstName::try_from(req.first_name),
           LastName::try_from(req.last_name),
           BirthdayDate::try_from(req.birthday_date),
           CityName::try_from(req.city)) {
        (Ok(firstName), Ok(lastName), Ok(birthdayDate), Ok(cityName)) => {

            let res = repo.insert(DbUser {
                id: Uuid::new_v4().to_string(),
                first_name: String::from(firstName),
                last_name: String::from(lastName),
                birthday_date: NaiveDate::from(birthdayDate),
                city: String::from(cityName),
            }).await;
            todo!()
          /* match res {
                Ok(DbUser { id, first_name, last_name, birthday_date, city }) => Ok(Response {
                    id,
                    first_name,
                    last_name,
                    birthday_date,
                    city,
                }),
                Err(InsertError::Conflict) => Err(Error::Conflict),
                Err(InsertError::Unknown) => Err(Error::Unknown),
            }*/
        }
        _ => Err(Error::BadRequest),
    }
}