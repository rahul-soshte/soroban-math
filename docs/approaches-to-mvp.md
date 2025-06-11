# Approach 1

Go from the ground up, rather than directly using the primitive types that are already provided by the rust/soroban. Create your own 128-bit signed integer and then use that instead of the i128 type already provided. Follow first princple thinking.


# Approach 2

Follow the same approach as approach 1, but you reimplement the soroban-fixed-point-math library and then do perfomance comparison and then prove that it is better in terms of memory consumed / fees. So yeah first principle thinking but for already existing product.


# Considerations

1. Follow the same directory structure as these. Feels pretty standard
https://github.com/paritytech/parity-common/tree/master/uint
https://github.com/nlordell/ethnum-rs
https://gitlab.com/tspiteri/fixed

## Precision Handling with Bytes

So in theory, while doing a computation, we could calculate the end result's number has how many decimals it has (i.e how many number it has after the decimal point), and then if we limit it to say, 18 decimals, then when in bytes format if a number goes 18 decimals, we can just edit out the uneeded part of the 18.

So lower precision is always bad. But the problem as well is that the i128 can handle only "x" amount of decimals, so if you perform calculation in 18 decimals, you will always have problems with losing precision when converting it. Might be wrong what I have said here.


# Approach 3

Why not seperate crate for each math function? So we don't make just one crate a cocktail for madness.


# Approach 4

Build custom macros that can be usable for any developer, and let him customize his types to be fit for his use case. Because most code is repeatable and we need to be just more smarter with our time.
Also error adjustment is needed, precision error become bigger and bigger, as many operations are performed.(Need to check and prove this hypotheses)