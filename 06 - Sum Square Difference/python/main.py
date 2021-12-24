def sumOfSquares(upTo):
    sum = 0
    for i in range(1, upTo + 1):
        sum += pow(i, 2)
    return sum

def squareOfSum(upTo):
    sum = 0
    for i in range(1, upTo + 1):
        sum += i
    return pow(sum, 2)

def sumSquareDifference(upTo):
    return squareOfSum(upTo) - sumOfSquares(upTo)

result = sumSquareDifference(100)
print("result: " + str(result))