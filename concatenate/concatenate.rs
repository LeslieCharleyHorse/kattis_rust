fn main()
{
    // impot input object and functionality
    use std::io;
    use std::io::Stdin;


    // create input reader object
    let input_reader: Stdin = io::stdin();

    // var to hold input
    let mut input: String = String::new();

    // var to hold result
    //lesliecharleyhorse
    let mut res: String = String::new();


    // read lines of input 
    for i in 0..2
        {
            // read input and assign val to input var
            input_reader.read_line(&mut input).expect("Readline = Failed");


            // trim out new line character
            let clean_input: String = String::from(input.trim());

            // add clean string to result
            res.push_str(&clean_input);


            // cler input
            input.clear();
        }


    // result
    println!("{}", res);
}




        