import std/algorithm

proc reverse_number(num: int): int =
    var num = num
    var reversed: int = 0
    while num > 0:
        reversed = (reversed * 10) + (num mod 10)
        num = num div 10
    reversed

proc is_palindrome(num: int): bool =
    num == reverse_number(num)

proc largest_palindrome_product(low: int, high: int): int =
    var palindromes = newSeq[int]()
    for i in low ..< high:
        for j in low ..< high:
            let product: int = i * j
            if is_palindrome(product):
                palindromes.add(product)
    palindromes.sort()
    palindromes[^1]

let result: int = largest_palindrome_product(100, 1000)
echo "result: ", result
