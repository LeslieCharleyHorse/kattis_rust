fn main()
{
    // import input read functionality 
    use std::io;
    use std::io::Stdin;

    
    // delcare input reader object
    let input_reader: Stdin = io::stdin();

    // declare variable hold input 
    let mut input: String = String::new();


    // readline hold value in input
    input_reader.read_line(&mut input).expect("Readline = Failed");

    println!("{}", input.trim().len());




}
