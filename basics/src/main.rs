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
}
