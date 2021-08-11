
pub fn permutation_chars(s:&str) -> Vec<Vec<u8>> {
    return permutation(s.as_bytes());
}

pub fn permutation<T:Copy>(s: &[T]) -> Vec<Vec<T>> {
    assert!(s.len() > 0);
    let mut possibilities = 1;
    for i in 1..=s.len() {
        possibilities *= i;
    }

    permutation_dfs(s, &vec![false; s.len()])
    // result
}

fn permutation_dfs<T: Copy>(s: &[T], used: &Vec<bool>) -> Vec<Vec<T>>{
    assert_eq!(s.len(), used.len());
    // eprintln!("s: {}, used: {:?}", s.len(), used);
    let mut result_list = Vec::new();

    for pos in 0..s.len() {
        if used[pos] {continue;}
        let mut new_used = used.clone();
        new_used[pos] = true;
        let select = s[pos];

        let recursive_results = permutation_dfs(s, &new_used);
        if recursive_results.is_empty() {
            let mut r = Vec::with_capacity(s.len());
            r.push(select);
            result_list.push(r);
        } else {
            for v in recursive_results {
                let mut r = v;
                r.push(select);
                result_list.push(r);
            }
        }

    }



    result_list
}