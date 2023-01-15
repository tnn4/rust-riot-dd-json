// Example parse of championsFull.json from Riot's Data Dragon assets

use std::fs;

use std::path::Path;

use std::error::Error;

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::{Result,Value};

/*
#[derive(Serialize, Deserialize)]
struct DdJson { // Dd: data dragon
    // field-name: type
    _type: String,
    _format: String,
    version: String,
    // dd_data
}

struct DdJsonError;

impl FromStr for DdJson {
    type Err = DdJsonError;
    
    fn from_str(s: &str) -> Result<Self> {


        Ok(DdJson {
            _type: "_type".to_string(),
            _format: "_format".to_string(),
            version: "version".to_string()
        })
    }
}

struct DdData {

}
*/
/* serde_json::Value
https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html
enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}
 */
fn main() -> Result<() /*, Box<dyn Error> */> {

    println!("Hello, Data Dragon world!");
    let path: String = "./data/championFull.json".to_string();

    let path2 = std::path::Path::new("./data/championFull.json");
    let path2_parent  = path2.parent().unwrap();
    let path2_p2 = path2_parent.parent().unwrap();
    println!("path2_parent: {}", path2_parent.display());
    println!("path2_p2: {}", path2_p2.display());

    let data = fs::read_to_string(path)
        .expect("[Error]: file could not be opened");
    let v: Value = serde_json::from_str(&data)?;
    
    println!("type: {}", v["type"]);
    println!("format: {}", v["format"]);
    println!("version: {}", v["version"]);
    
    // data:
    // Get data for first champion alphabetically (Aatrox)
    println!("-- DATA --");
    // println!("aatrox: {}", v["data"]["Aatrox"]);
    let aatrox = &v["data"]["Aatrox"];
    let aatrox_image = &aatrox["image"];
    println!("aatrox skin 1: {}", v["data"]["Aatrox"]["skins"][0]);
    let aatrox_skins = &v["data"]["Aatrox"]["skins"]; // a json array
    
    // Values have to be manually transformed to desired object
    assert!(aatrox_skins.is_array()); // return TRUE if value is an array
    let aatrox_skins_array = aatrox_skins.as_array().unwrap();
    println!("-- SKINS LIST --");
    for skin in aatrox_skins_array {
        println!("{}", skin);
    }
    /*    
        for skin in aatrox_skins.iter() {
        println!("skin [{}]: {}", skin);
        idx+=1;
    } 
    */


    
    Ok(())
}