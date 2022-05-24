use std::{ops::Index, fmt::{Debug, Display}};

macro_rules! call_with_larch {
    ($callback:ident) => { $callback!(larch) };
}

macro_rules! expand_to_larch {
    () => { larch };
}

macro_rules! recognize_tree {
    (larch) => { println!("#1, the Larch.") };
    (redwood) => { println!("#2, the Mighty Redwood.") };
    (fir) => { println!("#3, the Fir.") };
    (chestnut) => { println!("#4, the Horse Chestnut.") };
    (pine) => { println!("#5, the Scots Pine.") };
    ($($other:tt)*) => { println!("No idea wagwan") };
}


macro_rules! count_expr {
    () => {0};
    ($x: expr) => {1};
    ($x:expr, $($exp: expr),*) => {1 + count_expr!($($exp),*)};
}
    macro_rules! fibonnaci {
        ($a: ident [$n: ident]: $sty:ty = $($inits: expr),+ ; ... ; $recur: expr) => {
            {

                const MEM_SIZE: usize = count_expr!($($inits),+);
                struct Recurrence {
                    mem: [$sty; MEM_SIZE],
                    pos: usize,
                }
                struct IndexOffset<'a> {
                    slice: &'a [$sty; MEM_SIZE],
                    offset: usize,
                }
        
                impl<'a> Index<usize> for IndexOffset<'a> {
                    type Output = $sty;
        
                    // #[inline(always)]
                    fn index<'b>(&'b self, index: usize) -> &'b $sty {
                        use std::num::Wrapping;
        
                        let index = Wrapping(index);
                        let offset = Wrapping(self.offset);
                        let window = Wrapping(MEM_SIZE);
        
                        let real_index = index - offset + window;
                        &self.slice[real_index.0]
                    }
                }
        
                impl Iterator for Recurrence {
                    type Item = $sty;
        
                    fn next(&mut self) -> Option<Self::Item> {
                        if self.pos < MEM_SIZE {
                            let next_val = self.mem[self.pos];
                            self.pos += 1;
                            Some(next_val)
                        } else {
                            let next_val = {
                                let $n = self.pos;
                                let $a = IndexOffset { slice: &self.mem, offset: $n };
                                ($recur)
                            };
        
                            {
                                use std::mem::swap;
        
                                let mut swap_tmp = next_val;
                                // Reverse the iterator so it works as exepcted
                                for i in (0..MEM_SIZE).rev() {
                                    swap(&mut swap_tmp, &mut self.mem[i]);
                                }
                            }
        
                            self.pos += 1;
                            Some(next_val)
                        }
                    }
                }
        
                Recurrence {
                    mem: [$($inits),+],
                    pos: 0 
                }
            }
  
        }
    }


    fn main() {

        trait My_Trait {
            fn checking<T>(t1: T) where T:Display ;
        }

        struct Testing {
        
        }

        impl My_Trait for Testing {
            fn checking<T>(t3: T) where T: Debug{
                T + 30;
            }
        }


        recognize_tree!(expand_to_larch!());
        // What this is saying is that we expect the output to be Larch, however
        //You cannot pass a marco info from the expansion of another macro, therefore
        // In this case, expand_to_larch!() is interpreted as a sequence of tokens, hence
        // The final line is matched!!!!
        call_with_larch!(recognize_tree);


        let fib = fibonnaci![a[n]: u64 = 0,1 ; ... ; a[n-1] + a[n-2]];
    
        for e in fib.take(10) { println!("{}", e) }
        macro_rules! find_min {
            // Base case:
            ($x:expr) => ($x);
            // `$x` followed by at least one `$y,`
            ($x:expr, $($y:expr),+) => (
                // Call `find_min!` on the tail `$y`
                std::cmp::min($x, find_min!($($y),+))
            )
        }
            println!("{}", find_min!(1u32));
            println!("{}", find_min!(1u32 + 2, 2u32));
            println!("{}", find_min!(5u32, 2u32 * 3, 4u32, 50, 300));
    
        }
