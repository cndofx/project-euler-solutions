proc multiples_of_3_and_5(limit: int): int =
    var sum: int = 0
    for i in 1 ..< limit:
        if i mod 3 == 0 or i mod 5 == 0:
            echo i
            sum += i
    sum

let result: int = multiples_of_3_and_5(1000)
echo "result: ", result