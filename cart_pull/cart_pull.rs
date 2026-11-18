fn main()
{
    // input lib 
    use std::io;
    
    // var to hold input 
    let mut input: String = String::new();
    
    // var to hold res
    let mut res: i32 = 0;
    
    // read num 
    io::stdin().read_line(&mut input).unwrap();
    
    res = input.trim().parse::<i32>().unwrap();
    
    // read nums
    io::stdin().read_line(&mut input).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    
    // lesliecharleyhorse
    
    // result
    println!("{}", res * 2);
    
    
}



