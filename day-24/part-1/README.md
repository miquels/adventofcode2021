# Day 24 ALU Solution.

The idea was to build an ALU that can work with ranges as well as discrete numbers.
Inputs that are not set have the value "1..9".

This idea worked at first for calculating the highest model number.

However, the algorithm did not implement all the arithmetic correctly.

The problem is range multiplications.

A naive range multiplication would look like:

```
[2 .. 4] x [7 ..9] = [14 .. 36].
```

This is not correct. The right solution is:

```
[2 .. 4] x [7 ..9] = [2*7, 2*8, 2*9, 3*7, 3*8, 3*9, 4*7, 4*8, 4*9]

                     [14, 16, 18, 21, 24, 27, 28, 32, 36]
```

Ofcourse there are short solutions for `[0..0], [0..1] and [1..1]`.

But this gets out of hand quickly, if you have ranges like `12000-23000`.

I've tried to add a few shortcuts to sort of handle this, but it
makes the calculations too imprecise to get the correct answer.

Perhaps I'll get back to it one day, it is interesting...it works
for some cases at least!

