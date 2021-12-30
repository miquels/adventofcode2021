# Day 19, Beacon Scanner.

So, I did see at once that we needed to rotate all coordinates through
all 24 ways you can rotate a cube - and I knew that you could do this
by using 3D matrix multiplicaton.

So, [my first attempt at solving this](https://github.com/miquels/adventofcode2021/tree/585d9d2f652b8e37a6ba7581b559fa6fe7d82b78/day-19)
was by looking up all possible
rotations as a as set matrices which I remembered from maths at highschool. Well not
entirely, so I used [this site](https://www.euclideanspace.com/maths/algebra/matrix/transforms/examples/) to find the transformations.

That worked out beautifully. However, some of my friends were still at it,
trying to solve it from first principles. Honest enough. So, I took
a shoebox, wrote down all rotations, turned that into [an algorithm
to build the matrices](https://github.com/miquels/adventofcode2021/tree/7f5511ae810b8a9601f3e8b99859520702f6c459/day-19).

Then eventually I realized that the whole matrix multiplication stuff
was actually not needed - just a list (rust: `match` with 24 cases) is
enough. So the [final solution was a bit shorter still](https://github.com/miquels/adventofcode2021/tree/22dbc254648e176871d1d6b31c152d78d5f8085f/day-19).

I benchmarked all 3 solutions - completely the same. Alas :|

