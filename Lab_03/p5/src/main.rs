#[derive(PartialEq, Eq, Debug)]

enum Eroare {
    DivByZero,
}

fn impartire(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

fn impartire_2(x: i32, y: i32) -> Result<i32, Eroare> {
    if y == 0 {
        Err(Eroare::DivByZero)
    } else {
        Ok(x / y)
    }
}
fn main() {
    impartire(100, 12);
    impartire(234, 0);
    let r1 = impartire_2(13, 4);
    println!("{:?}", r1);
    let r2 = impartire_2(1212, 0);
    println!("{:?}", r2);
}
