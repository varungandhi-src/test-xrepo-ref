pub trait T {}

pub struct S {}

impl S {
  pub fn f<A>(&mut self, a: A) where A: T {}
}
