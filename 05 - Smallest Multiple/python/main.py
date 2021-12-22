import math

def smallestMultiple(upTo):
    # sqrt(10) = 3.16
    # up to 10: 2^3 * 3^2 * 5 * 7 = 2520
    #           p[i] ^ a[i] ...
    # p^a = k
    # a log(p) = log(k)
    # a = log(k) / log(p)
    result = 1
    continueChecking = True
    limit = math.sqrt(upTo) # no point checking higher
    primes = generatePrimes(upTo)
    exponents = [0]
    for i, val in enumerate(primes):
        exponents.append(1)
        if continueChecking:
            if primes[i] < limit:
                exponents[i] = math.floor(math.log(upTo) / math.log(primes[i]))
            else:
                continueChecking = False # dont check future exponents if p^2 > sqrt(upTo)
        result *= math.pow(primes[i], exponents[i])
        print("{} ^ {} = {} *".format(primes[i], exponents[i], result))
    return int(result)

def generatePrimes(upTo):
    composites = []
    primes = []
    for i in range (2, 7+1): # generate all composite numbers
        for j in range (2*i, upTo+1, i):
            composites.append(j)
    for i in range (2, upTo+1): # get all numbers not in composites
        if i not in composites:
            primes.append(i)
    return primes

result = smallestMultiple(5)
print("result: " + str(result))