# Hello World for embedded programming
Embedded programmings answer to the classic "Hello, World!" program. Since this is my first small Rust program i thought it would be appropriate to start with this old classic. This project is primarely for my own learning, so it is not by any means the perfect way to do it, but i figuered someone maybe could benefit from my experiences. I will go over the code line by line and link to the relevant data sheets. Feel free to message me improvements. Let's start!

## Project setup
TODO: Make project setup
Currently working on a cargo command that will set up the structure automaticly. Check my other rust project on my GitHub.

## The code

### Attributes
The first two lines are called _inner attributes_. An inner attribute is a piece of code that applies to the item it is declared in. For example a function, or in this case the whole main.rs file [link][1]. The other type of attributes are the _outer attributes_. These are written the same way as the inner ones, but without the exlemation mark (_!_). The difference is that the outer attributes only apply to the next piece of code, for example a struct or a function.

The _#![no_std]_ attribute tells the compiler that it should build our code without the standard rust library. This is because we are programming in a bare metal enviroment (no OS).

## Techical info and data sheets
Development board: STM32B-L47E

## References
[1]: https://doc.rust-lang.org/reference/attributes.html