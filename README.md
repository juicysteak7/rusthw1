# rusthw1

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
