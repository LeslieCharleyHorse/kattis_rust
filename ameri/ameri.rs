fn main()
{
    //lesliecharleyhorse
    // input lib
    use std::io;
    
    
    // var to hold input
    let mut input: String = String::new();
    
    // var to hold res
    let mut res: f64 = 0.09144;
    
    
    // read input 
    io::stdin().read_line(&mut input).expect("readline = failed");
    
    
    // math
    res = res * input.trim().parse::<f64>().expect("convrt to f64 = failed");
    
    
    //result
    println!("{res}");
}