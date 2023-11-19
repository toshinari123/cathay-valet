use serde::{Deserialize, Serialize};
use thisError::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Passenger {
    //pub id: String,
    pub firstName: String,
    pub lastName: String,
    pub prefix: String,
    //pub passengerTypeCode: String,
    //pub gender: String,
    //pub purposeOfVisit: String,
    //pub regulatoryRequirements: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Booking {
    pub id: String,
    pub boardPointCode: String,
    pub offPointCode: String,
    //pub accessibilities: Vec<String>,
    pub timeBOT: String,
    pub timeSTA: String,
    pub segmentIds: Vec<String>,
    pub passenger: Passenger,
    pub maxBagAllowanceWeight: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Process {
    pub start: String,
    pub end: String,
    pub cost: u8,
    pub time: u8,
    pub comfort: ComfortLevel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComfortLevel {
    Uncomfortable,
    Normal,
    Comfortable
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Output {
    pub cheapest: Vec<Process>,
    pub fastest: Vec<Process>,
    pub mostComfy: Vec<Process>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("error while fetching cathay API")]
    Fetch(#[from] reqwest::Error),
    #[error("fetched object missing required fields")]
    MissingFields,
    #[error("key does not correspond to desired type")]
    WrongType,
}
