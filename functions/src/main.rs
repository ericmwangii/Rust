fn main() {
    println!("Hello, world!");

    another_function();

    another_function_example(5);

    print_labelled_measurements(5, 'h');


    //expressions and statements
    let z = {
        let d = 3;
        d + 1
    };

    println!("The value of z is: {z}");

    let u = five();

    println!("the value of u is : {u}");

    let plus_one = plus_one(5);

     println!("the value of plus_one is : {plus_one}");




}

fn another_function() {
    println!("Another Function!")
}


fn another_function_example(x: i32) {
    println!("The value of x is: {x}");
}


fn print_labelled_measurements(value: i32, unit_label: char){
    println!("The measurement is {value}{unit_label}")
}

//functions with return values
fn five() -> i32 {
    5
}

fn plus_one(x: i32)->i32 {
    x + 1 
}




