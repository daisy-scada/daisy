use serde_json::{Result};
use serde::{Serialize, Deserialize};

use std::fs::File;
use std::io::BufReader;

// fn untyped_example() -> Result<()> {
//     // Some JSON input data as a &str. Maybe this comes from the user.
//     let data = r#"
//         {
//             "name": "John Doe",
//             "age": 43,
//             "phones": [
//                 "+44 1234567",
//                 "+44 2345678"
//             ]
//         }"#;
//
//     // Parse the string of data into serde_json::Value.
//     let v: Value = serde_json::from_str(data)?;
//
//     // Access parts of the data by indexing with square brackets.
//     println!("Please call {} at the number {}", v["name"], v["phones"][0]);
//
//     Ok(())
// }
//
// fn cfg() -> Result<()> {
//
//     let contents = fs::read_to_string("./cfg.json").expect("some wrong");
//
//     let v: Value = serde_json::from_str(contents.as_str())?;
//
//     println!("cfg:{}",v["projects"][0]);
//
//     Ok(())
// }



#[derive(Serialize,Deserialize,Debug)]
struct Connection {
  address:String,
  properties:String,
}

#[derive(Serialize,Deserialize,Debug)]
struct Protocol {
    name: String,
    connection: Connection,
}

#[derive(Serialize,Deserialize,Debug)]
struct Ds {
    name: String,
    protocol: Protocol,
}

#[derive(Serialize,Deserialize,Debug)]
struct Project {
    name: String,
    dss: Vec<Ds>,
}

#[derive(Serialize,Deserialize,Debug)]
struct DaisyCfg {
    name: String,
    projects: Vec<Project>,
}

fn cfg_des() -> Result<DaisyCfg> {
    let file = File::open("cfg.json").unwrap();
    let reader = BufReader::new(file);
    let d: DaisyCfg = serde_json::from_reader(reader)?;
    Ok(d)
}

#[tokio::main]
async fn main() {
    let d = cfg_des().unwrap();
    println!("{:#?}", d);
}