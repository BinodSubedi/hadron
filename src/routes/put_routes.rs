use rocket::{serde::{Serialize,Deserialize, json::Json}, form::Form};
use std::fs;


#[derive(Debug,Clone,FromForm,Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PutStandardInputFormat<'a>{
    data: &'a str,
}


#[derive(Debug,Clone,Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
enum ResponseStatus{
    Success,
    Failed,
    NotFound
}

#[derive(Debug,Clone,Deserialize,Serialize)]
#[serde(crate="rocket::serde")]
pub struct PutStandardResponse{
    
    status: u16,
    response: ResponseStatus


}


#[put("/<collection>",format = "json",data="<body>")]
pub async fn put_one(collection:String, body:Json<PutStandardInputFormat<'_>>)-> Json<PutStandardResponse>{
//pub async fn put_one(collection:String, form:Form<PutStandardInputFormat<'_>>)-> Json<PutStandardResponse>{
    //println!("{:?}",form);
     
  //  simple_logger::init_with_level(log::Level::Warn).unwrap();

    //log::warn!("just looking");

    

//    println!("{:?}", data);
        
    //result
    //String::from("Sent")
    //
    

    let directory = String::from("/home/qubit/Documents/hadron/.data/data");

    let files = fs::read_dir(&directory).unwrap();

    //println!("{:?}",files);

    //let models_file_vec: Vec<&str> = Vec::<&str>::new();
      
    let mut required_model_file:Option<String> = None;


    //by the way this by_ref as took the value out of the files so may want to look into this
    //sometime soon

  //  println!("{:?}", files.by_ref().count());


    for file in files{

        
        let string_file_val = file.expect("well there\'s something fishy").path();

        let string_format = string_file_val.into_os_string().into_string().expect("format change error");

        let splitted = string_format.split("/").collect::<Vec<&str>>();


        let file_name = splitted[splitted.len()-1];

       // println!("{:?}", file_name);

        let name_splitted = file_name.split(".").collect::<Vec<&str>>();

        
        println!("{:?}",name_splitted);

        println!("{}", collection.to_lowercase());

        if name_splitted[0] == collection.to_lowercase().as_str(){

           required_model_file = Some(name_splitted[0].to_string());
            println!("matched");
            break;

        }

            


    }

    if required_model_file == None {
        
        let res = PutStandardResponse{
            
            status: 404,
            response: ResponseStatus::NotFound

        };

        return Json(res);


    }

    let res = PutStandardResponse{
        status: 200,
        response: ResponseStatus::Success

    };

    Json(res)

}
