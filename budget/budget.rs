fn main()
{
    // input lib
    use std::io;

    // var to hold input
    let mut input: String = String::new();

    // tuple var to hold res (transaction count, profit)
    let mut res: (i32, i32) = (0, 0);

    // read transaction count
    io::stdin().read_line(&mut input).unwrap();

    // convert input to int val, assign to res.0
    res.0 = 2 * input.trim().parse::<i32>().unwrap();


    // loop fomr 0 to transaction count * 2
    for i in 0..res.0
        {
            
           // clear input 
           input.clear();

           // read input 
           io::stdin().read_line(&mut input).unwrap();


           // check if i odd = input is number
           if i % 2 != 0
            {
                res.1 += input.trim().parse::<i32>().unwrap();
            }

            // lesliecharleyhorse
                

        }

        //result
        // loss profit 
        if res.1 < 0
            {
                println!("Nekad");

            }

        // gain profit 
        else if res.1 > 0
            {
                println!("Usch, vinst");
            }

          // break even  
        else {println!("Lagom");}



}