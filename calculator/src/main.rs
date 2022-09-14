use std::env::args;


fn main() {
    // calling the fn not the struct 

    let mut args = args();
    println!("{:?}",args);
    
    // args.nth returns an iterator 
    // so we are unwraping to consume 

    let first = args.nth(2).unwrap();
    
    // unwrap consumes element and then moves on 
    // so we are getting the 0 th element after every call


    let operator  = args.nth(0).unwrap();
    let second = args.nth(0).unwrap();
    
    println!("the operation performed is  {:?} {:?} {:?}",first,operator,second);

    let first_number:f32 = first.parse().unwrap();
    let second_number:f32 = second.parse().unwrap();



    println!("{:?} {:?} {:?}",first_number,operator,second_number);


}

fn operate(operator:char, first_number:f32,second_number:f32) -> f32 {

    if operator == '+' {
        return first_number + second_number;
    }
    else if operator == '-' {
        return  first_number - second_number
    }

    else if operator == '/' {
        return  first_number/second_number
    }
    else if operator == '*' {
        return  first_number - second_number
    }

    else {
        
        return 0.0; 
    }

} 


