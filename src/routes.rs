use rocket::serde::json::Json;
use rocket::http::{Status};
use rocket::response::status;
use reqwest::header;
use serde_json::{Value, Map};
use crate::models::*;

fn curl(passenger_id: &str) -> Result<String, Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert("accept", "application/json".parse().unwrap());
    headers.insert("apiKey", "i8oVMsIqzEidSx6BdRZyY8IM4Q775xFY".parse().unwrap());
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    client.get(&("https://developers.cathaypacific.com/hackathon-apigw/airport/customers/".to_string() + + "/details"))
        .headers(headers)
        .send()?
        .text()?
}

fn get_obj(obj: Map<String, Value>, key: &str) -> Result<Value, Error> {
    match obj.get(key).ok_or(Error::MissingFields)? {
        Value::Object(ob) => Ok(ob),
        _ => Err(Error::WrongType),
    }
}

fn get_arr(obj: Map<String, Value>, key: &str) -> Result<Vec<Value>, Error> {
    match obj.get(key).ok_or(Error::MissingFields)? {
        Value::Array(ob) => Ok(ob),
        _ => Err(Error::WrongType),
    }
}

fn get_str(obj: Map<String, Value>, key: &str) -> Result<String, Error> {
    match obj.get(key).ok_or(Error::MissingFields)? {
        Value::String(ob) => Ok(ob),
        _ => Err(Error::WrongType),
    }
}

fn convert(obj: Map<String, Value>) -> Result<Booking, Error> {
    let data = get_obj(obj, "data")?;
    let name = get_obj(get_obj(data, "traveler")?, "name")?;
    let dfs: Vec<Map<String, Value>>  = get_obj(get_obj("dictionaries")?, "datedFlight")?.iter().collect();
    let flight = dfs[0];
    let v = get_arr(flight, "flightPoints")?;
    Booking {
        id: get_str(data, "recordLocator")?,
        boardPointCode: get_str(v[0], "iataCode")?,
        offPointCode: get_str(v[v.size() - 1], "iataCode")?,
        timeBOT: get_str(get_arr(get_obj(v[0], "departure")?, "timings")?[0], "value")?,
        timeSTA: get_str(get_arr(get_obj(v[v.size() - 2], "arrival")?, "timings")?[0], "value")?,
        segmentIds: vec![],
        passenger: Passenger {
            firstName: get_str(name, "firstName")?,
            lastName: get_str(name, "lastName")?,
            prefix: get_str(name, "prefix")?,
        },
        maxBagAllowanceWeight: 0,
    }
}

#[get("/<passenger_id>")]
pub fn get(passenger_id: &str) -> Json<Output> {
    match curl(passenger_id) {
        Ok(s) => {
            let parsed: Value = serde_json::from_str(&s)?;
            let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
        },
        Err(e) => {

        },
    }
}
