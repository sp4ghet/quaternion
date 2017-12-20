extern crate quaternion;

use quaternion::Quaternion;

fn main() {

    let q0 = Quaternion::factory(0.9659258, 0.1929123, 0.07716493, 0.1543299);
    let q1 = Quaternion::factory(0.5, 0.3633762, 0.4360515, 0.6540772);

    let p = Quaternion::factory(0.8676022, 0.2722757, 0.2148108, 0.3543367);
    let v : quaternion::Vector3 = quaternion::Vector3::factory(1., 1., 1.);

    println!("q0:\n{:?}", q0);
    println!("q1:\n{:?}", q1);
    println!("");

    println!("(q0*q1).conjugate():\n{:?}", (q0 * q1).conjugate());
    println!("");

    println!("q1.conjugate() * q0.conjugate():\n{:?}", q1.conjugate() * q0.conjugate());
    println!("");

    println!("q1*q1.inverse():\n{:?}", q1 * q1.inverse());
    println!("");

    println!("slerp {:?}", Quaternion::slerp(q0, q1, 0.34));
}
