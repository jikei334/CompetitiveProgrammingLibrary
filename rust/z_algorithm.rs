fn z_algorithm(s: &Vec<char>) -> Vec<u64> {
    let n = s.len();
    let mut z = vec![0u64; n];
    z[0] = n as u64;
    let mut i = 1usize;
    let mut j = 0usize;
    while i < n {
        while i + j < n && s[i+j] == s[j] {
            j += 1;
        }
        z[i] = j as u64;
        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1usize;
        while k < j && k + (z[k] as usize) < j {
            z[i+k] = z[k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    return z;
}
