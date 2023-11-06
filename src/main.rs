#[macro_use]
extern crate rocket;
use std::process;
mod input_filter_engine;
mod processor;
mod input_and_schema_compare;
mod routes;

#[launch]
fn rocket() -> _ {
    let result = processor::processor();

    if let Ok(val) = result {
        val
    } else {
        process::exit(0);
    }
}
