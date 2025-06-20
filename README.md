[![progress-banner](https://backend.codecrafters.io/progress/grep/6da2927f-2c03-4f47-99c0-3e1a7068e34a)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

This is a starting point for Rust solutions to the
["Build Your Own grep" Challenge](https://app.codecrafters.io/courses/grep/overview).

[Regular expressions](https://en.wikipedia.org/wiki/Regular_expression)
(Regexes, for short) are patterns used to match character combinations in
strings. [`grep`](https://en.wikipedia.org/wiki/Grep) is a CLI tool for
searching using Regexes.

<h1 align="center">Grep from scratch in Rust</h1>

<div align="center">
    <img src="/image.png" alt="Project completion image">
</div>


### Stages:
1. Match a literal character 
2. Match digits - `\d`
3. Match alphanumeric characters - `\w`
4. Positive character groups - Ex: `[abc]`
5. Negative character groups - Ex: `[^abc]`
6. Combining all character classes above in a pattern
7. Start of string anchor - `^`
8. End of string anchor - `$`
9. Match one or more items - `+`
   1.  In first iteration implemented `Greedy approach`
       1.  where for example Input: caaat and Pattern: ca+at, `+` will consume all three `a`s and next `a` will not have matching character in input and will return `false`
   2.  In second iteration added `Backtracking` condition
       1.  `+` takes all three `a`s and next `a` no match returns `false`
       2.  `+` takes two `a`s and next `a` has match next `t` has match so returns `true`