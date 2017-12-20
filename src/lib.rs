#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr) => ({
            let (a, b) = (&$a, &$b);
            let e = 1.0e-6;
            assert!((a.w - b.w).abs() < e
                    && (a.x - b.x).abs() < e
                    && (a.y - b.y).abs() < e
                    && (a.z - b.z).abs() < e,
                    "{:?} is not approximately equal to {:?}", *a, *b);
        })
    }

    #[test]
    fn it_can_be_created() {
        let q0 = Quaternion::factory(0.2, 0.1, 0.2, 1.0);
    }

    #[test]
    fn it_is_equatable(){
        let q0 = Quaternion::factory(0.2, 0.1, 0.2, 1.0);
        let q1 = Quaternion::factory(0.2, 0.1, 0.2, 1.0);

        assert_approx_eq!(q0, q1);
    }

    #[test]
    fn it_is_createable_from_angle_axis(){
        let q0 = Quaternion::factory(0.9659258, 0.1929123, 0.07716493, 0.1543299);

        assert_approx_eq!(q0, Quaternion::from_axis_angle(Vector3::factory(10., 4., 8.), 30.));
    }

    #[test]
    fn it_has_a_product(){
        let q0 = Quaternion::factory(0.96592, 0.19291, 0.07716, 0.15432);
        let q1 = Quaternion::factory(0.5, 0.36337, 0.43605, 0.65407);

        let p = Quaternion::factory(0.27828056, 0.43061814, 0.389668, 0.76502013);

        assert_approx_eq!(p, q0*q1);
    }

    #[test]
    fn it_slerps(){
        let q0 = Quaternion::factory(0.9659258, 0.1929123, 0.07716493, 0.1543299);
        let q1 = Quaternion::factory(0.5, 0.3633762, 0.4360515, 0.6540772);

        let p = Quaternion::factory(0.8676022, 0.2722757, 0.2181232, 0.3543367);

        assert_approx_eq!(p, Quaternion::slerp(q0, q1, 0.34));
    }

    #[test]
    fn it_lerps(){
        let q0 = Quaternion::factory(0.9659258, 0.1929123, 0.07716493, 0.1543299);
        let q1 = Quaternion::factory(0.5, 0.3633762, 0.4360515, 0.6540772);

        let p = Quaternion::factory(0.8708531, 0.2705486, 0.2148108, 0.349678);

        assert_approx_eq!(p, Quaternion::lerp(q0, q1, 0.34));
    }
}

use std::ops;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    w: f32,
    x: f32,
    y: f32,
    z: f32
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3{
    x: f32,
    y: f32,
    z: f32
}

impl Quaternion{
    // factory methods
    pub fn factory(w:f32, x: f32, y: f32, z: f32) -> Quaternion {
        return Quaternion{ w, x, y, z };
    }

    pub fn factory_scalar_vector(s: f32, v: Vector3) -> Quaternion {
        Quaternion::factory(s, v.x, v.y, v.z)
    }

    pub fn identity() -> Quaternion{
        Quaternion::factory(1., 0., 0., 0.)
    }

    pub fn from_axis_angle(axis: Vector3, angle: f32) -> Quaternion {
        let radian = angle * PI / 180.;
        let half_angle = radian / 2.;
        let half_sine = half_angle.sin();

        let normalized_axis = axis.normalize();

        Quaternion::factory(
            half_angle.cos(),
            half_sine * normalized_axis.x,
            half_sine * normalized_axis.y,
            half_sine * normalized_axis.z)
    }

    // properties

    pub fn norm(&self) -> f32 {
        (self.w*self.w + self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    pub fn normalized(&self) -> Quaternion {
        let len = self.norm();
        Quaternion::factory(self.w / len, self.x / len, self.y / len, self.z / len)
    }

    pub fn conjugate(&self) -> Quaternion{
        Quaternion::factory(self.w, -self.x, -self.y, -self.z)
    }

    pub fn inverse(self) -> Quaternion {
        if self.norm() == 0. {
            panic!();
        }
        self.conjugate() * (1. / (&self.norm() * &self.norm()))
    }

    // Products

    pub fn mult(&self, a: f32) -> Quaternion {
        Quaternion::factory(self.w * a, self.x * a, self.y * a, self.z * a)
    }

    pub fn product(&self, p: Quaternion) -> Quaternion {
        Quaternion::factory(
            self.w * p.w - self.x * p.x - self.y * p.y - self.z * p.z, //s0*s1 - v0 * v1
            self.w * p.x + p.w * self.x + self.y * p.z - self.z * p.y, //s0*v1 + s1*v0 + v0xv1
            self.w * p.y + p.w * self.y + self.z * p.x - self.x * p.z,
            self.w * p.z + p.w * self.z + self.x * p.y - self.y * p.x
        )
    }

    pub fn dot(&self, p: Quaternion) -> f32 {
        self.w * p.w + self.x * p.x + self.y * p.y + self.z * p.z
    }

    // lerp

    pub fn lerp(from: Quaternion, to: Quaternion, t: f32) -> Quaternion {
        let rotation = if t > 1.{
            to
        } else if t < 0.{
            from
        } else {
            from + (to - from)*t
        };

        rotation.normalized()
    }

    pub fn slerp(from : Quaternion, to : Quaternion, t : f32) -> Quaternion {

        let step = if t > 1. {
                1.
            } else if t < 0. {
                0.
            } else {
                t
            };

        let theta : f32 = from.dot(to).acos();

        if theta.sin() == 0.{
            return Quaternion::lerp(from, to, step);
        }

        let sine : f32 = theta.sin();

        let blended_from = ((1. - step) * theta).sin() / sine;
        let blended_to = (step*theta).sin() / sine;

        (from*blended_from + to*blended_to).normalized()
    }
}

impl Vector3 {

    pub fn factory(x: f32, y: f32, z:f32) -> Vector3 {
        Vector3{x,y,z}
    }

    pub fn rotate(self, q: Quaternion) -> Vector3{
        let mut p = Quaternion::factory_scalar_vector(0., self);
        p = q*p*q.inverse();

        Vector3::factory(p.x, p.y, p.z)
    }

    pub fn normalize(self) -> Vector3{
        let len = (self.x*self.x + self.y*self.y + self.z*self.z).sqrt();

        if len == 0. {
            return Vector3::factory(0., 0., 0.);
        }

        Vector3::factory(self.x / len, self.y / len, self.z / len)
    }
}

// Operator Overrides

impl ops::Mul<f32> for Quaternion{
    type Output = Quaternion;

    fn mul(self, _rhs: f32) -> Quaternion {
        self.mult(_rhs)
    }
}

impl ops::Mul<Quaternion> for Quaternion{
    type Output = Quaternion;

    fn mul(self, _rhs: Quaternion) -> Quaternion{
        self.product(_rhs)
    }
}

impl ops::Mul<Vector3> for Quaternion{
    type Output = Vector3;

    fn mul(self, _rhs: Vector3) -> Vector3{
        _rhs.rotate(self)
    }
}

impl ops::Sub<Quaternion> for Quaternion{
    type Output = Quaternion;

    fn sub(self, _rhs: Quaternion) -> Quaternion{
        Quaternion::factory(self.w - _rhs.w, self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z)
    }
}

impl ops::Add<Quaternion> for Quaternion{
    type Output = Quaternion;

    fn add(self, _rhs: Quaternion) -> Quaternion{
        Quaternion::factory(self.w + _rhs.w, self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z)
    }
}
