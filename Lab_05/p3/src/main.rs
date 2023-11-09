use anyhow::Result;
use serde_derive::Deserialize;
use std::{array, fs};

#[derive(Debug, Deserialize)]
struct JsonEntry {
    name: Option<String>,
    category: Option<String>,
    group: Option<String>,
    html_code: Vec<String>,
    unicode: Vec<String>,
    //mod_report: Option<Vec<ModReport>>,
    variants: Option<Variants>,
}

#[derive(Debug, Deserialize, Clone)]
struct Variants {
    fild: Option<String>,
}
fn ex3() -> Result<(), anyhow::Error> {
    let body = fs::read_to_string("input2.json")?;
    //print!("{}", body);
    let created_json = serde_json::from_str::<Vec<JsonEntry>>(&body).unwrap();

    // match &created_json[0].mod_report.is_empty(){
    // 	Some(s)=>print!("merge"),
    // 	None=>print!("no"),
    // }
    print!(
        "{}",
        &created_json[0].variants.clone().unwrap().fild.is_none()
    );
    Ok(())
}
fn main() {
    let ex = ex3();
}
