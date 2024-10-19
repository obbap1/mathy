# Mathy
Mathy is a proc macro that adds math operations for all the non-float integers on your struct. 

Eg.
```rust
extern crate mathy;
use mathy::Mathy;

#[derive(Mathy)]
struct Goals {
    messi: usize,
    ronaldo: usize,
    beckham: usize,
    anthony: usize
}

let mut goals = Goals {messi: 10, ronaldo: 5, beckham: 4, anthony: 0}

goals.total() // 19
goals.min() // 0
goals.max() // 10
goals.sub() // 1
goals.mul() // 200
```