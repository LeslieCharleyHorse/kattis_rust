fn main()
{
    // import input object + functionality 
    use std::io;
    use std::io::Stdin;


    // create input reader object
    let input_reader: Stdin = io::stdin();

    // var for input
    let mut input: String = String::new();

    // var to hold vals of nums
    let mut n1: i32 = 0;
    let mut n2: i32 = 0;
    //lesliecharleyhorse


    // read first number 
    input_reader.read_line(&mut input).expect("Readline - Failed");

    // assign  value
    n1 = input.trim().parse::<i32>().expect("Convert to int = Failed");


    // clear input
    input.clear();

    // read second number 
    input_reader.read_line(&mut input).expect("Readline - Failed");

    // assign  value
    n2 = input.trim().parse::<i32>().expect("Convert to int = Failed");



    // find answer
    println!("{}", n1 % n2);


}