pub type ValueRange = (u8, u8);

pub struct RangeToRangeMapper {
    source: ValueRange,
    target: ValueRange,
}

impl RangeToRangeMapper {
    pub fn new(source: ValueRange, target: ValueRange) -> Self {
        RangeToRangeMapper { source, target }
    }

    pub fn map(&self, value: u8) -> u8 {
        if value <= self.source.0 {
            self.target.0
        } else if value >= self.source.1 {
            self.target.1
        } else {
            let source_range = i64::from(self.source.1 - self.source.0);
            let target_range = i64::from(self.target.1 - self.target.0);
            (target_range * i64::from(value - self.source.0)  / source_range + i64::from(self.target.0)) as u8
        }
    }
}

/// Notes below 10 are often used for special purposes.
pub static ALL_REAL_NOTES: ValueRange = (10, 127);


#[cfg(test)]
mod tests {
    use crate::utils::range_mapper::{RangeToRangeMapper};

    #[test]
    fn test() {
        let mapper = RangeToRangeMapper::new((10, 20), (40, 80));

        assert_eq!(mapper.map(9), 40);
        assert_eq!(mapper.map(10), 40);

        assert_eq!(mapper.map(20), 80);
        assert_eq!(mapper.map(21), 80);

        assert_eq!(mapper.map(15), 60);
        assert_eq!(mapper.map(11), 44);
    }

    #[test]
    fn test_large() {
        let mapper = RangeToRangeMapper::new((10, 240), (20, 250));
        assert_eq!(mapper.map(100), 110);
    }
}
