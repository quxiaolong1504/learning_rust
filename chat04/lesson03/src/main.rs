/*原生类型
https://kaisery.gitbooks.io/rust-book-chinese/content/content/Primitive%20Types%20%E5%8E%9F%E7%94%9F%E7%B1%BB%E5%9E%8B.html
*/
fn main() {
    let x = true;
    let x = '1'; // 4个字节
    let x = '🍑';
    let x = 10; // i32
    let x = 10.2; // f64
    let x = [1, 2, 3];
    //let x = [0; 20];
    println!("x has {} elements.", x.len());

    let names = ["Odin", "Zeus", "Hugin"];

    println!("The second name is: {}!", names[1]);

    let midle = &x[1..2];
    println!("{}", midle[0]);

    let tuple = (1, 2, 3, 4);
    println!("{}", tuple.0);

}