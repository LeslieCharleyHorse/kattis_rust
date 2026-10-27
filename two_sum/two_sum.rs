fn main()
{
    // input lib
    use std::io;

    // SplitWhitespace lib
    use std::str::SplitWhitespace;

    // var to hold input
    let mut input: String = "".to_string();

    // var to hold split input
    let mut nums: SplitWhitespace = "".split_whitespace();

    // var to hold result
    let mut res: i32 = 0;

    // read input numbers
    io::stdin().read_line(&mut input).expect("Readline = Failed");

    // assign split vals to nums var
    nums = input.split_whitespace();


    // loop thru nums; convert vals to int and add to res
    for num in nums
        {
            res += num.trim().parse::<i32>().expect("Convert to i32 = Failed");
        }


    //lesliecharleyhorse
    //result
    println!("{res}");
}

    