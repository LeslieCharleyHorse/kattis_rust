fn main()
{
    // import input reader object and functionality
    use std::io;
    use std::io::Stdin;

    // create input reader object
    let input_reader: Stdin = io::stdin();

    // var to hold input
    let mut input: String = String::new();

    // read user input and store value in input var
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // print result
    println!("Kvedja,");
    println!("{}", input);

    //lesliecharleyhorse
}

