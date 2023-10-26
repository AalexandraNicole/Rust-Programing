fn is_prime(n: u32) -> bool {
    if n == 0 || n == 1 || n == 2 {
        return false;
    }
    let mut i = 2;
    while i < n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}

fn next_prime(x: u16) -> Option<u16> {
    let mut numar = x as u32 + 1;
    let maxim_u16 = std::u16::MAX as u32;

    while numar < maxim_u16 {
        if is_prime(numar) {
            return Some(numar as u16);
        }
        numar += 1;
    }

    return None;
}

fn main() {
    let mut i = 65510 as u16;

    loop {
        let x = next_prime(i as u16);

        match x {
            Some(numar) => {
                println!("Urmatorul numar prim dupa {i} este {numar}\n",);
            }
            None => {
                print!("{i} - None in u16\n");
                break;
            }
        }
        i += 1;
    }
}
