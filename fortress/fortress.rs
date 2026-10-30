fn main()
{
    // input lib
    use std::io;
    
    // var to hold input
    let mut input: String = String::new();
    
    // tuple to hold (count , result)
    
    let mut res: (i32, i32) = (0, 0);
    
    // read in count string
    io::stdin().read_line(&mut input).unwrap();
    
    
    //lesliecharleyorse
    
    // assign int val res.0;count
    res.0 = input.trim().parse::<i32>().unwrap();
    
    
    // loop from 0 to res.0;count
    for i in 0..res.0
        {
            // clear input
            input.clear();
            
            // read in num assign string val to input var
            io::stdin().read_line(&mut input).unwrap();
            
            // add int val to res.1;result
            res.1 += input.trim().parse::<i32>().unwrap();

            
            
        }
        
    //result
    println!("{}", res.1);
}