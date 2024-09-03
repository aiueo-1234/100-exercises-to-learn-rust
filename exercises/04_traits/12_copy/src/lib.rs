// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::ops::Add for WrappingU32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // let res = self.value.overflowing_add(rhs.value);
        // if res.1 == true {
        //     self
        // } else {
        //     WrappingU32::new(res.0)
        // }
        WrappingU32::new((self.value as i32 + rhs.value as i32) as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
