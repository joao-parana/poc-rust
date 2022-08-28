use std::io;
// Using io module from std crate.

fn main() {
    let x = 4;
    println!("x = {}", x);

    {
        let x = x + 5;
        println!("x = {}", x);
    }

    let x = x + 1;
    println!("x = {}", x);

    const NANO_SECONDS_SINCE_1970_01_01:u128 = 1661634723658975621;
    println!("NANO_SECONDS_SINCE_1970_01_01 = {}", 
                        NANO_SECONDS_SINCE_1970_01_01);
    
    let a: bool = true;
    let b: char = 'π';
    let c: f32 = std::f32::consts::PI;
    println!("a = {}, b = {}, c = {}", a, b, c);


    // Cast numerical types in Rust
    let p: u8 = 4;
    let q = p + 3;
    // u32::try_from(p).ok(); -> Option<i8>
    let r: u32 = q as u32 + 9;
    println!("p = {}, q = {}, r = {}", p, q, r);

    // Tupples in Rust
    let tup = (true, 'π', std::f32::consts::PI);
    println!("second element of tup = {}", tup.1);
    let tup:(bool, char, f32) = (true, 'π', std::f32::consts::PI);
    println!("tup = {:?}", tup);
    let t = (1u8, 2u16, 6.1f64, "hello", String::from(", world"));
    println!("t = {:?}", t);

    // Mutable tupple
    let mut mtup = (true, 'π', std::f32::consts::PI);
    println!("mtup = {:?}", mtup);
    mtup.1 = '∆';
    println!("mtup = {:?}", mtup);
    println!("mtup.1 = {}", mtup.1);

    mtup = (false, '®', 6.729_096_4);
    println!("mtup = {:?}", mtup);

    // Arrays in Rust
    let arr = [1, 2, 3];
    println!("arr = {:?}", arr);

    let mut marr: [i32; 3] = [1, 2, 3];
    println!("marr = {:?}", marr);
    marr[1] = 9;
    println!("marr = {:?}", marr);

    // Using io module to read stdin to a mutable String
    let mut input = String::new();
    println!("Please, type something interesting and press ENTER");
    io::stdin().read_line(&mut input)
                .expect("Error: failed to read line");
    println!("input = {}", input);

    let int_input: i64 = input.trim().parse().unwrap();
    println!("int_input = {}", int_input);


}
