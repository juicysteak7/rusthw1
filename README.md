# rusthw1

Pedro Gonzalez

Portland State University

CS410/510

Professor Bart Massey

## Background

A commonly desired operation in various algorithms is to calculate an exponential modulo some (usually prime) number

It turns out that there is a fast way to compute modexp() without overflow. Here is some pseudocode from Wikipedia


    modexp(x, y, m):
        if m = 0 or (m - 1)2 would overflow
            fail
        if m = 1
            return 0
        z ← 1
        while y > 0
            if y mod 2 == 1
                z ← (z x) mod m
            y ← y div 2
            x ← x2 mod m
        return z

## Assignment

Implement a command-line calculator modexp as a Rust “binary crate”. Usage is the obvious: to compute modexp(2,20,17) invoke your calculator with

cargo run 2 20 17

Your program should print 16 on standard output in this case.

Your program should accept only non-negative x and y and positive m, with at least one of x and y positive: all inputs should be less than 2**64. If the program is run with one or more invalid inputs it should print a usage message on stderr and stop with exit status 1.

(Yes, there is a Rust crate with an implementation of this algorithm. You may use it for testing purposes [although I cannot recommend it, as it has some bugs], but do not look at its source code until you are done with yours.)

## Project Description

The professor suggested looking at our textbook, Rust Programming, for an implementation of gcd that might help with this assignment.

The implementation showed how to declare functions with arguments and return values, assert something ( this is mainly used for testing but can be used otherwise ), while loops, and if statements.

The book also covers how to handle command line arguments, print statements, vectors ( fancy arrays ), mutable variables, and exiting a program.

I used ChatGPT for suggestions on how to handle overflows which lead me to the checked arithmetic functions.

Using all this knowledge the implementation of this algorithm was fairly strait forward.

## Testing

The first thing I did to test the program was use the test cases the professor gave us.

After I had those tests I thought of basic acceptance cases, starting with the case given to use, the inputs 2 20 and 17 should give us 16, and if making sure if m is 1 the result is 0.

I thought of some different edge cases, if x or y is 0.

I thought of cases that should fail. This lead me to research how to expect something to fail in rust which I was delighted to know it is handled nicely.

The other cases I thought of were numbers that were too large, and if m was 0.

I tried to make a test that passes a negative value into the function, but because Rust is a strongly typed language it the compiler said "oh no you cannot do that". Passing a negative argument via the command line closes the program with an appropriate error message.