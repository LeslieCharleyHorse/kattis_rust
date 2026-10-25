fn main()
{
    // input + output library
    use std::io;
    use std::io::Stdin;

    // SplitWhitespace library
    use std::str::SplitWhitespace;

    // var to hold input
    let mut input: String = String::new();

    // input reader object
    let mut ir: Stdin = io::stdin();

    // var hold result
    let mut res: (String, i64) = (String::new(), 0);


    // lesliecharleyhorse

    // var to hold guest count
    let mut guests: i64 = 0;

    // read guests input size
    ir.read_line(&mut input).expect("Readline = Failed");


    // assign conv input val
    guests = input.trim().parse::<i64>().expect("Convert to i64 = Failed");


    //loop thru as many guest
    for num in 0..guests
        {
            // var to hold split input
            let mut split_input: SplitWhitespace = "".split_whitespace();

            // clear input 
            input.clear();

            // read input
            ir.read_line(&mut input).expect("Readline = Failed");

            // assign split input vals to split_input var
            split_input = input.trim().split_whitespace();

            // var to hold temp name + temp score
            let mut name: String = String::new();
            let mut score: i64 = 0;


            // assign vals
            name = String::from(split_input.next().expect("Access index = Failed"));
            score = split_input.next().expect("Access index = Failed").parse::<i64>().expect("Convert to i64 = Failed");

            // check for best guest
            if score > res.1
                {
                    // assign new best guest
                    res.0 = name;
                    res.1 = score;
                }

            
        }


    // result
    println!("{}", res.0);




}

