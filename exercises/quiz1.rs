// quiz1.rs
// This is a quiz for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 Rustbucks, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount. No hints this time!



// Put your function here!
fn calculate_apple_price(num:i32) -> i32{
    let mut price:i32 = 0;
    if num > 40 {
        price = num;
    } else {
        price = num*2;
    }
    return price;
}

// shorter and cleaner method by using the apples in the input field:
fn calculate_apple_price_answ(apples: i32) -> i32 {
    if apples <= 39 {
        apples*2
    } else {
        apples
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
}
