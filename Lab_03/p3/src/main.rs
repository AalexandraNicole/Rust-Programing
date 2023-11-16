#[derive(Debug)]
enum Error {
    OverFlow,
}

fn checked_addition(x: u32, y: u32) -> Result<u32, Error> {
    let max_u32 = std::u32::MAX as u64;
    let add = x as u64 + y as u64;
    if add > max_u32 {
        Err(Error::OverFlow)
    } else {
        Ok(add as u32)
    }
}

fn checked_multiplication(x: u32, y: u32) -> Result<u32, Error> {
    let max_u32 = std::u32::MAX as u64;
    let multi = x as u64 * y as u64;
    if multi > max_u32 {
        Err(Error::OverFlow)
    } else {
        Ok(multi as u32)
    }
}

fn main() {
    let sum:Result<u32, Error> = checked_addition(12323, 12321);
    println!("Suma = {:?}", sum);
    let multi = checked_multiplication(126534, 63547);
    println!("{:?}", multi);
}
