fn main()
{
    // import input reader object and functionality 
    use std::io;
    use std::io::Stdin;
    
    // import iterator object and reverse functionality 
   // use std::iterator;
    //use std::iterator::R


    // declare varible to hold input string 
    let mut input: String = String::new();

    // create input reader object 
    let input_reader: Stdin = io::stdin();


    // readline of input and hold value in input varible
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // print reversed string 
    println!("{}", input.chars().rev().collect::<String>());
}