fn main() {
    let x = 4;
    println!("x = {}", x);

    {
        let x = x + 5;
        println!("x = {}", x);
    }

    let x = x + 1;
    println!("x = {}", x);

    const SECONDS_IN_MINUTE:u8 = 60;
    println!("SECONDS_IN_MINUTE = {}", SECONDS_IN_MINUTE);

    const SECONDS_IN_MINUTE:u16 = 60;
    println!("SECONDS_IN_MINUTE = {}", SECONDS_IN_MINUTE);
}
