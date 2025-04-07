# Approach 1

Go from the ground up, rather than directly using the primitive types that are already provided by the rust/soroban. Create your own 128-bit signed integer and then use that instead of the i128 type already provided. Follow first princple thinking.


# Approach 2

Follow the same approach as approach 1, but you reimplement the soroban-fixed-point-math library and then do perfomance comparison and then prove that it is better in terms of memory consumed / fees.