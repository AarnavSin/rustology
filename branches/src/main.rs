fn main(){
    let number = 6;

    println!("\n");
    if number < 5 {
        println!("Condition is true.\n");
    }
    else {
        println!("Condition is false.\n");
    }

    //Divisibility Test

    if number % 4 == 0 {
        println!("Number is divisible by 4.");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3.");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2.");
    } else {
        println!("Number not divisible by 2, 3 and 4.");
    }

    //If in a variable

    let condition = false;
    let number = if condition {5} else {6};

    println!("The number is {number}.");

    //Loops

    //Loop

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}.");

    //Loop labelling

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
}