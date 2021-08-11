pub mod pr {
    #[inline(always)]
    #[allow(dead_code)]
    pub fn ln<T: std::fmt::Display>(x: T) {
        print!("{}\n", x)
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub fn e<T: std::fmt::Display>(x: T) {
        print!("{} ", x)
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub fn pb() {
        print!(" ")
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub fn endl() {
        print!("\n")
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub fn slice<T: std::fmt::Display>(v: &[T]) {
        for x in v {
            print!("{} ", &x);
        }
        print!("\n")
    }
}
pub mod algo {
    pub fn calculate_array_inversions<T: std::cmp::PartialOrd + Copy + Ord>(arr: &[T]) -> i64 {
        let (answer, _) = _calculate_array_inversions(arr);
        return answer;
    }
    fn _calculate_array_inversions<T: std::cmp::PartialOrd + Copy + Ord>(
        arr: &[T],
    ) -> (i64, Vec<T>) {
        let n = arr.len();
        match n {
            0 | 1 => return (0, arr.to_vec()),
            3..=10 => return _calculate_array_inversions_brute_force(arr),
            _ => (),
        }
        let mid = n / 2;
        let left = &arr[0..mid];
        let right = &arr[mid..];
        let (left_val, left_sorted) = _calculate_array_inversions(left);
        let (right_val, right_sorted) = _calculate_array_inversions(right);
        let (swap_count, vec_sorted) = _merge_sorted_vecs(left_sorted, right_sorted);
        let answer = left_val + right_val + swap_count;
        return (answer, vec_sorted);
    }
    fn _merge_sorted_vecs<T: std::cmp::PartialOrd + Copy + Ord>(
        left: Vec<T>,
        right: Vec<T>,
    ) -> (i64, Vec<T>) {
        assert!(!left.is_empty());
        assert!(!right.is_empty());
        let mut sorted = Vec::with_capacity(left.len() + right.len());
        let mut swap_count = 0;
        let mut lp: usize = 0;
        let mut rp: usize = 0;
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
        return (swap_count, sorted);
    }
    fn _calculate_array_inversions_brute_force<T: std::cmp::PartialOrd + Copy + Ord>(
        arr: &[T],
    ) -> (i64, Vec<T>) {
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
}
pub mod nd {
    pub struct Arr<T> {
        _max_n: usize,
        _max_m: usize,
        _data: Vec<T>,
    }
    impl<T: Copy> Arr<T> {
        pub fn new(size: (usize, usize), default: T) -> Self {
            let dimensions = 2;
            let container_size = (size.0 + 1) * (size.1 + 1) + 1;
            let mut v = Vec::with_capacity(container_size);
            for _ in 0..container_size {
                v.push(default);
            }
            Arr {
                _max_n: size.0,
                _max_m: size.1,
                _data: v,
            }
        }
        fn _get_pos(&self, n: usize, m: usize) -> usize {
            assert!(n <= self._max_n);
            assert!(m <= self._max_m);
            n * self._max_m + m
        }
        pub fn get(&self, n: usize, m: usize) -> T {
            let p = self._get_pos(n, m);
            self._data[p]
        }
        pub fn set(&mut self, n: usize, m: usize, ans: T) -> T {
            let p = self._get_pos(n, m);
            self._data[p] = ans;
            ans
        }
    }
    impl<T: std::fmt::Display + Copy> std::fmt::Debug for Arr<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Dim={}, ({},{})\n", 2, self._max_n, self._max_m);
            for i in 0..self._max_n {
                write!(f, "    [");
                for j in 0..self._max_m {
                    write!(f, "{}", self.get(i, j));
                    if j + 1 < self._max_m {
                        write!(f, ",");
                    }
                }
                write!(f, "]\n");
            }
            write!(f, "")
        }
    }
}
pub mod multi_queue {
    use std::collections::HashMap;
    use std::collections::VecDeque;
    pub struct MultiQueue<KeyT, ValT>
    where
        KeyT: Eq + std::hash::Hash,
    {
        _data: HashMap<KeyT, VecDeque<ValT>>,
    }
    impl<KeyT: Eq + std::hash::Hash, ValT> MultiQueue<KeyT, ValT> {
        pub fn new() -> Self {
            MultiQueue {
                _data: HashMap::new(),
            }
        }
        pub fn push(&mut self, key: KeyT, value: ValT) {
            let mut entry = self._data.entry(key).or_insert(VecDeque::new());
            entry.push_back(value);
        }
        #[doc = " Return None if 1. key not found 2. queue consumed"]
        pub fn pop(&mut self, key: KeyT) -> Option<ValT> {
            let mut queue = self._data.get_mut(&key);
            if queue.is_none() {
                return None;
            }
            let mut queue = queue.expect("Get Queue");
            queue.pop_front()
        }
    }
}
pub mod nums {
    #[doc = " Returns a vector representing the bits."]
    #[doc = "     i=63 is the highest bit, i=0 is the lowest bit"]
    #[doc = "     x = sigma v[i]*(2^i)"]
    #[doc = " # Arguments"]
    #[doc = ""]
    #[doc = " * `x`: A 64-bit number to be represented in bits"]
    #[doc = ""]
    #[doc = " returns: Vec<u8>"]
    #[doc = ""]
    #[allow(dead_code)]
    pub fn represent_into_bits(x: u64) -> Vec<u8> {
        let mut result = Vec::with_capacity(64);
        let mut x = x;
        for _ in 0..64 {
            let bit = x & 1;
            let bit = bit as u8;
            result.push(bit);
            x >>= 1;
        }
        assert_eq!(result.len(), 64);
        result
    }
    pub fn represent_from_bits(a: &Vec<u8>) -> u64 {
        assert_eq!(a.len(), 64);
        let mut result: u64 = 0;
        for i in 0..64 {
            result += a[i] as u64 * (1u64 << i);
        }
        result
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub fn multi_mod(x: i32, y: i32, modulo: i32) -> i32 {
        let mid: i64 = (x as i64) * (y as i64);
        let mid = mid % (modulo as i64);
        return mid as i32;
    }
    #[allow(dead_code)]
    pub fn pow_mod(base: i32, exp: u32, modulo: i32) -> i32 {
        let mut answer: i64 = 1;
        if exp > 10 {
            let sub_exp = exp / 2;
            let mut p: i64 = pow_mod(base, sub_exp, modulo) as i64;
            p = p as i64 * p as i64 % modulo as i64;
            if exp % 2 == 1 {
                p = p * base as i64 % modulo as i64;
            }
            return p as i32;
        }
        for _ in 0..exp {
            answer *= base as i64;
            answer %= modulo as i64;
        }
        answer as i32
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub fn in_closed_range(left: i32, right: i32, x: i32) -> bool {
        left <= x && x <= right
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub fn in_closed_range_reversible(left: i32, right: i32, x: i32) -> bool {
        match left.cmp(&right) {
            std::cmp::Ordering::Less => in_closed_range(left, right, x),
            std::cmp::Ordering::Equal => x == left,
            std::cmp::Ordering::Greater => in_closed_range(right, left, x),
        }
    }
    pub fn select_lowest_kth_bit(x: u32, b: u32) -> u32 {
        let result = (x >> b) & 1;
        assert!(result == 0 || result == 1);
        result
    }
    #[allow(dead_code)]
    pub fn calc_combination_with_mod(a: i32, b: i32, modulo: i32) -> i32 {
        let mut num: i64 = 1;
        let mut den: i64 = 1;
        for i in 0..b {
            num = num * (a - i) as i64 % modulo as i64;
            den = den * (i + 1) as i64 % modulo as i64;
        }
        let answer = num * pow_mod(den as i32, modulo as u32 - 2, modulo) as i64;
        let answer = answer % modulo as i64;
        answer as i32
    }
}
pub mod iters {
    pub fn permutation_chars(s: &str) -> Vec<Vec<u8>> {
        return permutation(s.as_bytes());
    }
    pub fn permutation<T: Copy>(s: &[T]) -> Vec<Vec<T>> {
        assert!(s.len() > 0);
        let mut possibilities = 1;
        for i in 1..=s.len() {
            possibilities *= i;
        }
        permutation_dfs(s, &vec![false; s.len()])
    }
    fn permutation_dfs<T: Copy>(s: &[T], used: &Vec<bool>) -> Vec<Vec<T>> {
        assert_eq!(s.len(), used.len());
        let mut result_list = Vec::new();
        for pos in 0..s.len() {
            if used[pos] {
                continue;
            }
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
}
use std::error;
use std::fmt;
use std::str::FromStr;
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum Error {
    MissingMatch,
    MissingClosingBrace,
    UnexpectedValue(u8, Option<u8>),
    InvalidUtf8(Vec<u8>),
    PartialUtf8(usize, Vec<u8>),
    Parse(String, &'static str),
}
impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::Error::*;
        use std::str::from_utf8;
        match *self {
            InvalidUtf8(ref raw) => write!(f, "input was not valid utf8: {:?}", raw),
            Parse(ref s, arg) => write!(f, "could not parse {} as target type of {}", s, arg),
            UnexpectedValue(exp, act) => write!(
                f,
                "found value {:?} not matching the pattern value {}",
                act.map(|b| b as char),
                exp as char
            ),
            PartialUtf8(n, ref raw) => write!(
                f,
                "input was only partially valid utf8: \"{}\" followed by {:?}",
                from_utf8(&raw[..n]).unwrap(),
                &raw[n..]
            ),
            MissingMatch => write!(f, "Bad read! format string: did not contain {{}}"),
            MissingClosingBrace => write!(
                f,
                "found single open curly brace at the end of the format string"
            ),
        }
    }
}
pub fn match_next(expected: u8, iter: &mut dyn Iterator<Item = u8>) -> Result<(), Error> {
    let next = iter.next();
    if next != Some(expected) {
        return Err(Error::UnexpectedValue(expected, next));
    }
    Ok(())
}
pub fn parse_capture<T>(
    name: &'static str,
    next: Option<u8>,
    iter: &mut dyn Iterator<Item = u8>,
) -> Result<T, Error>
where
    T: FromStr,
    <T as FromStr>::Err: ::std::fmt::Debug,
{
    static WHITESPACES: &[u8] = b"\t\r\n ";
    let raw: Vec<u8> = match next {
        Some(c) => iter.take_while(|&ch| ch != c).collect(),
        None => iter
            .skip_while(|ch| WHITESPACES.contains(ch))
            .take_while(|ch| !WHITESPACES.contains(ch))
            .collect(),
    };
    match String::from_utf8(raw) {
        Ok(s) => FromStr::from_str(&s).map_err(|_| Error::Parse(s, name)),
        Err(e) => {
            let n = e.utf8_error().valid_up_to();
            let raw = e.into_bytes();
            if n == 0 {
                Err(Error::InvalidUtf8(raw))
            } else {
                Err(Error::PartialUtf8(n, raw))
            }
        }
    }
}
#[macro_export]
macro_rules ! try_read (() => { $ crate :: try_read ! ("{}") } ; ($ text : expr) => { { (|| -> std :: result :: Result < _ , $ crate :: Error > { let __try_read_var__ ; $ crate :: try_scan ! ($ text , __try_read_var__) ; Ok (__try_read_var__) }) () } } ; ($ text : expr , $ input : expr) => { { (|| -> std :: result :: Result < _ , $ crate :: Error > { let __try_read_var__ ; $ crate :: try_scan ! ($ input => $ text , __try_read_var__) ; Ok (__try_read_var__) }) () } } ;) ;
#[macro_export]
macro_rules ! try_scan (($ pattern : expr , $ ($ arg : expr) ,*) => { use :: std :: io :: Read ; $ crate :: try_scan ! (:: std :: io :: stdin () . bytes () . map (std :: result :: Result :: unwrap) => $ pattern , $ ($ arg) ,*) ; } ; ($ input : expr => $ pattern : expr , $ ($ arg : expr) ,*) => { { $ crate :: try_scan ! (@ impl question_mark ; $ input => $ pattern , $ ($ arg) ,*) } } ; (@ question_mark : $ ($ e : tt) +) => { { ($ ($ e) +) ? } } ; (@ unwrap : $ ($ e : tt) +) => { { ($ ($ e) +) . unwrap () } } ; (@ impl $ action : tt ; $ input : expr => $ pattern : expr , $ ($ arg : expr) ,*) => { { #! [allow (clippy :: try_err)] use $ crate :: { Error , match_next , parse_capture } ; let pattern : &'static str = $ pattern ; let stdin : & mut Iterator < Item = u8 > = & mut ($ input) ; let mut pattern = pattern . bytes () ; $ ($ arg = loop { match $ crate :: try_scan ! (@$ action : pattern . next () . ok_or (Error :: MissingMatch)) { b'{' => match $ crate :: try_scan ! (@$ action : pattern . next () . ok_or (Error :: MissingClosingBrace)) { b'{' => $ crate :: try_scan ! (@$ action : match_next (b'{' , stdin)) , b'}' => break $ crate :: try_scan ! (@$ action : parse_capture (stringify ! ($ arg) , pattern . next () , stdin)) , _ => return $ crate :: try_scan ! (@$ action : Err (Error :: MissingClosingBrace)) , } , c => $ crate :: try_scan ! (@$ action : match_next (c , stdin)) , } } ;) * for c in pattern { $ crate :: try_scan ! (@$ action : match_next (c , stdin)) } format_args ! ($ pattern , $ ($ arg) ,*) ; } } ;) ;
#[doc = " All text input is handled through this macro"]
#[macro_export]
macro_rules ! read (($ ($ arg : tt) *) => { $ crate :: try_read ! ($ ($ arg) *) . unwrap () } ;) ;
#[doc = " This macro allows to pass several variables so multiple values can be read"]
#[macro_export]
macro_rules ! scan (($ text : expr , $ ($ arg : expr) ,*) => { use :: std :: io :: Read ; $ crate :: scan ! (:: std :: io :: stdin () . bytes () . map (std :: result :: Result :: unwrap) => $ text , $ ($ arg) ,*) ; } ; ($ input : expr => $ pattern : expr , $ ($ arg : expr) ,*) => { { $ crate :: try_scan ! (@ impl unwrap ; $ input => $ pattern , $ ($ arg) ,*) } } ;) ;
#[inline(always)]
#[allow(dead_code)]
pub fn read_ivec(n: usize) -> Vec<i32> {
    let mut vec = Vec::with_capacity(n);
    for _ in 0..n {
        let x: i32 = read!();
        vec.push(x);
    }
    return vec;
}
#[inline]
#[allow(dead_code)]
pub fn read_ived_1(n: usize) -> Vec<i32> {
    let mut vec = Vec::with_capacity(n + 1);
    vec.push(0);
    for _ in 0..n {
        let x: i32 = read!();
        vec.push(x);
    }
    return vec;
}
#[inline(always)]
#[allow(dead_code)]
pub fn read_uvec(n: usize) -> Vec<u32> {
    let mut vec = Vec::with_capacity(n);
    for _ in 0..n {
        let x: u32 = read!();
        vec.push(x);
    }
    return vec;
}
fn solve() -> Option<i32> {
    None
}
fn main() {
    let testcases: i32 = read!();
    for _ in 0..testcases {
        solve_and_print();
    }
}
#[inline]
fn solve_and_print() {
    let answer = solve();
    match answer {
        None => (),
        _ => pr::ln(answer.unwrap()),
    }
}
fn smaller_pair(a: u32, b: u32) -> (u32, u32) {
    let smaller = a.min(b);
    let larger = a.max(b);
    return (smaller, larger);
}
fn read_01_vec() -> Vec<u8> {
    let s: String = read!();
    let mut ret = Vec::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '0' => ret.push(0),
            '1' => ret.push(1),
            _ => panic!("Unexpected char"),
        }
    }
    ret
}
