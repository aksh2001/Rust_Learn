use std::{collections::HashMap, error};



struct Database{ // Stack allocated
    // field1 : String,
    //field2 : u8  ---> u8 is a variables that takes 8 bytes of memory

    map:HashMap<String, String>,
}
impl Database{  // Implementation of Database
    fn new() -> Result<Database, std::io::Error>{ // This is like Constructor
        // read kv.db
        // let contents = match std::fs::read_to_string("kv.db"){
        //     Ok(c) => c,
        //     Err(error)=> {
        //         return Err(error);
        //     }
        // };
        let contents = std::fs::read_to_string("kv.db")?;
        // The above statement spans to the whole match std statements it is so common in rust that they added a ?
        // parse the string



        // populate our map 
        Ok(Database { 
            map:HashMap::new(),
        })
    }

}
fn main() {
    // println!("Hello, world!");
    let mut argument = std::env::args().skip(1);

    // let key = argument.next().unwrap(); 
    let key = argument.next().expect("key was not here"); 
    let value = argument.next().unwrap();
    println!("The key is '{}' and the value is '{}'", key, value);
    let contents = format!("{}\t{}\n", key, value);
    
    // let write_result =  ...// This writes on the file
    
    //std::fs::write("kv.db", contents).unwrap();

    let database = Database::new().expect("Database::new() crashed");
    // This is like try and catch in java
    // match write_result{
    //     Ok(()) => {

    //     } 
    //     Err(e){

    //     }
    // }162534
    let db = Database::new();
}


