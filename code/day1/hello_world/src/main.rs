fn main() {
    println!("Hello, world!");
    let x = 3;
    let v = vec![1,3,4,5,6];

    v.iter().for_each(move |foo| {
        println!("{}", foo * x);
    });

    for foo in &v {
        println!("{}", foo);
    }
}
