fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }

    if number !=0 {
        println!("Number was something other than zero")
    }

    let condition = true;

    let number = if condition {5} else {6};

    println!("The value of number is: {number}");
    

    //loop keyword
    // loop {
    //     println!("again");
    // }


    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    //naming loops

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

            remaining -=1;
        }

        count +=1;
    }

    println!("End count = {count}");


    //while loop
    let mut w_number = 3;

    while w_number != 0 {
        println!("{w_number}!");

        w_number -= 1;
    }

    println!("LIFTOFF!!");

    //for loop

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}")
    }


    //liftoff

    for k in (1..4).rev() {
        println!("{k}");
    }

    println!("Rev Liftofff");

}
