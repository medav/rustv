
#[macro_export]
macro_rules! bit_range_get {
    ($v:expr, $r:tt) => {
        ($v >> $r.0) & ((1 << ($r.1 - $r.0 + 1)) - 1)
    };
    ($v:literal, $r:tt) => {
        ($v >> $r.0) & ((1 << ($r.1 - $r.0 + 1)) - 1)
    };
}

#[macro_export]
macro_rules! bit_range_set {
    ($v:expr, $r:tt) => {
        ($v & ((1 << ($r.1 - $r.0 + 1)) - 1)) << $r.0
    };
    ($v:literal, $r:tt) => {
        ($v & ((1 << ($r.1 - $r.0 + 1)) - 1)) << $r.0
    };
}

#[macro_export]
macro_rules! bit_range_map {
    ($v:expr, $f:tt, $t:tt) => {
        bit_range_set!(bit_range_get!($v, $f), $t)
    };
    ($v:literal, $f:tt, $t:tt) => {
        bit_range_set!(bit_range_get!($v, $f), $t)
    };
}

#[test]
fn test1() {
    assert_eq!(bit_range_get!(0xF0, (4, 7)), 0xF);
}

#[test]
fn test2() {
    assert_eq!(bit_range_get!(0xF000000000000000 as u64, (60, 63)), 0xF);
}

#[test]
fn test3() {
    assert_eq!(bit_range_get!(0xF000000000000000 as u64, (59, 63)), 0x1E);
}

#[macro_export]
macro_rules! sign_ext64 {
    ($f:literal, $v:expr) => {
        if $v & (1 << ($f - 1)) != 0 {
            (!((1 << $f) - 1)) | $v
        }
        else {
            $v as u64
        }
    };
    ($f:expr, $v:expr) => {
        if $v & (1 << ($f - 1)) != 0 {
            (!((1 << $f) - 1)) | $v
        }
        else {
            $v as u64
        }
    };
}

#[test]
fn test_sign_ext64_1() {
    assert_eq!(0xFFFFFFFFFFFFFFFE, sign_ext64!(2, 2));  
}


#[test]
fn test_sign_ext64_2() {
    assert_eq!(0xFFFFFFFFFFFF0000, sign_ext64!(17, 0x10000));  
}
