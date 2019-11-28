use r_bindgen::r_bindgen;
use r_interop::{FromSEXP, IntoSEXP, SEXP};

#[no_mangle]
pub extern "C" fn hello_wrapper(name: SEXP) -> SEXP {
    let n: String = String::from_sexp(name);
    let s = format!("Hello there, {}\n", n);
    s.into_sexp()
}

#[no_mangle]
pub extern "C" fn meaning(val: SEXP) -> SEXP {
    let n = i32::from_sexp(val);
    assert_eq!(n, 42);
    42.into_sexp()
}

#[no_mangle]
pub extern "C" fn circ(val: SEXP) -> SEXP {
    let n = f64::from_sexp(val);
    (2.0 * 3.14 * n.powf(2.0)).into_sexp()
}

#[no_mangle]
pub extern "C" fn is_awesome(val: SEXP) -> SEXP {
    let b = bool::from_sexp(val);
    assert!(!b);
    true.into_sexp()
}

#[no_mangle]
pub extern "C" fn create_vec() -> SEXP {
    (0..100).collect::<Vec<i32>>().into_sexp()
}

#[no_mangle]
pub extern "C" fn create_vec_nested() -> SEXP {
    vec![vec![1, 2, 3], vec![4, 5, 6]].into_sexp()
}

#[no_mangle]
pub extern "C" fn print_vec_nested(v: SEXP) -> SEXP {
    let x = Vec::<Vec<i32>>::from_sexp(v);
    println!("{:?}", x);
    true.into_sexp()
}

#[no_mangle]
pub extern "C" fn nihilism() -> SEXP {
    ().into_sexp()
}

#[r_bindgen]
pub fn hello_bindgen() {
    println!("Hello, R!");
}

#[r_bindgen]
pub fn its_a_vec(v: Vec<i32>) -> Vec<i32> {
    v.iter().map(|i| i + 1).collect()
}

#[r_bindgen]
pub fn rust_sort(mut v: Vec<i32>) -> Vec<i32> {
    v.sort();
    v
}
