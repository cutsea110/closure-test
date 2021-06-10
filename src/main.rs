use std::io::Error;
use std::thread;

fn add_n(n: u32) -> impl Fn(u32) -> u32 {
    move |x| x + n
}

fn add1(x: u32) -> u32 {
    (add_n(1))(x)
}

fn twice<F>(f: F) -> impl Fn(u32) -> u32
where
    F: Fn(u32) -> u32,
{
    move |x| f(f(x))
}

fn apply_twice<F>(f: F, x: u32) -> u32
where
    F: Fn(u32) -> u32,
{
    f(f(x))
}

async fn async_add_n(n: u32) -> impl Fn(u32) -> u32 {
    move |x| x + n
}

async fn async_add1(x: u32) -> Result<u32, Error> {
    let f = async_add_n(1).await;
    Ok(f(x))
}

fn main() {
    println!("{}", add1(3));
    println!("{}", (twice(Box::new(add1)))(3));
    println!("{}", (apply_twice(add1, 3)));

    let child = thread::spawn(move || add1(7));
    match child.join() {
        Ok(n) => println!("{}", n),
        Err(e) => println!("error: {:?}", e),
    }
}
