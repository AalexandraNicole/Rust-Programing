use anyhow::Result;
use std::{array, fs};
#[derive(Debug, Clone)]
struct Person {
    name: String,
    number: String,
    age: String,
}

fn ex1(path: String) -> Result<String> {
    let mut oldest: Person = Person {
        name: ("name".to_string()),
        number: ("number".to_string()),
        age: ("0".to_string()),
    };
    let mut youngest: Person = Person {
        name: ("name".to_string()),
        number: ("number".to_string()),
        age: ("255".to_string()),
    };
    let file = fs::read_to_string(path)?;
    for v in file.split("\n") {
        let mut nume: String = String::new();
        let mut numar: String = String::new();
        let mut varsta: String = String::new();
        for (j, i) in v.split(',').enumerate() {
            if j == 0 {
                nume = i.clone().to_string();
            }
            if j == 1 {
                numar = i.clone().to_string();
            }
            if j == 2 {
                varsta = i.clone().to_string();
            }
        }
        let current = Person {
            name: nume,
            number: numar,
            age: varsta,
        };

        if current.age < youngest.age {
            youngest = current.clone();
        }
        if current.age > oldest.age {
            oldest = current.clone();
        }
    }
    let response = oldest.name + " " + &oldest.age + " " + &youngest.name + " " + &youngest.age;
    println!("{response}");
    Ok(response)
}

fn main() {
    let studenti = ex1("Students".to_string());
    println!("{:?}", studenti);
    return;
}
