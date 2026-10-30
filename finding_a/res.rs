fn main()
{
    //lesliecharleyhorse
    //input lib
    use std::io;
    
    // var to hold input
    let mut input: String = String::new();
    
    // var to hold res (add to string? : bool, String )
    let mut res: (bool, String) = (false, "".to_string());
    
    // read input
    io::stdin().read_line(&mut input).expect("readline = failed");
    
    
    // loop thru input chars
    for c in input.chars()
        {
            // check for add to res string
            if res.0 == true
                {
                    // add to res string 
                    res.1.push(c);
                    
                }
                
            // check for a
            else
                {
                    // check for a
                    if c == 'a'
                        {
                            // change add to string bool
                            res.0 = true;
                            // add to res string 
                            res.1.push(c);
                        }
                }
                
        }
        
        
    // result
    println!("{}", res.1);
}