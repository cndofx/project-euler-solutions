#![allow(non_snake_case)]

fn reverseNumber(mut num: i32) -> i32
{
    let mut reversed: i32 = 0;
    while num > 0
    {
        reversed = (reversed * 10) + (num % 10);
        num = num / 10;
    }
    return reversed;
}

fn isPalindrome(num: i32) -> bool
{
    return num == reverseNumber(num);
}

fn largestPalindromeProduct(low: i32, high: i32) -> i32
{
    let mut palindromes: Vec<i32> = Vec::new();
    for i in low..high
    {
        for j in low..high
        {
            let product: i32 = i * j;
            if isPalindrome(product)
            {
                palindromes.push(product);
            }
        }
    }
    palindromes.sort();
    return *palindromes.last().unwrap();
    //let test: i32 = palindromes[-1]; 
    //return test;
}

fn main()
{
    let result: i32 = largestPalindromeProduct(100, 1000);
    println!("result: {}", result);
}