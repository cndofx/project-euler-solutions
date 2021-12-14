def multiplesOf3and5(limit):
    sum = 0
    for i in range(1, limit):
        if(i % 3 == 0 or i % 5 == 0):
            print(i)
            sum += i
    return sum

result = multiplesOf3and5(1000)
print("result:" + str(result))