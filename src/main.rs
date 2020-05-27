extern crate rand;
use rand::Rng;

fn main() {
    //let p = 9887;
    let p = generate_safe_prime();
    //let q = 8699;
    let q = generate_safe_prime();
    let pq = (p - 1)*(q - 1);
    let mut e = generate_prime();
    while gcd(pq, e) != 1 {
        e =generate_prime();
    }
    println!("{}", e);
    let d = extended_euclidean(pq, e);
    println!("{}", d);
    let m = 7777;
    let m1 = pow_mod(m,e,p*q);
    println!("{}",m1);
    let m2 = pow_mod(m1,d,p*q);
    println!("{}",m2);
}

//Output squared value.
/*
fn pow(a: i64, n: i64) -> i64 {
    let a = a;
    let mut b = a.clone();
    let n = n;
    for _x in 0..n-1 {
        b *= a;
    }
    b
}
*/

//Output greatest common divisor.
fn gcd(a: i64, b:i64) -> i64{
    let mut a = a;
    let mut b = b;
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

fn generate_safe_prime() -> i64 {
    let mut e = generate_prime();
    let mut a = e as u16;
    while !is_prime(a-1/2) {
        e = generate_prime();
        a = e as u16;
    }
    e
}

fn generate_prime() -> i64 {
    let mut a: u16 = rand::thread_rng().gen();
    while !is_prime(a) {
        a = rand::thread_rng().gen();
    }
    let e = a as i64;
    e
}


//Check if the number is prime.
fn is_prime(n: u16) -> bool {
    let n = n as i64;
    if n == 2 {
        return true
    } else if n == 1 || n & 1 == 0 {
        return false
    }
    let mut d = n - 1 >> 1;
    while d & 1 == 0 {
        d >>= 1;
    }
    for _x in 0..100 {
        let a = rand::thread_rng().gen_range(1, n - 1);
        let mut t = d;
        let mut y = pow_mod(a, t, n);
        while t != n - 1 && y != 1 && y != n -1 {
            y = (y * y) % n;
            t <<= 1;
        }
        if y != n - 1 && t & 1 == 0 {
            return false
        }
    }
    true
}


fn pow_mod(x: i64, n: i64, m: i64) -> i64 {
    let mut i = 1;
    let mut p = 1;
    let mut x = x;
    while i <= n {
        if i & n != 0 {
            p = p*x%m;
        }
        x = x*x%m;
        i = i << 1;
    }
    p
}

//Execute Extended_euclidean and output t0(inverse of u).
fn extended_euclidean(u: i64, v: i64) -> i64 {
    let mut r0 = u;
    let mut r1 = v;
    let mut s0 = 1;
    let mut s1 = 0;
    let mut t0 = 0;
    let mut t1 = 1;
    while r1 != 0 {
        let q = r0 / r1;
        let r = r0 - q * r1;
        let s = s0 - q * s1;
        let t = t0 - q * t1;
        r0 = r1;
        s0 = s1;
        t0 = t1;
        r1 = r;
        s1 = s;
        t1 = t;
    }
    println!("{} * {} + {} * {} = {}", s0, u, t0, v, r0);
    if t0 < 0 {
        t0 + u
    } else {
        t0
    }

}
