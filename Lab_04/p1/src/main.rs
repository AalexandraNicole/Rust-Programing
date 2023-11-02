use std::fs;
fn p1(path: String) -> (String, String) {
    let file = fs::read_to_string(path).expect("Path Gresit");
    let mut bytes: String = String::new();
    let mut longest: String = String::new();
    for v in file.split("\n") {
        if bytes.len() < v.len() {
            bytes = v.to_string();
        }
        if longest.chars().count() < v.chars().count() {
            longest = v.to_string();
        }
    }

    return (longest, bytes);
}

fn main() {
    let a = p1("fisier".to_string());
    println!("{}\n {}\n", a.0, a.1);
}
