import math

def reverseNumber(num):
    reversed = 0
    while (num > 0):
        reversed = (reversed * 10) + (num % 10)
        num = math.floor(num / 10)
    return reversed

def isPalindrome(num):
    return num == reverseNumber(num)

def largestPalindromeProduct(low, high):
    palindromes = []
    for i in range (low, high):
        for j in range (low, high):
            product = i * j
            if isPalindrome(product):
                palindromes.append(product)
    palindromes.sort()
    return palindromes[-1]

result = largestPalindromeProduct(100, 1000)
print(result)