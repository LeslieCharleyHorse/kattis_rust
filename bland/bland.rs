fn main()
{
    // input + output lib
    use std::io;


    // var to hold input
    let mut input: String = String::new();

    // read input number
    io::stdin().read_line(&mut input).expect("Readline = Failed");

    // lesliecharleyhorse

    
    // mixed
    if input.trim() == String::from("2")
        {
            println!("blandad best");
        }
    // sinlge
    else
        {
            input.clear();
            io::stdin().read_line(&mut input).expect("Readline = Failed");
            println!("{}", input);

        }
}


            