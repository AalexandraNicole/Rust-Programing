fn root13(text: String) -> String {
    let mut root13text: String = String::from(" ");
    for i in text.chars() {
        let mut charr: char = i;

        if i > 'A' && i < 'M' {
            let mut aux: u8;
            aux = i as u8;
            aux = aux + 13;
            charr = aux as char;
        }
        if i > 'M' && i < 'Z' {
            let mut aux: u8;
            aux = i as u8;
            aux = aux - 13;
            charr = aux as char;
        }
        if i > 'a' && i < 'm' {
            let mut aux: u8;
            aux = i as u8;
            aux = aux + 13;
            charr = aux as char;
        }
        if i > 'm' && i < 'z' {
            let mut aux: u8;
            aux = i as u8;
            aux = aux - 13;
            charr = aux as char;
        }
        if i == ' ' {
            charr = ' ';
        }
        let caracter = charr.to_string();
        root13text = root13text + &caracter;
    }
    return root13text;
}

fn main() {
    let string: String = String::from("HELLO URYYB");
    let conversed_strin = root13(string);
    print!("{}\n", conversed_strin);
}
