//! ## Duo-Combinators
//! ### Atop
//! ```
//!       /x:A
//! M1-D2<
//!       \y:B 
//! 
//! M1.AT(D2) (x,y)
//! M1: Fn(C)->D
//! D2: Fn(A,B)->C
//! ```
//!
//! ### Appose
//! ```
//!    /M2-x:A
//! D1<
//!    \M2-y:A
//! 
//! D1.AP(M2) (x,y)
//! D1: Fn(B,B)->C
//! M2: Fn(A)->B
//! ```
//!
//! ### Compose
//! ```
//! M1-M2-y:B
//!
//! M1.CP(M2) (y)
//! M1: Fn(D)->E
//! M2: Fn(B)->C
//! ```
//!
//! ### Hook
//! ```
//!    /x:A
//! D1<
//!    \M2-y:B
//! 
//! D1.HK(M2)(x,y)
//! D1: Fn(A,C)->D
//! M2: Fn(B)->C
//! ```
//!
//! ### MonoHook
//! ```
//!    (y)
//!    / \
//! D1<   \
//!    \M2-y:B
//! 
//! D1.MH(M2)(y)
//! D1: Fn(B,C)->D
//! M2: Fn(B)->C
//! ```
//! ### RevHook
//! ```
//!     (y)
//!    /   \ /M-x:A
//! D1<     X
//!    \   / \-y:B
//!  (M2(x))
//! 
//! D1.RH(M2)(x,y)
//! D1: Fn(B,C)->D
//! M2: Fn(A)->C
//! ```
//! ## Mono-Combinator
//! ### Relflex
//! ```
//!    (y)
//!    / \
//! D1<   >y:B
//!    \ /
//!    (y)
//! 
//! D1.RF()(y)
//! D1 : (B,B)->C
//! ```
//!
//! ### Flip
//! ```
//!    (y)
//!    / \ /x:A
//! D1<   X
//!    \ / \y:B
//!    (x)
//! 
//! D1.FL()(x,y)
//! D1 : Fn(B,A)->C
//! ```




pub trait MonoAtop<'a,A,B,C,D> where Self:Sized{
    #![allow(non_snake_case)]
    /// ### Atop
    /// ```
    /// M1.AT(D2)(x,y)
    /// 
    ///       /x
    /// M1-D2<
    ///       \y
    /// ``` 
    fn AT<Duo>(self,d: Duo)-> Box<dyn Fn(A,B)->D + 'a> 
    where Self: Fn(C)->D +'a , Duo:Fn(A,B)->C +'a 
    { Box::new(move|x,y|self(d(x,y))) }
}
impl<'a,A,B,C,D,Fun> MonoAtop<'a,A,B,C,D> for Fun where Fun: Fn(C)->D {}
pub trait MonoComposition<'a,B,C,D> where Self: Sized{
    #![allow(non_snake_case)] 
    /// ### Compose
    /// ```
    /// M1.CP(M2)(y)
    /// 
    /// M1-M2-y
    /// ```
    fn CP<Mono2>(self, m2: Mono2)->Box<dyn Fn(B)->D +'a>
    where Self: Fn(C)->D + 'a, Mono2: Fn(B)->C + 'a
    { Box::new(move|y|self(m2(y))) }
}
impl<'a,B,C,D,Fun> MonoComposition<'a,B,C,D> for Fun where Fun: Fn(C)->D{}

pub trait DuoComposition<'a,A,B,C,D> where Self:Sized{
    #![allow(non_snake_case)]

    /// ### Hook
    /// ```
    /// D1.HK(M2)(x,y)
    /// 
    ///    /x
    /// D1<
    ///    \M2-y
    /// ``` 
    fn HK<Mono>(self,m:Mono)-> Box<dyn Fn(B,A)->D + 'a>
    where Self: Fn(B,C)->D + 'a, Mono: Fn(A)->C +'a
    { Box::new(move|x,y|self(x,m(y))) }

    /// ### RevHook
    /// ```
    /// D1.HK(M2)(x,y)
    /// 
    ///    (y)
    ///    /  \ /M2-x
    /// D1<    X
    ///    \  / \y
    ///  (M2(x)) 
    /// ```
    fn RH<Mono>(self, m:Mono)-> Box<dyn Fn(A,B)->D + 'a>
    where Self: Fn(B,C)->D + 'a, Mono: Fn(A)->C + 'a
    { Box::new(move|x,y|self(y,m(x))) }
}
impl<'a,A,B,C,D,Fun> DuoComposition<'a,A,B,C,D> for Fun where Fun: Fn(B,C)->D{}

pub trait DuoFlipMHook<'a,B,C,D> where Self:Sized{
    #![allow(non_snake_case)]
    /// ### Flip
    /// ```
    /// D1.F()(x,y)
    /// 
    ///    (y)
    ///    / \ /x
    /// D1<  X
    ///    \ / \y
    ///    (x)
    /// ```
    fn FL(self)-> Box<dyn Fn(C,B)->D +'a>
    where Self: Fn(B,C)->D + 'a
    { Box::new(move|x,y|self(y,x)) }
    
    /// ### MonoHook
    /// ```
    /// D1.MH(M2)(y)
    /// 
    ///    (Y)
    ///    / \
    /// D1<   \
    ///    \M2-y
    /// ```
    fn MH<Mono>(self, m: Mono)-> Box<dyn Fn(B)->D +'a>
    where Self: Fn(B,C)->D + 'a, Mono: Fn(B)->C +'a ,B:Clone
    { Box::new(move|y|self(y.clone(),m(y))) }
        
}
impl<'a,B,C,D,Fun> DuoFlipMHook<'a,B,C,D> for Fun where Fun: Fn(B,C)->D{}

