fn main() {
    println!("{}", log2(100));
}

fn log2(x: u64) -> f64 {
    let mut i = 0.0;
    let mut y = 1;
    while y <= x {
        i += 1.0;
        y *= 2;
    }
    let mut s = 0;
    let mut m = 2.0;
    let mut z = 0.0;
    while i >= 0.0 {
        print!("[i={i}] ");
        if s + y <= x {
            z += i * m;
            s += y;
            println!("{s} + {y} <= {x} ==> {z}");
        } else {
            println!("{s} + {y} > {x}");
        }
        i -= 1.0;
        m /= 2.0;
        y /= 2;
    }
    z
}

fn log(x: u64, base: u64) -> f64 {
    log2(x) / log2(base)
}

#[cfg(test)]
mod log {
    use super::*;

    #[test]
    fn easy() {
        assert_eq!(log2(1), 0.0);
        assert_eq!(log2(2), 1.0);
        assert_eq!(log2(4), 2.0);
        assert_eq!(log2(8), 3.0);
        assert_eq!(log2(16), 4.0);
        assert_eq!(log2(32), 5.0);
        assert_eq!(log2(64), 6.0);
        assert_eq!(log2(128), 7.0);
    }
}
