fn main()
{
    // input+output library
    use std::io;

    // var to hold input
    let mut input: String = String::new();

    // readinput
    io::stdin().read_line(&mut input).expect("Readline = Failed");

    
    // check if input contains
    // LESLIECHARLEYHORSE
    if input.trim().contains(&String::from("COV"))
        {
            println!("Veikur!");
        }
    else
        {
            println!("Ekki veikur!");
        }
}

