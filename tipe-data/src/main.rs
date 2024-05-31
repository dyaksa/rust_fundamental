fn main() {
    // tipe data primitive integer
    let numerik1: i8 = 24;
    let numerik2: i16 = 25;
    let numerik3: i32 = 26;
    let numerik4: i64 = 27;

    println!("{} | {} | {} | {}", numerik1, numerik2, numerik3, numerik4);

    let min_i8 = i8::MIN;
    let max_i8 = i8::MAX;

    println!("{} | {}", min_i8, max_i8);

    // tipe data primitive unsigned integer
    let numerik5: u8 = 24;
    let numerik6: u16 = 28;
    let numerik7: u32 = 29;

    println!("{} | {} | {}", numerik5, numerik6, numerik7);

    let min_u8 = u8::MIN;
    let max_u8 = u8::MAX;
    println!("min = {} | max = {}", min_u8, max_u8);

    // tipe data primitive float
    let float1: f32 = 3.14;
    let float2: f64 = 3.1415663736736;
    println!("{} | {:.5}", float1, float2);

    let min_f32 = f32::MIN;
    let max_f32 = f32::MAX;
    println!("min = {} | max = {}", min_f32, max_f32);

    // tipe data boolean
    let bool1: bool = true;
    let bool2: bool = false;
    println!("{} | {}", bool1, bool2);

    // tipe data char
    let c1: char = 'a';
    let c2: char = '2';
    let c3: char = '😁';
    println!("{} | {} | {}", c1, c2, c3);

    // pointer scalar
    let pointer1: &i32;
    pointer1 = &23;
    println!("{}", pointer1);
}
