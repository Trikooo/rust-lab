use hyperloglog::hyperloglog::utils::{Vec4, calculate_hash};

#[test]
fn test_calculate_hash_consistency() {
    let value1 = "test";
    let value2 = "test";
    assert_eq!(calculate_hash(&value1), calculate_hash(&value2));
}

#[test]
fn test_calculate_hash_different_values() {
    let value1 = "test1";
    let value2 = "test2";
    assert_ne!(calculate_hash(&value1), calculate_hash(&value2));
}

#[test]
fn test_calculate_hash_integers() {
    let num1 = 42;
    let num2 = 42;
    let num3 = 43;
    assert_eq!(calculate_hash(&num1), calculate_hash(&num2));
    assert_ne!(calculate_hash(&num1), calculate_hash(&num3));
}

#[test]
fn test_vec4_get_set() {
    let mut v = Vec4 { data: vec![0u8; 2] };
    v.set(0, 0xA);
    v.set(1, 0xB);
    v.set(2, 0xC);
    v.set(3, 0xD);
    assert_eq!(v.get(0), 0xA);
    assert_eq!(v.get(1), 0xB);
    assert_eq!(v.get(2), 0xC);
    assert_eq!(v.get(3), 0xD);
    assert_eq!(v.len(), 4);
}

#[test]
fn test_vec4_boundary_values() {
    let mut v = Vec4 { data: vec![0u8; 1] };
    v.set(0, 0xF);
    v.set(1, 0x0);
    assert_eq!(v.get(0), 0xF);
    assert_eq!(v.get(1), 0x0);
}

#[test]
fn test_vec4_overwrite() {
    let mut v = Vec4 { data: vec![0u8; 1] };
    v.set(0, 0x5);
    assert_eq!(v.get(0), 0x5);
    v.set(0, 0xA);
    assert_eq!(v.get(0), 0xA);
}
