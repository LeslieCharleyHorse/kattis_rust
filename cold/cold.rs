fn main()
{
    // input lib
    use std::io;
    
    // SplitWhitespace lib
    use std::str::SplitWhitespace;
    
    
    // var to hold input
    let mut input: String = String::new();
    
    // var to holf split input
    let  mut split: SplitWhitespace = "".split_whitespace();
    
    // var to hold res
    let mut res: i32 = 0;
    
    
    // read number count
    io::stdin().read_line(&mut input).unwrap();
    
    // clear input 
    input.clear();
    
    
    // read numbers
    io::stdin().read_line(&mut input).unwrap();
    
    
    // split vals =, assign to split var
    split = input.trim().split_whitespace();

    //lesliecharleyhorse
    
    
    // loop thru split string vals
    for num in split
        {
            // check for lower than 0
            if num.parse::<i32>().unwrap() < 0
                {
                    // add to result
                    res += 1;
                }
        }
        
    println!("{}", res);

    
    
}