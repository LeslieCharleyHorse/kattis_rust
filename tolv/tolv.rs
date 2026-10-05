fn main()
{
    // import input read functionality and object
    use std::io;
    use std::io::Stdin;


    // create input reader object
    let input_reader: Stdin = io::stdin();

    // var to hold input 
    let mut input: String = String::new();

    // var hold input as int
    let mut num: i32 = 0;


    // read line of input store value in input
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // convert input to int
    num = input.trim().parse::<i32>().expect("Convert to int = Failed");


    // print answer
    println!("{}", num - 1);
}