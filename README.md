# Quaternion

Implementation of Quaternion in rustlang.
Inspired by the Unity3D Quaternion/Hamilton's Quaternion.
Based on f32 for now, might make it generic in the future.

```rust
use extern crate quaternion;
use quaternion::Quaternion;
use quaternion::Vector3;

let q0 = Quaternion::factory(0.9659258, 0.1929123, 0.07716493, 0.1543299);
let q1 = Quaternion::from_axis_angle(Vector3::factory(10., 4., 8.), 30.);
let id = Quaternion::identity();
// q0 == q1
let v : Vector3 = Vector3::factory(5., 3., 2.);

// quaternion dot product (commutative), is equal to cos(theta/2) of angle between two rotations
let half_cosine: f32 = q0.dot(q1);
q0 * 12.; // quaternion scalar product
q0 * q1; // quaternion quaternion product (non-commutative)
q0 + q1; // addition (commutative)
q0 - q1; // subtraction

Quaternion::lerp(q0, q1, 0.5);
Quaternion::slerp(q0, q1, 0.5);

q0.norm(); // length / norm
q0.normalize(); // make the rotation a unit quaternion
q0.conjugate();
q0.inverse();

v.rotate(q0); // rotate vector3 using quaternion
```
