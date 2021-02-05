// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` for hints :)



fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}
// legge til mut her er nok:
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

 // en løsning, men tror de vil at jeg skal løse på en annen måte
// fn main() {
//     let mut vec0 = Vec::new();

//     let mut vec1 = fill_vec(&mut vec0);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec.to_vec()         // compiler anbefalte denne, aner ikke hva det egt gjør lol...
// }
