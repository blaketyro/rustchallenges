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

pub fn problem1_solution2() -> i32 {
    let limit = 1000;

    let triangular = |n| n * (n + 1) / 2;
    let helper = |n| n * triangular((limit - 1) / n);
    helper(3) + helper(5) - helper(3 * 5)
}
