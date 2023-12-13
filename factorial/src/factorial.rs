use num_bigint::{BigUint, ToBigUint};
use num_traits::One;
use tailcall::tailcall;

pub fn factorial(num: u64) -> BigUint {
    #[tailcall]
    fn factorial_inner(accummulator: BigUint, num: u64) -> BigUint {
        if num > 0 {
            let current: BigUint = num.to_biguint().unwrap();
             factorial_inner(accummulator * current, num - 1)
        } else {
             accummulator
        }
    }
    factorial_inner(One::one(), num)
}
