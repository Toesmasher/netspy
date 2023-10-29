mod args;

fn main() {
    let a = args::Args::get();

    println!("{:?}", a);
}
