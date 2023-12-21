use country_boundaries::{CountryBoundaries, LatLon, BOUNDARIES_ODBL_360X180};
use isocountry::CountryCode;
use rocket::http::Status;
use rocket::{get, routes, Route, State};
use s2::cellid::CellID;
use s2::latlng::LatLng;

#[get("/coords/<binary>")]
fn coords_binary(binary: String) -> Result<String, Status> {
    let binary: u64 = u64::from_str_radix(&binary, 2).map_err(|_| Status::BadRequest)?;
    Day21::coords(binary)
}

#[get("/country/<binary>")]
fn country_binary(binary: String, day21: &State<Day21>) -> Result<String, Status> {
    let binary: u64 = u64::from_str_radix(&binary, 2).map_err(|_| Status::BadRequest)?;
    day21.country(binary)
}

pub struct Day21 {
    country_boundaries: CountryBoundaries,
}
impl Default for Day21 {
    fn default() -> Self {
        Day21 {
            country_boundaries: CountryBoundaries::from_reader(BOUNDARIES_ODBL_360X180)
                .expect("Could not parse country boundaries"),
        }
    }
}

impl Day21 {
    pub fn routes() -> Vec<Route> {
        routes![coords_binary, country_binary]
    }

    fn coords(cell_id: u64) -> Result<String, Status> {
        let cell_id = CellID(cell_id);
        let lat_lng = LatLng::from(cell_id);
        Ok(format!(
            "{} {}",
            Day21::convert_latitude_to_degree_decimal(lat_lng.lat.deg()),
            Day21::convert_longitude_to_degree_decimal(lat_lng.lng.deg()),
        ))
    }

    fn country(&self, cell_id: u64) -> Result<String, Status> {
        let cell_id = CellID(cell_id);
        let lat_lng = LatLng::from(cell_id);
        let lat_lng =
            LatLon::new(lat_lng.lat.deg(), lat_lng.lng.deg()).map_err(|_| Status::BadRequest)?;
        let matched_ids = self.country_boundaries.ids(lat_lng);
        let country_code = matched_ids.last().ok_or(Status::NotFound)?;
        let name = CountryCode::for_alpha2(country_code)
            .map_err(|_| Status::BadRequest)?
            .name();

        Ok(if name == "Brunei Darussalam" {
            "Brunei".to_string()
        } else {
            name.to_string()
        })
    }

    fn convert_longitude_to_degree_decimal(longitude: f64) -> String {
        let direction = if longitude >= 0.0 { "E" } else { "W" };

        let longitude = longitude.abs();
        let degrees = longitude.trunc() as i32;
        let minutes = (longitude - degrees as f64) * 60.0;
        let whole_minutes = minutes.trunc() as i32;
        let seconds = (minutes - whole_minutes as f64) * 60.0;
        let rounded_seconds = (seconds * 1000.0).round() / 1000.0;

        format!(
            "{}°{}'{:.3}''{}",
            degrees, whole_minutes, rounded_seconds, direction
        )
    }

    fn convert_latitude_to_degree_decimal(latitude: f64) -> String {
        let direction = if latitude >= 0.0 { "N" } else { "S" };

        let latitude = latitude.abs();
        let degrees = latitude.trunc() as i32;
        let minutes = (latitude - degrees as f64) * 60.0;
        let whole_minutes = minutes.trunc() as i32;
        let seconds = (minutes - whole_minutes as f64) * 60.0;
        let rounded_seconds = (seconds * 1000.0).round() / 1000.0;

        format!(
            "{}°{}'{:.3}''{}",
            degrees, whole_minutes, rounded_seconds, direction
        )
    }
}
