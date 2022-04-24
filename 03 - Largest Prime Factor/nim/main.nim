proc largest_prime_factor(input: int64): int64 =
    var num: int64 = input
    var factor: int64 = 3
    var previous: int64 = 3
    while factor <= num:
        if num mod factor == 0:
            previous = factor
            while num mod factor == 0:
                num = num div factor
        if factor * factor > input:
            break
        factor += 1
    previous

let result: int64 = largest_prime_factor(600851475143)
echo "result: ", result
