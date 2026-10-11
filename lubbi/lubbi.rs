fn main()
{
    // import input reader object and functionality 
    use std::io;
    use std::io::Stdin;


    // create input reader object
    let input_reader: Stdin = io::stdin();

    // var to hold input
    let mut input: String = String::new();


    // read input and assign val to input var
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // print result by breaking input into its characters and returning first character
    println!("{}", input.trim().chars().nth(0).expect("no char"));
    //lesliecharleyhorse
}



