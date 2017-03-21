// 解构 和 枚举
// https://rustcc.gitbooks.io/rustprimer/content/quickstart/struct-enum.html
use std::cell::Cell;

fn main() {
    // struct
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point{x: 10, y: 0};

    // tuple struct
    struct Color(u8, u8, u8);
    let android_green = Color(0xa4, 0xc6, 0x39);
    let Color(red, green, blue) = android_green; // 解构 结构体

    // A tuple struct's constructors can be used as function
    struct Digit(i32);
    let v = vec![0, 1, 2];
    let d: Vec<Digit> = v.into_iter().map(Digit).collect();

    // struct Point {
    //     x: i32,
    //     y: Cell<i32>,
    // }
    // let mut point = Point { x: 5, y: Cell::new(6) };
    // point.y.set(7);

    // enum
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move { x: i32, y:i32},
        Write(String),
    }
    let x: Message = Message::Move { x: 3, y: 4 };

}
