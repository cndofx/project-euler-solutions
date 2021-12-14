import math

def largestPrimeFactor(num):
    factor = 3
    previous = 1
    while factor < int(math.sqrt(num))+1:
        if num % factor == 0:
            previous = factor
            while num % factor == 0:
                num /= factor
        factor += 2

    return previous

result = largestPrimeFactor(600851475143)
print("result: " + str(result))