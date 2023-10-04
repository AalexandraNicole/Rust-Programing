// - Laboratorul 1 -

// EX 1, 2, 3.

/*
Primul exercitiu
*/

fn is_prime(n: i32) {
    let mut i: i32;
    i = 2;
    if n <= 1 {
        println!("Numarul {} nu este prim!", n);
    }
    while i < n {
        if n % i == 0 {
            println!("Numarul {} nu este prim!", n);
            return;
        }
        i += 1;
    }

    println!("Numarul {} este prim!", n);
    return;
}

/*
Al-II-lea Exercitiu
*/

fn is_coprime(x: i32, y: i32) {
    let mut i: i32;
    i = 2;
    if x <= 1 || y <= 1 {
        //println!("Numerele {}, {} nu sunt coprime!",x,y);
        return;
    }
    if x <= y {
        while i <= x {
            if x % i == 0 && y % i == 0 {
                //println!("Numerele {}, {} nu sunt coprime!", x,y);
                return;
            }
            i += 1;
        }
    }

    if x >= y {
        return;
    }

    println!("Numerele {} , {} sunt coprime!", x, y);
    return;
}

/*
Al-III-lea Exercitiu
*/

fn boots() {
    let mut i: i32;
    i = 99;
    while i != 0 {
        println!("{} bottles of beer on the wall,", i);
        println!("{} bottles of beer.", i);
        println!("Take one down, pass it around,");
        if i != 1 {
            println!("{} bottles of beer on the wall.\n", i - 1);
        }
        if i == 1 {
            println!("No bottles of beer on the wall.");
            return;
        }

        i -= 1;
    }
}
fn main() {
    let mut n: i32 = 0;
    print!("PRIME");

    while n <= 100 {
        is_prime(n);
        n += 1;
    }
    let mut x: i32;
    let mut y: i32;
    x = 0;
    y = 0;

    print!("COPRIME");

    while x <= 100 {
        while y <= 100 {
            is_coprime(x, y);
            y += 1;
        }
        x += 1;
        y = 0;
    }

    boots();
    return;
}
