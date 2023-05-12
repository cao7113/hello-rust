fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);

    if let Some(i) = v.pop() {
        assert_eq!(i, 4)
    }
    // match v.pop() {
    //     Some(i) => assert_eq!(i, 4),
    //     None => (),
    // }
    println!("vector v = {v:?}");

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("vector v = {v:?}");
}
