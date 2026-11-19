fn main()
{
    // input lib
    use std::io;

    // SplitWhitespace lib
    use std::str::SplitWhitespace;

    //lesliecharleyhorse


    // var to hold input
    let mut input: String = String::new();

    // var to hold split vals
    let mut split: SplitWhitespace = "".split_whitespace();

    

    // tuple var to hold (bill count, group size, final total)
    let mut res: (i32, i32, i32) = ( 0, 0, 0);


    // read input; bill count group size string assign to input var
    io::stdin().read_line(&mut input).unwrap();


    // split input assign to res.0 var
    split = input.split_whitespace();


    // assign bill count and group size from converted split string vals in res.0
    res.1 = split.next().unwrap().parse::<i32>().unwrap();
    res.0 = split.next().unwrap().parse::<i32>().unwrap();


    // loop from 0 to bill count
    for i in 0..res.0
        {
            // clear input
            input.clear();

            // read in input num assign to input var
            io::stdin().read_line(&mut input).unwrap();


            // add converted int val to final total
            res.2 += input.trim().parse::<i32>().unwrap();


        }




    // result check if bill can be split
    if res.2 % res.1 == 0
        {
            println!("1");
        }
    else{println!("0");}




}