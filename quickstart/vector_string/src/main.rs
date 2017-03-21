// 数组、动态数组和字符串
// https://rustcc.gitbooks.io/rustprimer/content/quickstart/vector-string.html

fn main() {
    let mut array: [i32; 3] = [0; 3];
    array[1] = 1;
    array[2] = 2;
    assert_eq!([1, 2], &array[1..]);

    for x in &array {
        println!("{}", x);
    }

    // 动态数组
    let v: Vec<i32> = Vec::new();
    
    // 用宏创建 vec
    let v: Vec<i32> = vec![];

    let v  = vec![1, 2, 3, 4, 5];

    let v = vec![0; 10];

    let mut v = vec![1, 2];
    v.push(3);

    let mut v = vec![1, 2];
    let two = v.pop(); 

    let mut v = vec![1, 2, 3];
    let three = v[2];
    v[1] = v[1] + 5; 
    v.push(10);

    // 字符串
    let hello = "hello world";
    // 附带显式类型标识
    let hello: &'static str = "hello wold";

    let mut s = String::new();
    let mut hello = String::from("Hello, ");
    // 压入字符和字符串切片
    hello.push('w');
    hello.push_str("orld!");

    println!("{}", hello);

    // 弹出字符串
    let mut s = String::from("foo");
    assert_eq!(s.pop(), Some('o'));
    assert_eq!(s.pop(), Some('o'));
    assert_eq!(s.pop(), Some('f'));
    assert_eq!(s.pop(), None);
}
