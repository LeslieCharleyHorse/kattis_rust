fn main()
{
    //input lib
    use std::io;


    // SplitWhitespace lib
    use std::str::SplitWhitespace;

    // var to hold input
    let mut input: String = "".to_string();

    // var to hold split input
    let mut split_input: SplitWhitespace = "".split_whitespace();

    // var to hold count 
    let mut count: i32 = 0;

    // var to hold score
    let mut score: i32 = 0;


    // var to hold res
    let mut res: String = String::from("Jebb");


    // read input count and score
    io::stdin().read_line(&mut input).expect("Readline = Failed");

    // hold split vals in split_input var
    split_input = input.split_whitespace();


    // assign vals for count and score
    //lesliecharleyhorse
    count = split_input.next().expect("Access index = Failed").parse::<i32>().expect("Convert to i32 = Failed");
    score = split_input.next().expect("Access index = Failed").parse::<i32>().expect("Convert to i32 = Failed");


    //lesliecharleyhorse
    // loop thru numbers
    for i in 0..count
        {
            // clear input
            input.clear();

            // read input number
            io::stdin().read_line(&mut input).expect("Readline = Failed");

            // subtract converted int val from score
            score -= input.trim().parse::<i32>().expect("COnvert to i32 = Failed");

            // check if score negative
            if score < 0
                {
                    // change res
                    res = String::from("Neibb");
                    break;
                }

        }

    // result
    println!("{res}");


}

        