use std::fs;
fn inlocuire(path: String) {
    let file = fs::read_to_string(&path).expect("Path Gresit");
    let mut file1= file.replace(" ", " ");
    let prescurtari = [
        ("dl", "domnul"),
        ("pt", "pentru"),
        ("dna", "doamna"),
        ("ptr", "pentru"),
    ];
    for cuvant in file.split(" ") {
        for i in prescurtari {
            if cuvant == i.0 {
                file1 = file1.replace(cuvant, i.1);
            }
            
        }
       
    }
    fs::write(path, &file1).expect("Erroare la write");
}

fn main() {
    inlocuire("text".to_string());
}
