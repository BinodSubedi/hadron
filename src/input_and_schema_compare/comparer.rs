use serde_json::Value;

pub fn schema_comparer(input:Value, schema:Value){
    

    println!("{:?}", schema);



    if let Value::Object(obj) = schema{

            
        for (k,v) in obj.iter(){
            
            match v{
                    
                Value::String(s)=>{
                    println!("{}",s);
                }

                _=>{
                
                    println!("{:?}",v)

                }


            }



        }


    }


}

