fn main()
{
    // input lib
    use std::io;
    
    // var to hold input
    let mut input: String = String::new();
    
    // var to hold res 
    let mut res: f64 = 0.0;
    
    // read input
    io::stdin().read_line(&mut input);
    
    
    // assign converted val 
    res = input.trim().parse::<f64>().unwrap() - 0.3;
    
    println!("{res}");
    
    // lesliecharleyhorse
}

