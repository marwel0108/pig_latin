# Pig Latin in Rust
A simple program in rust that takes the user's input and prints out its pig latin form.



## How does it works?
The program separates the input and makes it a iterator so we can loop over it. Then we check the first character:

If that character is a vowel it adds "-hay" to the end of the word.

Otherwise the first character is moved to te end of the word and "ay" is added.

### For example:
```
Input: welcome ellie
Output:
    - elcome-way
    - ellie-ay

```