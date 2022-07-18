use std::cmp::PartialEq;
use std::convert::TryFrom;
use std::fmt::Display;
use chrono::{NaiveDate, ParseResult};
use uuid::{Error, Uuid};

#[derive(Clone)]
pub struct BirthdayDate(NaiveDate);

impl From<BirthdayDate> for NaiveDate {
    fn from(t: BirthdayDate) -> Self {
        t.0
    }
}

impl TryFrom<String> for BirthdayDate {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        let parse_from_str = NaiveDate::parse_from_str;
        match parse_from_str(&n, "%Y-%m-%d") {
            Ok(date) => { Ok(Self(date)) }
            Err(_) => { Err(()) }
        }
    }
}


#[cfg(test)]
impl BirthdayDate {
    pub fn date() -> Self {
        let parse_from_str = NaiveDate::parse_from_str;
        Self(parse_from_str("1994-10-03", "%Y-%m-%d").unwrap())
    }
    pub fn date_native() -> NaiveDate {
        NaiveDate::parse_from_str("1994-10-03", "%Y-%m-%d").unwrap()
    }
    pub fn date_string() -> &'static str {
        "1994-10-03"
    }

    pub fn bad() -> Self {
        let parse_from_str = NaiveDate::parse_from_str;
        Self(parse_from_str("03-10-1994", "%Y-%m-%d").unwrap())
    }
}

#[derive(Clone)]
pub struct FirstName(String);

impl TryFrom<String> for FirstName {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        if n.is_empty() {
            Err(())
        } else {
            Ok(Self(n))
        }
    }
}

impl From<FirstName> for String {
    fn from(n: FirstName) -> Self {
        n.0
    }
}

#[cfg(test)]
impl FirstName {
    pub fn name() -> Self {
        Self(String::from("name"))
    }

    pub fn bad() -> Self {
        Self(String::from(""))
    }
}

#[derive(Clone)]
pub struct LastName(String);

impl TryFrom<String> for LastName {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        if n.is_empty() {
            Err(())
        } else {
            Ok(Self(n))
        }
    }
}


impl From<LastName> for String {
    fn from(n: LastName) -> Self {
        n.0
    }
}

#[cfg(test)]
impl LastName {
    pub fn name() -> Self {
        Self(String::from("last name"))
    }

    pub fn bad() -> Self {
        Self(String::from(""))
    }
}


#[derive(Clone)]
pub struct CityName(String);

impl TryFrom<String> for CityName {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        if n.is_empty() {
            Err(())
        } else {
            Ok(Self(n))
        }
    }
}

impl From<CityName> for String {
    fn from(n: CityName) -> Self {
        n.0
    }
}

#[cfg(test)]
impl CityName {
    pub fn name() -> Self {
        Self(String::from("nice"))
    }

    pub fn bad() -> Self {
        Self(String::from(""))
    }
}


#[derive(Clone, Debug)]
pub struct UserId(Uuid);

impl From<UserId> for Uuid {
    fn from(t: UserId) -> Self {
        t.0
    }
}

impl UserId {
    pub fn my_to_String(&self) -> String {
        self.0.to_string()
    }
}

impl TryFrom<String> for UserId {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match Uuid::parse_str(s.as_str()) {
            Ok(uuid) => { Ok(Self(uuid)) }
            Err(_) => { Err(()) }
        }
    }
}

#[cfg(test)]
impl UserId {
    pub fn id() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn bad() -> Self {
        Self("".parse().unwrap())
    }
}