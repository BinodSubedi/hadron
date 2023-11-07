use rocket::{serde::{Serialize,Deserialize, json::Json}, form::Form};
use std::fs;
use std::collections::HashMap;
use serde_json::{Value};
use std::ops::Deref;
extern crate regex;
use crate::input_and_schema_compare::comparer; 

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

    fn changeMe(&self)-> HashMap<String,String>;

     //   let fitered_list: Vec<&str> = self.data.split(",").collect::vec<&str>();

    fn comma_formatter(&self)-> String;

}


impl Jsonifyable for PutStandardInputFormat<'_>{
    
    fn comma_formatter(&self)-> String{

        // now need to split by comma in the outer most layer
        
        let mut comma_level_state = 0;

        let mut final_string = String::from("");

        for char in self.data.chars(){
            
            match char {
                    
                '{'|'['=>{
                    
                    final_string.push(char);
            
                }

                ':' | ']' | '}' | ','=>{
                    if comma_level_state == 1{ 
                    comma_level_state -= 1;
                    final_string.push('"');
                    final_string.push(char);
                    }else{

                    final_string.push(char);

                    }


                }

                _=>{
                    if comma_level_state == 0{
                        
                        final_string.push('"');
                        final_string.push(char);
                        comma_level_state += 1;

                    }else{
                        
                        final_string.push(char);

                    }



                }



            }

           

        }

            
        final_string

    }

    fn changeMe(&self)-> HashMap<String,String>{
            
       // let filtered_list: Vec<&str> = self.data.split(",").collect::<Vec<&str>>();
        
        println!("normal-data{:?}", self.data);
            
        
        let mut changed_few = String::from("");

        for char in self.data.chars(){
            
           // println!("{}",char);

            if char != '"'{

                let val = &mut changed_few;
            
                *val+= &char.to_string();
            
            }

        }

        println!("{}",changed_few);

        let regex_pattern = regex::Regex::new(r"[,{}:]").unwrap();
        let list_all: Vec<&str> = regex_pattern.split(&changed_few).collect();
            
        println!("{:?}", list_all);

        let cleared_out_string_vec = &list_all[1..(list_all.len()-1)];

       println!("{:?}", cleared_out_string_vec);

        
        let mut final_val = HashMap::new();
        
        let mut counter = 0;

        loop{

            println!("counter-breakPoint:{}",cleared_out_string_vec.len());    

            if counter >= (cleared_out_string_vec.len()-1){
                
                break;

            }

            //let referenced = &mut final_val;

            final_val.insert(cleared_out_string_vec[counter].to_string(), cleared_out_string_vec[counter+1].to_string());

            counter = counter + 2;
            println!("{}",counter);


        }

       /* for inside in cleared_out_string_vec.iter().enumerate(){
            
           // let splitted_unit: Vec<_> = val.split(":").collect();
            
           // let referenced = &mut final_val;

            println!("index_here:{i}");

            
             //   *referenced  += format!(r#""{}":"{}","#,splitted_unit[0],splitted_unit[1]).as_str();
                final_val.insert(splitted_unit[0].to_string(),splitted_unit[1].to_string());






            }  */


       println!("final___{:?}", final_val);

        

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
    println!("something");
    for file in dirRead{
            
     //   println!("{:?}",file.expect("error reading file").path());

        let mut config_file_path = file.expect("error while reading filr").path();

        let config_file_arr = config_file_path.to_str().expect("error while converting to str").split("/").collect::<Vec<&str>>();

      //  println!("{:?}",config_file_arr.last());

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

            

 //           println!("{:?}", readFileConf_jsonified.changeMe());
           
            let readFileSchema_jsonified: Value = serde_json::from_str(&readFileSchema).unwrap();

/*

            if let Value::Object(obj) = cheking_context{
                
                for (k,v) in obj.iter() {
                    
                    println!("If this works I will look very stupid:{},{}",k,v);

                }

            }

*/
 //           println!("{:?}", readFileSchema_jsonified.changeMe());
//            println!("{:?}", readFileSchema_jsonified["age"]["type"]);
//


            // 1.)Now below this section is supposed to be input value field and type checking
            // 2.)Encrypting by applying padding (total bytes%16, remainder + (16-remainder) =>
            //   which will be the padding)
            // 3.) Decrypting back jsut to check if worked properly

                
        //    println!("{:?}", body);
        //
        //
                
            


            let value_input = body.deref().comma_formatter();


            let itterable_body_data:Value = serde_json::from_str(&value_input).unwrap();
           // let itterable_body_data = &body.deref();
 
           

            
            comparer::schema_comparer(itterable_body_data,readFileSchema_jsonified); 
            


        }
        


    }

    let res = PutStandardResponse{
        status: 200,
        response: ResponseStatus::Success

    };

    Json(res)

}
