use std::thread;
use std::time::Duration;

fn add1(x: u32) -> u32 {
    x + 1
}

fn double(x: u32) -> u32 {
    x + x
}

fn twice<T>(f: impl Fn(T) -> T) -> impl Fn(T) -> T {
    move |x| f(f(x))
}

struct Cacher<T, F>
where
    F: Fn(T) -> T,
{
    calculation: F,
    value: Option<T>,
}
impl<T, F> Cacher<T, F>
where
    T: Copy,
    F: Fn(T) -> T,
{
    fn new(calculation: F) -> Cacher<T, F> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: T) -> T {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    println!("Hello, world!");

    let x = vec![1, 2, 3];
    let eq_to_x = |z| z == x;
    println!("{:?}", x);
    let y = vec![1, 2, 3];
    assert!(eq_to_x(y));

    println!("twice with add1 for 4: {}", twice(add1)(4));
    println!("twice with double for 4: {}", twice(double)(4));

    let dbl = |x: f32| -> f32 { x + x };

    println!("twice with double for 4.0: {}", twice(dbl)(4.0));

    let mut expensive_result = Cacher::new(|num: u32| -> u32 {
        println!("calculationg slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    println!("1st time: {}", expensive_result.value(12));
    println!("2nd time: {}", expensive_result.value(12));
}
