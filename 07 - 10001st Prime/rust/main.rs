fn main() {
    let result = get_nth_prime(10001);
    println!("result: {}", result);
}

fn sieve(num: u32) -> Vec<u32> {
    let mut primes: Vec<bool> = Vec::new();
    for _ in 1..=num {
        primes.push(true);
    }
    let mut i: u32 = 2;
    while i.pow(2) <= num {
        if primes[i as usize] == true {
            for j in (i.pow(2)..num).step_by(i as usize) {
                primes[j as usize] = false;
            }
        }
        i += 1;
    }
    let mut result: Vec<u32> = Vec::new();
    for k in 2..num {
        if primes[k as usize] == true {
            result.push(k);
        }
    }
    result
}

fn get_nth_prime(nth: u32) -> u32 {
    let mut count: usize = 0;
    let mut size: u32 = 2;
    let mut limit: u32 = nth * size;
    let mut primes: Vec<u32> = Vec::new();
    while count < nth as usize {
        primes = sieve(limit);
        count = primes.len();
        size += 1;
        limit = nth * size;
    }
    primes[(nth-1) as usize]
}