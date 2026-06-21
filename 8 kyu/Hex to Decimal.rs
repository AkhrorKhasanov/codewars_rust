use std::collections::HashMap;
fn hex_to_dec(hex_string: &str) -> u32 {
    let data = HashMap::from([
        ('0', 0),
        ('1', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('a', 10),
        ('b', 11),
        ('c', 12),
        ('d', 13),
        ('e', 14),
        ('f', 15),
    ]);
    hex_string
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let digit = data.get(&c).unwrap_or(&0); 
            (*digit as u32) * 16u32.pow(i as u32)  
        })
        .sum()
}

