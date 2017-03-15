
/*变量绑定
https://kaisery.gitbooks.io/rust-book-chinese/content/content/Variable%20Bindings%20%E5%8F%98%E9%87%8F%E7%BB%91%E5%AE%9A.html
*/

fn main() {
    let x: i32 = 5;
    {
        let y = 10;
        println!("x:{} y:{}", x, y); // 5, 10

        let x = 20;
        println!("x:{} y:{}", x, y); // 20, 10
    }
    println!("x:{}", x);
    // println!("y:{}", y); error[E0425]: unresolved name `y`
}
