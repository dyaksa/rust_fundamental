fn main() {
    let (num1, num2) = (10,15);

    let tambah = num1 + num2;
    println!("{} + {} = {}", num1, num2, tambah);

    let kurang = num2 - num1;
    println!("{1} - {0} = {2}", num1, num2, kurang);

    let kali = num1 * num2;
    println!("{} * {} = {}", num1, num2, kali);

    let bagi = num2 / num1;
    println!("{1} / {0} = {2}", num1, num2, bagi);

    let modulo = num2 % num1;
    println!("{1} % {0} = {2}", num1, num2, modulo);

    /* operator perbandingan */
    let a = 10;
    let b = 20;

    println!("a = {}", a);
    println!("b = {}", b);
    println!("a == b = {}", a == b);
    println!("a != b = {}", a != b);
    println!("a > b = {}", a > b);
    println!("a < b = {}", a < b);
    println!("a >= b = {}", a >= b);
    println!("a <= b = {}", a <= b);

    let named_argument = "berisi string";
    println!("{named_argument}");

    /* operator negasi */
    let (value_left, value_right) = (-12, 12);
    let res_one = value_left == -value_right;
    let res_two = !(value_left == value_right);

    println!("{} == -{} = {}", value_left, value_right, res_one);
    println!("!({} == {}) = {}", value_left, value_right, res_two);

    /* operatore logika */
    let (bool_left, bool_right) = (true, false);
    println!("{} && {} = {}", bool_left, bool_right, (bool_left && bool_right));
    println!("{} || {} = {}", bool_left, bool_right, (bool_left || bool_right));
    println!("!{} = {}", bool_left, !bool_left);
}
