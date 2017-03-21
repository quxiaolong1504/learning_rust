// 控制流
// https://rustcc.gitbooks.io/rustprimer/content/quickstart/control-flow.html

fn main() {
    
    // if 
    let x = 5;
    let y = if x == 5 { 10 } else { 15 };
    println!("{}", y);
    if x == 5{
        // do something
        println!("x is {}", x);
    } else {
        // do otherthing
        print!("x is {}", x);
    }

    // for
    for x in [0, 1, 2].iter() {
        println!("{}", x);
    }

    // loop
    'outer: loop {
        println!("Entered the outer loop!");

        'inner: loop {
            println!("Entered the inner loop!");
            break 'outer;
        }
        println!("This point will never be reached.");
    }

    println!("Exited outer loop!");

    // match
    let day = 5;
    match day {
        0|6 => println!("weekend"),
        1 ... 5 => println!("weekday"),
        _ => println!("invalid")
    }

    let x = 1;
    match x {
        e @ 1 ... 5 => println!("got a range element {}", e),
        _ => println!("anything")
    }

    let x = 5;
    let mut y = 1;
    match x {
        ref x => println!("Got a reference to {}", x),
    }

    match y {
        ref y => println!("Got a mutable reference to {}", y)
    }

    let pair  = (0, -2);
    match pair {
        (0, y) => println!("x is 0, y is {}", y),
        (x, 0) => println!("x is {}, y is 0", x),
        _ => println!("It doesn't matter what they are")
    }

    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point{x: 1, y: 2};
    match point {
        Point {x, ..} => println!("x is {}", x)
    }
    
    for x in 0..10{
        println!("{}", x);
    }

    for (i, x) in (5..10).enumerate() {
        println!("{}: {}", i, x);
    }
    for (i, x) in (5..20).enumerate() {
        println!("{}, {}", i, x);
    }

}
