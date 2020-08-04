// src/lib.rs

use serde::{Deserialize, Serialize};
use std::str::FromStr;

use std::fmt::Formatter;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone,)]
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

impl fmt::Display for InsertablePerson {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\n,{},\n, {}, \n", self.last_name, self.first_name)
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

// this struct will be used to represent database record
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            id: 0,
            last_name: " ".into(),
            first_name: " ".into(),
        }
    }
}
