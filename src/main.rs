mod argument;

fn main() {
    let max = argument::fetch_max_value();
    println!("{}", max);
}
