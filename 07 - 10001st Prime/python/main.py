# https://stemhash.com/how-to-find-the-nth-prime-number/

def sieve(num):
    primes = [True for i in range(1, num+2)]
    i = 2
    while i ** 2 <= num:
        if primes[i] == True:
            for j in range(i ** 2, num + 1, i):
                primes[j] = False
        i += 1
    result = []
    for k in range(2, num+1):
        if primes[k]:
            result.append(k)
    return result

def get_nth_prime(nth):
    count = 0
    size = 2
    limit = nth * size
    while count < nth:
        primes = sieve(limit)
        count = len(primes)
        size += 1
        limit = nth * size
    return primes[nth-1]

result = get_nth_prime(10001)
print("result: " + str(result))