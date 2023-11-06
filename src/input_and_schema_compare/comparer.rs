use serde_json::Value;
use std::collections::HashMap;

pub fn schema_comparer(input:HashMap<String,String>, schema:Value){


    for (k,v) in input{
        
        println!{"Key is:{},and value goes as:{}", k,v}

    }



}

