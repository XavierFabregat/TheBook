fn main () {
  // There are two types of data types in Rust: scalar and compound
  // Scalar types represent a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
  // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

  // Integer types
  // Length  Signed  Unsigned Range
  // 8-bit   i8      u8       -128 to 127 or 0 to 255
  // 16-bit  i16     u16      -32,768 to 32,767 or 0 to 65,535
  // 32-bit  i32     u32      -2,147,483,648 to 2,147,483,647 or 0 to 4,294,967,295
  // 64-bit  i64     u64      -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807 or 0 to 18,446,744,073,709,551,615
  // 128-bit i128    u128     Way too big to write out. (not really, but it's a lot)
  // arch    isize   usize    -2,147,483,648 to 2,147,483,647 or 0 to 4,294,967,295 (depends on the kind of computer your program is running on)
  // The isize and usize types depend on the kind of computer your program is running on: 
  // 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
  // The calculation of the range is: 2^(n-1) to 2^(n-1) - 1 for signed and 0 to 2^n - 1 for unsigned
  let x: i8 = 127;
  let x: u8 = 255;
  let x: i16 = 32767;
  let x: u16 = 65535;
  let x: i32 = 2147483647;
  let x: u32 = 4294967295;
  let x: i64 = 9223372036854775807;
  let x: u64 = 18446744073709551615;
  let x: i128 = 170141183460469231731687303715884105727;
  let x: u128 = 340282366920938463463374607431768211455;


  // Number literals
  // Number literals can be written in various bases: 98_222, 0xff, 0o77, 0b1111_0000, b'A'
  // Literals   Example
  // Decimal    98_222
  // Hex        0xff
  // Octal      0o77
  // Binary     0b1111_0000
  // Byte (u8 only) b'A'


  // Boolean type
  let x = true;
  let x: bool = false;


  // Character type
  let x = 'z';
  let x: char = 'ℤ';
  let x: char = '😻';


  // Compound types
  // Tuples
  // A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
  // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  // We can use pattern matching to destructure a tuple value
  let (x, y, z) = tup;
  println!("The value of y is: {y}");

  // We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access
  let x: (i32, f64, u8) = (500, 6.4, 1);
  let five_hundred = x.0;
  let six_point_four = x.1;
  let one = x.2;


  // Arrays
  // An array is a collection of multiple values of the same type.
  // Arrays in Rust have a fixed length, like tuples.

  let arr = [1, 2, 3, 4, 5];

  // we can type it like this
  let arr: [i32; 5] = [1, 2, 3, 4, 5];

}