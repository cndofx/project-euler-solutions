def fiboEvenSum(limit):
    x = 1
    y = 2

    sum = y;

    while (y <= limit):
        buf = x + y
        x = y
        y = buf

        if (y % 2 == 0):
            sum += y

    return sum

result = fiboEvenSum(4000000)
print("result: " + str(result))