use helios::{Operation, Op};

fn main() {
    let myop = Operation::new(Op::AddI, vec![0, 3, 14, 64]);

    println!("{}", myop.perform());
}