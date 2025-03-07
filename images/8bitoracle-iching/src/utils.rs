use crate::types::LineValue;

/// Generate a line value using the exact probabilities:
/// - Young Yang (7): 5/16
/// - Old Yang (9):   3/16
/// - Young Yin (8):  7/16
/// - Old Yin (6):    1/16
pub fn generate_line_value(random_bytes: &[u8]) -> LineValue {
    // Use first byte for randomness (0-255)
    let value = random_bytes[0];
    
    // Map 0-255 to our 16 probability units
    // Each unit represents 1/16 probability (256/16 = 16 values per unit)
    match value / 16 {
        0 => LineValue::OldYin,                    // 1/16
        1..=3 => LineValue::OldYang,               // 3/16
        4..=10 => LineValue::YoungYin,             // 7/16
        _ => LineValue::YoungYang,                 // 5/16 (remaining)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_probabilities() {
        let mut counts = [0; 4];
        const ITERATIONS: usize = 160_000; // Large number for statistical significance
        
        for i in 0..ITERATIONS {
            let random_bytes = [(i % 256) as u8];
            let line = generate_line_value(&random_bytes);
            match line {
                LineValue::OldYin => counts[0] += 1,
                LineValue::OldYang => counts[1] += 1,
                LineValue::YoungYin => counts[2] += 1,
                LineValue::YoungYang => counts[3] += 1,
            }
        }
        
        // Check probabilities within 1% margin
        let tolerance = (ITERATIONS as f64 * 0.01) as usize;
        
        // Old Yin (6) - 1/16
        assert!((counts[0] - ITERATIONS/16).abs() < tolerance);
        
        // Old Yang (9) - 3/16
        assert!((counts[1] - 3*ITERATIONS/16).abs() < tolerance);
        
        // Young Yin (8) - 7/16
        assert!((counts[2] - 7*ITERATIONS/16).abs() < tolerance);
        
        // Young Yang (7) - 5/16
        assert!((counts[3] - 5*ITERATIONS/16).abs() < tolerance);
    }
} 