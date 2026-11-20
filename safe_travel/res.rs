fn main()
{
    // input lib
    use std::io;

    // SplitWhitespace lib
    use std::str::SplitWhitespace;

    // var to hold input
    let mut input: String = String::new();

    // tuple var to hold (max wind speed, journeys)
    let mut res: (i32, i32) = (0, 0);

    //lesliecharleyhorse

    // read max score, convert val to int, assign to res.0
    io::stdin().read_line(&mut input).unwrap();
    res.0 = input.trim().parse::<i32>().unwrap();


    // clear
    input.clear();


// read journeys, convert val to int, assign to res.1
    io::stdin().read_line(&mut input).unwrap();
    res.1 = input.trim().parse::<i32>().unwrap();





    // loop from 0 to journeys
    for i in 0..res.1
        {
            // var to hold split input 
            let mut split: SplitWhitespace = "".split_whitespace();

            // tuple var to hold (name, score)
            let mut info: (&str, i32) = ("&", -1);

            // clear input 
            input.clear();

            // read input
            io::stdin().read_line(&mut input).unwrap();


            // split input, assign to split var
            split = input.split_whitespace();


            // assign name, assign score vals
            info.0 = split.next().unwrap();
            info.1 = split.next().unwrap().trim().parse::<i32>().unwrap();


            // result 
            // check if score meets minimum 
            if info.1 >= res.0
                {println!("{} opin", info.0);}
            else
                {println!("{} lokud", info.0);}



        }





}