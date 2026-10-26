fn main()
{
    // input + output lib
    use std::io;


    // var to hold input
    let mut input: String = "".to_string();

    // var to hold count
    let mut count: i64 = 0;

    // read input
    io::stdin().read_line(&mut input).expect("Readline = Failed");

    // assign converted val to count var
    count = input.trim().parse::<i64>().expect("Convert to i64 = Failed");

    // result
    for i in 0..count
        {
            println!("{} Abracadabra", i+1);
        }
}