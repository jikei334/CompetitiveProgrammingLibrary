#[derive(Clone, Copy, Debug)]
struct ModInt {
    v: usize,
}

impl ModInt {
    fn new(v: usize) -> Self {
        return ModInt {
            v: v % MOD
        };
    }

    fn pow(&self, exp: usize) -> Self {
        return Self {
            v: modpow(self.v, exp, MOD)
        };
    }
}

impl std::ops::Add<ModInt> for ModInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return ModInt {
            v: (self.v + other.v) % MOD
        };
    }
}

impl std::ops::AddAssign<ModInt> for ModInt {
    fn add_assign(&mut self, other: Self) {
        *self = ModInt {
            v: (self.v + other.v) % MOD
        };
    }
}

impl std::ops::Sub<ModInt> for ModInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        return ModInt {
            v: (MOD + self.v - other.v) % MOD
        };
    }
}

impl std::ops::SubAssign<ModInt> for ModInt {
    fn sub_assign(&mut self, other: Self) {
        *self = ModInt {
            v: (MOD + self.v - other.v) % MOD
        };
    }
}

impl std::ops::Mul<ModInt> for ModInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        return ModInt {
            v: (self.v * other.v) % MOD
        };
    }
}

impl std::ops::Mul<usize> for ModInt {
    type Output = Self;

    fn mul(self, other: usize) -> Self {
        return Self {
            v: (self.v * other) % MOD
        };
    }
}

impl std::ops::Div<ModInt> for ModInt {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        return self * other.pow(MOD-2);
    }
}

impl std::ops::Div<usize> for ModInt {
    type Output = Self;

    fn div(self, other: usize) -> Self::Output {
        return Self {
            v: (self.v * modpow(other, MOD-2, MOD)) % MOD
        };
    }
}

impl std::fmt::Display for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.v);
    }
}

impl std::ops::Mul<ModInt> for usize {
    type Output = ModInt;

    fn mul(self, other: ModInt) -> Self::Output {
        return Self::Output {
            v: (self * other.v) % MOD
        };
    }
}
