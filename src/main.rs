use rum::rumload;
use rum::rumrun;
use std::env;
fn main() {
    let input = env::args().nth(1);
    let instructions = rumload::load(input.as_deref());
    rumrun::run(instructions);
}