use typegen_core::typegen;

fn main() {
    let filename = std::env::args().skip(1).next().unwrap();
    let f = std::fs::File::open(filename).unwrap();
    let output = typegen(f);
    println!("{}", output);
}
