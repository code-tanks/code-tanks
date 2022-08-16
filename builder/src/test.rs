
fn main() {
    let args: Vec<String> = env::args().collect();

    println!("url={}, lang={}", &args[1], &args[2]);

    build(&args[1], &args[2]);
}
