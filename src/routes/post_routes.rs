use rocket::{serde::{Serialize,Deserialize, json::Json}, form::Form};
use std::fs;
use serde_json::{Value};
use std::ops::Deref;
extern crate regex;
extern crate uuid;
use uuid::Uuid;
use crate::input_and_schema_compare::comparer; 


#[derive(Debug,Clone,Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
enum ResponseStatus{
    Success,
    Failed,
    NotFound
}

#[derive(Debug,Clone,Deserialize,Serialize)]
#[serde(crate="rocket::serde")]
pub struct PostStandardResponse{
    
    status: u16,
    response: ResponseStatus


}




#[derive(Debug,Clone,Deserialize,Serialize)]
#[serde(crate="rocket::serde")]
pub struct PostStandardInputFormat{
    
    pub data: Value,

}

 fn raw_id_formatter(values :Vec<String>)-> Vec<String>{

        // now need to split by comma in the outer most layer
     
        let mut value_list = Vec::<String>::new();

        for val in values{

        let mut curly_braces_level = 0;

        let mut final_string = String::from("");



        for char in val.chars(){
            
            match char {
                    
                '{'=>{
                    
                    final_string.push(char);
                    if curly_braces_level == 0 {
                        
                        //here goes all the logic for putting id:uuid+glue($)+superposition factor
                       //and a comma at last
                    
                        let id = uuid::Uuid::new_v4();
                        let final_id_struct = String::from(r#""id":""#) + &id.to_string()+r#"","#;
                        * &mut final_string += &final_id_struct;
                        
                       


                    }

                    curly_braces_level += 1;
            
                }

                _=>{
                    
                    final_string.push(char);                    

                }




            }
            

           

        }

        value_list.push(final_string);
        
        
            
        }


        value_list
    }


/*
trait Jsonifyable {


    fn comma_formatter(&self)-> Vec<String>;

}


impl Jsonifyable for PostStandardInputFormat{
    
    fn comma_formatter(&self)-> Vec<String>{

        // now need to split by comma in the outer most layer
        // since it is post request modified version, we need to loop in the vector of strings of
        // data

        let mut returner: Vec<String> = Vec::new();

        for val in &self.data{
        
        let mut comma_level_state = 0;

        let mut curly_braces_level=0;

        let mut final_string = String::from("");

        for char in val.chars(){
            
            match char {
                    
                '{'=>{
                    
                    final_string.push(char);
                    if curly_braces_level == 0 {
                        
                        //here goes all the logic for putting id:uuid+glue($)+superposition factor
                       //and a comma at last
                    
                        let id = Uuid::new_v4();

                        let final_id_struct = String::from("id") + &id.to_string()+",";
                        * &mut final_string += &final_id_struct;
                        
                       


                    }

                    curly_braces_level += 1;
            
                }

                '['=>{
                    
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

            
      returner.push(final_string);
    }


        returner

    }

    


}
*/





#[post("/<collection>",format = "json",data="<body>")]
pub async fn post_one(collection:String, body:Json<PostStandardInputFormat>)-> Json<PostStandardResponse>{


    let directory = String::from("/home/qubit/Documents/hadron/.data/data");

    let files = fs::read_dir(&directory).unwrap();

    //println!("{:?}",files);

    //let models_file_vec: Vec<&str> = Vec::<&str>::new();
      
    let mut required_model_file:Option<String> = None;


    //by the way this by_ref as took the value out of the files so may want to look into this
    //sometime soon

  //  println!("{:?}", files.by_ref().count());

    let mut total_file_num = 0;

    for file in files{

        
        let string_file_val = file.expect("well there\'s something fishy").path();

        let string_format = string_file_val.into_os_string().into_string().expect("format change error");

        let splitted = string_format.split("/").collect::<Vec<&str>>();


        let file_name = splitted[splitted.len()-1];

       // println!("{:?}", file_name);

        let name_splitted = file_name.split(".").collect::<Vec<&str>>();

        
        println!("{:?}",name_splitted);

        println!("{}", collection.to_lowercase());

        

        if name_splitted[0].split('-').collect::<Vec<&str>>()[0] == collection.to_lowercase().as_str(){

           total_file_num += 1;

           required_model_file = Some(name_splitted[0].to_string());
            println!("matched");

            //Didn't break here cause we need to find exactly the number of files where the data
            //are stored and use our magic number(super-position factor) to have a more probable
            //pinpoint to the data
        }

            


    }

    if required_model_file == None {
        
        let res = PostStandardResponse{
            
            status: 404,
            response: ResponseStatus::NotFound

        };

        return Json(res);


    }


    //Now the file or model exists so, we are going to go through files
    //searching for initially established config file to specific models
   
//    let mut established_model_schema: Json;

    println!("total_number_of_files{:?}",total_file_num);

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
                
            


            //let value_input = body.deref().comma_formatter();



            let mut input_value_raw_formatted : Vec<String>  = Vec::new();

            if let Value::Array(ref value_input) =&body.data{


            for val in value_input{


//`          let itterable_body_data:Value = serde_json::from_str(&val).unwrap();               
             let raw_value_unit = Value::to_string(val);

             input_value_raw_formatted.push(raw_value_unit);


            }


            }
            

            println!("{:?}", input_value_raw_formatted);

            let raw_with_id=raw_id_formatter(input_value_raw_formatted);


           let mut file_name = collection.clone().to_lowercase();

            if total_file_num >1{
                
                let adder = format!("-{}",total_file_num-1);

               * &mut file_name += &adder; 


            } 



           // let itterable_body_data = &body.deref();
 
           

            
            comparer::schema_comparer_many(body.data.clone(),readFileSchema_jsonified,raw_with_id,file_name); 
            


        }
        


    }





    return Json(PostStandardResponse{ 
        status : 200,
        response : ResponseStatus::Success 

    })

}
