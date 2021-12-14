#![allow(non_snake_case)]

fn multiplesOf3and5(limit: i32) -> i32
{
    let mut sum: i32 = 0;
    for i in 1..limit
    {
        if i % 3 == 0 || i % 5 == 0
        {
            println!("{}", i);
            sum += i;
        }
    }
    return sum;
}

fn main()
{
    let result: i32 = multiplesOf3and5(1000);
    println!("result: {}", result);
}