use rocket::{serde::{Serialize,Deserialize, json::Json}, form::Form};
use std::fs;
use serde_json::{Value};
use std::ops::Deref;
extern crate regex;


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

// Creating a trait to be implemented to PutStandardInputFormat
// for converting plain string to jsonifyable data format

trait Jsonifyable {

    fn changeMe(&self)-> String;

     //   let fitered_list: Vec<&str> = self.data.split(",").collect::vec<&str>();


}


impl Jsonifyable for PutStandardInputFormat<'_>{

    fn changeMe(&self)-> String{
            
       // let filtered_list: Vec<&str> = self.data.split(",").collect::<Vec<&str>>();
        
        //println!("{:?}", filtered_list);


        let regex_pattern = regex::Regex::new(r"[,{}]").unwrap();
        let list_all: Vec<&str> = regex_pattern.split(self.data).collect();
            
        //println!("{:?}", list_all);

        let cleared_out_string_vec = &list_all[1..(list_all.len()-1)];

      //  println!("{:?}", cleared_out_string_vec);

        
        let mut final_val = String::from(r#"{"#);


        for (i,val) in cleared_out_string_vec.iter().enumerate(){
            
            let splitted_unit: Vec<_> = val.split(":").collect();
            
            let referenced = &mut final_val;

            println!("index_here:{i}");

            if i != (cleared_out_string_vec.len()-1) {
            
                *referenced  += format!(r#""{}":"{}","#,splitted_unit[0],splitted_unit[1]).as_str();

            }else{

                *referenced  += format!(r#""{}":"{}""#,splitted_unit[0],splitted_unit[1]).as_str();
           

        }





            }

        final_val += "}";

       println!("{:?}", final_val);

        

        final_val

    }


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


    //Now the file or model exists so, we are going to go through files
    //searching for initially established config file to specific models
   
//    let mut established_model_schema: Json;

    const config_file_location: &str = "./";

    let dirRead = fs::read_dir(config_file_location).expect("diretory read failed");
//    println!("{}",dirRead.count());
    for file in dirRead{
            
//        println!("{:?}",file.expect("error reading file").path());

        let mut config_file_path = file.expect("error while reading filr").path();

        let config_file_arr = config_file_path.to_str().expect("error while converting to str").split("/").collect::<Vec<&str>>();

//        println!("{:?}",config_file_arr.last());

        let config_file_exact = config_file_arr.last().unwrap().to_string();
        
        // Well I kinda forgot that we also have a normal schema which is in the non-config file
        // so we could have equated to that too, but anyways we will read botht the schema file and
        // the config file

        if (collection.to_lowercase().clone()+"_config.json") == config_file_exact{
    
            println!("Found the config file!!");

            println!("{}",&config_file_exact);
//            println!("{}",format!("./{}.json",&collection.to_lowercase()));

            let readFileConf = String::from_utf8(fs::read(format!("./{}", &config_file_exact)).unwrap()).unwrap();
            let readFileSchema = String::from_utf8(fs::read(format!("./{}.json", &collection.to_lowercase())).unwrap()).unwrap();

            
    //         println!("{:?}", readFileConf);
 //           println!("{:?}", readFileSchema);
            
            let readFileConf_jsonified: Value = serde_json::from_str(&readFileConf).unwrap();
            println!("{:?}", readFileConf_jsonified);
           
            let readFileSchema_jsonified: Value = serde_json::from_str(&readFileSchema).unwrap();
            println!("{:?}", readFileSchema_jsonified);
//            println!("{:?}", readFileSchema_jsonified["age"]["type"]);


            // 1.)Now below this section is supposed to be input value field and type checking
            // 2.)Encrypting by applying padding (total bytes%16, remainder + (16-remainder) =>
            //   which will be the padding)
            // 3.) Decrypting back jsut to check if worked properly

                
           // println!("{:?}", body);


           // let itterable_body_data:Value = serde_json::from_str(&body.data).unwrap();
           // let itterable_body_data = &body.deref();
            let itterable_body_data = &body.deref().clone();
 
            println!("{:?}", &itterable_body_data.changeMe());
           
            let itterable_final_val:Value = serde_json::from_str(&itterable_body_data.changeMe()).unwrap();

            println!("{:?}", itterable_final_val);



        }
        


    }

    let res = PutStandardResponse{
        status: 200,
        response: ResponseStatus::Success

    };

    Json(res)

}
