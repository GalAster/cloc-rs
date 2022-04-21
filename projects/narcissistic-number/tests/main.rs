use narcissistic::narcissistic_numbers;

#[test]
fn narcissistic_base10() {
    for (i, n) in narcissistic_numbers(16).enumerate().skip(200) {
        println!("#{}: {}", i + 1, n);
    }
}
