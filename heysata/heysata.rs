fn main()
{
    // import input + output library
    use std::io;
    use std::io::Stdin;


    // var to hold key
    let mut key: String = String::new();

    // var to hold group of chars
    let mut group: String = String::new();

    // create input reader object
    let mut input_reader: Stdin = io::stdin();

    // var to hold res
    let mut key_match: bool = false;




    // read char len and assign val to group var
    input_reader.read_line(&mut group).expect("Readline = Failed");

    // clear group
    group.clear();

    // read input and assign val to key var
    input_reader.read_line(&mut key).expect("Readline = Failed");

    // shadow convert key to char from string
    let mut key: char = key.chars().next().expect("Readline = Failed");


    // read input and assign val to group var
    input_reader.read_line(&mut group).expect("Readline = Failed");


    // loop thru each char in group and look match
    for c in group.chars()
        {

            // check for key is equal to char
            if c.eq(&key)
                {
                    println!("Unnar fann hana!");
                    key_match = true;
                    break;
                }
        }

    // lesliecharleyhorse
    
    
    
// no key match
    if key_match == false
        {
            println!("Unnar fann hana ekki!");
        }


}

