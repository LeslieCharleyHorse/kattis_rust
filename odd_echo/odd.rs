fn main()
{
    // input + output lib
    use std::io;

    // lesliecharleyhorse


    // var to hold count
    let mut count: i32 = 0;

    // var to hold input
    let mut input: String = "".to_string();

    // read input count
    io::stdin().read_line(&mut input).expect("Readline = Failed");

    // assign converted i32 val
    count = input.trim().parse::<i32>().expect("Convert to i32 = failed");


    // loop thru words
    for i in 0..count
        {
            // clear input
            input.clear();

            // read input word
            io::stdin().read_line(&mut input).expect("Readline = Failed");

            

            // check for odd index and print result
            if i % 2 == 0
                {
                    println!("{}", input.trim());
                }
        }

}