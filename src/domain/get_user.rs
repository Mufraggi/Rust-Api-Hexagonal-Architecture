use std::sync::Arc;
use chrono::NaiveDate;
use crate::domain::entities::UserId;
use crate::repository::user::{DbUser, FetchOneError, InsertError, Repository};

#[derive(Debug)]
pub struct Request {
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
    NotFound,
    Unknown,
}

pub async fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Response, Error> {
    match (UserId::try_from(req.id)) {
        Ok(userId) => {
            let res = repo.get(userId.my_to_String()).await;
            match res {
                Ok(DbUser {
                       id,
                       first_name,
                       last_name,
                       birthday_date,
                       city,
                   }) => Ok(Response { id, first_name, last_name, birthday_date, city }),
                Err(FetchOneError::NotFound) => Err(Error::NotFound),
                Err(FetchOneError::Unknown) => Err(Error::Unknown),
            }
        }
        _ => Err(Error::BadRequest)
    }
}


#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::domain::entities::{BirthdayDate, CityName, FirstName, LastName, UserId};
    use crate::domain::get_user::{Error, execute, Request};
    use crate::PostgresRepository;
    use crate::repository::user::{DbUser, Repository};

    #[tokio::test]
    async fn get_user_domain_works() {
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let repository = PostgresRepository::new_pool(url).await.unwrap();
        let repo = Arc::new(repository);

        let dbUser = &repo.insert(DbUser {
            id:UserId::id().my_to_String(),
            first_name: String::from(FirstName::name()),
            last_name: String::from(LastName::name()),
            birthday_date: BirthdayDate::date_native(),
            city: String::from(CityName::name()),
        }).await.unwrap();
        let result = dbUser.clone();
        let request = Request { id: dbUser.id.to_string() };
        let res = execute(repo, request).await.unwrap();
        assert_eq!(res.id, result.id);
        assert_eq!(res.last_name, String::from(LastName::name()));
        assert_eq!(res.first_name, String::from(FirstName::name()));
        assert_eq!(res.birthday_date, BirthdayDate::date_native());
        assert_eq!(res.city, String::from(CityName::name()));
    }
    #[tokio::test]
    async fn get_user_domain_fail_notfound() {
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let repository = PostgresRepository::new_pool(url).await.unwrap();
        let repo = Arc::new(repository);


        let id =UserId::id().my_to_String();
        let request = Request { id };
        let res = execute(repo, request).await;
        assert_eq!( res.err().unwrap(), Error::NotFound);
    }
    #[tokio::test]
    async fn get_user_domain_fail_badRequest() {
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let repository = PostgresRepository::new_pool(url).await.unwrap();
        let repo = Arc::new(repository);


        let request = Request { id: "".parse().unwrap() };
        let res = execute(repo, request).await;
        assert_eq!( res.err().unwrap(), Error::BadRequest);
    }
}
