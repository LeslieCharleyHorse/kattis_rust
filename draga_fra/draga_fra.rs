fn main()
{
    // import input reader object and functionlaity 
    use std::io;
    use std::io::Stdin;


    // create input reader object
    let input_reader: Stdin = io::stdin();


    // var to hold input
    let mut input: String = String::new();

    // array to hold value of nums
    let mut nums_int: [i32; 2] = [0, 0];


    // read first number 
    input_reader.read_line(&mut input).expect("Readline = Failed");
    //lesliecharleyhorse

    // assingn value
    nums_int[0] = input.trim().parse::<i32>().expect("Convert to int = Failed");


    // clear input
    input.clear();


    // read second number 
    input_reader.read_line(&mut input).expect("Readline = Failed");

    // assingn value
    nums_int[1] = input.trim().parse::<i32>().expect("Convert to int = Failed");


    // result
    println!("{}", nums_int[0] - nums_int[1]);


}