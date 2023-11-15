  use mylib::*;
//    ^^^^^ reference local 0
  
  struct TI {}
//       ^^ definition rust-analyzer cargo test-xrepo-ref 0.1.0 TI#
//       documentation ```rust
  
  impl T for TI {}
//     ^ reference rust-analyzer cargo mylib 0.1.0 T#
//           ^^ reference rust-analyzer cargo test-xrepo-ref 0.1.0 TI#
  
  fn main() {
//   ^^^^ definition rust-analyzer cargo test-xrepo-ref 0.1.0 main().
//   documentation ```rust
      S{}.f(TI{});
//    ^ reference rust-analyzer cargo mylib 0.1.0 S#
//        ^ reference rust-analyzer cargo mylib 0.1.0 S#f().
//          ^^ reference rust-analyzer cargo test-xrepo-ref 0.1.0 TI#
      println!("Hello, world!");
  }
  
