fn main()
{
    // import input reader functionality
    use std::io;


    // lesliecharleyhorse
    // var to hold input
    let mut input: String = String::new();

    // read input and assign value to input var
    io::stdin().read_line(&mut input).expect("Readline = Failed");


    // result
    println!("{}", input.trim().parse::<f64>().expect("Convert to i32 = Failed") / 4.0);
    
}



        