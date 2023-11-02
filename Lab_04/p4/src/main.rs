use std::fs;
fn hosts(path: String) -> String {
    let file = fs::read_to_string(path).expect("Path Gresit");
    let mut rezultat: String = String::new();
    for line in file.split("\n") {
        if line.starts_with("#") {
            continue;
        }
        let mut rez: String = String::new();
        for i in line.split_whitespace() {
            rez.push_str(i);
        }
        rezultat.push_str(&rez);
    }
    return rezultat;
}

fn main() {
    let host = hosts("C:/Windows/System32/drivers/etc/hosts".to_string());
    print!("{}\n", host);
}
