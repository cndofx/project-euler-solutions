#include <iostream>
#include <vector>
#include <cmath>

std::vector<uint> sieve(uint num)
{
    std::vector<bool> primes = {};
    for (int i = 0; i <= num; i++)
    {
        primes.push_back(true);
    }
    uint i = 2;
    while (pow(i, 2) <= num)
    {
        if (primes[i] == true)
        {
            for (int j = pow(i, 2); j < num; j += i)
            {
                primes[j] = false;
            }
        }
        i++;
    }
    std::vector<uint> result = {};
    for (int k = 2; k < num; k++)
    {
        if (primes[k] == true)
        {
            result.push_back(k);
        }
    }
    return result;
}

uint getNthPrime(uint nth)
{
    uint count = 0;
    uint size = 2;
    uint limit = nth * size;
    std::vector<uint> primes = {};
    while (count < nth)
    {
        primes = sieve(limit);
        count = primes.size();
        size++;
        limit = nth * size;
    }
    return primes[nth-1];
}

int main()
{
    uint result = getNthPrime(10001);
    std::cout << "result: " << result << std::endl;
}