fn main()
{
    //input lib
    use std::io;
    
    // SplitWhitespace lib;
    use std::str::Split;
    
    
    // var to hold input
    let mut input: String = String::new();
    
    // var to hold split input 
    let mut split_input: Split<'_, char> = "".split('.');
    
    // var to hold res
    let mut res: &str = "";
    
    
    // read input and assign val to input var
    io::stdin().read_line(&mut input).expect("Readline = Failed");
    
    // lesliecharleyhorse
    //split input, assign vals to split_input var
    split_input = input.split('.');
    
    
    // loop thru split input
    for val in split_input
        {
            // clear res
            res = "";
            
            // make res = val
            res = val;
        }
        
        
    // result
    println!(".{}", res);
    
    
}