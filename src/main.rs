use shuffle_vector::ShuffleVector;

fn main() {
    println!("main.rs is happening now");
    let mut v = ShuffleVector::new (vec!());

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    dbg!(v.clone());

    dbg!(v.pop());
    dbg!(v.pop());
    dbg!(v.pop());

    dbg!(v.clone());
}