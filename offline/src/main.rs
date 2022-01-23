use actix_web::{http, middleware, web, App, HttpResponse, HttpServer, Result};
use lazy_static::lazy_static;
use reverse_geocoder::{Locations, Record, ReverseGeocoder};
use serde_derive::Deserialize;

use std::fmt;

#[derive(Debug)]
enum ReverseGeocoderWebError {
    NotFound,
}

impl fmt::Display for ReverseGeocoderWebError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReverseGeocoderWebError::NotFound => write!(f, "Not found"),
        }
    }
}

impl actix_web::error::ResponseError for ReverseGeocoderWebError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ReverseGeocoderWebError::NotFound => HttpResponse::new(http::StatusCode::NOT_FOUND),
        }
    }
}

#[derive(Deserialize)]
struct LatLong {
    lat: f64,
    lon: f64,
}

async fn index(
    lat_long: web::Query<LatLong>,
) -> Result<web::Json<Record>, ReverseGeocoderWebError> {
    lazy_static! {
        static ref LOCATIONS: Locations = Locations::from_memory();
        static ref GEOCODER: ReverseGeocoder<'static> = ReverseGeocoder::new(&LOCATIONS);
    }

    let search_result = match GEOCODER.search((lat_long.lat, lat_long.lon)) {
        Some(result) => result,
        None => return Err(ReverseGeocoderWebError::NotFound),
    };

    Ok(web::Json(search_result.record.clone()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
    })
    .keep_alive(10)
    .bind("127.0.0.1:3999")?
    .run()
    .await
}
