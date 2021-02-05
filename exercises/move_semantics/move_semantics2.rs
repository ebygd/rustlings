// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)


// prøv hint 2 og 3 etterhvert

fn main() {
    // la til mut
    let mut vec0 = Vec::new();

    // let mut vec1 = fill_vec(vec0.clone()); // .clone() er 1 løsning
    // la til &mut
    let mut vec1 = fill_vec(&mut vec0); 

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}
    // la til &mut
fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.to_vec()
    // vec
}

// fn fill_vec_no_return(vec: &mut Vec<i32>) {
//     vec.push(22);
//     vec.push(44);
//     vec.push(66);
// }