use hyperloglog::hyperloglog::hyperloglog_presto::HyperLogLogPresto;

#[test]
fn test_new_hll_presto() {
    let hll = HyperLogLogPresto::new(4);
    assert_eq!(hll.get_cardinality(), 0.0);
}

#[test]
fn test_add_single_element() {
    let mut hll = HyperLogLogPresto::new(4);
    hll.add_elem(&"test");
    let cardinality = hll.compute_cardinality();
    assert!(cardinality > 0.0);
}

#[test]
fn test_large_cardinality() {
    let mut hll = HyperLogLogPresto::new(12);

    for i in 0..10000 {
        hll.add_elem(&i);
    }

    let cardinality = hll.compute_cardinality();
    let error = (cardinality - 10000.0).abs() / 10000.0;

    assert!(error < 0.05);
}

#[test]
fn test_overflow_buckets() {
    let mut hll = HyperLogLogPresto::new(8);

    let high_trailing_zero_value = 0xFFFFFFFFFFFF0000u64;
    hll.add_elem(&high_trailing_zero_value);

    let cardinality = hll.compute_cardinality();
    assert!(cardinality > 0.0);
}

#[test]
fn test_compute_updates_cardinality() {
    let mut hll = HyperLogLogPresto::new(8);

    for i in 0..100 {
        hll.add_elem(&i);
    }

    let computed = hll.compute_cardinality();
    let stored = hll.get_cardinality();

    assert_eq!(computed, stored);
}

#[test]
fn test_dense_vs_overflow_storage() {
    let mut hll = HyperLogLogPresto::new(8);

    for i in 0..1000 {
        hll.add_elem(&i);
    }

    hll.compute_cardinality();
    assert!(hll.get_cardinality() > 0.0);
}
