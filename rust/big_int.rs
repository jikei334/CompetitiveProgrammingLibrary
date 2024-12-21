#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct BigInt([u64; BigInt::NUM_MOD]);

impl BigInt {
    const NUM_MOD: usize = 3;
    const MODS: [u64; BigInt::NUM_MOD] = [
        998244353,
        1_000_000_000 + 7,
        1_000_000_000 + 9,
    ];

    fn pow(&self, n: u64) -> Self {
        let mut val = [0u64; BigInt::NUM_MOD];
        for i in 0..BigInt::NUM_MOD {
            val[i] = modpow(self.0[i], n, BigInt::MODS[i]);
        }
        return Self(val);
    }
}

impl From<Vec<char>> for BigInt {
    fn from(item: Vec<char>) -> Self {
        let mut val = [0u64; BigInt::NUM_MOD];
        for &c in &item {
            for i in 0..BigInt::NUM_MOD {
                val[i] = (10 * val[i] + ((c as u8 - '0' as u8) as u64)) % BigInt::MODS[i];
            }
        }
        Self(val)
    }
}

impl From<u64> for BigInt {
    fn from(item: u64) -> Self {
        let mut val = [0u64; BigInt::NUM_MOD];
        for i in 0..BigInt::NUM_MOD {
            val[i] = item % BigInt::MODS[i];
        }
        return Self(val);
    }
}

impl std::ops::Add for BigInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut val = [0u64; BigInt::NUM_MOD];
        for i in 0..BigInt::NUM_MOD {
            val[i] = (self.0[i] + other.0[i]) % BigInt::MODS[i];
        }
        return Self(val);
    }
}

impl std::ops::Sub for BigInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut val = [0u64; BigInt::NUM_MOD];
        for i in 0..BigInt::NUM_MOD {
            val[i] = (BigInt::MODS[i] + self.0[i] - other.0[i]) % BigInt::MODS[i];
        }
        return Self(val);
    }
}

impl std::ops::Mul for BigInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut val = [0u64; BigInt::NUM_MOD];
        for i in 0..BigInt::NUM_MOD {
            val[i] = (self.0[i] * rhs.0[i]) % BigInt::MODS[i];
        }
        return Self(val);
    }
}

impl std::ops::Div for BigInt {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut val = [0u64; BigInt::NUM_MOD];
        for i in 0..BigInt::NUM_MOD {
            val[i] = (self.0[i] * modpow(rhs.0[i], BigInt::MODS[i]-2, BigInt::MODS[i])) % BigInt::MODS[i];
        }
        return Self(val);
    }
}
