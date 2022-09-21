use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "type")]
    response_type: String,
    properties: Properties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    timeseries: Vec<Datapoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Datapoint {
    time: String,
    data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    instant: Instant,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instant {
    details: InstantDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstantDetails {
    air_pressure_at_sea_level: f64,
    air_temperature: f64,
    cloud_area_fraction: f64,
    relative_humidity: f64,
    wind_from_direction: f64,
    wind_speed: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let weather_url =
        "https://api.met.no/weatherapi/locationforecast/2.0/compact?lat=60.264&lon=22.296";
    let resp = Client::builder()
        .user_agent("magdan-saa-appis")
        .build()?
        .get(weather_url)
        .send()?;
    let data = resp.json::<Response>()?;
    let threshold = 9.0;
    let mut last_lamp_on = false;
    for i in data.properties.timeseries {
        let air_temperature = i.data.instant.details.air_temperature;
        let lamp_on = air_temperature < threshold;
        if lamp_on != last_lamp_on {
            println!(
                "{}: Turn lamp {}, temperature is {}",
                i.time,
                if lamp_on { "on" } else { "off" },
                air_temperature
            );
            last_lamp_on = lamp_on;
        }
    }
    Ok(())
}
