fn addN(n: u32) -> impl Fn(u32) -> u32 {
    move |x| x + n
}

fn add1(x: u32) -> u32 {
    (addN(1))(x)
}

fn twice(f: Box<dyn Fn(u32) -> u32>) -> Box<dyn Fn(u32) -> u32> {
    Box::new(move |x| f(f(x)))
}

fn apply_twice<F: Fn(u32) -> u32>(f: F, x: u32) -> u32 {
    f(f(x))
}

fn main() {
    println!("{}", add1(3));
    println!("{}", (twice(Box::new(add1)))(3));
    println!("{}", (apply_twice(add1, 3)));
}
