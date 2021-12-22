use std::convert::TryInto;

fn main() {
    let result: i64 = smallest_multiple(20);
    println!("result: {}", result);
}

fn smallest_multiple(up_to: i64) -> i64 {
    let mut result: i64 = 1;
    let mut continue_checking: bool = true;
    let limit: i64 = (up_to as f64).sqrt() as i64 + 1;
    let primes: Vec<i64> = generate_primes(up_to);
    let mut exponents: Vec<i64> = vec![0];
    for (i, val) in primes.iter().enumerate() {
        exponents.push(1);
        if continue_checking {
            if val < &limit {
                exponents[i] = ((up_to as f64).log(10.0) / (primes[i] as f64).log(10.0)) as i64;
            }
            else {
                continue_checking = false;
            }
        }
        result *= primes[i].pow(exponents[i].try_into().unwrap());
        println!("{} ^ {} = {} *", primes[i], exponents[i], result);
    }
    result
}

fn generate_primes(up_to: i64) -> Vec<i64> {
    let mut composites: Vec<i64> = Vec::new();
    let mut primes: Vec<i64> = Vec::new();
    for i in 2..=7 {
        let mut j = 2 * i;
        while j <= up_to {
            composites.push(j);
            j += i;
        }
    }
    for i in 2..=up_to {
        if !composites.contains(&i) {
            primes.push(i);
        }
    }
    primes
}