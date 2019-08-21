use time::precise_time_ns;
use std::rc::Rc;
use util::ListI::*;

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

pub enum ListI<T>{
    Nil,
    Cons(T, Rc<ListI<T>>)
}

pub type List<T> = Rc<ListI<T>>;

pub fn nil<T : 'static>() -> List<T>{
    Rc::new(Nil)
}

pub fn cons<T : 'static>(x : T, tail : List<T>) -> List<T>{
    Rc::new(Cons(x, tail.clone()))
}

pub fn map<A : 'static, B : 'static>(xs : List<A>, f : impl Fn(&A) -> B) -> List<B>{
    match &*xs{
        Nil => nil(),
        Cons(ref head, tail) => cons(f(head), map(tail.clone(), f))

    }
}

pub fn foldl<A : 'static, B : 'static>(xs : List<A>, b : B, f : impl Fn(&B, &A) -> B) -> B{
    match &*xs{
        Nil => b,
        Cons(ref head, tail) => foldl(tail.clone(), f(&b, head), f)
    }
}