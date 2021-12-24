fn main() {
    sieve(999);
}

fn sieve(num: u64) {
    let mut primes: Vec<bool> = vec![true; (num+1) as usize];

    let mut i: usize = 2;
    while i.pow(2) as u64 <= num {
        if primes[i] == true {
            println!("{} is prime", i);
            let mut j: usize = i.pow(2);
            while j as u64 <= num {
                primes[j] = false;
                println!("crossed out {}", j);
                j += i;
            }
        }
        i += 1;
    }

    let mut i: usize = 2;
    while i as u64 <= num {
        if primes[i] == true {
            println!("{}", i);
        }
        i += 1;
    }
}

// https://www.geeksforgeeks.org/sieve-of-eratosthenes/
