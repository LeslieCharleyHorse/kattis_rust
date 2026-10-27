fn main()
{
    //lesliecharleyhorse
    use std::io;
    
    // var to hold input 
    let mut input: String = String::new();
    
    // var to hold input int val
    let mut num: i32 = 0;
    
    
    // var to hold res
    let mut res: i32 = 0;
    
    // read input
    io::stdin().read_line(&mut input).expect("Readline = Failed");
    
    
    // assign input val to num var
    num = input.trim().parse::<i32>().expect("Convert to i32 = Failed");
    
    
    for i in 0..12
        {
            res += num;
            
            println!("{}", res);
        }
    
    
    
}