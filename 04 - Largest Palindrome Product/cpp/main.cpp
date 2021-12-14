#include <iostream>
#include <vector>
#include <algorithm>

int reverseNumber(int num)
{
    int reversed = 0;
    while (num > 0)
    {
        reversed = (reversed * 10) + (num % 10);
        num = num / 10;
    }
    return reversed;
}

bool isPalindrome(int num)
{
    return num == reverseNumber(num);
}

int largestPalindromeProduct(int low, int high)
{
    std::vector<int> palindromes = {};
    for (int i = low; i < high; i++)
    {
        for (int j = low; j < high; j++)
        {
            int product = i * j;
            if (isPalindrome(product))
            {
                palindromes.push_back(product);
            }
        }
    }
    std::sort(palindromes.begin(), palindromes.end());
    return palindromes.back();
}

int main()
{
    int result = largestPalindromeProduct(100, 1000);
    std::cout << "result: " << result << std::endl;
}