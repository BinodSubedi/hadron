use chrono::prelude::*;
use rocket::{
    form::{validate::Len, Form},
    http::Status,
    outcome::Outcome,
    request,
    serde::{json::Json, Deserialize, Serialize},
    Request,
};
use serde_json::Value;
use std::ops::Deref;
use std::{cmp::Ordering, fs};
use std::{env, iter::Iterator, str::FromStr};
extern crate regex;
use crate::input_and_schema_compare::comparer;
use aes::cipher::{
    generic_array::{typenum::U16, GenericArray},
    BlockDecrypt, BlockEncrypt, KeyInit,
};
use aes::Aes128;

use hmac::Hmac;
use jwt::VerifyWithKey;
use rocket::request::FromRequest;
use sha2::Sha256;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GetStandardInputFormat {
    pub data: Value,
}

#[derive(Debug)]
pub struct Token(Option<String>);

#[derive(Debug)]
pub enum ApiTokenError {
    NotFound,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ApiTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("token");
        match token {
            Some(token) => {
                // check validity
                Outcome::Success(Token(Some(token.to_string())))
            }
            None => Outcome::Success(Token(None)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
enum ResponseStatus {
    Success,
    Failed,
    NotFound,
    JWTExpiry,
    InvalidJWT,
    NotLinked,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GetStandardResponse {
    status: u16,
    response: ResponseStatus,
    data: Vec<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GetOneStandardResponse {
    status: u16,
    response: ResponseStatus,
    data: Value,
}

#[get("/<collection>")]
pub async fn get_all(collection: String) -> Json<GetStandardResponse> {
    format!("The collection of choice for whole query is {}", collection);

    //Here we see if the request for get is pointing to user type in config file
    //or reference_id in schema, if it does we return with not allowed

    let config_schema_dir = String::from("/home/qubit/Documents/hadron/");

    let config_contents =
        fs::read_to_string(config_schema_dir.clone() + &collection.to_lowercase() + "_config.json");

    let schema_contents =
        fs::read_to_string(config_schema_dir + &collection.to_lowercase() + ".json");

    if let Ok(val) = config_contents {
        let value_config = Value::from_str(&val).unwrap();

        //println!("typoo");
        //println!("{:?}", value_config["typoo"]);
        println!("{:?}", value_config.get("type"));

        if let Some(Value::String(config_type)) = value_config.get("type") {
            if config_type == "User" || config_type == "user" {
                return Json(GetStandardResponse {
                    status: 404,
                    response: ResponseStatus::Failed,
                    data: vec![Value::Null],
                });
            }
        }
    } else {
        return Json(GetStandardResponse {
            status: 404,
            response: ResponseStatus::Failed,
            data: vec![Value::Null],
        });
    };

    if let Ok(val) = schema_contents {
        let value_schema = Value::from_str(&val).unwrap();

        //println!("typoo");
        //println!("{:?}", value_config["typoo"]);
        println!("{:?}", value_schema);
        println!("{:?}", value_schema.get("reference_id"));

        if let Some(_) = value_schema.get("reference_id") {
            return Json(GetStandardResponse {
                status: 404,
                response: ResponseStatus::Failed,
                data: vec![Value::Null],
            });
        }
    } else {
        return Json(GetStandardResponse {
            status: 404,
            response: ResponseStatus::Failed,
            data: vec![Value::Null],
        });
    }

    let directory = String::from("/home/qubit/Documents/hadron/.data/data");

    let files = fs::read_dir(&directory).unwrap();

    //println!("{:?}",files);

    //let models_file_vec: Vec<&str> = Vec::<&str>::new();

    let mut required_model_file: Option<String> = None;

    let mut total_number_of_files = 0;

    //by the way this by_ref as took the value out of the files so may want to look into this
    //sometime soon

    //  println!("{:?}", files.by_ref().count());

    for file in files {
        let string_file_val = file.expect("well there\'s something fishy").path();

        let string_format = string_file_val
            .into_os_string()
            .into_string()
            .expect("format change error");

        let splitted = string_format.split("/").collect::<Vec<&str>>();

        let file_name = splitted[splitted.len() - 1];

        // println!("{:?}", file_name);

        let name_splitted = file_name.split(".").collect::<Vec<&str>>();

        //        println!("{:?}",name_splitted);

        //       println!("{}", collection.to_lowercase());

        if name_splitted[0].split('-').collect::<Vec<&str>>()[0]
            == collection.to_lowercase().as_str()
        {
            required_model_file = Some(name_splitted[0].to_string());
            println!("matched");

            total_number_of_files += 1;
        }
    }

    if required_model_file == None {
        let res = GetStandardResponse {
            status: 404,
            response: ResponseStatus::NotFound,
            data: vec![Value::Null],
        };

        return Json(res);
    }

    //If got time, the file read will be made multi-threaded but for now
    //It's just single threaded

    let mut vec_key = Vec::new();

    let env_variables: Vec<String> = env::args().collect();

    println!("{:?}", env_variables.last().unwrap());

    let key_str = env_variables.last().unwrap();

    let key_val = key_str.as_bytes();

    let mut final_values_list: Vec<Value> = Vec::new();

    for &byte in key_val.iter() {
        vec_key.push(byte);
    }

    let key = GenericArray::from_slice(&vec_key);

    for i in 1..(total_number_of_files + 1) {
        let file_name: String;

        println!("{}", i);
        if i == 1 {
            file_name = collection.clone().to_lowercase() + ".dat";
        } else {
            file_name = collection.clone().to_lowercase() + "-" + &(i - 1).to_string() + ".dat";
        }

        println!("{}", file_name);

        //here we read file and decrypt and push to some outer scope vector

        //        let directory = String::from("/home/qubit/Documents/hadron/.data/data");

        let current_chosen_file = fs::read(directory.clone() + "/" + &file_name).unwrap();

        // let mut block:GenericArray<u8,U16> = GenericArray::from([0u8;16]);

        let mut blocks: Vec<GenericArray<u8, U16>> = Vec::new();

        let mut counter = 0;

        println!("total_val:{:?}", &current_chosen_file);
        println!("val:{:?}", &current_chosen_file[0..16]);

        loop {
            //println!("counterrr:{}, {}",counter,contents.len());

            blocks.push(
                GenericArray::from_slice(&current_chosen_file[counter..(counter + 16)]).clone(),
            );

            //println!("{}",&counter);

            counter = counter + 16;

            if counter == current_chosen_file.len() {
                //println!("{counter}");

                break;
            }
        }

        // let mut block:GenericArray<u8,U16> = GenericArray::from_slice(&contents).clone();
        //  let mut block:GenericArray<u8,U16> = GenericArray::from_slice(&vec_key).clone();

        let cipher = Aes128::new(&key);

        // cipher.decrypt_block(&mut block);

        cipher.decrypt_blocks(&mut blocks);

        let mut final_string: String = String::from("");

        for block in blocks {
            *&mut final_string += &String::from_utf8(block.to_vec()).unwrap();
        }

        println!("{}", final_string);
        //Now the string need to split with , but only from the area where there is no {}
        //might have to write custom splitter

        //        let final_string_vec:Vec<&str> = final_string.split(',').collect::<Vec<&str>>();

        // Custom comma separator////////////////////
        //

        let mut str_splitted: Vec<Value> = Vec::new();

        let mut now_val: String = "".to_string();

        let mut curly_level = 0;

        for char in final_string.chars() {
            match char {
                '{' => {
                    now_val.push(char);
                    curly_level += 1;
                }

                '}' => {
                    now_val.push(char);
                    curly_level -= 1;
                }

                ',' => {
                    if curly_level == 1 {
                        now_val.push(char);
                    } else {
                        let pushed = now_val.clone();

                        str_splitted.push(Value::from_str(&pushed.trim()).unwrap());

                        *&mut now_val = "".to_string();
                    }
                }

                _ => {
                    now_val.push(char);
                }
            }
        }

        //        println!("{:?}", str_splitted);

        //         println!("{:?}",final_string_vec);
        //       println!("{:?}",final_string_vec[0]);

        final_values_list.extend(str_splitted);
    }

    //    println!("final values:{:?}",final_values_list);

    Json(GetStandardResponse {
        status: 200,
        response: ResponseStatus::Success,
        data: final_values_list,
    })
}
//Here we change the get many route to only let data out which has reference id
#[get("/<collection>/<number>")]
pub async fn get_any_related(
    collection: String,
    number: u8,
    token: Token,
) -> Json<GetStandardResponse> {
    format!(
        "The query in {}, and number of element is {}",
        collection, number
    );

    let directory = String::from("/home/qubit/Documents/hadron/.data/data");

    let files = fs::read_dir(&directory).unwrap();

    //println!("{:?}",files);

    //let models_file_vec: Vec<&str> = Vec::<&str>::new();

    let mut required_model_file: Option<String> = None;

    let mut total_number_of_files = 0;

    //by the way this by_ref as took the value out of the files so may want to look into this
    //sometime soon

    //  println!("{:?}", files.by_ref().count());

    for file in files {
        let string_file_val = file.expect("well there\'s something fishy").path();

        let string_format = string_file_val
            .into_os_string()
            .into_string()
            .expect("format change error");

        let splitted = string_format.split("/").collect::<Vec<&str>>();

        let file_name = splitted[splitted.len() - 1];

        // println!("{:?}", file_name);

        let name_splitted = file_name.split(".").collect::<Vec<&str>>();

        //        println!("{:?}",name_splitted);

        //       println!("{}", collection.to_lowercase());

        if name_splitted[0].split('-').collect::<Vec<&str>>()[0]
            == collection.to_lowercase().as_str()
        {
            required_model_file = Some(name_splitted[0].to_string());
            println!("matched");

            total_number_of_files += 1;
        }
    }

    if required_model_file == None {
        let res = GetStandardResponse {
            status: 404,
            response: ResponseStatus::NotFound,
            data: vec![Value::Null],
        };

        return Json(res);
    }

    //If got time, the file read will be made multi-threaded but for now
    //It's just single threaded

    //Here we read the collection schema and recheck if this collection really has reference id or
    //not and if not return

    let mut vec_key = Vec::new();

    let env_variables: Vec<String> = env::args().collect();

    println!("{:?}", env_variables.last().unwrap());

    let key_str = env_variables.last().unwrap();

    let key_val = key_str.as_bytes();

    let mut final_values_list: Vec<Value> = Vec::new();

    for &byte in key_val.iter() {
        vec_key.push(byte);
    }

    let key = GenericArray::from_slice(&vec_key);

    let reference_id: Option<String>;

    {
        let readFileSchema =
            String::from_utf8(fs::read(format!("./{}.json", &collection.to_lowercase())).unwrap())
                .unwrap();

        if let Value::Object(obj) = serde_json::from_str(&readFileSchema).unwrap() {
            let mut found: Option<bool> = None;

            for (k, _) in obj {
                if k.to_lowercase() == "reference_id" {
                    found = Some(true);
                }
            }

            if found == None {
                return Json(GetStandardResponse {
                    status: 400,
                    response: ResponseStatus::NotLinked,
                    data: vec![Value::Null],
                });
            }

            //still even it has it, we need to see if there is even a sent jwt token in header or not
            //we check the value of id and expiry from jwt token

            if let Token(Some(token_val)) = token {
                let key: Hmac<Sha256> = Hmac::new_from_slice(&vec_key).unwrap();

                let claims: BTreeMap<String, String>;

                if let Ok(claimed_value) = token_val.verify_with_key(&key) {
                    claims = claimed_value;
                } else {
                    return Json(GetStandardResponse {
                        status: 400,
                        response: ResponseStatus::Failed,
                        data: vec![Value::Null],
                    });
                }

                println!("This is the id::::{}", claims["id"]);

                //here we check for expiry and if not expired then create
                //a global variable and pass in the decrypted id to it

                let not_expired = claims["expiry"].parse::<i64>().unwrap() > Utc::now().timestamp();

                if !not_expired {
                    return Json(GetStandardResponse {
                        status: 400,
                        response: ResponseStatus::Failed,
                        data: vec![Value::Null],
                    });
                }

                reference_id = Some(claims["id"].clone());
            } else {
                return Json(GetStandardResponse {
                    status: 400,
                    response: ResponseStatus::Failed,
                    data: vec![Value::Null],
                });
            }
        } else {
            return Json(GetStandardResponse {
                status: 400,
                response: ResponseStatus::Failed,
                data: vec![Value::Null],
            });
        }
    }

    for i in 1..(total_number_of_files + 1) {
        let file_name: String;

        println!("{}", i);
        if i == 1 {
            file_name = collection.clone().to_lowercase() + ".dat";
        } else {
            file_name = collection.clone().to_lowercase() + "-" + &(i - 1).to_string() + ".dat";
        }

        println!("{}", file_name);

        //here we read file and decrypt and push to some outer scope vector

        //        let directory = String::from("/home/qubit/Documents/hadron/.data/data");

        let current_chosen_file = fs::read(directory.clone() + "/" + &file_name).unwrap();

        // let mut block:GenericArray<u8,U16> = GenericArray::from([0u8;16]);

        let mut blocks: Vec<GenericArray<u8, U16>> = Vec::new();

        let mut counter = 0;

        println!("total_val:{:?}", &current_chosen_file);
        println!("val:{:?}", &current_chosen_file[0..16]);

        loop {
            //println!("counterrr:{}, {}",counter,contents.len());

            blocks.push(
                GenericArray::from_slice(&current_chosen_file[counter..(counter + 16)]).clone(),
            );

            //println!("{}",&counter);

            counter = counter + 16;

            if counter == current_chosen_file.len() {
                //println!("{counter}");

                break;
            }
        }

        // let mut block:GenericArray<u8,U16> = GenericArray::from_slice(&contents).clone();
        //  let mut block:GenericArray<u8,U16> = GenericArray::from_slice(&vec_key).clone();

        let cipher = Aes128::new(&key);

        // cipher.decrypt_block(&mut block);

        cipher.decrypt_blocks(&mut blocks);

        let mut final_string: String = String::from("");

        for block in blocks {
            *&mut final_string += &String::from_utf8(block.to_vec()).unwrap();
        }

        println!("{}", final_string);
        //Now the string need to split with , but only from the area where there is no {}
        //might have to write custom splitter

        //        let final_string_vec:Vec<&str> = final_string.split(',').collect::<Vec<&str>>();

        // Custom comma separator////////////////////
        //

        let mut str_splitted: Vec<Value> = Vec::new();

        let mut now_val: String = "".to_string();

        let mut curly_level = 0;

        for char in final_string.chars() {
            match char {
                '{' => {
                    now_val.push(char);
                    curly_level += 1;
                }

                '}' => {
                    now_val.push(char);
                    curly_level -= 1;
                }

                ',' => {
                    if curly_level == 1 {
                        now_val.push(char);
                    } else {
                        let pushed = now_val.clone();

                        str_splitted.push(Value::from_str(&pushed.trim()).unwrap());

                        *&mut now_val = "".to_string();
                    }
                }

                _ => {
                    now_val.push(char);
                }
            }
        }

        //        println!("{:?}", str_splitted);

        //         println!("{:?}",final_string_vec);
        //       println!("{:?}",final_string_vec[0]);

        final_values_list.extend(str_splitted);
    }

    //    println!("final values:{:?}",final_values_list);

    // here we filter the list and provide the number of output that they need
    // but if the filtered list has less just send the less number list

    //    println!("reference_id-decrypted_id:{}-{:?},{}",final_values_list[0]["reference_id"], reference_id.as_ref().unwrap(), reference_id.as_ref().unwrap().eq(&final_values_list[0]["reference_id"].to_string()));

    if let Value::String(val) = &final_values_list[0]["reference_id"] {
        println!("val:{}", reference_id.as_ref().unwrap().eq(val));
    }

    let final_filtered: Vec<Value> = final_values_list
        .into_iter()
        .map(|val| val)
        .filter(|val| {
            if let Value::String(id) = &val["reference_id"] {
                reference_id.as_ref().unwrap().eq(id)
            } else {
                false
            }
        })
        .collect();

    let final_response_val;

    if final_filtered.len().cmp(&number.into()) == Ordering::Greater {
        final_response_val = final_filtered.get(0..number.to_string().parse::<usize>().unwrap());

        return Json(GetStandardResponse {
            status: 200,
            response: ResponseStatus::Success,
            data: final_response_val.unwrap().to_owned(),
        });
    }

    Json(GetStandardResponse {
        status: 200,
        response: ResponseStatus::Success,
        data: final_filtered,
    })
}

#[get("/<collection>/one/<id>")]
pub async fn get_one(collection: String, id: String, token: Token) -> Json<GetOneStandardResponse> {
    format!("The query in {collection}, and the id is {id}");

    let mut check_id: Option<String> = None;

    //we also check if this collection is usertype or not
    let env_variables: Vec<String> = env::args().collect();
    let key_str = env_variables.last().unwrap();

    let key_val = key_str.as_bytes();

    let mut vec_key = Vec::new();

    for &byte in key_val.iter() {
        vec_key.push(byte);
    }

    let key = GenericArray::from_slice(&vec_key);

    if id.to_lowercase() == "pushlogin" {
        //here we need to check for header with
        //bearer authorization and found out if the token is correct

        println!("This is token::::::::{:?}", token);

        if let Token(Some(val)) = token {
            //Decrypt the token into data

            let key: Hmac<Sha256> = Hmac::new_from_slice(&vec_key).unwrap();

            let claims: BTreeMap<String, String>;

            if let Ok(claimed_value) = val.verify_with_key(&key) {
                claims = claimed_value;
            } else {
                return Json(GetOneStandardResponse {
                    status: 400,
                    response: ResponseStatus::InvalidJWT,
                    data: Value::Null,
                });
            }

            println!("This is the id::::{}", claims["id"]);

            //here we check for expiry and if not expired then create
            //a global variable and pass in the decrypted id to it

            let not_expired = claims["expiry"].parse::<i64>().unwrap() > Utc::now().timestamp();

            if !not_expired {
                return Json(GetOneStandardResponse {
                    status: 400,
                    response: ResponseStatus::JWTExpiry,
                    data: Value::Null,
                });
            }

            check_id = Some(claims["id"].clone());
        } else {
            return Json(GetOneStandardResponse {
                status: 400,
                response: ResponseStatus::Failed,
                data: Value::Null,
            });
        }

        //but in this scope we just check date of creation and expiry
        //data
        //we just reassign id out of the token to id
    }

    let directory = String::from("/home/qubit/Documents/hadron/.data/data");

    let files = fs::read_dir(&directory).unwrap();

    //println!("{:?}",files);

    //let models_file_vec: Vec<&str> = Vec::<&str>::new();

    let mut required_model_file: Option<String> = None;

    let mut total_number_of_files = 0;

    //by the way this by_ref as took the value out of the files so may want to look into this
    //sometime soon

    //  println!("{:?}", files.by_ref().count());

    for file in files {
        let string_file_val = file.expect("well there\'s something fishy").path();

        let string_format = string_file_val
            .into_os_string()
            .into_string()
            .expect("format change error");

        let splitted = string_format.split("/").collect::<Vec<&str>>();

        let file_name = splitted[splitted.len() - 1];

        // println!("{:?}", file_name);

        let name_splitted = file_name.split(".").collect::<Vec<&str>>();

        //        println!("{:?}",name_splitted);

        //       println!("{}", collection.to_lowercase());

        if name_splitted[0].split('-').collect::<Vec<&str>>()[0]
            == collection.to_lowercase().as_str()
        {
            required_model_file = Some(name_splitted[0].to_string());
            println!("matched");

            total_number_of_files += 1;
        }
    }

    if required_model_file == None {
        let res = GetOneStandardResponse {
            status: 404,
            response: ResponseStatus::NotFound,
            data: Value::Null,
        };

        return Json(res);
    }

    //If got time, the file read will be made multi-threaded but for now
    //It's just single threaded

    println!("{:?}", env_variables.last().unwrap());

    let mut deletedData: Option<Value> = None;

    let mut found_file_data: Option<Vec<Value>> = None;

    let mut patching_index: Option<usize> = None;

    let mut found_document_num = 0;

    for i in 1..(total_number_of_files + 1) {
        let file_name: String;

        println!("{}", i);
        if i == 1 {
            file_name = collection.clone().to_lowercase() + ".dat";
        } else {
            file_name = collection.clone().to_lowercase() + "-" + &(i - 1).to_string() + ".dat";
        }

        println!("{}", file_name);

        //here we read file and decrypt and push to some outer scope vector

        //        let directory = String::from("/home/qubit/Documents/hadron/.data/data");

        let current_chosen_file = fs::read(directory.clone() + "/" + &file_name).unwrap();

        // let mut block:GenericArray<u8,U16> = GenericArray::from([0u8;16]);

        let mut blocks: Vec<GenericArray<u8, U16>> = Vec::new();

        let mut counter = 0;

        println!("total_val:{:?}", &current_chosen_file);
        println!("val:{:?}", &current_chosen_file[0..16]);

        loop {
            //println!("counterrr:{}, {}",counter,contents.len());

            blocks.push(
                GenericArray::from_slice(&current_chosen_file[counter..(counter + 16)]).clone(),
            );

            //println!("{}",&counter);

            counter = counter + 16;

            if counter == current_chosen_file.len() {
                //println!("{counter}");

                break;
            }
        }

        // let mut block:GenericArray<u8,U16> = GenericArray::from_slice(&contents).clone();
        //  let mut block:GenericArray<u8,U16> = GenericArray::from_slice(&vec_key).clone();

        let cipher = Aes128::new(&key);

        // cipher.decrypt_block(&mut block);

        cipher.decrypt_blocks(&mut blocks);

        let mut final_string: String = String::from("");

        for block in blocks {
            *&mut final_string += &String::from_utf8(block.to_vec()).unwrap();
        }

        println!("{}", final_string);
        //Now the string need to split with , but only from the area where there is no {}
        //might have to write custom splitter

        //        let final_string_vec:Vec<&str> = final_string.split(',').collect::<Vec<&str>>();

        // Custom comma separator////////////////////
        //

        let mut str_splitted: Vec<Value> = Vec::new();

        let mut now_val: String = "".to_string();

        let mut curly_level = 0;

        for char in final_string.chars() {
            match char {
                '{' => {
                    now_val.push(char);
                    curly_level += 1;
                }

                '}' => {
                    now_val.push(char);
                    curly_level -= 1;
                }

                ',' => {
                    if curly_level == 1 {
                        now_val.push(char);
                    } else {
                        let pushed = now_val.clone();

                        str_splitted.push(Value::from_str(&pushed.trim()).unwrap());

                        *&mut now_val = "".to_string();
                    }
                }

                _ => {
                    now_val.push(char);
                }
            }
        }

        //        println!("{:?}", str_splitted);

        //         println!("{:?}",final_string_vec);
        //       println!("{:?}",final_string_vec[0]);

        //Here we need to search for the same id as provided in the request and if found
        //we need to replace the data with data in the last file
        //

        for (index, val) in str_splitted.iter().enumerate() {
            if let Value::Object(obj) = val {
                for (k, v) in obj {
                    if k == "id" {
                        if let Value::String(val) = v {
                            println!("val::{}", val);

                            println!("check_id::{}", check_id.as_ref().unwrap());

                            println!("{}", check_id.as_ref().unwrap().eq(&val.to_string()));

                            if val.eq(check_id.as_ref().unwrap()) {
                                found_file_data = Some(str_splitted.clone());

                                patching_index = Some(index);

                                found_document_num = i;

                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    if found_document_num == 0 {
        return Json(GetOneStandardResponse {
            status: 200,
            response: ResponseStatus::Success,
            data: Value::Null,
        });
    }

    if let Some(mut data) = found_file_data {
        data[patching_index.unwrap()]["password"] = Value::Null;

        return Json(GetOneStandardResponse {
            status: 200,
            response: ResponseStatus::Success,
            data: data[patching_index.unwrap()].clone(),
        });
    } else {
        Json(GetOneStandardResponse {
            status: 400,
            response: ResponseStatus::Failed,
            data: Value::Null,
        })
    }
}
#[get("/<collection>/custom_filter")]
pub async fn get_custom_filter(collection: String) -> String {
    format!("The custom filter is ran in {collection} with filter custom filter")
}
