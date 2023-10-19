fn add_space(mut n: i32, mut s: String) -> String {
    if n == 1000 {
        s.push('\n');
        return s;
    }
    while n > 0 {
        s.push(' ');
        n -= 1;
    }
    return s;
}

fn add_str(mut s: String, s_toadd: &str) -> String {
    s.push_str(&s_toadd);
    return s;
}
fn oglindit(mut n: i32) -> i32 {
    let mut ogl: i32 = 0;
    let mut cifra: i32;
    while n != 0 {
        cifra = n % 10;
        ogl = ogl * 10 + cifra;
        n = n / 10;
    }
    return ogl;
}
fn add_integer(mut s: String, number: i32) -> String {
    let mut num: String = String::from("");
    let mut char: char;
    let mut len: i32;
    let mut cifra: u8;

    let mut numar_oglindit = oglindit(number);
    while numar_oglindit != 0 {
        len = numar_oglindit % 10 + 48;
        numar_oglindit = numar_oglindit / 10;
        cifra = len as u8;
        char = cifra as char;
        num.push(char);
    }
    s.push_str(&num);
    return s;
}


fn add_float(mut s: String, number: f32) -> String {
    let mut num: String = String::from("");
    let mut number_copy: f32 = number;
    let mut char: char;
    let mut len: i32;
    let mut cifra: u8;

    let mut zecimale: f32;
    let mut count_zecimale:i32=0;
    zecimale = number % 10.0;
    let mut int :i32 = zecimale as i32;
    let mut int_float: f32 = int as f32;
    
    while zecimale != int_float {
        count_zecimale = count_zecimale +1;
        zecimale = zecimale * 10.0;
        zecimale = zecimale % 10.0;
        int = zecimale as i32;
        int_float = int as f32;
    } 
    let mut zecimale_oglindit : i32 = oglindit(zecimale as i32);

    let mut char1: char;
    let mut len1: i32;
    let mut cifra1: u8;
    let mut i:i32=0;
    while zecimale_oglindit != 0 {
        if i == count_zecimale {
            num.push('.');
            i += 1;
        }
        else {
            len1 = zecimale_oglindit % 10 + 48;
            zecimale_oglindit = zecimale_oglindit / 10;
            cifra1 = len1 as u8;
            char1 = cifra1 as char;
            num.push(char1);
            i += 1;
        }
        
    }
    
    number_copy = number_copy /10.0;
    let mut numar_oglindit = oglindit(number_copy as i32);
    while numar_oglindit != 0 {
        len = numar_oglindit % 10 + 48;
        numar_oglindit = numar_oglindit / 10;
        cifra = len as u8;
        char = cifra as char;
        num.push(char);
    }
    s.push_str(&num);
    return s;
}

fn main() {
    let mut s: String = String::from(" ");
    s = add_space(50, s);
    s = add_str(s, "I 💚");
    s = add_space(1000, s);
    s = add_space(53, s);
    s = add_str(s, "RUST.");
    s = add_space(1000, s);
    s = add_space(10, s);
    s = add_str(s, "Most");
    s = add_space(20, s);
    s = add_str(s, "create");
    s = add_space(10, s);
    s = add_integer(s, 306_437_968);
    s = add_space(10, s);
    s = add_str(s, "and");
    s = add_space(5, s);
    s = add_str(s, "lastest");
    s = add_space(10, s);
    s = add_str(s, "is");
    s = add_space(1000, s);
    s = add_space(23, s);
    s = add_str(s, "downloaded");
    s = add_space(10, s);
    s = add_str(s, "has");
    s = add_space(15, s);
    s = add_str(s, "dowloades");
    s = add_space(6, s);
    s = add_str(s, "the");
    s = add_space(10, s);
    s = add_str(s, "version");
    s = add_space(6, s);
    s = add_float(s, 0.1);
    println!("{}", s);
}
