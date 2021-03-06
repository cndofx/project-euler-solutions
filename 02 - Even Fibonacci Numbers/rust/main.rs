fn main()
{
    let result: i32 = fibo_even_sum(4000000);
    println!("result: {}", result);
}

fn fibo_even_sum(limit: i32) -> i32
{
    let mut x: i32 = 1;
    let mut y: i32 = 2;

    let mut sum: i32 = y;

    while y < limit
    {
        let buf: i32 = x + y;
        x = y;
        y = buf;

        if y % 2 == 0
        {
            sum += y;
        }
    }

    return sum;
}