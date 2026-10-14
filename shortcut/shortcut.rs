fn main()
{
    // import input/output library
    use std::io;


    // var to hold input string
    let mut input: String = String::new();


    // readline and assign value to string
    io::stdin().read_line(&mut input).expect("Readline = Failed");


    // result
    println!("{}", (input.trim().parse::<i32>().expect("Convert to int = Failed") as i32 + 5) * 3  - 10 );
    // leslicharleyhorse
}

