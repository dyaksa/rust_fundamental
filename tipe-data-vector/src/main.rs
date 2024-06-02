use std::{collections::VecDeque, vec};

fn main() {
    let mut data_one = vec!["Batman","Superman", "Spiderman"];
    println!("data vector {:?}", data_one);
    println!("data len = {} & capacity = {}", data_one.len(), data_one.capacity());

    data_one.pop();
    println!("data vector {:?}", data_one);
    println!("data len = {} & capacity = {}", data_one.len(), data_one.capacity());

    data_one.remove(1);
    println!("data vector {:?}", data_one);
    println!("data len = {} & capacity = {}", data_one.len(), data_one.capacity());

    data_one.push("Black Panther");
    data_one.push("Thanos");
    data_one.push("Captain America");
    println!("data vector {:?}", data_one);
    println!("data len = {} & capacity = {}", data_one.len(), data_one.capacity());

    data_one[2] = "Red Hood";
    println!("data vector {:?}", data_one);
    println!("data len = {} & capacity = {}", data_one.len(), data_one.capacity());

    let is_empty = data_one.is_empty();
    println!("is_empty = {}", is_empty);

    data_one.clear();
    println!("data vector {:?}", data_one);
    println!("data len = {} & capacity = {}", data_one.len(), data_one.capacity());

    let mut result_a = vec![3,2,1];
    let mut result_b = vec![4,6,5];

    result_a.append(&mut result_b);
    println!("result_a vector {:?}", result_a);
    println!("result_a len = {} & capacity = {}", result_a.len(), result_a.capacity());

    result_a.append(&mut  vec![7,9,8]);
    println!("result_b vector {:?}", result_a);
    println!("result_b len = {} & capacity = {}", result_a.len(), result_a.capacity());

    println!("result_a before sort = {:?}", result_a);
    result_a.sort();
    println!("result_a after sort = {:?}", result_a);

    /* pendefinisian vector kosong */
    // let mut vec_a = vec![1,5,6];
    // let mut vec_b: Vec<i32> = vec![2,3,4];

    // let mut vec_c: Vec<&str> = vec![];
    // let mut vec_c: Vec<&str> = Vec::new();

    /* for in data vector */
    let vec_a = vec![1,2,3,4,5];
    for i in vec_a {
        println!("i = {:?}", i);
    }

    let vec_b = vec!["a","b","c","d","e"];
    for i in 0..vec_b.len() {
        println!("i = {:?}", vec_b[i]);
    }

    let vec_c = vec![1,2,3,4,5];
    for i in vec_c.iter() {
        println!("i = {:?}", i);
    }

    for i in 0..vec_c.len() {
        println!("i = {:?}", i);
    }

    let vec_population = vec![100,200,300,400,500];
    let total_population = &vec_population[0..1];
    println!("total_population = {:?}", total_population);

    /* vecdeque */
    let mut vec_deque = VecDeque::from(vec!["a", "b", "c"]);

    vec_deque.pop_front();
    vec_deque.push_front("Z");
    println!("vec_deque = {:?}", vec_deque);

    vec_deque.pop_back();
    vec_deque.push_back("X");
    println!("vec_deque = {:?}", vec_deque);
}
