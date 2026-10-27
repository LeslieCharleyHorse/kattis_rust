fn main()
{
    // input lib
    use std::io;

    // var to hold input
    let mut input: String = "".to_string();

    // read input
    io::stdin().read_line(&mut input).expect("Readline = Failed");


    // check for "ss"
    // lesliecharleyhorse
    if input.trim().contains("ss")
        {
            println!("hiss");
        }

    else
        {
            println!("no hiss");
        }
}


        