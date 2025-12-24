mod hyperloglog;
use hyperloglog::{HyperLogLog, hyperloglog_presto::HyperLogLogPresto};

fn main() {
    println!("=== HyperLogLog Cardinality Estimation ===\n");

    test_regular_hll();
    println!();
    test_presto_hll();
}

fn test_regular_hll() {
    println!("--- Regular HyperLogLog ---");
    let mut hll = HyperLogLog::new(10);

    let cardinality = hll.get_cardinality();
    println!("Initial cardinality: {}", cardinality);

    println!("Inserting 1,000,000 elements...");
    for i in 0..1_000_000 {
        hll.add_elem(&i);
        if i % 200_000 == 0 && i != 0 {
            println!("  - Inserted {} elements", i);
        }
    }

    let cardinality = hll.compute_cardinality();
    println!("Final cardinality: {:.2}", cardinality);
    println!("Error: {:.2}%", ((cardinality - 1_000_000.0).abs() / 1_000_000.0) * 100.0);
}

fn test_presto_hll() {
    println!("--- Presto HyperLogLog ---");
    let mut hll = HyperLogLogPresto::new(10);

    let cardinality = hll.get_cardinality();
    println!("Initial cardinality: {}", cardinality);

    println!("Inserting 1,000,000 elements...");
    for i in 0..1_000_000 {
        hll.add_elem(&i);
        if i % 200_000 == 0 && i != 0 {
            println!("  - Inserted {} elements", i);
        }
    }

    let cardinality = hll.compute_cardinality();
    println!("Final cardinality: {:.2}", cardinality);
    println!("Error: {:.2}%", ((cardinality - 1_000_000.0).abs() / 1_000_000.0) * 100.0);
}
