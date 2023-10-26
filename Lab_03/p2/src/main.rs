fn checked_addition(x: u32, y: u32) {
    let max_u32 = std::u32::MAX as u64;
    let add = x as u64 + y as u64;
    if add > max_u32 {
        panic!("Rezultatul trece de u32! -- overflow");
    } else {
        print!("{x}+{y}={add}\n");
    }
}

fn checked_multiplication(x: u32, y: u32) {
    let max_u32 = std::u32::MAX as u64;
    let multi = x as u64 * y as u64;
    if multi > max_u32 {
        panic!("Rezultatul trece de u32 -- overflow!");
    } else {
        print!("{x}x{y}={multi}\n");
    }
}

fn main() {
    checked_addition(12323, 12321);
    checked_multiplication(126534, 63547);
}
