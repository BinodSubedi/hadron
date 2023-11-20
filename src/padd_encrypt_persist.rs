use std::str::from_utf8;


pub fn padd_encrypt_persist(input: String){

    padd(input);
}


fn padd(raw: String){
    /*

    let strr: String = String::from("hello")+ from_utf8(&[32;3]).unwrap();


    println!("{}end",strr);

    for byte in strr.bytes(){
    

        println!("{:?}", byte);

    }

    */

    let mut blocks:Vec<u8> = Vec::new();

    for &byte in raw.as_bytes().iter(){


        blocks.push(byte);

    }


    println!("blocks:{:?}",blocks);

    println!("{}",blocks.len());
    

    let remainder_padd_add_len = 16- (blocks.len() % 16);  


    blocks.extend(vec![32;remainder_padd_add_len]);


    println!("{}",blocks.len());

    println!("{:?}",blocks);



}

fn encrypt(){

}


fn persist(){


}
