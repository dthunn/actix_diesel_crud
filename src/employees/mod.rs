mod model;
mod routes;
mod db;
mod error_handler;


extern crate diesel;
pub use model::*;
pub use routes::init_routes;