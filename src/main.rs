fn not_compelte_fn(x: usize) -> bool {
    if x > 10 {
        return true;
    }

    todo!("Hey, me finish this later");
}

fn main() {
    println!("{}", not_compelte_fn(12));
    println!("{}", not_compelte_fn(2));

    let foo = Some(5); // Option - i32 | undefined
    let bar = foo.unwrap(); // takes a value form foo (even if it can be undefined, dangerous)
}
