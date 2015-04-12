extern crate judy;

use judy::JudyL;

#[test]
fn it_works() {
    let mut h = JudyL::<u32>::new();
    assert!(h.insert(123, &456));
    match h.get(123) {
        Some(x) => assert_eq!(456, *x),
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
