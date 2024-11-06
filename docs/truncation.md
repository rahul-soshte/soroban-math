# Truncations

What is the point of SCALE_OUT at all? Why not just put all the fractional data upto the point where it is feasible? What does feasibility depend upon exactly? Some question I have to still answer and figure out.

One of things will be appropriate here, 
1. Should have a Limitation of the SCALE_OUT param (for eg, 18)
2. Should have a Limitation of the CALC_SCALE param (for eg, 10)
3. Having a seperate field in the SoroNum struct for the fractional value
4. If a developer needs only lets say till 7 bits of the fraction we return only 7 bits.
5. So the answer is to combine fixed point and arbitrary precision arithmetic, into one single unified library.