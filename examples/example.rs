use foldr::FoldR;

fn main() {
    println!("Example 1: foldr sum");
    let numbers = vec![1, 2, 3, 4, 5];

    let sum = numbers.clone().into_iter().foldr(0, |x, acc| x + acc);
    println!("foldr sum: {}", sum); // 15

    println!("\nExample 2: foldr subtraction (to show right-fold order)");
    let diff = numbers.clone().into_iter().foldr(0, |x, acc| x - acc);
    println!("foldr subtraction: {}", diff);
    // 1 - (2 - (3 - (4 - (5 - 0)))) = 3
}
