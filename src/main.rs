#[macro_use] extern crate rocket;
use std::process;
mod routes;
mod input_filter_engine;
mod processor;

#[launch]
fn rocket() -> _ {

    let result = processor::processor(); 

    if let Ok(val) = result {
        val
    }else{

        process::exit(0);

    }

}