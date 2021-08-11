// IN PROGRESS!
pub struct Note<FType, ArgT, ResultT>
{
    calculate: FType,
    limit: ArgT,
    max_n: usize,
    max_m: usize,
    _notes: Vec<ResultT>
}

pub fn create_notebook<FType, ArgType:Into<usize>>(f: FType, limit:ArgType) -> bool{
    let dimensions = 1;
    let n:usize = limit.into() + 1;

    return true;
}
// pub fn create_notebook<FType>(f: FType, limit:(usize,usize)) -> bool{
//     let dimensions = 1;
//     let n = usize::from(limit) + 1;
//     return true;
// }
// impl<FType, ArgT, ResultT>  Note<FType, ArgT, ResultT> {
    // pub fn new(f: FType, limit:ArgT) -> Note<FType, ArgT, ResultT>  {
    //     let dimensions = 1;
    //     let n = usize::from(limit);
    //     let container_size = n + 1;
    //     let mut v = Vec::with_capacity(container_size);
    //     for i in 0..container_size {
    //         v.push(-1);
    //         // v[i] = -1;
    //     }
    //     Note {
    //         calculate: f,
    //         limit:limit.copy(),
    //         max_n: n+1,
    //         max_m: 11,
    //         _notes: v,
    //     }
    // }
    // pub fn new(n:usize, m:usize) -> Self {
    //     let container_size = (n+1)*(m+1);
    //     let mut v = Vec::with_capacity(container_size);
    //     for i in 0..container_size {
    //         v.push(-1);
    //         // v[i] = -1;
    //     }
    //     Note {
    //         max_n: n+1,
    //         max_m: m+1,
    //         _notes: v,
    //     }
    // }
    // fn _get_pos(&self, n:usize, m:usize) -> usize {
    //     assert!(n >= 0);
    //     assert!(n <= self.max_n);
    //     assert!(m >= 0);
    //     assert!(m <= self.max_m);
    //     n * self.max_m + m
    // }
    // // TODO: I should use Opton here
    // pub fn has(&self, n:usize, m:usize) -> bool {
    //     let p = self._get_pos(n, m);
    //     if self._notes[p] < 0 {
    //         return false;
    //     }
    //     return true;
    // }
    // pub fn get(&self, n:usize, m:usize) -> i64 {
    //     let p = self._get_pos(n, m);
    //     self._notes[p]
    // }
    // pub fn set(&mut self, n:usize, m:usize, ans:i64) -> i64 {
    //     let p = self._get_pos(n, m);
    //     self._notes[p] = ans;
    //     ans
    // }
// }