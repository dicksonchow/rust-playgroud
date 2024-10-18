use std::vec;


fn main() {
    println!("{}", (-4_i32).abs());
    println!("{}", i32::abs(-4));
    println!("Hello, world!");
}

#[test]
fn test_integers() {
    assert_eq!(10_i8 as u16, 10_u16); // in range
    assert_eq!(2525_u16 as i16, 2525_i16); // in range
    assert_eq!(-1_i16 as i32, -1_i32); // sign-extended
    assert_eq!(65535_u16 as i32, 65535_i32); // zero-extended

    // Conversions that are out of range for the destination
    // produce values that are equivalent to the original modulo 2^N,
    // where N is the width of the destination in bits. This
    // is sometimes called "truncation."
    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!(65535_u32 as i16, -1_i16);

    assert_eq!(-1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);

    assert_eq!(2_u16.pow(4), 16);
    assert_eq!((-4_i32).abs(), 4);
    assert_eq!(0b101101_u8.count_ones(), 4);
}

#[test]
fn test_tuples() {
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    let text1 = "I see the eigenvalue in thine eye";
    let temp = text1.split_at(21);
    let head1 = temp.0;
    let tail1 = temp.1;
    assert_eq!(head1, "I see the eigenvalue ");
    assert_eq!(tail1, "in thine eye");

}

#[test]
fn test_array() {
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    assert!(sieve[211]);
    assert!(!sieve[9876]);

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
}

#[test]
fn test_vector() {
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    assert_eq!(pal, ["step", "on", "no", "pets"]);

    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);

    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    palindrome.reverse();
    assert_eq!(palindrome, ["panama", "a canal", "a plan", "a man"]);

    let mut v1 = Vec::with_capacity(2);
    assert_eq!(v1.len(), 0);
    assert_eq!(v1.capacity(), 2);

    v1.push(1);
    v1.push(2);
    assert_eq!(v1.len(), 2);
    assert_eq!(v1.capacity(), 2);

    v1.push(3);
    assert_eq!(v1.len(), 3);
    println!("capacity is now {}", v1.capacity());

    let mut v2 = vec![10, 20, 30, 40, 50];

    v2.insert(3, 35);
    assert_eq!(v2, [10, 20, 30, 35, 40, 50]);

    v2.remove(1);
    assert_eq!(v2, [10, 30, 35, 40, 50]);

    let mut v3 = vec!["Snow Puff", "Glass Gem"];
    assert_eq!(v3.pop(), Some("Glass Gem"));
    assert_eq!(v3.pop(), Some("Snow Puff"));
    assert_eq!(v3.pop(), None);
}

#[test]
fn test_slice() {
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, 0.707, 1.0, 0.707];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    fn print_slice(n: &[f64]) {
        for elt in n {
            println!("{}", elt);
        }
    }

    print_slice(&v);
    print_slice(&a);
    println!("----------------------------------------");
    print_slice(&v[0..2]);
    print_slice(&a[2..]);
    print_slice(&sv[1..3]);
}