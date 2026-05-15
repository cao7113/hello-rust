use std::mem;

fn main() {
    // inspect integer
    let i1: i16 = 3;
    let bytes = mem::size_of_val(&i1);
    let bsize = bytes * 8;
    println!(
        "integer i1 {} take {} bytes in binary {:0>bsize$b}",
        i1, bytes, i1
    );

    assert_eq!(4, mem::size_of::<i32>());

    // tuple
    let tp = (1, 2, 3);
    println!("tuple tp: {:?}", tp)
}
