use serde_json::Value;
use crate::padd_encrypt_persist::padd_encrypt_persist;


pub fn schema_comparer(input:Value, schema:Value, raw:String,file_name:String,total_num_values:usize){
    

    println!("{:?}", schema);



    if let Value::Object(obj) = schema{

            
        for (k,v) in obj.iter(){
            
            match v{
                    
                Value::String(s)=>{
                    println!("value of s:{}",s);
                    
                if let Value::String(value) = input.get(k).unwrap(){
                    
                    if s.to_lowercase() == "number"{
                        
                        value.parse::<i32>().unwrap();

                    } 


                }
                
                }

                Value::Object(ob)=>{

                    println!("{:?}",ob);
                
                    println!("{:?}", ob.get("type"));                    

                    if let Value::String(value) = ob.get("type").unwrap(){
                        
                    
                        println!("{}",value);

                        println!("{:?}",input.get(k).unwrap());

                        if value.to_lowercase() == "string"{
                        

                            println!("string type here{:?}",input.get(k).unwrap().is_string());



                        }else if value.to_lowercase() == "number" {

                                
                             println!("Number type here{:?}",input.get(k).unwrap().is_string());                               

                            
                             if let Value::String(number_val) = input.get(k).unwrap(){

                            
                                 number_val.parse::<i32>().unwrap();
                                    
                             }

                        }  

                            



                    }

                }


                _=> {

                    panic!("There seems to be some problem with input format!");
                }


            }



        }


    }

    // Now checking the type wether it's number, string or comply with model is done, Now it's time
    // to send it to encryption method and add padding if needed and store
    // Also, need to add array type for later


    println!("{}",file_name);

    padd_encrypt_persist(vec![raw],file_name,total_num_values);

}

pub fn schema_comparer_many(input_list:Value, schema:Value, raw:Vec<String>,file_name:String,total_num_files:usize){
    

    println!("{:?}", schema);

    if let Value::Array(input_list_values) = input_list{



    for input in input_list_values{


    if let Value::Object(ref obj) = schema{

            
        for (k,v) in obj.iter(){
            
            match v{
                    
                Value::String(s)=>{
                    println!("value of s:{}",s);
                    
                if let Value::String(value) = input.get(k).unwrap(){
                    
                    if s.to_lowercase() == "number"{
                        
                        value.parse::<i32>().unwrap();

                    } 


                }
                
                }

                Value::Object(ob)=>{

                    println!("{:?}",ob);
                
                    println!("{:?}", ob.get("type"));                    

                    if let Value::String(value) = ob.get("type").unwrap(){
                        
                    
                        println!("{}",value);

                        println!("{:?}",input.get(k).unwrap());

                        if value.to_lowercase() == "string"{
                        

                            println!("string type here{:?}",input.get(k).unwrap().is_string());



                        }else if value.to_lowercase() == "number" {

                                
                             println!("Number type here{:?}",input.get(k).unwrap().is_string());                               

                            
                             if let Value::String(number_val) = input.get(k).unwrap(){

                            
                                 number_val.parse::<i32>().unwrap();
                                    
                             }

                        }  

                            



                    }

                }


                _=> {

                    panic!("There seems to be some problem with input format!");
                }


            }



        }


    }

}

    }


    // Now checking the type wether it's number, string or comply with model is done, Now it's time
    // to send it to encryption method and add padding if needed and store
    // Also, need to add array type for later


    padd_encrypt_persist(raw,file_name,total_num_files);

}








