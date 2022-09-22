use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Arguments", about = "For Sieve of Eratosthenes.")]
struct Opt {
    /// A max value of Sieve of Eratosthenes execution.
    #[structopt(short = "m", long = "max", default_value = "10000")]
    max: u64,
}

pub fn fetch_max_value() -> u64 {
    let opt = Opt::from_args();
    let max = opt.max;
    if check_max_value(max) {
        return max;
    }

    panic!("max value must be at least 10000.")
}

fn check_max_value(value: u64) -> bool {
    const MAX_AVAILABLE_VALUE: u64 = 10000;
    value <= MAX_AVAILABLE_VALUE
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn value_under_10000_should_pass_check_max_value() {
        assert!(check_max_value(10000));
    }

    #[test]
    fn value_over_10001_should_not_pass_check_max_value() {
        assert!(!check_max_value(10001));
    }
}
