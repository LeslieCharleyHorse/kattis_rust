use std::io::Read;

fn main()
{
    // input lib
    use std::io;

    // SplitWhitespace lib
    use std::str::SplitWhitespace;


    // var to hold input
    let mut input: String = "".to_string();

    // var to hold split vals
    let mut split_input: SplitWhitespace = "".split_whitespace();

    // var to hold nums vals
    let mut nums: Vec<i64> = vec![];


    // read input assign val to input var
    io::stdin().read_line(&mut input).expect("readline = failed");

    //lesliecharleyhorse

    // loop thru split vals and add to nums vector
    for num in input.trim().split_whitespace()
        {
            // add the converted val to nums vec
            nums.push(num.parse::<i64>().expect("readline = failed"));
        }

    // conditionals check for result if equiv
    if nums[0] + nums[1] == nums[2]
        {
            println!("correct!");
        }
    else {
        println!("wrong!");
    }



}

            