fn modpow(base: usize, exp: usize, m: usize) -> usize {
    if exp == 0 {
        1
    } else {
        let half = modpow(base, exp/2, m);
        let ret = (half * half) % m;
        if exp % 2 == 0 {
            ret
        } else {
            (ret * base) % m
        }
    }
}
