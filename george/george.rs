fn main()
{
    // input + output lib
    use std::io;

    // SplitWhitespace lib
    use std::str::SplitWhitespace;

    // var to hold input
    let mut input: String = String::new();

    // var to hold split input
    let mut si: SplitWhitespace = "".split_whitespace();

    // var to hold num1 num2
    let mut res: [i32; 2] = [0, 0];

    // read input
    io::stdin().read_line(&mut input).expect("Readline = Failed");

    // split input
    si = input.split_whitespace();


    // assign vals
    res[0] = si.next().expect("Access index = Failed").parse::<i32>().expect("Convert to i32 = Failed");

    si.next().expect("Access index = Failed");

    res[1] = si.next().expect("Access index = Failed").parse::<i32>().expect("Convert to i32 = Failed");

    // find answer
    //lesliecharleyhorse
    if res[0] > res[1]
        {
            println!(">");
        }
    else if res[1] > res[0]
        {
            println!("<");
        }

    else {
        println!("Goggi svangur!");
        }



}


    