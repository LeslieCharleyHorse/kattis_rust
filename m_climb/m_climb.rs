fn main()
{
    // import input reader object and functionality
    use std::io;


    // var to hold res
    let mut res: i32 = 1;
    

    // for loop two take in 2 numbers
    for i in 0..2
        {
            // temp var to hold string input
            let mut temp_input: String = String::new();
            
            // read string 
            io::stdin().read_line(&mut temp_input).expect("Readline = Failed");


            res *= temp_input.trim().parse::<i32>().expect("Convert to int = Failed");
        }

    //lesliecharleyhorse
    //result

    println!("{}", res);
}



