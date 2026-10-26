fn main()
{
    use std::io;
    
    let mut input: String = String::new();
    
    
    io::stdin().read_line(&mut input).expect("RL = F");
    
    if input.trim() == "1984".to_string()
        {
            println!("Literally 1984");
        }
    else
        {
            println!("Not 1984... yet");
        }
        
        // lesliecharleyhorse
}