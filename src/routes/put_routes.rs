use rocket::{serde::{Serialize,Deserialize}, form::Form};


#[derive(Debug,Clone,FromForm,Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct StandardInputFormat<'a>{
    Name: &'a str
}


#[put("/<collection>", data="<form>")]
pub async fn put_one(collection:String, form:Form<StandardInputFormat<'_>>)-> String{
    println!("{:?}",form.into_inner().Name);

//    println!("{:?}", data);
        
    //result
    String::from("Sent")

}
