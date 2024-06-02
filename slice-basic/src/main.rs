fn main() {
    let numbers = [34,56,34,5]; //owner
    println!("numbers {:?} len {}", numbers, numbers.len());
    println!("number 0 {}", numbers[0]);
    println!("number 1 {}", numbers[1]);

    let slice_a = &numbers[0..3]; // index 0 sampai sebelum 3 //borrowing
    println!("slice_a {:?} len {}", slice_a, slice_a.len());
    println!("slice_a 0 {}", slice_a[0]);
    println!("slice_a 1 {}", slice_a[1]);

    let slice_b = &slice_a[1..=2]; // index 1 sampai 2
    println!("slice_b {:?} len {}", slice_b, slice_b.len());
    println!("slice_b 0 {}", slice_b[0]);
    println!("slice_b 1 {}", slice_b[1]);

    let data = ["a","b","c","d"];
    let sliced_data = &data[1..3];
    println!("sliced_data {:?}", sliced_data);
    let sliced_data1 = &data[1..=3];
    println!("sliced_data1 {:?}", sliced_data1);

    let end_index = &data[..3];
    println!("end_index {:?}", end_index);

    let end_index1 = &data[..=3];
    println!("end_index1 {:?}", end_index1);

    let start_index = &data[1..];
    println!("start_index {:?}", start_index);

    let start_index1 = &data[..];
    println!("start_index1 {:?}", start_index1);

    let mut current_number = [45,34,12,33,50];
    println!("============ before =============");
    println!("current_number {:?}", current_number);

    let borrowed_number = &mut current_number[..=4];
    borrowed_number[0] = 100;
    println!("============= after =============");
    println!("borrowed_number {:?}", borrowed_number);
    println!("current_number {:?}", current_number);

    /* for in slice */
    let scores = [7,6,10,9];
    for i in &scores[..] {
        println!("score {}", i);
    }

    /* for in mutable slice */
    let mut scores2 = [7,6,10,9];
    println!("before scores2 {:?}", scores2);

    let slice_mut_scores = &mut scores2[..];

    for i in &mut slice_mut_scores[..] {
        *i += 1;
    }

    println!("after scores2 {:?}", scores2);

}
