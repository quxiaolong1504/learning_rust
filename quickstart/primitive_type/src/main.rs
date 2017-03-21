// 原生类型
// https://rustcc.gitbooks.io/rustprimer/content/quickstart/primitive-type.html

fn main() {
    // 类型推断
    let a1 = 3;
    let a2: i32 = 3;
    assert_eq!(a1, a2);

    // let 解构
    let (a, mut b): (bool, bool) = (true, false);
    println!("a:{}, b:{}", a, b);
    b = true;
    assert_eq!(a, b);
    
    // bool type
    let a = true;
    let b: bool = false;

    // char type
    let c = 'c';

    // numeric types
    let x = 42;
    let y: u32 = 1_231_456;
    let z: f64 = 1.23e+2;
    let zero = z.abs_sub(123.4);
    let bin = 0b0000_0110;
    let oct = 0o7320_1546;
    let hex = 0xf23a_b049;

    // string type
    let str = "Hello, Rust!";
    let mut string = str.to_string();

    // array and slices
    let a = [0, 1, 2, 3, 4];
    let middle = &a[1..4];
    let ten_zeroes: [i32; 10] = [0; 10];

    // tuples
    let tuple: (i32, &str) = (50, "hello");
    let (fifty, _) = tuple;
    let hello = tuple.1;

    // raw pointers
    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe { *raw };

    // function
    fn foo(x:i32) -> i32 {
        x
    }
    let bar: fn(i32) -> i32 = foo;
    println!("{}", bar(10));
}
