#[macro_use]
extern crate nickel;
extern crate serde_json;

use serde_json::{Value, Error};

use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};
use nickel::status::StatusCode::{self};

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

// bson
use bson::{Bson, Document};
use bson::oid::ObjectId;


// MongoDB
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::error::Result as MongoResult;


fn hello_world<'mw>(_req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {

    //Connect to the database
    let client = Client::connect("localhost", 27017)
        .ok().expect("Error establishing connection.");
    // The users collection
    let coll = client.db("rust-mailbox").collection("messages");
    let mut cursor = coll.find(None, None).unwrap();
    let mut data_result = "{\"messages\":[".to_owned();
    let mut flag = true;
    for result in cursor {
        if let Ok(item) = result {
            if let Some(&Bson::String(ref message)) = item.get("message") {
                if flag{
                    data_result.push_str("'");
                    data_result.push_str(message);  
                    data_result.push_str("'");
                    flag = false;
                }else{
                    data_result.push_str(", '");
                    data_result.push_str(message);
                    data_result.push_str("'");
                }
                println!("message: {}", message);
            }
        }
    }
    data_result.push_str("]}");
    println!("{}",data_result); 
    res.send(data_result)
}

fn main(){
    let mut server = Nickel::new();
    server.get("/mails", hello_world);
    server.listen("127.0.0.1:6767").unwrap();

}