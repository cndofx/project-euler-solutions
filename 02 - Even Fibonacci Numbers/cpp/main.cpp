#include <iostream>

int fiboEvenSum(int limit)
{
    int x = 1;
    int y = 2;

    int sum = y;

    while (y <= limit)
    {
        int buf = x + y;
        x = y;
        y = buf;

        if (y % 2 == 0)
        {
            sum += y;
        }
    }

    return sum;
}

int main()
{
    int result = fiboEvenSum(4000000);
    std::cout << "result: " << result << std::endl;
}