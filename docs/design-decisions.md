# Rules for Arithmetic

## Division at the end

The only place where precision loss occurs in fixed point math is in division, so it is best to do delay it whenever possible.

## Target Type for SoroNum Arithmetic

The point of having a target type is that it is useful to map the output to multiple possible types of an enum

## Scale as parameter for SoroNum

It is important to specifiy a scale for a SoroNum, since we are trying to have symbolic representation of floating value. 

Suppose 1.23 is an example, it can be initialized in SoroNum as 

```rust
let x = SoroNum::new(123_i128, 2); // Symbolic representation of 1.23
```

## CALC_SCALE and OUT_SCALE generic

If two numbers are of different scales, the intermediate calculation must be done in much higher scale and we need to operate on them. And the OUT_SCALE is expected scale of the output.


## Flooring and Ceiling

TODO: ??

## Types of Errors Define

1. Overflow
2. Underflow


## Downcast and Upcasting (for eg, u256 to u512)

Need to conversion for each type, even if it is available inside rust core, or the soroban-sdk.

## Not Ignoring Remainder or Truncation

If there is any truncation happening in a division, it would be good practice to return the truncated part as well, even if it might be not useful. 

## Switchable Algorithm

Multiple algorithms can be used for the same arithmetic operation. Considering that to improve usability, it can be a feature to be able to switch between the algorithms.

# Allocation of Fraction and Integer Bits

For example, provide a functionality so that the developer for a n-bit number is able to allocate x bits for integer part, and n - x bits for fraction part.
ref: https://docs.rs/fixed/latest/fixed/