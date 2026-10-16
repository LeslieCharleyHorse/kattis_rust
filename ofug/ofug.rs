fn main()
{
    // import input + output library  and input reader object
    use std::io;
    use std::io::Stdin;


    // create input reader object 
    let input_reader: Stdin = io::stdin();


    // var to hold input 
    let mut input: String = String::new();


    // var to hold amount of numbers going thru
    let mut nums: i32 = 0;

    // vector to hold numbers
    let mut res: Vec<i32> = vec![];


    // read amount of numbers to loop thru
    input_reader.read_line(&mut input).expect("Readline = Failed");

    // assign converted int to nums
    nums = input.trim().parse::<i32>().expect("Convert to i32 = Failed");


    // loop thru as many times as num and add converted String -> int vals in vector
    for i in 0..nums
        {
            // clear input
            input.clear();


            // read input assign val to input var
            input_reader.read_line(&mut input).expect("Readline = Failed");


            // add converted strings -> int to res vector
            res.push(input.trim().parse::<i32>().expect("Convert to i32 = Failed"));

        }


    // sort vector asc order
    res.reverse();


    // printing sorted numbers
    //lesliecharleyhorse
    for num in res
        {
            // res print num
            println!("{}", num);
            
        }

}



        