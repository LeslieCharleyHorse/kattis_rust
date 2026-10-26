fn main()
{
    // input + output lib
    use std::io;


    // var to hold input
    let mut input: String = String::new();

    //lesliecharleyhorse

    // read input
    io::stdin().read_line(&mut input).expect("Readline = Failed");


    // result
    println!("{} {} {}", input, input, input);
        
}

