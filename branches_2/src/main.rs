fn main() {
    let mut number = 5;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5{
        println!("the value is {}", a[index]);
        index += 1;
    }

    println!("\n");

    //We can accomplish the same thing via a for loop

    let b = [10, 15, 20, 25, 30];

    for element in b {
        println!("the value is: {element}");
    }

    for number in (1..4).rev(){
        println!("{number}");
    }
    println!("LIFTOFF!")
}
