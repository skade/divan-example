pub fn fibonacci_iter_naive(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }

    let mut history: Vec<u64> = vec![1,1];

    for i in 2..=n {
        //dbg!(n, i);
        history.push(history[(i - 1) as usize] + history[(i - 2) as usize])
    }
    
    *history.last().unwrap()
}

pub fn fibonacci_iter_opt(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;

    for _i in 1..=n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }

    sum
}

pub fn fibonacci_rec(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        fibonacci_rec(n - 2) + fibonacci_rec(n - 1)
    }
}

#[test]
fn self_test() {
    for i in 1..8 {
        assert_eq!(fibonacci_rec(i), fibonacci_iter_naive(i))
    }

    for i in 1..8 {
        assert_eq!(fibonacci_rec(i), fibonacci_iter_opt(i))
    }
}
