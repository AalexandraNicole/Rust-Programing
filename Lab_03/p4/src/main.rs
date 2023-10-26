#[derive(PartialEq, Debug)]
enum Error {
    CharacterIsNotAscii,
    CharacterIsNotDigit,
    CharacterIsNotBase16Digit,
    CharacterIsNotLetter,
    CharacterIsNotPrintable,
}

fn to_uppercase(caracter: char) -> Result<char, Error> {
    if (caracter < 'A' || caracter > 'Z') && (caracter < 'a' || caracter > 'z') {
        return Err(Error::CharacterIsNotLetter);
    } else {
        let c = caracter as u8 - ' ' as u8;
        let c_upper = c as char;
        if caracter >= 'A' || caracter <= 'Z' {
            println!("{caracter}");
            return Ok(caracter);
        } else {
            //println!("{c_upper}");
            Ok(c_upper)
        }
    }
}
fn to_lowercase(caracter: char) -> Result<char, Error> {
    if (caracter < 'A' || caracter > 'Z') && (caracter < 'a' || caracter > 'z') {
        return Err(Error::CharacterIsNotLetter);
    } else {
        let c = caracter as u8 + ' ' as u8;
        let c_upper = c as char;
        if caracter >= 'a' || caracter <= 'z' {
            println!("{caracter}");
            return Ok(caracter);
        } else {
            println!("{c_upper}");
            Ok(c_upper)
        }
    }
}
fn print_char(caracter: char) -> Result<char, Error> {
    if (caracter >= 'a' && caracter <= 'z')
        || (caracter >= 'A' && caracter <= 'Z')
        || (caracter >= '0' && caracter <= '9')
        || (caracter == ' ')
    {
        println!("{caracter}");
    } else {
        return Err(Error::CharacterIsNotPrintable);
    }
    Ok(caracter)
}

fn char_to_number(caracter: char) -> Result<u8, Error> {
    let ascii = caracter as u8;
    if ascii < 1 || ascii > 127 {
        return Err(Error::CharacterIsNotAscii);
    }
    if caracter <= '0' || caracter >= '9' {
        return Err(Error::CharacterIsNotDigit);
    } else {
        print!("The decimal number for {caracter} is {} \n", ascii);
        Ok(ascii)
    }
}
fn char_to_number_hex(caracter: char) -> Result<String, Error> {
    let ascii = caracter as u8;
    if ascii < 1 || ascii > 127 {
        return Err(Error::CharacterIsNotAscii);
    }
    if (caracter < '0' || caracter > '9') && (caracter < 'A' || caracter > 'F') {
        return Err(Error::CharacterIsNotDigit);
    } else {
        print!("{} \n", ascii as char);
        let hex_string = format!("The hex number for {caracter} is : {:x}", ascii);
        return Ok(hex_string);
    }
}
fn print_error_2(eroare: Result<char, Error>) {
    match eroare {
        Err(eroare) => println!("{:#?}", eroare),
        Ok(rez) => println!("{rez}"),
    }
}

fn print_error(eroare: Error) {
    if eroare == Error::CharacterIsNotAscii {
        println!(
            "Caracterul introdus nu este Ascii / Nu apartine intervalului numeric (0,127) ..."
        );
    }
    if eroare == Error::CharacterIsNotBase16Digit {
        println!("Caracterul introdus nu este o cifra in Baza 16 ...");
    }
    if eroare == Error::CharacterIsNotDigit {
        println!("Caracterul introdus nu este o cifra! ...");
    }
    if eroare == Error::CharacterIsNotLetter {
        println!("Caracterul introdus nu este o litera! ...");
    }
    if eroare == Error::CharacterIsNotPrintable {
        println!("Caracterul introdus nu este printabil...")
    }
}

fn main() {
    print_error_2(to_uppercase('a'));
    print_error_2(to_lowercase('0'));
    let n = char_to_number('1');
    println!("{:?}", n);
    let m = char_to_number_hex('D');
    println!("{:?}", m);
    print_error_2(print_char('a'));
    print_error(Error::CharacterIsNotAscii);
}
