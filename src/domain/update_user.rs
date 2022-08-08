use actix_web::web::Data;
use chrono::NaiveDate;
use uuid::Uuid;
use crate::domain::entities::{BirthdayDate, CityName, FirstName, LastName, UserId};
use crate::PostgresRepository;
use crate::repository::user::{DbUser, Repository, UpdateError};


#[derive(Debug)]
pub struct Request {
    pub first_name: String,
    pub last_name: String,
    pub birthday_date: String,
    pub city: String,
}

#[derive(Debug)]
pub struct Params {
    pub id: String,
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
    NotFound,
}

pub async fn execute(repo: Data<PostgresRepository>, params: Params, req: Request) -> Result<Response, Error> {
    match (UserId::try_from(params.id),
           FirstName::try_from(req.first_name),
           LastName::try_from(req.last_name),
           BirthdayDate::try_from(req.birthday_date),
           CityName::try_from(req.city)
    ) {
        (Ok(id), Ok(firstName),
            Ok(lastName), Ok(birthdayDate),
            Ok(cityName)) => {
            let res = repo.update(id.my_to_String(), DbUser {
                id: id.my_to_String(),
                first_name: String::from(firstName),
                last_name: String::from(lastName),
                birthday_date: NaiveDate::from(birthdayDate),
                city: String::from(cityName),
            }).await;
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
                Err(UpdateError::NotFound) => Err(Error::NotFound),
                Err(UpdateError::Unknown) => Err(Error::Unknown),
            }
        }
        _ => Err(Error::BadRequest),
    }
}

#[cfg(test)]
mod tests {
    use actix_web::web::Data;
    use crate::domain::entities::{BirthdayDate, CityName, FirstName, LastName, UserId};
    use crate::domain::update_user::{execute, Params, Request};
    use crate::PostgresRepository;
    use crate::repository::user::{DbUser, Repository};

    #[tokio::test]
    async fn create_domain_works() {
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let repository = PostgresRepository::new_pool(url).await.unwrap();
        let repo = Data::new(repository);
        let request = Request {
            first_name: String::from("aaaaaa"),
            last_name: String::from(LastName::name()),
            birthday_date: BirthdayDate::date_string().parse().unwrap(),
            city: String::from("CityName::name()"),
        };

        let dbUser = &repo.insert(DbUser {
            id:UserId::id().my_to_String(),
            first_name: String::from(FirstName::name()),
            last_name: String::from(LastName::name()),
            birthday_date: BirthdayDate::date_native(),
            city: String::from(CityName::name()),
        }).await.unwrap();
        let result = dbUser.clone();
        let id = dbUser.id.clone();
        let res = execute(repo, Params{id:id},request).await.unwrap();
        assert_eq!(res.last_name, String::from(LastName::name()));
        assert_eq!(res.first_name, String::from("aaaaaa"));
        assert_eq!(res.birthday_date, BirthdayDate::date_native());
        assert_eq!(res.city, "CityName::name()");
    }
}