use narcissistic::narcissistic_number;

#[test]
fn test() {
    for i in narcissistic_number(3) {
        println!("{}", i);
    }
}
