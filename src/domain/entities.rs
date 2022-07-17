use std::cmp::PartialEq;
use std::convert::TryFrom;
use chrono::{NaiveDate, ParseResult};

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
        match parse_from_str(&n, "%Y-%m-%d")  {
            Ok(date) => {Ok(Self(date))},
            Err(_) => {Err(())}
        }

    }
}


#[cfg(test)]
impl BirthdayDate {
    pub fn date() -> Self {
        let parse_from_str = NaiveDate::parse_from_str;
        Self(parse_from_str("1994-10-03", "%Y-%m-%d").unwrap())
    }
    pub fn  date_native() -> NaiveDate {
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



#[derive(Clone)]
pub struct InputUser {
    pub first_name: FirstName,
    pub last_name: LastName,
    pub birthday_date: NaiveDate,
    pub city: CityName,

}

impl InputUser {
    pub fn new(first_name: FirstName, last_name: LastName, birthday_date: NaiveDate, city: CityName) -> Self {
        Self {
            first_name,
            last_name,
            birthday_date,
            city,
        }
    }
}