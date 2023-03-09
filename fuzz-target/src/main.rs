#[macro_use]
extern crate afl;
extern crate fuzzyfuzz;

fn main() {
    fuzz!(|data: usize| {
        let _ = fuzzyfuzz::add(data, data);
    });
}