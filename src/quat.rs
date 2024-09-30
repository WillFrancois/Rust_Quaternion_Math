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

pub fn quat_add(q_1: Quaternion, q_2: Quaternion) -> Quaternion {
    Quaternion {
        r: q_1.r + q_2.r,
        i: q_1.i + q_2.i,
        j: q_1.j + q_2.j,
        k: q_1.k + q_2.k,
    }
}

fn quat_mult(q_1: Quaternion, q_2: Quaternion) -> Quaternion {
    Quaternion {
        r: (q_1.r * q_2.r - q_1.i * q_2.i - q_1.j * q_2.j - q_1.k * q_2.k),
        i: (q_1.r * q_2.i + q_1.i * q_2.r + q_1.j * q_2.k - q_1.k * q_2.j),
        j: (q_1.r * q_2.j - q_1.i * q_2.k + q_1.j * q_2.r + q_1.k * q_2.i),
        k: (q_1.r * q_2.k + q_1.i * q_2.j - q_1.j * q_2.i + q_1.k * q_2.r),
    }
}

pub fn quat_rotate(q_1: Quaternion, point: Quaternion) -> Quaternion {
    let inv_q = Quaternion {
        r: q_1.r,
        i: -q_1.i,
        j: -q_1.j,
        k: -q_1.k,
    };
    let inter_quat = quat_mult(q_1, point);
    quat_mult(inter_quat, inv_q)
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
