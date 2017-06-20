/*
     p_0 (x) = sqrt(b-a)
\hat p_1 (x) = x - ⟨1, 1⟩ 1
             = x - (b-a)
     p_1 (x) = \int_a^b x^2 - 2 x (b-a) + (b-a)^2 dx
             = [ x^3 / 3 - x^2 (b-a) + x (b-a) ]_a^b
             = (b^3 - 4 a^3) / 3 + (b a^2 - a b^2) (b-a) + b^2 + a^2 - 2 a b
             
\int_a^b sin(x) x dx
*/

use optimization::{Minimizer, GradientDescent, NumericalDifferentiation, Func};
