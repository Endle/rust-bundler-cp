// pr means print

#[inline(always)]
#[allow(dead_code)]
pub fn ln<T: std::fmt::Display>(x : T) {
    print!("{}\n", x)
}


#[inline(always)]
#[allow(dead_code)]
// print single Element
pub fn e<T: std::fmt::Display>(x : T) {
    print!("{} ", x)
}
#[inline(always)] #[allow(dead_code)]
pub fn pb() { print!(" ") }

#[inline(always)]
#[allow(dead_code)]
pub fn endl() { print!("\n") }

#[inline(always)]
#[allow(dead_code)]
pub fn slice<T: std::fmt::Display>(v: &[T]) {
    for x in v {
        print!("{} ", &x);
    }
    print!("\n")
}