fn main()
{
    //lesliecharleyhorse
    //input lib
    use std::io;
    
    // var to hold input
    let mut input: String = String::new();
    
    // var to hold res
    let mut res: i32 = 0;
    
    // read input
    io::stdin().read_line(&mut input).expect("readline = failed");
    
    // assign int val to res var
    res = input.trim().parse::<i32>().expect("Convert to i32 = failed");
    
    // cube res
    res = res.pow(3);
    
    //result
    println!("{res}");
}