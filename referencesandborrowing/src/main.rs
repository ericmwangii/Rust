fn main() {
    let mut s = String::from("hello");

    // change(&mut s);

    //let r1 = &mut s;
    // let r2 = &mut s;     // println!("{}, {}", r1, r2);  -> error, can't borrow s as a mutable more than once

    {
        let r1 = &mut s;
    }

    let r2 = &mut s;

}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}