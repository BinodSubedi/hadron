#[get("/<collection>")]
pub async fn get_all(collection:String)-> String{
   format!("The collection of choice for whole query is {}",collection)   
}

#[get("/<collection>/many/<number>")]
pub async fn get_many(collection:String,number:i8) -> String {
   format!("The query in {}, and number of element is {}",collection,number) 
}

#[get("/<collection>/one/<id>")]
pub async fn get_one(collection:String,id:i32)-> String{
   format!("The query in {collection}, and the id is {id}")
}