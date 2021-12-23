#include <vector>
#include <cstdint>
#include <algorithm>
#include <iostream>
#include <math.h>

std::vector<int64_t> generatePrimes(int64_t upTo) {
    std::vector<int64_t> composites = {};
    std::vector<int64_t> primes = {};
    for (int i = 2; i<= 7; i++) 
    {
        for (int j = 2 * i; j <= upTo; j += i)
        {
            primes.push_back(j);
        }
    }
    for (int i = 2; i <= upTo; i++)
    {
        if (std::find(primes.begin(), primes.end(), i) == primes.end())
        {
            composites.push_back(i);
        }
    }
    return composites;
}

int64_t smallestMultiple(int64_t upTo) {
    bool continueChecking = true;
    int64_t result = 1;
    int64_t limit = pow(upTo, 0.5) + 1;
    std::vector<int64_t> primes = generatePrimes(upTo);
    std::vector<int64_t> exponents = {};

    int i = 0;
    while (i < primes.size())
    {
        exponents.push_back(1);
        if (continueChecking) 
        {
            if (primes[i] < limit)
            {
                exponents[i] = std::floor(std::log(upTo) / std::log(primes[i]));
            }
            else
            {
                continueChecking = false;
            }
        }
        result *= pow(primes[i], exponents[i]);
        std::cout << primes[i] << " ^ " << exponents[i] << " = " << result << " *" << std::endl;
        i++;
    }
    return result;
}

int main() 
{
    int64_t result = smallestMultiple(20);
    std::cout << "result: " << result << std::endl;
}