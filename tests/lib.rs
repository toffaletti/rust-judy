extern crate judy;

use judy::Judy1;
use judy::JudyL;
use judy::JudySL;
use judy::JudyHS;

#[test]
fn test_judysl() {
    let mut h = JudySL::new();
    assert!(h.insert("a", 1));
    match h.get("a") {
        Some(x) => assert_eq!(1, x),
        None => panic!(),
    }
    assert_eq!(None, h.get("b"));
    assert!(h.insert("b", 1));
    {
        let mut it = h.iter();
        let (k, v) = it.next().unwrap();
        assert_eq!("a", std::str::from_utf8(k.to_bytes()).unwrap());
        assert_eq!(1, v);

        let (k, v) = it.next().unwrap();
        assert_eq!("b", std::str::from_utf8(k.to_bytes()).unwrap());
        assert_eq!(1, v);
    }
    assert!(h.remove("b"));
    assert!(h.free() > 0);
    assert!(h.is_empty());
}

#[test]
fn test_judyhs() {
    let mut h = JudyHS::new();
    assert!(h.insert("123", 456));
    match h.get("123") {
        Some(x) => assert_eq!(456, x),
        None => panic!(),
    }
    assert!(h.insert("456", 789));
    assert!(h.remove("456"));
    assert!(h.free() > 0);
    assert!(h.is_empty());
}

#[test]
fn test_judyl() {
    let mut h = JudyL::new();
    assert!(h.is_empty());
    assert!(h.insert(123, 456));
    match h.get(123) {
        Some(x) => assert_eq!(456, x),
        None => panic!(),
    }

    {
        let mut it = h.iter();
        assert_eq!(Some((123, 456)), it.next());
        assert_eq!(None, it.next());
    }
    assert!(h.insert(456, 1));
    assert_eq!(2, h.len());
    assert!(h.remove(&456));
    assert!(h.free() > 0);
}

#[test]
fn test_judy1() {
    let mut h = Judy1::new();
    assert!(h.is_empty());
    assert_eq!(true, h.set(123));
    assert_eq!(false, h.set(123));

    assert_eq!(true, h.test(123));
    assert_eq!(false, h.test(456));

    assert_eq!(true, h.unset(123));
    assert_eq!(false , h.unset(123));

    assert_eq!(true, h.set(123));
    assert_eq!(true, h.set(456));

    assert_eq!(2, h.len());
    {
        let mut it = h.iter();
        assert_eq!(Some(123), it.next());
        assert_eq!(Some(456), it.next());
        assert_eq!(None, it.next());
    }

    assert!(h.free() > 0);
    assert_eq!(0, h.free());
    assert!(h.is_empty());
}
