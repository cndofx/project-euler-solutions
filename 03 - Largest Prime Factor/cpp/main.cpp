#include <iostream>
#include <math.h>

int64_t largestPrimeFactor(int64_t num)
{
    int64_t original = num;
    int64_t factor = 3;
    int64_t previous = 1;
    while (factor <= num)
    {
        if (num % factor == 0)
        {
            previous = factor;
            while (num % factor == 0)
            {
                num /= factor;
            }
        }

        if (factor * factor > original)
        {
            break;
        }

        factor++;
    }

    return previous;
}

int main()
{
    int64_t result = largestPrimeFactor(600851475143);
    std::cout << "result: " << result << std::endl;
}