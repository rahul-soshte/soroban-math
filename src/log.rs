use crate::SoroNum;

pub trait Logarithm {
    fn log2(&self) -> Option<u32>;
    fn log10(&self) -> Option<u32>;
}

impl Logarithm for SoroNum<i128> {
    fn log2(&self) -> Option<u32> {
        if self.value <= 0 {
            None
        } else {
            Some(self.value.abs().leading_zeros() ^ 127)
        }
    }

    fn log10(&self) -> Option<u32> {
        if self.value <= 0 {
            None
        } else {
            let mut count = 0;
            let mut num = self.value;
            while num >= 10 {
                num /= 10;
                count += 1;
            }
            Some(count)
        }
    }
}

impl Logarithm for SoroNum<u128> {
    fn log2(&self) -> Option<u32> {
        Some(128 - self.value.leading_zeros() - 1)
    }

    fn log10(&self) -> Option<u32> {
        let mut count = 0;
        let mut num = self.value;
        while num >= 10 {
            num /= 10;
            count += 1;
        }
        Some(count)
    }
}
