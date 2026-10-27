fn main()
{
    //lesliecharleyhorse
    // input lib
    use std::io;
    
    // var to hold input
    let mut input: String = String::new();
    
    
    // var to hold res
    let mut res: usize = 0;
    
    // read input num
    io::stdin().read_line(&mut input).expect("Readline = Failed");
    
    
    // result
    res =  input.trim().chars().filter(|&character| character == '1').count();
    
    println!("{res}");
    
}

