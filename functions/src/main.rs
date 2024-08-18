fn main() {
    another_function(5);
    print_labeled_measurements(15,"meters");

    let y = {
        let x = 5;
        x + 1
    };

    println!("y = {y}");

    let five_five = five();
    println!("Five is {five_five}");

    let string_gen_var = string_gen();
    println!("{string_gen_var}");

    let arith_x = arith(5);
    println!("+32 = {arith_x}") 
}

fn another_function(x: i32){
    println!("The value of x is {x}.");
}

fn print_labeled_measurements(value: i32, unit_label: &str){ //
    println!("The measurement is {value} {unit_label}.");
}

fn five() -> i32 {
    5
}

fn string_gen() -> String {
    String::from("this is a string!")
}

fn arith(arith_x: i32) -> i32 {
    arith_x + 32 //comment!
}

/*
MULTILINE COMMENT!
*/