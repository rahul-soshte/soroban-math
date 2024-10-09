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
