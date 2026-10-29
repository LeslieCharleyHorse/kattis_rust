fn main()
{
    //lesliecharleyhorse
    use std::io;
    
    // var to hold input
    let mut input: String = String::new();
    
    // var to hold res
    let mut res: i16 = 0;
    
    // read first number
    io::stdin().read_line(&mut input).expect("Readlline = Failed");
    
    // assign first int val to res var
    res = input.trim().parse::<i16>().expect("Convert to i16 = failed");
    
    
    // operations clear input, math
    input.clear();
    res -= 1;
    
    // read number 2
    io::stdin().read_line(&mut input).expect("Readlline = Failed");
    
    // operation 2
    res *= input.trim().parse::<i16>().expect("Convert to i16 = failed");
    
    // result
    println!("{res}");

    
}