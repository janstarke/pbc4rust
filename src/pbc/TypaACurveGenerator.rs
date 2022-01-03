use gmp::mpz::{Mpz, ProbabPrimeResult};
use gmp::rand::RandState;
use rand::*;
use std::cell::RefCell;

pub enum CurveType {
    A
}

pub trait NextInteger<R> {
    fn next_integer(&self, max: &R) -> R;
}

pub struct CurveOptions {
    pub curve_type: CurveType,
    pub q: Mpz,
    pub h: Mpz,
    pub r: Mpz,

    pub exp1: u32,
    pub exp2: u32,
    pub sign0: i32,
    pub sign1: i32
}

pub struct TypeACurveGenerator {
    rbits: u32,
    qbits: u32,
    rng: RefCell<rand::prelude::ThreadRng>,
    randstate: RefCell<RandState>,
}

impl TypeACurveGenerator {
    pub fn new(rbits: u32, qbits: u32) -> Self {
        let mut rng1 = rand::thread_rng();
        let mut randstate = RandState::new();
        randstate.seed(Mpz::from(rng1.next_u64()));

        Self {
            rbits, qbits,
            rng: RefCell::new(rng1),
            randstate: RefCell::new(randstate)
        }
    }

    pub fn generate(&self) -> CurveOptions {
        let mut exp1 = 0;
        let mut exp2 = 0;
        let mut sign0 = 0;
        let mut sign1 = 0;
        let mut h: Mpz;

        loop {
            // r is picked to be a Solinas prime, that is,
            // r has the form 2a +- 2b +- 1 for some integers 0 < b < a.
            let mut r = Mpz::zero();
            if self.next_integer(&std::u32::MAX) % 2 != 0 {
                exp2 = self.rbits - 1;
                sign1 = 1;
            } else {
                exp2 = self.rbits;
                sign1 = -1;
            }
            r.setbit(exp2 as usize);

            let mut q = Mpz::zero();
            exp1 = (self.next_integer(&std::u32::MAX) % (exp2 - 1)) + 1;
            q.setbit(exp1 as usize);

            if sign1 > 0 {
                r += q;
            } else {
                r -= q;
            }

            if self.next_integer(&std::u32::MAX) % 2 != 0 {
                sign0 = 1;
                r += Mpz::one();
            } else {
                sign0 = -1;
                r -= 1;
            }

            if r.probab_prime(20) != ProbabPrimeResult::Prime {
                continue;
            }

            for i in 1..10 {
                q = Mpz::zero();
                q.setbit(std::cmp::min(3, self.qbits - self.rbits - 4 + 1) as usize);

                h = self.next_integer(&q) * Mpz::from(12);
                q = (h * r) - Mpz::one();
                if q.probab_prime(20) == ProbabPrimeResult::Prime {
                    return CurveOptions {
                        curve_type: CurveType::A,
                        h,
                        q,
                        r,
                        exp1,
                        exp2,
                        sign0,
                        sign1,
                    }
                }
            }
        }
    }
}

impl NextInteger<Mpz> for TypeACurveGenerator {
    fn next_integer(&self, max: &Mpz) -> Mpz {
        self.randstate.borrow_mut().urandom(max)
    }
}
    
impl NextInteger<u32> for TypeACurveGenerator {
    fn next_integer(&self, max: &u32) -> u32 {
        self.rng.borrow_mut().next_u32()%max
    }
}
