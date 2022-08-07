use actix_web::web::Data;
use chrono::NaiveDate;
use crate::domain::entities::UserId;
use crate::PostgresRepository;
use crate::repository::user::{DbUser, FetchAllError, Repository};

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

pub async fn execute(repo: Data<PostgresRepository>) -> Result<Vec<Response>, Error> {
    let res = repo.fetch_all().await;
    match res {
        Ok(users) => {
           Ok(users.into_iter().map(|x| {
                Response {
                    id: x.id,
                    first_name: x.first_name,
                    last_name: x.last_name,
                    birthday_date: x.birthday_date,
                    city: x.city,
                }
            }
            ).collect())
        }
        Err(FetchAllError::Unknown) => Err(Error::Unknown),
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use actix_web::web::Data;
    use chrono::NaiveDate;
    use random_string::generate;
    use uuid::Uuid;
    use crate::PostgresRepository;
    use crate::repository::user::{DbUser, Repository};

    #[tokio::test]
    async fn list_user_domain_works() {
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let repository = PostgresRepository::new_pool(url).await.unwrap();
        let repo = Data::new(repository);
        let users = insert_users_test().await;
        let res = repo.fetch_all().await;
        let resArray = res.unwrap();
        let mut hashmap = HashMap::new();
        for user in resArray {
            hashmap.insert(user.id.clone(), user);
        }


        for expectedUser in users {
            let a = hashmap.get(expectedUser.id.as_str()).unwrap();
            assert_eq!(expectedUser.id, a.id);
            assert_eq!(expectedUser.city, a.city);
            assert_eq!(expectedUser.birthday_date, a.birthday_date);
            assert_eq!(expectedUser.last_name, a.last_name);
            assert_eq!(expectedUser.first_name, a.first_name);
        }
    }

    async fn insert_users_test() -> Vec<DbUser> {
        let url = "postgres://postgres:somePassword@localhost:5432/postgres";
        let repo = PostgresRepository::new_pool(url).await.unwrap();
        let mut users = Vec::new();
        let charset = "abcdefghijkl";
        for n in 1..10 {
            let user = DbUser {
                id: Uuid::new_v4().to_string(),
                last_name: generate(6, charset),
                first_name: generate(6, charset),
                city: generate(6, charset),
                birthday_date: NaiveDate::from_ymd(2015, 3, 14),
            };

            let res = repo.insert(user).await.unwrap();
            users.push(res)
        }
        users
    }
}