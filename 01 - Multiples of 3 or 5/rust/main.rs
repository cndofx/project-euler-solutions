fn main()
{
    let result: i32 = multiples_of_3_and_5(1000);
    println!("result: {}", result);
}

fn multiples_of_3_and_5(limit: i32) -> i32
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