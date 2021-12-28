# Day 24 part 2 solution.

All processing blocks have the same code:

```
inp w		<-- input the next number
mul x 0
add x z
mod x 26
div z C1	z /= C1 (1 or 26)
add x C2	number -15 .. 15
eql x w         true or false (common: false) -> MATCH
eql x 0         false or true (common: true)   <--- STATE
mul y 0
add y 25
mul y x
add y 1
mul z y         if !MATCH { z *= 26 }
mul y 0
add y w
add y C3
mul y x
add z y         if !MATCH { z += C3 * input }
```

This code transforms `z` depending on the input:

- when z mod(26) does not match the constant C2 + input, it always increases
  on every step, and will never reach zero, as we can see:

```
  z /= C1  (1 or 26)
  z *= 26
  z += C3 * input
```

- when z mod(26) matches the constant + input

```
  z /= C1  (1 or 26)
```

So it is important that the input makes the comparison `eql x w` `true`.

This part of the code checks the input:

```
mul x 0
add x z
mod x 26
add x C2	number -15 .. 15
eql x w         true or false
```

This means that in the step before this, we can predict which digit will
match, if we look at the constant `C2` in the next part of the code.

Note that there can be no valid input digits if `C2 <= -25 || C2 >= 10`.
Since the calculation does depend on previous input, it might happen
that there is a digit in input position X which we can predict, but
for which we can not predict a valid input based on the current value
of `z`. In that case we must rewind and try again with different digits
leading up to X.

In my solution, I've emulated the entire ALU, and just run it
step-by-step, using prediction and recursion to the to the valid
output (the lowest valid serial number).

