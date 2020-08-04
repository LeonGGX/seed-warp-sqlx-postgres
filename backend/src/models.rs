// src/models.rs

use std::str::FromStr;

use serde::{Deserialize, Serialize};

use sqlx::{FromRow, PgPool, Row};
use sqlx::postgres::PgRow;

use warp::reply::Response;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct InsertablePerson {
    pub first_name: String,
    pub last_name: String,
}

impl InsertablePerson {
    pub fn from_person(person: Person) -> InsertablePerson {
        InsertablePerson {
            first_name: person.first_name,
            last_name: person.last_name,
        }
    }

    pub fn to_string(&self) -> String {
        let mut str = String::new();
        str.push_str(&self.last_name);
        str.push_str(" ");
        str.push_str(&self.first_name);
        str
    }

}

impl FromStr for InsertablePerson {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InsertablePerson {
            first_name: "".to_string(),
            last_name: "".to_string(),
        })
    }
}

#[derive(Serialize, Deserialize, FromRow, Debug, Eq, PartialEq)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

// si on veut une sortie String et non Json ...
// donc pas très utile.
impl warp::reply::Reply for Person {
    fn into_response(self) -> Response {
        Response::new(
            format!(
                "id: {}\n nom: {}\n prenom: {}",
                self.id, self.first_name, self.last_name
            )
            .into(),
        )
    }
}

impl warp::reply::Reply for InsertablePerson {
    fn into_response(self) -> Response {
        Response::new(format!("nom: {}\n prenom: {}", self.first_name, self.last_name).into())
    }
}
