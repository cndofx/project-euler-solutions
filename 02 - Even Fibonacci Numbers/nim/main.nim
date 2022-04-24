proc fibo_even_sum(limit: int): int =
    var x: int = 1
    var y: int = 2
    var sum: int = y;

    while y < limit:
        let buf: int = x + y
        x = y
        y = buf

        if y mod 2 == 0:
            sum += y

    sum

let result: int = fibo_even_sum(4000000)
echo "result: ", result
