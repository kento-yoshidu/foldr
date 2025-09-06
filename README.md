# foldr

## 🚀 Toy crate for fun and learning purposes!

This is a small experimental Rust crate that provides a foldr method (right fold) for all iterators.

Right fold is like the standard fold, but it applies the function from the rightmost element to the leftmost element. This can be useful when you want to accumulate values in a right-associative manner.

---

## ✨ Example

```rust
use foldr::FoldR;

let v = vec![1, 2, 3];

// Sum using foldr
let sum = v.into_iter().foldr(0, |x, acc| x + acc);
assert_eq!(sum, 6);

// Right-associative subtraction
let sub = v.into_iter().foldr(0, |x, acc| x - acc);
assert_eq!(sub, 2); // 1 - (2 - (3 - 0))
```

💡 In contrast to fold (left fold), foldr starts from the rightmost element. This can change the result when the operation is not associative, like subtraction or string concatenation.

## 📌 Note

This crate is a toy project created for fun, learning, and experimentation. It is not intended for production use, but feel free to use it, fork it, or extend it as you like!

## 🔧 Why this exists?

Because sometimes, the standard left fold (fold) is too restrictive. You might want to fold elements from the right, especially when dealing with operations that are not commutative or associative. foldr gives you that flexibility in a simple, easy-to-use way.

💡 Inspired by Haskell's `foldr`, this crate brings the familiar right fold behavior to Rust iterators.
