use std::io;

fn main() {
    let guess: u32 = "421".parse().expect("Not a number");

    println!("{}", guess);

    let x = 2.0;

    let y: f32 = 3.0;

    println!("{}", x);
    println!("{}", y);

    let t = true;
    let f: bool = false;

    println!("{}", t);
    println!("{}", f);

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{}", c);
    println!("{}", z);
    println!("{}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (p,_o,_k) = tup;

    println!("The value of p : {}", p);

    println!("{}", tup.1);
    
}