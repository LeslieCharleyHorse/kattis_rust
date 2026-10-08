fn main()
{
    // import input reader object and functionality 
    use std::io;
    use std::io::Stdin;


    // create input reader object
    let input_reader: Stdin = io::stdin();


    // var to hold input 
    let mut input: String = String::new();

    // var to hold input string 
    let mut input_string: String = String::new();

    // var to hold input num
    let mut num: i32 = 0;


    // read input string
    input_reader.read_line(&mut input).expect("Readline = Failed");

    // assign string value 
    input_string = input.trim().to_string();

    // clear input 
    input.clear();


    // read input number
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // assign int value
    num = input.trim().parse::<i32>().expect("Convert to i32 = Failed");


    // result
    println!("{}", input_string.repeat(num as usize));
    //lesliecharleyhorse



}