fn add1(x: u32) -> u32 {
    x + 1
}

fn double(x: u32) -> u32 {
    x + x
}

fn twice<T>(f: impl Fn(T) -> T) -> impl Fn(T) -> T {
    move |x| f(f(x))
}

fn main() {
    println!("Hello, world!");

    println!("twice with add1 for 4: {}", twice(add1)(4));
    println!("twice with double for 4: {}", twice(double)(4));

    let dbl = |x: f32| -> f32 { x + x };

    println!("twice with double for 4.0: {}", twice(dbl)(4.0));
}
