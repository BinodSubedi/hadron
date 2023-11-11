use serde_json::Value;
use std::process;

pub fn schema_comparer(input:Value, schema:Value){
    

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

                    process::exit(0);
                }


            }



        }


    }

    // Now checking the type wether it's number, string or comply with model is done, Now it's time
    // to send it to encryption method and add padding if needed and store
    // Also, need to add array type for later

}

