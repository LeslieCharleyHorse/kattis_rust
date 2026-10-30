fn main()
{
    //lesliecharleyhorse
    // input lib
    use std::io;
    
    
    // var to hold input
    let mut input: String = "".to_string();
    
    
    // read input 
    io::stdin().read_line(&mut input).expect("readline = failed");
    
    
    // check for palindrome
    if input.trim() == input.trim().chars().rev().collect::<String>()
        {println!("Palindrome!");}
        
    else
        {println!("Nothing special about this string :(");}
    
}