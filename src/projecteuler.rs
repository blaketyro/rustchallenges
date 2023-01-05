/// https://projecteuler.net/problem=1
pub fn problem1() -> i32 {
    let limit = 1000;

    let mut total = 0;
    for i in 1..limit {
        if i % 3 == 0 || i % 5 == 0 {
            total += i;
        }
    }
    total
}

/// https://projecteuler.net/problem=1 alternate solution
pub fn problem1_solution2() -> i32 {
    let limit = 1000;

    let triangular = |n| n * (n + 1) / 2;
    let helper = |n| n * triangular((limit - 1) / n);
    helper(3) + helper(5) - helper(3 * 5)
}

/// https://projecteuler.net/problem=2
pub fn problem2() -> i32 {
    let limit = 4000000;
    let (mut current_fib, mut next_fib) = (1, 2);
    let mut total = 0;

    while current_fib < limit {
        if current_fib % 2 == 0 {
            total += current_fib;
        }
        (current_fib, next_fib) = (next_fib, current_fib + next_fib);
    }
    total
}
