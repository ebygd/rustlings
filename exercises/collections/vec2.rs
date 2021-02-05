// vec2.rs
// A Vec of even numbers is given. Your task is to complete the loop
// so that each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute the command `rustlings hint collections2` if you need
// hints.

// orginal oppg
// https://github.com/rust-lang/rustlings/blob/main/exercises/collections/vec2.rs
// klarer ikke.....

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for i in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        
        // v[*i as usize] *= 2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());
        let ans2 = vec_loop(v.clone());

        // assert_eq!(
        //     ans,
        //     v.iter()
        //         .map(|x| x * 2)
        //         .collect::<Vec<i32>>()
        // );
        
        assert_eq!(ans, ans2);
    }
}
