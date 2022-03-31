use attribute::*;

#[trace_vars(p, n)]
fn factorial_nth(mut n: u64) -> u64 {
    let mut p = 1;
    while n > 1 {
        p *= n;
        n -= 1;
    }
    p
}

#[test]
fn test_trace_vars() {
    let f = factorial_nth(10);
}
