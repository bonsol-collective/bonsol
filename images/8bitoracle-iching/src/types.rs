#[derive(Default, Clone, Copy)]
pub struct HexagramGeneration {
    pub lines: [LineValue; 6],
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum LineValue {
    #[default]
    YoungYang = 7,    // Unchanging yang (5/16)
    OldYang = 9,      // Changing yang (3/16)
    YoungYin = 8,     // Unchanging yin (7/16)
    OldYin = 6,       // Changing yin (1/16)
}

/// Implementation of I-Ching line value operations.
/// These methods are fundamental to I-Ching divination and will be used
/// in future implementations of hexagram transformation and interpretation.
#[allow(dead_code)]
impl LineValue {
    /// Determines if a line is in a changing state (OldYang or OldYin).
    /// This is crucial for calculating the transformed hexagram.
    pub fn is_changing(&self) -> bool {
        matches!(self, LineValue::OldYang | LineValue::OldYin)
    }

    /// Determines if a line represents yang energy (YoungYang or OldYang).
    /// This is used for basic hexagram interpretation and line analysis.
    pub fn is_yang(&self) -> bool {
        matches!(self, LineValue::YoungYang | LineValue::OldYang)
    }
} 