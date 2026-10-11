fn main()
{
    // import input reader object and functionality 
    use std::io;
    use std::io::Stdin;

    // create input reader object
    let input_reader: Stdin = io::stdin();

    // var to hold input
    let mut input: String = String::new();


    // read number and assign to input 
    //lesliecharleyhorse
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // if even even bob wins 
    if input.trim().parse::<i32>().expect("Convert to int = Failed") % 2 == 0
        {
            println!("Bob");
        }
    else
        {
            println!("Alice");
        }
}


