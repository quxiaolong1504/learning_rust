/* functions 
https://kaisery.gitbooks.io/rust-book-chinese/content/content/Functions%20%E5%87%BD%E6%95%B0.html
*/
fn main() {
    print_number(5);

    print_sum(10, 9);

    print_number(add_one(9));

    let f = print_number;
    
    f(99);
}

fn print_number(x: i32) {
    println!("x is :{}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn add_one(i: i32) -> i32 {
    i + 1
}