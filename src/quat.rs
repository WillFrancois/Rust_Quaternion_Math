use std::ops;

#[derive(Debug)]
pub struct Quaternion {
    r: f64,
    i: f64,
    j: f64,
    k: f64,
}

impl Quaternion {
    pub fn new(qr: f64, qi: f64, qj: f64, qk: f64) -> Quaternion {
        Quaternion {
            r: qr,
            i: qi,
            j: qj,
            k: qk,
        }
    }
}

impl ops::Add<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn add(self, _rhs: Quaternion) -> Quaternion {
        return Quaternion {
            r: self.r + _rhs.r,
            i: self.i + _rhs.i,
            j: self.j + _rhs.j,
            k: self.k + _rhs.k,
        };
    }
}

impl ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, _rhs: Quaternion) -> Quaternion {
        return Quaternion {
            r: (self.r * _rhs.r - self.i * _rhs.i - self.j * _rhs.j - self.k * _rhs.k),
            i: (self.r * _rhs.i + self.i * _rhs.r + self.j * _rhs.k - self.k * _rhs.j),
            j: (self.r * _rhs.j - self.i * _rhs.k + self.j * _rhs.r + self.k * _rhs.i),
            k: (self.r * _rhs.k + self.i * _rhs.j - self.j * _rhs.i + self.k * _rhs.r),
        };
    }
}

pub fn quat_rotate(q_1: Quaternion, point: Quaternion) -> Quaternion {
    let inv_q = Quaternion {
        r: q_1.r,
        i: -q_1.i,
        j: -q_1.j,
        k: -q_1.k,
    };
    let inter_quat = q_1 * point;
    inter_quat * inv_q
}

pub fn quat_rotate_by_angle(
    i_influence: f32,
    j_influence: f32,
    k_influence: f32,
    point: Quaternion,
    mut angle: f32,
) -> Quaternion {
    angle = angle.to_radians();

    let mut new_quat = Quaternion::new(
        angle.cos() as f64,
        (i_influence.sqrt() * angle.sin()) as f64,
        (j_influence.sqrt() * angle.sin()) as f64,
        (k_influence.sqrt() * angle.sin()) as f64,
    );
    if i_influence + j_influence + k_influence > 1.0 {
        println!(
            "Cannot take influences that add to larger than 1.0! Opting for identity quaternion."
        );
        new_quat = Quaternion::new(1.0, 0.0, 0.0, 0.0);
    }
    quat_rotate(new_quat, point)
}
