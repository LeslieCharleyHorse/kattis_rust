fn main()
{
    // import input reader object and functionality
    use std::io;
    use std::io::Stdin;

    // create input reader object
    let input_reader: Stdin = io::stdin();


    // var to hold result
    let mut res: i32 = 1;


    // for loop to find answer
    for num in 0..2
        {
            // var to hold input
            let mut temp_input: String = String::new();

            // read input line
            input_reader.read_line(&mut temp_input).expect("Readline = Failed");


            // do geo math
            res *= temp_input.trim().parse::<i32>().expect("Convert to int = Failed");
        } 

    println!("{res}");
}





        