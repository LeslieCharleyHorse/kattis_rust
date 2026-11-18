fn main()
{
    // input lib
    use std::io;

    // SplitWhitespace lib
    //use std::str::SplitWhitespace;


    // var to hold input
    let mut input: String = String::new();

    // var to hold res
    let mut res: f64 = 0.0;

    // var to hold split vals in vector
    let mut split: Vec<f64> = vec![];


    // read nums assign val to input var
    io::stdin().read_line(&mut input).unwrap();

    // convert vals to int , assign split vals to split var 
    split = input.split_whitespace().map(|num| num.parse::<f64>().unwrap()).collect::<Vec<f64>>();
    

    // result
    //lesliecharleyhorse
    println!("{}", (split[0]*split[2]) + (0.5 * (split[1]) * (split[2].powi(2))));

}