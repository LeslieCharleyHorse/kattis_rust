fn main()
{
    // import input read functionality
    use std::io;


    // var to hold input string 
    let mut input: String = String::new();


    // var to hold int value of input
    let mut input_int: i32 = 1;


    // read input and assign value to input string 
    io::stdin().read_line(&mut input).expect("Readline = Failed");


    // convert string to int and assign value to input_int
    input_int = input.trim().parse::<i32>().expect("Convert to i32 = Failed");



    // print result with for loopp 1..=n
    // lesliecharleyhorse
    for num in 1..=input_int
        {
            println!("{num}");
        }
}




        