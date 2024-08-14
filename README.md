# PrimeFinder
#Function Descriptions
1. is_prime(n: u128) -> bool

    This function checks if a given number n is prime. It returns true if the number is prime and false if it is not. The function works by testing divisibility from 2 up to the square root of n. If any divisor is found, the number is found to not be prime.

2. find_primes(n: u128) -> Vec<u128>

    This function generates a list of all prime numbers less than a given number n. It iterates through all numbers from 2 to n, using the is_prime function to check for primality. All prime numbers found are stored in a vector and returned.

3. check_if_composite(n: u128) -> String

  This function determines whether a given number n is prime or composite. It uses the is_prime function and returns a string indicating whether the number is prime or composite. This is a simple utility function that provides a user-friendly output.