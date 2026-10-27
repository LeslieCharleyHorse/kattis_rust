fn main()
{
    // input lib
    use std::io;
    
    // var to hold input
    let mut input: String = String::new();
    
    // var to hold input int val
    let mut num: i32 = 0;
    
    // read input
    io::stdin().read_line(&mut input).expect("Readline = Failed");
    
    
    // assign int val
    num = input.trim().parse::<i32>().expect("Convert to i32 = Failed");
    
    //lesliecharleyhorse
    
    // print result 
    for i in (1..=num).rev()
        {
            println!("{i}");
        }
    
    
}