pub trait DuoAppose<'a,A,B,C> where Self:Sized{
    #![allow(non_snake_case)]
    /// ### Appose
    /// ```
    /// D1.AP(M2)(x,y)
    /// 
    ///    /M2-x:T
    /// D1<
    ///    \M2-y:T
    /// ```
    fn AP<Mono>(self,m:Mono)-> Box<dyn Fn(A,A)->C + 'a>
    where Self: Fn(B,B)->C +'a, Mono: Fn(A)->B + 'a
    { Box::new(move|x,y|self( m(x), m(y) ))}
}    
impl<'a,A,B,C,Fun> DuoAppose<'a,A,B,C> for Fun where Fun: Fn(B,B)->C{}


pub trait DuoReflex<'a,B,C> where Self:Sized{
    #![allow(non_snake_case)]
    /// reflex
    /// ```
    /// D1.RF()(y)
    /// 
    ///    (y)
    ///    / \
    /// D1<   >y
    ///    \ /
    ///    (y)
    /// ```
    fn RF(self)-> Box<dyn Fn(B)->C + 'a>
    where Self: Fn(B,B)->C + 'a, B:Clone
    { Box::new(move|y| self(y.clone(),y) ) }    
}    
impl<'a,B,C,Fun> DuoReflex<'a,B,C> for Fun where Fun: Fn(B,B)->C{}


/// no overhead function composition
pub fn atop<A,B,C,D,Mono,Duo>(m: Mono,d: Duo)-> impl Fn(A,B)->D 
where Mono: Fn(C)->D, Duo:Fn(A,B)->C 
{ move |x,y|m(d(x,y)) }

pub fn appose<A,B,C,Duo,Mono>(d:Duo,m:Mono)-> impl Fn(A,A)->C
where Duo: Fn(B,B)->C, Mono: Fn(A)->B
{ move|x,y|d( m(x), m(y) ) }

pub fn compose<B,C,D,Mono1,Mono2>(m1: Mono1, m2: Mono2)-> impl Fn(B)->D
where Mono1: Fn(C)->D, Mono2: Fn(B)->C
{ move|y|m1(m2(y)) }

pub fn hook<A,B,C,D,Duo,Mono>(d:Duo,m:Mono)-> impl Fn(A,B)->D
where Duo: Fn(A,C)->D, Mono: Fn(B)->C
{ move|x,y|d(x,m(y)) }

pub fn monohook<B,C,D,Duo,Mono>(d: Duo, m: Mono)-> impl Fn(B)->D 
where Duo: Fn(B,C)->D, Mono: Fn(B)->C, B:Clone
{ move|y|d(y.clone(),m(y)) }

pub fn revhook<A,B,C,D,Duo,Mono>(d:Duo, m:Mono)-> impl Fn(A,B)->D + 
where Duo: Fn(B,C)->D, Mono: Fn(A)->C
{ move|x,y|d(y,m(x)) }

pub fn reflex<B,C,Duo>(d:Duo)-> impl Fn(B)->C
where Duo: Fn(B,B)->C, B:Clone
{ move|y| d(y.clone(),y) }

pub fn flip<A,B,C,Duo>(d: Duo)-> impl Fn(B,A)->C 
where Duo: Fn(A,B)->C
{ move|x,y|d(y,x) }


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

// implementation references ahead

// fn atop<'a,A,B,C,D,Mono,Duo>(m: Mono,d: Duo)-> Box<dyn Fn(A,B)->D + 'a> 
// where Mono: Fn(C)->D +'a , Duo:Fn(A,B)->C +'a 
// { Box::new(move |x,y|m(d(x,y))) }

// fn appose<'a,A,B,C,Duo,Mono>(d:Duo,m:Mono)-> Box<dyn Fn(A,A)->C + 'a>
// where Duo: Fn(B,B)->C +'a, Mono: Fn(A)->B + 'a
// { Box::new(move|x,y|d( m(x), m(y) ))}

// fn compose<'a,B,C,D,Mono1,Mono2>(m1: Mono1, m2: Mono2)->Box<dyn Fn(B)->D +'a>
// where Mono1: Fn(C)->D + 'a, Mono2: Fn(B)->C + 'a
// { Box::new(move|y|m1(m2(y))) }

// fn hook<'a,A,B,C,D,Duo,Mono>(d:Duo,m:Mono)-> Box<dyn Fn(A,B)->D + 'a>
// where Duo: Fn(A,C)->D + 'a, Mono: Fn(B)->C +'a
// { Box::new(move|x,y|d(x,m(y))) }

// fn monohook<'a,B,C,D,Duo,Mono>(d: Duo, m: Mono)-> Box<dyn Fn(B)->D +'a>
// where Duo: Fn(B,C)->D + 'a, Mono: Fn(B)->C +'a ,B:Clone
// { Box::new(move|y|d(y.clone(),m(y))) }

// fn revhook<'a,A,B,C,D,Duo,Mono>(d:Duo, m:Mono)-> Box<dyn Fn(A,B)->D + 'a>
// where Duo: Fn(B,C)->D + 'a, Mono: Fn(A)->C + 'a
// { Box::new(move|x,y|d(y,m(x))) }

// fn reflex<'a,B,C,Duo>(d:Duo)-> Box<dyn Fn(B)->C + 'a>
// where Duo: Fn(B,B)->C + 'a, B:Clone
// { Box::new(move|y| d(y.clone(),y) ) }

// fn flip<'a,A,B,C,Duo>(d: Duo)-> Box<dyn Fn(B,A)->C +'a>
// where Duo: Fn(A,B)->C + 'a
// { Box::new(move|x,y|d(y,x)) }
