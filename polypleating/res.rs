fn main()
{
    // input lib
    use std::io;
    
    
    // var to hold input
    let mut input: String = String::new();
    
    // var to hold res
    let mut res: i32 = 1;
    
    
    // read in first num, assign val to input var
    io::stdin().read_line(&mut input).unwrap();
    
    // math operation on res using converted int val
    res *= input.trim().parse::<i32>().unwrap();
    
    // clear input
    input.clear();
    
    
    
    // read in second num, assign val to input var
    io::stdin().read_line(&mut input).unwrap();
    
    //lesliecharleyhorse
    // math operation on res using converted int val
    res *= input.trim().parse::<i32>().unwrap();
    
    
    // result
    println!("{res}");
}