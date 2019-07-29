use time::precise_time_ns;

pub fn timed<T>(str_fn: &(Fn(u64) -> String), f : &mut (FnMut() -> T)) -> T{
    let t1 = precise_time_ns();
    let ret = f();
    let t2 = precise_time_ns();

    let dt = t2 - t1;

    println!("{}", str_fn(dt));

    ret
}

//F3 : FnMut(A) -> C
fn compose<'l, A, B, C, F1, F2>(f1 : & 'l Box<F1>, f2 : &'l Box<F2>) -> Box<Fn(A) -> C + 'l>
    where F1 : 'l + Fn(A) -> B,
          F2 : 'l + Fn(B) -> C,
{
    Box::new(move |a : A| {(*f2)((*f1)(a))})
}


unsafe fn compose2<'a, A,B,C, F, G>(f : *const F, g : *const G) -> Box<Fn(A) -> C + 'a> where
    F :  Fn(A) -> B + 'a,
    G :  Fn(B) -> C + 'a{
    unsafe{
        Box::new( move |a| (*g)( (*f)(a) ) )
    }
}