pub fn calculate_array_inversions
<T: std::cmp::PartialOrd + Copy + Ord>(arr: &[T]) -> i64 {
    let (answer, _) = _calculate_array_inversions(arr);
    return answer;
}

fn _calculate_array_inversions
<T: std::cmp::PartialOrd + Copy + Ord>(arr: &[T]) -> (i64, Vec<T>) {
    let n = arr.len();
    match n {
        0 | 1 => return (0, arr.to_vec()),
        3..=10 => return _calculate_array_inversions_brute_force(arr),
        _ => (),
    }
    let mid = n / 2; //mid >= 1, mid+1 < n. splitting to [0,mid), [mid,n)

    let left = &arr[0..mid];
    let right = &arr[mid..];
    let (left_val, left_sorted) = _calculate_array_inversions(left);
    let (right_val, right_sorted) = _calculate_array_inversions(right);
    let (swap_count, vec_sorted) = _merge_sorted_vecs(left_sorted, right_sorted);
    let answer = left_val + right_val + swap_count;
    return (answer, vec_sorted);
}

fn _merge_sorted_vecs<T: std::cmp::PartialOrd + Copy + Ord>(left: Vec<T>, right: Vec<T>)
                                                            -> (i64, Vec<T>) {
    assert!(!left.is_empty());
    assert!(!right.is_empty());
    let mut sorted = Vec::with_capacity(left.len() + right.len());
    let mut swap_count = 0;

    let mut lp: usize = 0;
    let mut rp: usize = 0;
    // do compare
    while lp < left.len() && rp < right.len() {
        match left[lp] <= right[rp] {
            true => {
                sorted.push(left[lp]);
                lp += 1;
            }
            false => {
                sorted.push(right[rp]);
                rp += 1;
                let left_remained = left.len() - lp;
                swap_count += left_remained as i64;
            }
        }
    }

    while lp < left.len() {
        sorted.push(left[lp]);
        lp += 1;
    }
    while rp < right.len() {
        sorted.push(right[rp]);
        rp += 1;
    }

    assert_eq!(sorted.len(), left.len() + right.len());
    assert_eq!(left.len(), lp);
    assert_eq!(right.len(), rp);
    //dump remaining
    return (swap_count, sorted);
}

fn _calculate_array_inversions_brute_force
<T: std::cmp::PartialOrd + Copy + Ord>(arr: &[T]) -> (i64, Vec<T>) {
    let n = arr.len();
    let mut answer = 0;
    for i in 0..n {
        for j in i + 1..n {
            if arr[i] > arr[j] {
                answer += 1;
            }
        }
    }
    let mut v = arr.to_vec();
    v.sort_unstable();
    return (answer, v);
}


// Based on https://users.rust-lang.org/t/how-to-get-min-max-min-index-max-index/45324/3?u=zhenbo_endle
#[allow(dead_code)]
pub fn find_max_min_pos<T: std::cmp::PartialOrd + Copy>(slice: &[T]) -> (T, T, usize, usize) {
    std::assert!(slice.len() > 0);
    let mut max = &slice[0];
    let mut min = &slice[0];
    let mut max_pos: usize = 0;
    let mut min_pos: usize = 0;

    for index in 1..slice.len() {
        if slice[index] < *min {
            min = &slice[index];
            min_pos = index;
        }
        if slice[index] > *max {
            max = &slice[index];
            max_pos = index;
        }
    }
    (*max, *min, max_pos, min_pos)
}
