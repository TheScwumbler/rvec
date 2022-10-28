/* -- Rust Vector Library -- 
 * vector math in rust
 * meant for game development
 * or other math applications
 */

// allowing pass-by-value (#[derice(Clone, Copy)]) messes with dead code warnings
#![allow(dead_code)]

extern crate num_traits;

use std::ops;
use num_traits::cast::AsPrimitive;

// generate new vector
macro_rules! newvec {
    ( $x:expr, $y:expr, $z:expr ) => {
        Vector3{x: $x, y: $y, z: $z}
    };
}

// allow printing and pass-by-value like c structs
#[derive(Debug, Clone, Copy)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32
}

// miscellaneous vector functions
// i.e. no overloading
impl Vector3 {
    // negate every value in the vector
    fn invert(self) -> Vector3 {
        newvec!(-self.x, -self.y, -self.z)
    }

    // alias for invert
    fn negate(self) -> Vector3 {
        self.invert()
    }

    // find length of vector
    fn length(self) -> f32 {
        // this is unnecessarily convoluted but it squares x, y, & z and then square roots it
        f32::powf(self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0), 0.5)
    }

    // normalize vector
    fn norm(self) -> Vector3 {
        // the .as_() is redundant but the return type of vector3::length() might be subject to change
        let l: f32 = self.length().as_();

        newvec!(self.x / l, self.y / l, self.z / l)
    }
}

// add vector
impl ops::Add<Vector3> for Vector3 {
    type Output =  Vector3;

    fn add(self, vec: Vector3) -> Vector3 {
        // add and return
        newvec!(self.x + vec.x, self.y + vec.y, self.z + vec.z)
    }
}

// add scalar
impl<R: AsPrimitive<f32>> ops::Add<R> for Vector3 {
    type Output =  Vector3;

    fn add(self, _s: R) -> Vector3 {
        // convert input to f32
        let s: f32 = _s.as_();

        // add scalar to vector and return
        newvec!(self.x + s, self.y + s, self.z + s)
    }
}

// subtract vector
impl ops::Sub<Vector3> for Vector3 {
    type Output =  Vector3;

    fn sub(self, v: Vector3) -> Vector3 {
        newvec!(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

// subtract scalar
impl<R: AsPrimitive<f32>> ops::Sub<R> for Vector3 {
    type Output =  Vector3;

    fn sub(self, _s: R) -> Vector3 {
        let s: f32 = _s.as_();

        newvec!(self.x - s, self.y - s, self.z - s)
    }
}

/*
// multiply vector
impl ops::Mul<Vector3> for Vector3 {
    type Output =  Vector3;

    fn mul(self, v: Vector3) -> Vector3 {
        // TODO
        // research dot product
    }
}
*/

// multiply scalar
impl<R: AsPrimitive<f32>> ops::Mul<R> for Vector3 {
    type Output =  Vector3;

    fn mul(self, _s: R) -> Vector3 {
        // convert _s to float
        let s: f32 = _s.as_();

        newvec!(self.x * s, self.y * s, self.z * s)
    }
}

/*
// divide vector
impl ops::Div<Vector3> for Vector3 {
    type Output =  Vector3;

    fn div(self, v: Vector3) -> Vector3 {
        // TODO
        // dot product again (???)
    }
}
*/

// divide scalar
impl<R: AsPrimitive<f32>> ops::Div<R> for Vector3 {
    type Output =  Vector3;

    fn div(self, _s: R) -> Vector3 {
        // convert _s to float
        let s: f32 = _s.as_();

        newvec!(self.x / s, self.y / s, self.z / s)
    }
}

fn main() {
    
    let nv3: Vector3 = (newvec!(4.0, 5.0, 6.0) + 3).norm();

    // test operations
    println!("{:?}\nlen: {}", nv3, nv3.length());
}