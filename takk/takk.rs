fn main()
{
    // input + output lib
    use std::io;


    // var to hold count
    let mut count: i64 = 0;


    // var to hold input
    let mut input: String = String::new();

    // read in count 
    io::stdin().read_line(&mut input).expect("Readline = Failed");

    // convert val to i64
    count = input.trim().parse::<i64>().expect("Convert to i64 = Failed");

    //lesliecharleyhorse

    for i in 0..count
        {
            // clear input
            input.clear();

            // read ni name
            io::stdin().read_line(&mut input).expect("Readline = Failed");

            // print result
            println!("Takk {}", input)


        }
}


                