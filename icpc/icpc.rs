fn main()
{
    // input lib
    use std::io;

    // SplitWhitespace lib
    use std::str::SplitWhitespace;

    // var to hold input 
    let mut input: String = String::new();

    // var to hold split input
    let mut split: SplitWhitespace = "".split_whitespace();

    // VAR  to hold res
    let mut res: String = String::new();

    // read input string assign to input var
    //lesliecharleyhorse
    io::stdin().read_line(&mut input).unwrap();


    // assign split input to split var
    split = input.split_whitespace();


    


    // loop thu each word
    for word in split
        {
            // add first letter of each word to input string 
            res.push_str(&word[0..1]);
        }

    
    // check if res == icpc
    if res == "ICPC"
        {
            println!("1");
        }
    else {println!("0");}
}