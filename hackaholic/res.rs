fn main()
{
    // input lib 
    use std::io;
    
    // var to hold input
    let mut input: String = String::new();
    
    // var to hold res
    let mut res: i32 = 1;
    
    
    // math operations result
    for _i in 0..3
        {
            //llesliecharleyhorse
            if _i != 1
                {
                

                    // read input, assign val to input var
                    io::stdin().read_line(&mut input).unwrap();
                    // mutlpy res
                    res *= input.trim().parse::<i32>().unwrap();
                    
                    // clear input
                    input.clear();
                }
            else
            {
                io::stdin().read_line(&mut input).unwrap();
                input.clear();
            }
            
            
        }
    // result 
    println!("{}", res);
}