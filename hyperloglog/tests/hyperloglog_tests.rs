use hyperloglog::hyperloglog::HyperLogLog;

#[test]
fn test_new_hll() {
    let hll = HyperLogLog::new(4);
    assert_eq!(hll.get_cardinality(), 0.0);
}

#[test]
fn test_add_single_element() {
    let mut hll = HyperLogLog::new(4);
    hll.add_elem(&"test");
    let cardinality = hll.compute_cardinality();
    assert!(cardinality > 0.0);
}




#[test]
fn test_large_cardinality() {
    let mut hll = HyperLogLog::new(12);

    for i in 0..10000 {
        hll.add_elem(&i);
    }

    let cardinality = hll.compute_cardinality();
    let error = (cardinality - 10000.0).abs() / 10000.0;

    assert!(error < 0.05);
}


#[test]
fn test_compute_updates_cardinality() {
    let mut hll = HyperLogLog::new(8);

    for i in 0..100 {
        hll.add_elem(&i);
    }

    let computed = hll.compute_cardinality();
    let stored = hll.get_cardinality();

    assert_eq!(computed, stored);
}
