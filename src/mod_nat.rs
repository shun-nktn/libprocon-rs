#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mod<const N: usize> {
    pub value: usize
}

impl<const N: usize> Mod<N> {
    const _N_NOT_ZERO_OR_ONE: usize = 1 / ((N >= 2) as usize);

    pub fn new(value: usize) -> Self {
        Self { value: value % N }
    }

    pub fn pow(self, mut nth: usize) -> Self {
        let mut result = Self::new(1);
        let mut base = self;
        while nth > 0 {
            if nth % 2 == 1 {
                result = result * base;
            }
            base = base * base;
            nth /= 2;
        }
        result
    }
}

impl<const N: usize> std::ops::Add for Mod<N> {
    type Output = Mod<N>;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}

impl<const N: usize> std::ops::AddAssign for Mod<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const N: usize> std::ops::Mul for Mod<N> {
    type Output = Mod<N>;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.value * rhs.value)
    }
}

impl<const N: usize> std::ops::MulAssign for Mod<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const N: usize> std::fmt::Display for Mod<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_modulation() {
        let a: Mod<7> = Mod::new(10);
        // 10 mod 7 = 3
        assert_eq!(a.value, 3);
    }

    #[test]
    fn test_add() {
        let a: Mod<7> = Mod::new(3);
        let b: Mod<7> = Mod::new(5);
        let c = a + b;
        // (3 + 5) mod 7 = 8 mod 7 = 1
        assert_eq!(c.value, 1);
    }

    #[test]
    fn test_add_assign() {
        let mut a: Mod<7> = Mod::new(3);
        let b: Mod<7> = Mod::new(5);
        a += b;
        // (3 + 5) mod 7 = 1
        assert_eq!(a.value, 1);
    }

    #[test]
    fn test_mul() {
        let a: Mod<7> = Mod::new(3);
        let b: Mod<7> = Mod::new(5);
        let c = a * b;
        // (3 * 5) mod 7 = 15 mod 7 = 1
        assert_eq!(c.value, 1);
    }

    #[test]
    fn test_mul_assign() {
        let mut a: Mod<7> = Mod::new(3);
        let b: Mod<7> = Mod::new(5);
        a *= b;
        // (3 * 5) mod 7 = 1
        assert_eq!(a.value, 1);
    }

    #[test]
    fn test_pow_zero_exponent() {
        let a: Mod<7> = Mod::new(3);
        let b = a.pow(0);
        // any number^0 mod N should be 1
        assert_eq!(b.value, 1);
    }

    #[test]
    fn test_pow_small_exponent() {
        let a: Mod<13> = Mod::new(2);
        let b = a.pow(3);
        // 2^3 = 8 mod 13 = 8
        assert_eq!(b.value, 8);
    }

    #[test]
    fn test_pow_large_exponent() {
        let a: Mod<1_000_003> = Mod::new(7);
        let b = a.pow(100);
        // Calculate expected result by iterated multiplication
        let mut expected = 1;
        for _ in 0..100 {
            expected = (expected * 7) % 1_000_003;
        }
        assert_eq!(b.value, expected);
    }

    #[test]
    fn test_display() {
        let a: Mod<7> = Mod::new(10); // 10 mod 7 = 3
        assert_eq!(a.to_string(), "3");
    }
}