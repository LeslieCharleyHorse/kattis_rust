fn main()
{
    // input lib
    use std::io;
    
    // var to hold input
    let mut input: String = String::new();
    
    // var to hold res 
    let mut res: i32 = 1;
    
    // var to hold count 
    let mut count: i32 = 0;
    
    // read in count, assign val to input var
    io::stdin().read_line(&mut input).unwrap();
    
    // convert input string to int val, assign to count var
    count = input.trim().parse::<i32>().unwrap();
    
    
    // loop from 0 to count
    for _i in 0..count
        {
            // clear input 
            input.clear();
            
            // read num, assign to input var 
            io::stdin().read_line(&mut input).unwrap();
            
            
            // convert string to int val, add int - 1 to res
            res += input.trim().parse::<i32>().unwrap() - 1;
            
        }
        
        
    // result
    println!("{}", res);
}