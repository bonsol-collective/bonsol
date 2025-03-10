mod types;
mod utils;

use risc0_zkvm::{
    guest::{env, sha::Impl},
    sha::Sha256,
};

use types::{HexagramGeneration, LineValue};
use utils::generate_line_value;

// Constants for dev mode
const DEV_MODE_MARKER: u8 = 0xAA;

fn line_to_ascii(line: LineValue) -> String {
    match line {
        LineValue::OldYin => "---x---",    // yin changing into yang (7 chars)
        LineValue::YoungYin => "--- ---",   // yin, unchanging (7 chars)
        LineValue::OldYang => "---o---",    // yang changing into yin (7 chars)
        LineValue::YoungYang => "-------",  // yang, unchanging (7 chars)
    }.to_string()
}

fn hexagram_to_ascii(hexagram: &HexagramGeneration) -> String {
    let mut ascii_art = String::with_capacity(47); // 6 lines * 7 chars + 5 newlines
    
    // Build ASCII art representation from bottom to top (lines[0] is bottom)
    for (i, &line) in hexagram.lines.iter().enumerate() {
        env::log(&format!("Converting line {} ({:?}) to ASCII", i, line));
        let line_ascii = line_to_ascii(line);
        env::log(&format!("Line {} ASCII: '{}' (len={})", i, line_ascii, line_ascii.len()));
        
        // Add line to the beginning of the string (top lines first)
        if i > 0 {
            ascii_art.insert_str(0, "\n");
        }
        ascii_art.insert_str(0, &line_ascii);
    }
    
    env::log(&format!("Final ASCII art:\n{}", ascii_art));
    env::log(&format!("ASCII art length: {} bytes", ascii_art.len()));
    ascii_art
}

fn main() {
    env::log("Starting I Ching hexagram generation...");
    
    // Check if we're in dev mode
    let is_dev_mode = option_env!("RISC0_DEV_MODE").is_some();
    
    // Read the random seed
    let mut random_seed = [0u8; 32];
    env::read_slice(&mut random_seed);
    env::log(&format!("Received random seed ({}): {:02x?}", random_seed.len(), random_seed));
    
    // Generate hexagram
    let hexagram = if is_dev_mode {
        // In dev mode, generate a fixed hexagram for testing
        env::log("Dev mode: Generating fixed test hexagram");
        HexagramGeneration {
            lines: [
                LineValue::OldYin,     // Line 1 (bottom)
                LineValue::YoungYang,  // Line 2
                LineValue::OldYang,    // Line 3
                LineValue::YoungYin,   // Line 4
                LineValue::OldYin,     // Line 5
                LineValue::YoungYang,  // Line 6 (top)
            ]
        }
    } else {
        generate_hexagram(&random_seed)
    };
    
    env::log(&format!("Generated hexagram with lines: {:#?}", hexagram.lines));
    
    // Track total committed data size
    let mut total_committed = 0;
    
    // 1. Hash of random seed (or mock hash in dev mode)
    let seed_digest = if is_dev_mode {
        // Use consistent mock digest in dev mode
        let mock_digest = [0u8; 32];
        Impl::hash_bytes(&mock_digest)
    } else {
        Impl::hash_bytes(&random_seed)
    };
    
    let digest_bytes = seed_digest.as_bytes();
    env::log(&format!("Generated seed digest ({} bytes): {:02x?}", digest_bytes.len(), digest_bytes));
    env::commit_slice(digest_bytes);
    total_committed += digest_bytes.len();
    
    // 2. Commit hexagram values in a structured format
    let mut structured_output = vec![DEV_MODE_MARKER];
    structured_output.extend(hexagram.lines.iter().map(|&l| l as u8));
    env::log(&format!("Structured output ({} bytes): {:02x?}", structured_output.len(), structured_output));
    env::commit_slice(&structured_output);
    total_committed += structured_output.len();

    // 3. Generate and commit ASCII art representation
    let ascii_art = hexagram_to_ascii(&hexagram);
    env::log(&format!("ASCII art representation ({} bytes):\n{}", ascii_art.len(), ascii_art));
    let ascii_bytes = ascii_art.as_bytes();
    env::commit_slice(ascii_bytes);
    total_committed += ascii_bytes.len();
    
    env::log(&format!("Hexagram generation complete. Total committed data: {} bytes", total_committed));
    env::log("Journal data structure:");
    env::log(&format!("- Input digest: {} bytes", digest_bytes.len()));
    env::log(&format!("- Structured output: {} bytes", structured_output.len()));
    env::log(&format!("- ASCII art: {} bytes", ascii_bytes.len()));
}

fn generate_hexagram(random_seed: &[u8]) -> HexagramGeneration {
    let mut lines = [LineValue::default(); 6];
    
    env::log("Starting line generation...");
    for line_idx in 0..6 {
        // Generate each line using a different portion of the random seed
        let line_seed = &random_seed[line_idx*4..(line_idx+1)*4];
        env::log(&format!("Line {} seed ({} bytes): {:02x?}", line_idx + 1, line_seed.len(), line_seed));
        lines[line_idx] = generate_line_value(line_seed);
        env::log(&format!("Generated line {} = {:?} (value {})", 
            line_idx + 1, 
            lines[line_idx], 
            lines[line_idx] as u8
        ));
    }
    
    HexagramGeneration { lines }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexagram_generation() {
        let random_seed = [42u8; 32];
        let hexagram = generate_hexagram(&random_seed);
        
        // Verify all lines have valid values
        for line in &hexagram.lines {
            assert!(matches!(line, 
                LineValue::YoungYang | 
                LineValue::OldYang | 
                LineValue::YoungYin | 
                LineValue::OldYin
            ));
        }
    }

    #[test]
    fn test_ascii_art_generation() {
        let mut lines = [LineValue::default(); 6];
        lines[0] = LineValue::OldYin;     // Bottom line
        lines[1] = LineValue::YoungYin;
        lines[2] = LineValue::OldYang;
        lines[3] = LineValue::YoungYang;
        lines[4] = LineValue::OldYin;
        lines[5] = LineValue::YoungYang;   // Top line
        
        let hexagram = HexagramGeneration { lines };
        let ascii_art = hexagram_to_ascii(&hexagram);
        
        let expected = "--------\n---x---\n--------\n---o---\n---  ---\n---x---";
        assert_eq!(ascii_art, expected);
    }
} 