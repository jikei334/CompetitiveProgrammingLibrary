fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1i64, 0i64);
    } else {
        let q = a / b;
        let (g, x, y) = ext_gcd(b, a%b);
        return (g, y, x - q * y);
    }
}

fn crt(am: Vec<(i64, i64)>) -> Option<(i64, i64)> {
    let mut r = 0i64;
    let mut n = 1i64;
    for &(a, m) in &am {
        let (g, x, y) = ext_gcd(n, m);
        if (a - r) % g != 0 {
            return None;
        }
        let tmp = (a - r) / g * x % (m / g);
        r += n * tmp;
        n *= m / g;
    }
    return Some(((r % n + n) % n, n));
}

