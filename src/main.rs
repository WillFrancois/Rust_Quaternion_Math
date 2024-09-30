use quat::Quaternion;

mod quat;

fn main() {
    let point = Quaternion::new(0.0, 1.0, 0.0, 0.0);

    let quat = quat::quat_rotate_by_angle(0.3, 0.3, 0.3, point, 180.0);

    println!("{:?}", quat);
}
