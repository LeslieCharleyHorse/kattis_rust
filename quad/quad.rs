fn main()
{
    // input lib
    use std::io;
    
    // var to hold input
    let mut input: String = String::new();
    
    // vector to hold coordinates [x, y]
    let mut res: [i32; 2] = [0, 0];
    
    
    // lesliecharleyhorse
    // read in x coord
    io::stdin().read_line(&mut input).unwrap();
    
    // asign x val
    res[0] = input.trim().parse::<i32>().unwrap();
    
    // clear input
    input.clear();
    
// read in y coord
    io::stdin().read_line(&mut input).unwrap();
    
    // asign y val
    res[1] = input.trim().parse::<i32>().unwrap();
    
    
    // check for quadrant
    // check x val
    if res[0] >= 0
        {
            
            //lesliecharleyhorse
            //check y val
            if res[1] >= 0
                {println!("1");}
            else {println!("4");}
        }
        
    else
        {
            // check y val
            if res[1] >= 0
                {println!("2");}
            else
                {println!("3")}
                
        }
    
}