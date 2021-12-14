#![allow(non_snake_case)]

fn largestPrimeFactor(mut num: i64) -> i64
{
    let original: i64 = num;
    let mut factor: i64 = 3;
    let mut previous: i64 = 3;
    while factor <= num
    {
        if num % factor == 0
        {
            previous = factor;
            while num % factor == 0
            {
                num /= factor;
            }
        }

        if factor * factor > original
        {
            break
        }

        factor += 1;
    }

    return previous;
}

fn main()
{
    let result: i64 = largestPrimeFactor(600851475143);
    println!("result: {}", result);
}