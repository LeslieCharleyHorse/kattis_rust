fn main()
{
    // input lib
    use std::io;
    
    // splitwhitespace lib
    use std::str::SplitWhitespace;
    
    //lesliecharleyhorse
    
    // var to hold input
    let mut input: String = String::new();
    
    // var to hold split input
    let mut split: SplitWhitespace = "".split_whitespace();
    
    // var hold num 1 val, num 2 val
    let mut one: i32 = 0;
    let mut two: i32 = 0;
    
    
    
    // read nums
    io::stdin().read_line(&mut input).unwrap();

    // split input, assign vals to slpit var
    split = input.split_whitespace();
    
    
    // assign vals 
    one = split.next().unwrap().parse::<i32>().unwrap();
    two = split.next().unwrap().parse::<i32>().unwrap();
    
    
    // check for result
    if one < two
        {
            println!("{} {}", one.to_string(), two.to_string());
        }
        
    else
        {
            println!("{} {}", two.to_string(), one.to_string());
        }
    
    
}