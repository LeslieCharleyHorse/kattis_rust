fn main()
{
    //lesliecharleyhorse
    // input lib
    use std::io;


    // var to hold input
    let mut input: String = String::new();

    // readline
    io::stdin().read_line(&mut input).expect("Readline = Failed");

    // check for 555
    if String::from(&input.trim()[0..3]) == String::from("555")
        {
            println!("1");
        }

    else
        {
            println!("0");
        }
}