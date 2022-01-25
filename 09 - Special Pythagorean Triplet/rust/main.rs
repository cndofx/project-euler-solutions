fn main() {
    println!("result: {}", special_pythagorean_triplet(1000));
}

fn check_triplet(a: u32, b: u32, c: u32) -> bool {
    a.pow(2) + b.pow(2) == c.pow(2) && a < b && b < c
}

fn special_pythagorean_triplet(limit: u32) -> u64 {
    for a in 1..=limit {
        for b in 1..=limit {
            if a + b > limit {
                break;
            }
            let c = limit - a - b;
            if check_triplet(a, b, c) {
                println!("{}^2 + {}^2 = {}^2", a, b, c);
                return a as u64 * b as u64 * c as u64;
            }
        }
    }
    println!("no triplet found");
    0
}