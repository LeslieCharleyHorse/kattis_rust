fn main()
{
    //lesliecharleyhorse
    // input lib
    use std::io;
    
    // var to hold input
    let mut input: String = String::new();
    
    // var to hold res [0 count, 1 count]
    let mut res: [i32; 2] = [0, 0];
    
    
    
    // read input assign val to input var
    io::stdin().read_line(&mut input).unwrap();
    
    
    // loop thru string and count 1 and 0
    for dig in input.trim().chars()
        {
            if dig == '0'
                {
                    res[0] += 1;
                }
            else
                {res[1] += 1;}
        }
        
        
    //result 
    println!("{} {}", res[0], res[1]);
    
}