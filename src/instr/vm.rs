use consts::trig_poly;
use real::Real;

#[derive(Debug)]
pub enum Round {
    Up,
    Down
}

pub trait Vm {
    type Var;
    type Storage;

    // functions that need to be implemented
    fn make_const(&mut self, f64) -> Self::Var;
    fn make_source(&mut self, name: &str) -> Self::Var;
    fn make_sum(&mut self, parts: Vec<Self::Var>) -> Self::Var;
    fn make_product(&mut self, parts: Vec<Self::Var>) -> Self::Var;
    fn store(&mut self, var: &mut Self::Var, uses: usize) -> Self::Storage;
    fn load(&mut self, storage: &Self::Storage) -> Self::Var;
    fn round(&mut self, a: Self::Var, mode: Round) -> Self::Var;
    
    fn copy(&mut self, var: &mut Self::Var) -> Self::Var {
        let s = self.store(var, 1);
        self.load(&s)
    }
    fn mul(&mut self, a: Self::Var, b: Self::Var) -> Self::Var {
        self.make_product(vec![a, b])
    }
    fn add(&mut self, a: Self::Var, b: Self::Var) -> Self::Var {
        self.make_sum(vec![a, b])
    }
    fn sub(&mut self, a: Self::Var, b: Self::Var) -> Self::Var {
        let minus_one = self.make_int(-1);
        let minus_b = self.mul(b, minus_one);
        self.add(a, minus_b)
    }
    /// a * b + c
    fn mul_add(&mut self, a: Self::Var, b: Self::Var, c: Self::Var) -> Self::Var {
        let ab = self.mul(a, b);
        self.add(ab, c)
    }
    /// (a + b) * c
    fn add_mul(&mut self, a: Self::Var, b: Self::Var, c: Self::Var) -> Self::Var {
        let a_plus_b = self.add(a, b);
        self.mul(a_plus_b, c)
    }
    /// split split a = b + c, b: int, 0 <= c < 1 (at least approximatly)
    fn split_to_int_and_fraction(&mut self, mut a: Self::Var) -> (Self::Var, Self::Var) {
        let a_s = self.store(&mut a, 2);
        let a_copy = self.load(&a_s);
        let mut a_floor = self.round(a_copy, Round::Down);
        let floor_copy = self.copy(&mut a_floor);
        let a_copy = self.load(&a_s);
        let a_frac = self.sub(a_copy, floor_copy);
        (a_floor, a_frac)
    }

    /// return c, so that a = b + c and b is an integer < a, 0 <= c < 1
    fn fraction(&mut self, mut a: Self::Var) -> Self::Var {
        let a_copy = self.copy(&mut a);
        let a_floor = self.round(a_copy, Round::Down);
        self.sub(a, a_floor)
    }

    fn make_int(&mut self, i: i64) -> Self::Var {
        self.make_const(i as f64)
    }

    /// x^n
    fn pow_n(&mut self, mut x: Self::Var, mut n: u32) -> Self::Var {
        assert!(n > 0, "attempted to calculate x^0: this is a bug in the optimizer");

        // handle trailing powers (replace x by x²ⁿ)
        for _ in 0 .. n.trailing_zeros() {
            let x2 = self.copy(&mut x);
            x = self.mul(x, x2);
            n /= 2;
        }

        // for powers of two, the computation is complete
        if n == 1 {
            return x;
        }
        
        let mut y = self.copy(&mut x); // holds the power so far
        while n > 1 {
            if n & 1 == 1 {
                let x2 = self.copy(&mut x);
                y = self.mul(y, x2);
            }

            let x2 = self.copy(&mut x);
            x = self.mul(x, x2);
            n /= 2;
        }

        assert_eq!(n, 1);
        self.mul(x, y) // final multiplication
    }

    /// \sum_{i=0}^{n-1} x^{n-i-1} k[i]
    /// (((k[0] * x + k[1]) * x + k[2]) * x + k[3]) ... + k[n-1]
    fn poly(&mut self, k: &[f64], mut x: Self::Var) -> Self::Var {
        match k.len() {
            0 => self.make_int(0),
            1 => self.make_const(k[0]),
            n => {
                let x_s = self.store(&mut x, n - 2);
                let k_0 = self.make_const(k[0]);
                let k_1 = self.make_const(k[1]);
                let mut y = self.mul_add(k_0, x, k_1);
                
                for i in 2 .. n {
                    let x = self.load(&x_s);
                    let k_i = self.make_const(k[i]);
                    y = self.mul_add(y, x, k_i);
                }

                y
            }
        }
    }

    fn sin(&mut self, x: Self::Var) -> Self::Var {
        let pi = f64::PI;
        let two_pi_inv = self.make_const(0.5 / pi);
        let one_half = self.make_const(0.5);
        let y = self.mul_add(x, two_pi_inv, one_half); // y = x / (2 pi) + 1/2 | (2 pi y - pi) = x
        let z = self.fraction(y); // sin(2 pi x) = sin(2 pi x + 2 pi n)

        let minus_one_half = self.make_const(-0.5);
        let mut y = self.add(z, minus_one_half);
        let y2 = self.copy(&mut y);
        
        let k: Vec<_> = trig_poly::SIN_8_PI.iter().enumerate()
            .map(|(i, &p)| p * (2.0 * pi).powi(2 * (8 - i as i32) - 1)) // adjust for the fact that we feed x/(2pi)
            .collect();
        let y_square = self.pow_n(y, 2);
        let p = self.poly(&k, y_square);
        
        self.mul(p, y2)
    }

    fn cos(&mut self, x: Self::Var) -> Self::Var {
        let pi = f64::PI;
        let two_pi_inv = self.make_const(0.5 / pi);
        let one_half = self.make_const(0.5);
        let y = self.mul_add(x, two_pi_inv, one_half); // y = x / (2 pi) + 1/2 | (2 pi y - pi) = x
        let z = self.fraction(y); // cos(2 pi x) = cos(2 pi x + 2 pi n)

        let minus_one_half = self.make_const(-0.5);
        let mut y = self.add(z, minus_one_half);
        
        let k: Vec<_> = trig_poly::COS_8_PI.iter().enumerate()
            .map(|(i, &p)| p * (2.0 * pi).powi(2 * (8 - i as i32) - 2)) // adjust for the fact that we feed x/(2pi)
            .collect();
        let y_square = self.pow_n(y, 2);
        self.poly(&k, y_square)
    }

}
