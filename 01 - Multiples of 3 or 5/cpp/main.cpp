#include <iostream>

int multiplesOf3and5(int limit)
{
    int sum = 0;
    for (int i = 1; i < limit; i++)
    {
        if (i % 3 == 0 || i % 5 == 0)
        {
            std::cout << i << std::endl;
            sum += i;
        }
    }
    return sum;
}

int main()
{
    int result = multiplesOf3and5(1000);
    std::cout << "result: " << result << std::endl;
}