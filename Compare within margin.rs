fn close_compare<M: Into<Option<f64>>>(a: f64, b: f64, margin: M) -> i8 {
   let m = margin.into().unwrap_or(0.0);
    
    let abs_value = (a - b).abs();
    
    if m >= abs_value {
        0
    } else if a < b {
        -1
    } else {
        1
    }
} 