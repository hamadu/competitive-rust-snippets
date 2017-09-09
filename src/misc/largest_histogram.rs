fn largest(a: Vec<usize>) -> usize {
    let n = a.len();
    let mut right: Vec<usize> = vec![0; n];
    {
        let mut stk: Vec<usize> = vec![];
        for i in (0..n).rev() {
            while stk.len() >= 1 && a[i] <= a[stk[stk.len()-1]] {
                stk.pop();
            }
            right[i] = if stk.len() >= 1 { stk[stk.len()-1] } else { n };
            stk.push(i);
        }
    }

    let mut left: Vec<usize> = vec![0; n];
    {
        let mut stk: Vec<usize> = vec![];
        for i in 0..n {
            while stk.len() >= 1 && a[i] <= a[stk[stk.len()-1]] {
                stk.pop();
            }
            left[i] = if stk.len() >= 1 { stk[stk.len()-1]+1 } else { 0 };
            stk.push(i);
        }
    }

    let mut ans = 0;
    for i in 0..n {
        ans = max(ans, (right[i] - left[i]) * a[i]);
    }
    ans
}