use geoutils::Location;
use std::str::FromStr;

#[cfg(feature = "aotload")]
use serde::{Deserialize, Deserializer};

#[derive(Clone, Debug)]
pub struct Airport {
    pub ident: String,
    pub kind: String,
    pub name: String,
    pub elevation_ft: i32,
    pub continent: String,
    pub iso_country: String,
    pub iso_region: String,
    pub municipality: String,
    pub gps_code: String,
    pub iata_code: String,
    pub local_code: String,
    pub coordinates: Coords,
}

#[cfg(feature = "aotload")]
#[derive(Clone, Debug, Deserialize)]
struct AirportTemplate {
    ident: String,
    #[serde(rename = "type")]
    kind: String,
    name: String,
    elevation_ft: i32,
    continent: String,
    iso_country: String,
    iso_region: String,
    municipality: String,
    gps_code: String,
    iata_code: String,
    local_code: String,
    coordinates: String,
}

#[cfg(feature = "aotload")]
impl<'de> Deserialize<'de> for Airport {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let AirportTemplate {
            ident,
            kind,
            name,
            elevation_ft,
            continent,
            iso_country,
            iso_region,
            municipality,
            gps_code,
            iata_code,
            local_code,
            coordinates,
        } = AirportTemplate::deserialize(deserializer)?;

        Ok(Airport {
            ident,
            kind,
            name,
            elevation_ft,
            continent,
            iso_country,
            iso_region,
            municipality,
            gps_code,
            iata_code,
            local_code,
            coordinates: coordinates.parse().map_err(serde::de::Error::custom)?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Coords {
    latitude: f64,
    longitude: f64,
}

impl Coords {
    pub fn location(&self) -> Location {
        let &Coords {
            latitude,
            longitude,
        } = self;
        Location::new(latitude, longitude)
    }
}

impl FromStr for Coords {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',').map(|x| x.trim());
        let latitude = parts
            .next()
            .ok_or("Missing latitude")?
            .parse()
            .map_err(|_| "Bad latitude")?;
        let longitude = parts
            .next()
            .ok_or("Missing longitude")?
            .parse()
            .map_err(|_| "Bad longitude")?;
        Ok(Coords {
            latitude,
            longitude,
        })
    }
}
