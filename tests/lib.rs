extern crate judy;

use judy::Judy1;
use judy::JudyL;
use judy::JudyHS;

#[test]
fn test_judyhs() {
    let mut h = JudyHS::new();
    assert!(h.insert("123", 456));
    match h.get("123") {
        Some(x) => assert_eq!(456, x),
        None => panic!(),
    }
    assert!(h.free() > 0);
}

#[test]
fn test_judyl() {
    let mut h = JudyL::new();
    assert!(h.insert(123, 456));
    match h.get(123) {
        Some(x) => assert_eq!(456, x),
        None => panic!(),
    }

    let mut it = h.iter();
    assert_eq!(Some((123, 456)), it.next());
    assert_eq!(None, it.next());
    for (i, v) in h.iter() {
        println!("i: {:?} v: {:?}", i, v);
    }
    assert!(h.free() > 0);
}

#[test]
fn test_judy1() {
    let mut h = Judy1::new();
    assert_eq!(true, h.set(123));
    assert_eq!(false, h.set(123));

    assert_eq!(true, h.test(123));
    assert_eq!(false, h.test(456));

    assert_eq!(true, h.unset(123));
    assert_eq!(false , h.unset(123));

    assert_eq!(true, h.set(123));
    assert_eq!(true, h.set(456));

    let mut it = h.iter();
    assert_eq!(Some(123), it.next());
    assert_eq!(Some(456), it.next());
    assert_eq!(None, it.next());

    assert!(h.free() > 0);
    assert_eq!(0, h.free());
}
