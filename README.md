## Type safety

Javascript

![Screenshot 2024-04-22 at 10.37.36 PM.png](https://prod-files-secure.s3.us-west-2.amazonaws.com/085e8ad8-528e-47d7-8922-a23dc4016453/7a6337d0-0a42-4e32-aece-2a1a19199edd/Screenshot_2024-04-22_at_10.37.36_PM.png)

Rust

![Screenshot 2024-04-22 at 10.38.06 PM.png](https://prod-files-secure.s3.us-west-2.amazonaws.com/085e8ad8-528e-47d7-8922-a23dc4016453/935d18d9-3068-4158-a665-eab8fb8eb8bd/Screenshot_2024-04-22_at_10.38.06_PM.png)

<aside>
💡 Typescript was introduced to get rid of some of these problems in JS

</aside>

## Systems language

It is intended to be used (but not restricted to) to do lower level things

1. Building a Compiler
2. Building a browser
3. Working closer to the OS/kernel

## Generally faster

Rust has a separate compilation step (similar to C++) that spits out an optimised binary and does a lot of static analysis at compile time. 

JS does JIT compilation. 

![Screenshot 2024-04-23 at 1.10.01 AM.png](https://prod-files-secure.s3.us-west-2.amazonaws.com/085e8ad8-528e-47d7-8922-a23dc4016453/1ccd0bb2-5365-4bf6-b5b9-bb25fdf0b06f/Screenshot_2024-04-23_at_1.10.01_AM.png)

![Screenshot 2024-04-23 at 1.10.52 AM.png](https://prod-files-secure.s3.us-west-2.amazonaws.com/085e8ad8-528e-47d7-8922-a23dc4016453/13738166-aa9d-43fc-abb1-13367b49b68d/Screenshot_2024-04-23_at_1.10.52_AM.png)

## Concurrency

**Rust** has built-in support for concurrent programming allowing multiple threads to perform tasks simultaneously without risking data races

Javascript is single threaded generally (there are some projects that tried making it multi threaded but rarely used)

# Memory safe

Rust has a concept of owners, borrowing and lifetimes that make it extremely memory safe

<aside>
💡 Rust doesn't hide complexity from developers it offers them the right tools to manage all the complexity.

</aside>
