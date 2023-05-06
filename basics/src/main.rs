fn main() {
    // Variables
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Types

    // tuple
    let tup = (1, 2.3, 'a');
    println!("tuple value {tup:?}");
    // destructing
    let (_, y, _z) = tup;
    println!("tuple element y is: {y}");
    let y1 = tup.1;
    println!("tuple element y by index: {y1}");

    let unit = ();
    println!("empty tuple as unit value: {unit:?}");

    // array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array a value: {a:?}");
    let b = [3; 5];
    println!("array b value: {b:?}");

    // statements and expressions
    let y = {
        let x = 5;
        x + 1
    };
    println!("y value is {y}");

    try_loop();

    // slice

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
    println!(
        "rect1: {:#?} with width {} and height {}",
        rect1, rect1.width, rect1.height
    )
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn try_loop() {
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
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

// fn for_loop() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is: {element}");
//     }
// }

// fn count_down_loop() {
//     for number in (1..4).rev() {
//         println!("{number}!");
//     }
//     println!("LIFTOFF!!!");
// }
