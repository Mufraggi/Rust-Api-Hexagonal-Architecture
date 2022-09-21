use actix_web::web::Data;
use futures::future::err;
use crate::domain::entities::UserId;
use crate::PostgresRepository;
use crate::repository::user::{DeleteError, Repository};

#[derive(Debug)]
pub struct Request {
    pub id: String,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    BadRequest,
    NotFound,
    Unknown,
}

pub async fn execute(repo: Data<PostgresRepository>, req: Request) -> Result<(), Error> {
    match UserId::try_from(req.id) {
        Ok(userId) => {
            let res = repo.delete(userId.my_to_String()).await;
            match res {
                Ok(_) => Ok(()),
                Err(DeleteError::NotFound) => Err(Error::NotFound),
                Err(DeleteError::Unknown) => Err(Error::Unknown),
            }
        },

        _ => Err(Error::BadRequest),
    }
}

#[cfg(test)]
mod tests {
    use actix_web::web::Data;
    use crate::domain::delete_user::{execute, Request, Error};
    use crate::domain::entities::{BirthdayDate, CityName, FirstName, LastName, UserId};
    use crate::PostgresRepository;
    use crate::repository::user::{DbUser, Repository};

    #[tokio::test]
    async fn delete_domain_works() {
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let repository = PostgresRepository::new_pool(url).await.unwrap();
        let repo = Data::new(repository);

        let dbUser = &repo.insert(DbUser {
            id:UserId::id().my_to_String(),
            first_name: String::from(FirstName::name()),
            last_name: String::from(LastName::name()),
            birthday_date: BirthdayDate::date_native(),
            city: String::from(CityName::name()),
        }).await.unwrap();
        let result = dbUser.clone();
        let request = Request { id: dbUser.id.to_string() };
        let res = execute(repo, request).await;
        assert_eq!(res.is_ok(), true);
    }

    #[tokio::test]
    async fn delete_domain_not_found() {
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let repository = PostgresRepository::new_pool(url).await.unwrap();
        let repo = Data::new(repository);
        let id = UserId::id().my_to_String();
        let request = Request { id: id };
        let res = execute(repo, request).await;
        assert_eq!(res.err().unwrap(), Error::NotFound);
    }
}