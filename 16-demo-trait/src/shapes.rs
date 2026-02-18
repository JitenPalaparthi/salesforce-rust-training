
pub trait IShape{
     fn area(&self)->f64;
     fn perimeter(&self)->f64;
}

// impl and dyn
// static dispatch and dynamic dispatch
// bit of generics

#[derive(Debug,Copy,Clone)]
pub struct Square(f32);

impl Square
{
   pub fn new(s : f32)->Self{
        Self(s)
    }
}

 impl IShape for Square{
      fn area(&self)->f64{
        (self.0 * self.0) as f64
      }
       fn perimeter(&self)->f64{
        (self.0 *4.0) as f64
       }
}

#[derive(Debug,Copy,Clone)]
pub struct Rect {
    l: f32,
    b: f32
}

impl Rect {
    pub fn new(l: f32, b: f32) -> Self {
        Rect {
            l,
            b,
        }
    }

   pub fn defaults() -> Self {
        Rect {
            l: 0.0,
            b: 0.0,
        }
    }
}

impl IShape for Rect{
 fn area(&self) -> f64 {
        (self.l * self.b) as f64
    }

    fn perimeter(&self) -> f64 {
        return  2.0 * (self.l + self.b) as f64;
    }
}