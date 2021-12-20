fn main()
{
    let result: i32 = largest_palindrome_product(100, 1000);
    println!("result: {}", result);
}

fn reverse_number(mut num: i32) -> i32
{
    let mut reversed: i32 = 0;
    while num > 0
    {
        reversed = (reversed * 10) + (num % 10);
        num = num / 10;
    }
    return reversed;
}

fn is_palindrome(num: i32) -> bool
{
    return num == reverse_number(num);
}

fn largest_palindrome_product(low: i32, high: i32) -> i32
{
    let mut palindromes: Vec<i32> = Vec::new();
    for i in low..high
    {
        for j in low..high
        {
            let product: i32 = i * j;
            if is_palindrome(product)
            {
                palindromes.push(product);
            }
        }
    }
    palindromes.sort();
    return *palindromes.last().unwrap();
}