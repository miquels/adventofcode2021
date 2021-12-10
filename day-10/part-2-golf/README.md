# Code golf solution for day 10, part 2.

How does this work?

First, it helps to `rustfmt` the code :).

The ASCII value of opening braces have the last 2 bits set to the same value.
That can be '1' or '0', but they are the same. All closing braces happen to
have 2 different bits at the start. So XORing the bits will tell you if it
is an opening or closing brace.

So, to see if a character is an opening brace: `(x & 1) == (x & 2) / 2`.

If it is an opening brace we push the value onto a `Vec`.

If it is not, meaning it's a closing brace, we need to pop the last
value from the `Vec`, and check if the braces match.

For all brace-pairs the rule holds that bits 5, 6, and 7 are the same.
So, if we calculate opening-brace XOR closing-brace, bits 5, 6 and 7 should
be zero. That can be checked by ANDing it with 1110000 (112). We OR
that value into a flag, `z`, and at the end of the loop we can check if
`z==0` and if so, there was no syntax error.

Then if `z==0` we can walk over the remaining bytes in the `Vec`.
To calculate the score we need to map (,<,[,} to 1, 2, 3, 4.
With some bitfiddling this can be achieved: `((x + 96 * ((x & 4) / 4)) >> 5)`.

