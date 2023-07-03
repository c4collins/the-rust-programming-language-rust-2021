fn main() {
    // *********** Scalar types ******************* (single values)

    // Integers - a number without a fractional component
    let signed_8bit_min: i8 = std::i8::MIN;
    let signed_8bit_max: i8 = std::i8::MAX;
    let unsigned_8bit_min: u8 = std::u8::MIN;
    let unsigned_8bit_max: u8 = std::u8::MAX;
    println!("-> 8-bit");
    println!("Signed: {signed_8bit_min} <-> {signed_8bit_max}");
    println!("Unsigned: {unsigned_8bit_min} <-> {unsigned_8bit_max}");

    let signed_16bit_min: i16 = std::i16::MIN;
    let signed_16bit_max: i16 = std::i16::MAX;
    let unsigned_16bit_min: u16 = std::u16::MIN;
    let unsigned_16bit_max: u16 = std::u16::MAX;
    println!("-> 16-bit");
    println!("Signed: {signed_16bit_min} <-> {signed_16bit_max}");
    println!("Unsigned: {unsigned_16bit_min} <-> {unsigned_16bit_max}");

    let signed_32bit_min: i32 = std::i32::MIN;
    let signed_32bit_max: i32 = std::i32::MAX;
    let unsigned_32bit_min: u32 = std::u32::MIN;
    let unsigned_32bit_max: u32 = std::u32::MAX;
    println!("-> 32-bit");
    println!("Signed: {signed_32bit_min} <-> {signed_32bit_max}");
    println!("Unsigned: {unsigned_32bit_min} <-> {unsigned_32bit_max}");

    let signed_64bit_min: i64 = std::i64::MIN;
    let signed_64bit_max: i64 = std::i64::MAX;
    let unsigned_64bit_min: u64 = std::u64::MIN;
    let unsigned_64bit_max: u64 = std::u64::MAX;
    println!("-> 64-bit");
    println!("Signed: {signed_64bit_min} <-> {signed_64bit_max}");
    println!("Unsigned: {unsigned_64bit_min} <-> {unsigned_64bit_max}");

    let signed_128bit_min: i128 = std::i128::MIN;
    let signed_128bit_max: i128 = std::i128::MAX;
    let unsigned_128bit_min: u128 = std::u128::MIN;
    let unsigned_128bit_max: u128 = std::u128::MAX;
    println!("-> 128-bit");
    println!("Signed: {signed_128bit_min} <-> {signed_128bit_max}");
    println!("Unsigned: {unsigned_128bit_min} <-> {unsigned_128bit_max}");

    let signed_sizebit_min: isize = std::isize::MIN;
    let signed_sizebit_max: isize = std::isize::MAX;
    let unsigned_sizebit_min: usize = std::usize::MIN;
    let unsigned_sizebit_max: usize = std::usize::MAX;
    println!("-> system archtecture (probably x64)");
    println!("Signed: {signed_sizebit_min} <-> {signed_sizebit_max}");
    println!("Unsigned: {unsigned_sizebit_min} <-> {unsigned_sizebit_max}");

    // Integer Literals - types are inferred from the definition

    println!("-> Literals");

    let decimal = 98_222;
    println!("Decimal: {decimal} from 98_222");

    let hex = 0xff;
    println!("Hex: {hex} from 0xff");

    let octal = 0o77;
    println!("Octal: {octal} from 0o77");

    let binary = 0b1111_0000;
    println!("Binary: {binary} from 0b1111_0000");

    let byte = b'A';
    println!("Byte: {byte} from b'A'");

    // Floating point numbers

    let f32_min: f32 = std::f32::MIN;
    let f32_max: f32 = std::f32::MAX;
    let f64_min = std::f64::MIN; // f64 by default
    let f64_max = std::f64::MAX;
    println!("-> Floating points");
    println!("f32: {f32_min} <-> {f32_max}");
    println!("f64: {f64_min} <-> {f64_max}");

    // **************** Compound types *******************
}
