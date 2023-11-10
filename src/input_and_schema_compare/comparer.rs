use serde_json::Value;
use std::process;

pub fn schema_comparer(input:Value, schema:Value){
    

    println!("{:?}", schema);



    if let Value::Object(obj) = schema{

            
        for (k,v) in obj.iter(){
            
            match v{
                    
                Value::String(s)=>{
                    println!("{}",s);
                }

                Value::Object(ob)=>{

                    println!("{:?}",ob);
                
                    println!("{:?}", ob.get("type"));                    

                    if let Value::String(value) = ob.get("type").unwrap(){
                        
                    
                        println!("{}",value);


                    }

                }


                _=> {

                    process::exit(0);
                }


            }



        }


    }


}

