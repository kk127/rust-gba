# Gameboy Advance Emulator written in Rust

## Class Diagram
![class](images/class.drawio.svg)


## Appendix
### Shift
According to the [Rust documention](https://doc.rust-lang.org/reference/expressions/operator-expr.html#arithmetic-and-logical-binary-operators),
shift is defined as follows.
> Arithmetic right shift on signed integer types, logical right shift on unsigned integer types.

Example
```Rust
fn main() {
    let a: i8 = -64;
    println!("{:>4}, {:08b}", a, a);
    println!("{:>4}, {:08b}", a >> 4, a >> 4);
    println!("{:>4}, {:08b}", (a as u8) >> 4, (a as u8) >> 4);
    println!("{:>4}, {:08b}", a << 1, a << 1);
    println!("{:>4}, {:08b}", a << 2, a << 2);

    println!("--------------------");

    let b: u8 = 64;
    println!("{:>4}, {:08b}", b, b);
    println!("{:>4}, {:08b}", b >> 4, b >> 4);
    println!("{:>4}, {:08b}", b >> 7, b >> 7);
    println!("{:>4}, {:08b}", b << 1, b << 1);
    println!("{:>4}, {:08b}", b << 2, b << 2);
}
```
Output
```
 -64, 11000000
  -4, 11111100
  12, 00001100
-128, 10000000
   0, 00000000
--------------------
  64, 01000000
   4, 00000100
   0, 00000000
 128, 10000000
   0, 00000000
```
To apply logical right shift to signed integer,
implement as follows.
```Rust
fn main() {
    let a: i8 = -64;
    println!("{:>4}, {:08b}", a, a);
    println!("{:>4}, {:08b}", (a as u8) >> 4, (a as u8) >> 4);
    println!("{:>4}, {:08b}", a >> 4, a >> 4);
}
```
Output
```
 -64, 11000000
  12, 00001100
  -4, 11111100
```