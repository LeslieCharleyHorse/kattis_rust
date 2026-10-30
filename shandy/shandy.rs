fn main()
{
    // input lib
    use std::io;
    
    // var to hold input
    let mut input: String = String::new();
    
    // var to hold result\
    let mut res: i32 = 0;
    
    // read num 1
    io::stdin().read_line(&mut input).expect("readline = failed");
    
    // assign  val to res
    res = input.trim().parse::<i32>().expect("convert to i32 = failed");
    
    // clear input
    input.clear();
    
    // read num 2
    io::stdin().read_line(&mut input).expect("readline = failed");
    
    
    // look for min num and assign 
    if input.trim().parse::<i32>().expect("convert to i32 = failed") < res
        {
            res = input.trim().parse::<i32>().expect("convert to i32 = failed");
        }
        
    // math\
    res *= 2;
        
        
    // result
    //lesliecharleyhorse
    println!("{}", res);
    
    
    
    
}