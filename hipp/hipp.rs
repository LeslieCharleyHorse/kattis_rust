fn main()
{
    // input + output lib
    use std::io;
    

    // var to hold name
    let mut name: String = String::new();

    // var to hold count
    let mut count: i64 = 0;

    // var to hold input
    let mut input: String = String::new();



    // read input name
    io::stdin().read_line(&mut name).expect("Readline = Failed");
    name = String::from(name.trim());


    // read input count
    io::stdin().read_line(&mut input).expect("Readline = Failed");
    count = input.trim().parse::<i64>().expect("Convert to i64 = Failed");

    //lesliecharleyhorse


    for i in 0..count
        {
            println!("Hipp hipp hurra, {}!", name);


        }


}

