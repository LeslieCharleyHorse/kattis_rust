fn main()
{
    //lesliecharleyhorse
    // input lib
    use std::io;
    
    // var to hold input 
    let mut input: String = String::new();
    
    // read input
    io::stdin().read_line(&mut input).expect("readline = failed");
    
    // result
    if input.trim().parse::<i32>().expect("convert to i32 = failed") % 2 == 0
        {
            println!("second");
        }
        
    else
        {
            println!("first");
        }
}