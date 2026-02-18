use std::ops::{Add,Sub,AddAssign};

use std::{fmt, mem, ops::Not};
fn main() {
    let mut r1 = Rect::new(10.9, 14.0);
    let r2 = Rect::new(11.4, 12.9);

    let r3 = r1 + r2;
    let r4 = r1.add(r2);
    let r5 = r2-r1;
    let r6 = r2.sub(r1);

    r1 = r1+r2;

    r1= r1.add(r2);
    //r1+=r2; AddAssign


}

struct Rect {
    l: f32,
    b: f32,
    a: f64,
    p: f64,
}

impl Rect {
    fn new(l: f32, b: f32) -> Self {
        Rect {
            l,
            b,
            a: 0.0,
            p: 0.0,
        }
    }

    fn defaults() -> Self {
        Rect {
            l: 0.0,
            b: 0.0,
            a: 0.0,
            p: 0.0,
        }
    }

    fn area(&mut self) -> f64 {
        self.a = (self.l * self.b) as f64;
        return self.a;
    }

    fn perimeter(&mut self) -> f64 {
        self.p = 2.0 * (self.l + self.b) as f64;
        return self.p;
    }
}

impl Add for Rect {
    type Output = Rect; // asserted type , that is going to be the return type
    fn add(self, rhs: Rect) -> Self::Output {
        // Output asserted type
        Rect {
            l: self.l + rhs.l,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
            p: self.p + rhs.p,
        }
    }
}


impl Sub for Rect {
    type Output = Rect; // asserted type , that is going to be the return type
    fn sub(self, rhs: Rect) -> Self::Output {
        // Output asserted type
        Rect {
            l: self.l - rhs.l,
            b: self.b - rhs.b,
            a: self.a - rhs.a,
            p: self.p - rhs.p,
        }
    }
}

impl std::fmt::Display for Rect {
    //   use std::fmt::Result;
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(l:{}, b:{} a:{:.2} p:{:.2})",
            self.l, self.b, self.a, self.p
        )
    }
}

impl Clone for Rect {
    fn clone(&self) -> Self {
        Self {
            l: self.l,
            b: self.b,
            a: self.a,
            p: self.p,
        }
    }
}

impl Copy for Rect {} // trait bound for this
