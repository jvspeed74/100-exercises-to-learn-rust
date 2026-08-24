// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.

fn f(n: u32, memo: &mut Vec<Option<u32>>, calls: &mut u32) -> u32 {
    *calls += 1;

    // base cases
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    // if index (n) is not in the list, calculate and add it to the list
    // return the fib value on all paths
    let n_as_idx: usize = n as usize;

    if memo[n_as_idx].is_none() {
        let first = f(n - 1, memo, calls);
        let second = f(n - 2, memo, calls);
        memo[n_as_idx] = Some(first + second);
    }

    memo[n_as_idx].unwrap()
}

pub fn fibonacci(n: u32) -> u32 {
    // passthrough so i dont have to refactor the lesson API to account for memo
    let mut memo: Vec<Option<u32>> = vec![None; (n + 1) as usize];
    let mut calls = 0u32;

    // = F(n-1) + F(n-2)
    f(n, &mut memo, &mut calls)
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }

    #[test]
    fn memoization_bounds_call_count() {
        // With memoization, each index 2..=n is computed exactly once, on a
        // "miss" that spawns exactly 2 child calls (f(k-1), f(k-2)). Every
        // other visit to that index is a "hit" that returns immediately
        // without recursing. That gives an exact call count of 2n - 1 for
        // n >= 1 -- linear in n, not the ~O(phi^n) naive recursion would cost.
        //
        // If this assertion fails while `fibonacci` still returns the right
        // answer, the memo isn't being consulted -- it's silently falling
        // back to full recomputation on every call.
        let n = 30;
        let mut memo: Vec<Option<u32>> = vec![None; (n + 1) as usize];
        let mut calls = 0u32;

        let result = crate::f(n, &mut memo, &mut calls);

        assert_eq!(result, 832040);
        assert_eq!(calls, 2 * n - 1, "expected O(n) calls for a working memo");
    }
}
