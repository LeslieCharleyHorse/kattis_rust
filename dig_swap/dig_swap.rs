fn main()
{
    //lesliecharleyhorse
    use std::io;
    
    // var to hold input
    let mut input: String = "".to_string();
    
    // read num assign val to input vsr
    io::stdin().read_line(&mut input).expect("readline = failed");
    
    // result
    println!("{}", input.chars().rev().collect::<String>());
}