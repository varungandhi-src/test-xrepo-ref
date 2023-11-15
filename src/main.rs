use mylib::*;

struct TI {}

impl T for TI {}

fn main() {
    S{}.f(TI{});
    println!("Hello, world!");
}
