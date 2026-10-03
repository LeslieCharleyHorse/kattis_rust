fn main()
{
    // import input reading functionality 
    use std::io;
    use std::io::Stdin;
    // LeslieCharleyHorse

    
    // delcare variable to hold input
    let mut input: String = String::new();

    // delcare input reader object
    let input_reader: Stdin = io::stdin();

    // declare variable to hold result 
    let mut res: i32 = 0;
    

    // read first line of input
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // hold first value in result
    res = input.trim().parse::<i32>().expect("Convert to i32 = Failed");
    


    // clear input
    input =  String::new();


    // read second line of input
    input_reader.read_line(&mut input).expect("Readline = Failed");



    // divide res by second value
    res /= input.trim().parse::<i32>().expect("Convert to i32 = Failed");

    println!("{}", res+2022);




}