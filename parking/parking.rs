fn main()
{
    // input lib 
    use std::io;

    // var to hold input 
    let mut input: String = String::new();

    // var to hold res
    let mut res: i32 = 1;


    

    // loop thru nums 
    for i in 0..3
        {

            // clear input 
            input.clear();

            // read input 
            io::stdin().read_line(&mut input).unwrap();

            // check if multiply res
            if i % 2 == 0
                {
                    res *= input.trim().parse::<i32>().unwrap();
                }
        }

    //lesliecharleyhorse
    //result
    println!("{res}");
}