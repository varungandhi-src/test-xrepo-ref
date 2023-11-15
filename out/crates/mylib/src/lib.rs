  pub trait T {}
//          ^ definition rust-analyzer cargo mylib 0.1.0 T#
  
  pub struct S {}
//           ^ definition rust-analyzer cargo mylib 0.1.0 S#
  
  impl S {
//     ^ reference rust-analyzer cargo mylib 0.1.0 S#
    pub fn f<A>(&mut self, a: A) where A: T {}
//         ^ definition rust-analyzer cargo mylib 0.1.0 S#f().
//           ^ definition local 0
//           documentation ```rust
//                   ^^^^ definition rust-analyzer cargo mylib 0.1.0 (self)
//                   documentation ```rust
//                         ^ definition rust-analyzer cargo mylib 0.1.0 (a)
//                         documentation ```rust
//                            ^ reference local 0
//                                     ^ reference local 0
//                                        ^ reference rust-analyzer cargo mylib 0.1.0 T#
  }
  
