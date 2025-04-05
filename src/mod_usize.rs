#[derive(Clone, Copy)]
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