# Rust: Ownership

## Ownership Rule

- Each value has its own variable owner.
- They can only be one variable owner at the time.
- When the variable owner goes out of scope, the value will be automatically dropped.

## Scalar Types have the copy trait

Scarlar Types like

- Integers (i8, i16, i32, i64, i128, iarch, u8, u16, u32, u64, u126, usize)
- Floating Point (f32, f64)
- Boolean (bool)
- Character (char)

have the **Copy Trait** that can be used to assign from other variable to another variable with automatically clone assign. because its value stored at the **Stack Section** and it fixed.

```rs
fn main() {
  let a: <scalar_type> = <variable>;
  let b = a; // a just copy its value to b, not give the full ownership to b. Because the type has support "Copy Trait".
  println!("{}", a); // Not Error.
}
```

If the type has no **"Copy Trait"**, Recommended to use `.clone()` to clone the value to another variable like this instead of give the ownership of the value to another variable.
