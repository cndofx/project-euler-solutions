#include <cstdint>
#include <math.h>
#include <iostream>

int64_t sumOfSquares(int64_t upTo)
{
    int64_t sum = 0;
    for (int i = 1; i <= upTo; i++)
    {
        sum += pow(i, 2);
    }
    return sum;
}

int64_t squareOfSum(int64_t upTo)
{
    int64_t sum = 0;
    for (int i = 1; i <= upTo; i++)
    {
        sum += i;
    }
    return pow(sum, 2);
}

int64_t sumSquareDifference(int64_t upTo)
{
    return squareOfSum(upTo) - sumOfSquares(upTo);
}

int main()
{
    int64_t result = sumSquareDifference(100);
    std::cout << "result: " << result << std::endl;
}