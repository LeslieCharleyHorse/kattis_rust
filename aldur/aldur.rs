fn main()
{
    // import input + output library and input reader object type
    use std::io;
    use std::io::Stdin;




    // var to hold input 
    let mut input: String = String::new();

    // var to hold friend count will use as boolean for while loop
    let mut friends_count: i32 = 0;

    // var to hold res
    let mut res: i32 = -1;

    // create input reader object
    let mut input_reader: Stdin = io::stdin();




    // read number of friends in and assign to input var
    input_reader.read_line(&mut input).expect("Readline = Failed");


    // assign converted string to int val to friends_count var
    friends_count = input.trim().parse::<i32>().expect("Convert to i32 = Failed");

    //lesliecharleyhorse


    while friends_count > 0
        {
            // temp var to hold int value of input
            let mut temp_val: i32 = 0;

            // clear input 
            input.clear();


            // read in new age and assign val to var input 
            input_reader.read_line(&mut input).expect("Readline = Failed");


            // assign converted int value of string to temp_val var
            temp_val = input.trim().parse::<i32>().expect("Convert to i32 = Failed");


            // compare age against res and if smaller reassign reassign, if res is negative it will be reassigned on first number it sees
            if res > temp_val || res == -1
                {
                    res = temp_val;
                }


        // reduce friends_count after
        friends_count -= 1;



        }

    println!("{res}");
}



        