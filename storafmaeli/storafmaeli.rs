fn main()
{
    // import input functionality
    use std::io;

    

    // var to hold input
    let mut input: String = String::new();

    //leslicharleyhorse


    // read input and assign val to input var
    io::stdin().read_line(&mut input).expect("Readline = Failed");


    // condtional to find answer
    if input.trim().parse::<i32>().expect("Converto to i32 = Failed") % 10 == 0
        {
            println!("Jebb");
        }

    else
       {
            println!("Neibb");
       }
    
}


        