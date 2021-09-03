use peulerlib;

#[test]
fn it_returns_all_primes_less_than_10() {
    let expected = vec![2,3,5,7];
    
    let actual = peulerlib::primes_less_than_n(10);
    assert_eq!(expected, actual);
}

#[test]
fn it_returns_all_primes_less_than_20() {
    let expected = vec![2, 3, 5, 7, 11, 13, 17, 19];
    
    let actual = peulerlib::primes_less_than_n(20);
    assert_eq!(expected, actual);
}

#[test]
fn it_returns_all_primes_less_than_100() {
    let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
    
    let actual = peulerlib::primes_less_than_n(100);
    assert_eq!(expected, actual);
}

#[test]
fn it_returns_empty_vectory_when_given_1() {
    let expected : Vec<u64> = vec![];
    let actual = peulerlib::primes_less_than_n(1);
    assert_eq!(expected,actual);
}

#[test]
fn it_returns_empty_vectory_when_given_0() {
    let expected : Vec<u64> = vec![];
    let actual = peulerlib::primes_less_than_n(0);
    assert_eq!(expected,actual);
}