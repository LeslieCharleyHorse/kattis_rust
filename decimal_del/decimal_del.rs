fn main()
{
    // import input reader object and functionality
    use std::io;
    use std::io::Stdin;


    // create input reader object
    let input_reader: Stdin = io::stdin();

    // var for input
    let mut input: String = String::new();


    // var to hold folating point value
    let mut f_num: f64 = 0.0;


    // read in number
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // assign floating point value 
    f_num = input.trim().parse::<f64>().expect("Convert to float = Failed");


    // print result
    println!("{}", f_num.round() as i64);

    //lesliecharleyhorse
}
