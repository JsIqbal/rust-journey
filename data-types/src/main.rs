fn main() {
    // ----------- Scalar Types -----------

    // Integer types
    let a: i8 = -128;            // 8-bit signed integer
    let b: u8 = 255;             // 8-bit unsigned integer
    let c: i16 = -32_000;        // 16-bit signed
    let d: u16 = 65_000;         // 16-bit unsigned
    let e: i32 = -2_000_000;     // 32-bit signed
    let f: u32 = 4_000_000;      // 32-bit unsigned
    let g: i64 = -9_000_000_000; // 64-bit signed
    let h: u64 = 18_000_000_000; // 64-bit unsigned
    let i: isize = -10;          // signed, size depends on architecture
    let j: usize = 10;           // unsigned, size depends on architecture

    // Floating point types
    let k: f32 = 3.14;           // 32-bit float
    let l: f64 = 2.7182818284;   // 64-bit float (default)

    // Boolean
    let m: bool = true;

    // Character (Unicode scalar value)
    let n: char = '❤';

    // ----------- Compound Types -----------

    // Tuple - fixed-size, multiple types
    let o: (i32, f64, char) = (42, 6.28, 'π');

    // Array - fixed-size, same type
    let p: [i32; 5] = [1, 2, 3, 4, 5];

    // Slice - dynamically sized reference to part of an array
    let q: &[i32] = &p[1..4];

    // ----------- Printing All -----------

    println!("i8: {:?}", a);
    println!("u8: {:?}", b);
    println!("i16: {:?}", c);
    println!("u16: {:?}", d);
    println!("i32: {:?}", e);
    println!("u32: {:?}", f);
    println!("i64: {:?}", g);
    println!("u64: {:?}", h);
    println!("isize: {:?}", i);
    println!("usize: {:?}", j);

    println!("f32: {:?}", k);
    println!("f64: {:?}", l);

    println!("bool: {:?}", m);
    println!("char: {:?}", n);

    println!("tuple: {:?}", o);
    println!("array: {:?}", p);
    println!("slice: {:?}", q);

    shadowing();
    let x = add_one(5);
    println!("The return value of x is: {x}");
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn add_one(x: i32) -> i32 {
    x + 10
}