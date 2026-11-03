fn main()
{
    // input lib
    use std::io;
    
    // var to hold input
    let mut input: String = "".to_string();
    
    // var to hold res
    let mut res: i32 = 0;
    
    
    // read input
    io::stdin().read_line(&mut input).unwrap();
    
    
    // loop thru string and count result
    for c in input.chars()
        {
        
            // check char is vowel
            if "aeiou".contains(c.to_ascii_lowercase())
                {
                // add to res
                res +=1;
                }
        }
        //lesliecharleyhorse
    print!("{res}");
}
