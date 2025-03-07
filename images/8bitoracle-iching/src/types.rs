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

impl LineValue {
    pub fn is_changing(&self) -> bool {
        matches!(self, LineValue::OldYang | LineValue::OldYin)
    }

    pub fn is_yang(&self) -> bool {
        matches!(self, LineValue::YoungYang | LineValue::OldYang)
    }
} 