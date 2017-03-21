// 函数与方法
// https://rustcc.gitbooks.io/rustprimer/content/quickstart/function-method.html
use std::f64::consts::PI;

fn add_one(x: i32) -> i32 {
    x + 1
}

fn diverges() -> ! {
    panic!("This is function never returns!");
}


// 高阶函数, 有点类似于 python 的 decorator, 但没有@语法🍬
fn apply<F>(f: F, y: i32) -> i32 where F: Fn(i32) -> i32{
    f(y) * y
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle{
        Circle{
                x: x, 
                y: y, 
                radius: radius,
            }
    }

    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

fn main() {
    // let x: i32 = diverges();

    // 匿名函数
    let num = 5;
    let plus_num = |x: i32| x + num;
    let ten = plus_num(4);
    println!("{}", ten);

    let num = apply(add_one, 5);
    println!("{}", num);

    let circle = Circle{ x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", circle.area());
    println!("{}", Circle::new(0.0, 0.0, 2.0).area());
}
