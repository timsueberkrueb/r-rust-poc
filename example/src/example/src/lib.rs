use r_bindgen::r_bindgen;

#[r_bindgen]
pub fn rust_sort(mut v: Vec<i32>) -> Vec<i32> {
    v.sort();
    v
}

#[r_bindgen]
pub fn rust_mean(v: Vec<f64>) -> f64 {
    v.iter().sum::<f64>() / (v.len() as f64)
}
