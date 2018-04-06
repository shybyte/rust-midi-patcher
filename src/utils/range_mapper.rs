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
            return self.target.0;
        } else if value >= self.source.1 {
            return self.target.1;
        } else {
            let source_range = (self.source.1 - self.source.0) as i64;
            let target_range = (self.target.1 - self.target.0) as i64;
            (target_range * (value - self.source.0) as i64 / source_range + self.target.0 as i64) as u8
        }
    }
}


#[cfg(test)]
mod tests {
    use utils::range_mapper::{RangeToRangeMapper, ValueRange};

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
