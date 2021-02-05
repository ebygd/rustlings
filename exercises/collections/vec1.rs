// vec1.rs
// Your task is to create a `Vec` which holds the exact same elements
// as in the array `a`.
// Make me compile and pass the test!
// Execute the command `rustlings hint collections1` if you need hints.

// HVORFOR FUNKER IKKE METODE 2 AAAAAAAAAAAH


fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    // TODO: declare your vector here with the macro for vectors
    // let v = vec![10, 20, 30, 40]; // lurer på om det er en måte å kopiere fra a, for å gjøre det lettere

    //////////////// prøver noen greier //////////////////////
// HVORFOR FUNKER DET IKKE AAAAAAAAH

// edit: fikk det til, måtte gjøre v til mut og dereference element
    let mut v: Vec<i32> = Vec::new();

    for element in a.iter() {
        v.push(*element);
    }

    //////////////////////////////////////////////////////////
    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert!(a.iter().zip(v.iter()).all(|(x, y)| x == y));
    }
}
