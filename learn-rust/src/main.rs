use std::env;


fn main() {
    let mut numbers: Vec<u64> = Vec::new();

    // fetch the numbers from cmd line but skip the first one
    for arg in env::args().skip(1) {
        let arg_parsed = arg.parse::<u64>();
        match arg_parsed {
            Ok(n) => numbers.push(n),
            Err(err) => {
                eprintln!("Error parsing {} with error: {}", arg, err.to_string());
                std::process::exit(1);
            }
        }
    }
    
    assert!(numbers.len() >= 2);
    let num_1 = &numbers[0];
    let num_2 = &numbers[1];

    println!("GCD: {}", gcd(*num_1, *num_2));
}


fn gcd(mut n: u64, mut m: u64) -> u64 {
    // assertion is a macro
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    // return value of the fn need not be specified
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(25, 10), 4);
    assert_eq!(gcd(23, 1), 1);
}