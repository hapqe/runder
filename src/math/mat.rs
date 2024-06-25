use super::{transform::Transform, vec::Vec4};

#[derive(Clone, Copy, Debug)]
pub struct Mat4 {
    pub x: Vec4,
    pub y: Vec4,
    pub z: Vec4,
    pub w: Vec4,
}

impl Default for Mat4 {
    fn default() -> Self {
        Self {
            x: Vec4 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            y: Vec4 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
                w: 0.0,
            },
            z: Vec4 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
                w: 0.0,
            },
            w: Vec4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
        }
    }
}

impl Mat4 {
    /// rotating a matrix, according to Tait-Bryan angles in Z -> Y -> X order in radiants
    /// <br>
    /// Reference: https://en.wikipedia.org/wiki/Euler_angles#Rotation_matrix
    pub fn transform(t: &Transform) -> Self {
        let Transform {
            position,
            rotation,
            scale,
        } = t;

        let yaw = rotation.x;
        let pitch = rotation.y;
        let roll = rotation.z;

        let cos_yaw = yaw.cos();
        let sin_yaw = yaw.sin();
        let cos_pitch = pitch.cos();
        let sin_pitch = pitch.sin();
        let cos_roll = roll.cos();
        let sin_roll = roll.sin();

        Self {
            x: Vec4 {
                x: cos_yaw * cos_pitch * scale.x,
                y: (cos_yaw * sin_pitch * sin_roll - sin_yaw * cos_roll) * scale.y,
                z: (cos_yaw * sin_pitch * cos_roll + sin_yaw * sin_roll) * scale.z,
                w: 0.0,
            },
            y: Vec4 {
                x: sin_yaw * cos_pitch * scale.x,
                y: (sin_yaw * sin_pitch * sin_roll + cos_yaw * cos_roll) * scale.y,
                z: (sin_yaw * sin_pitch * cos_roll - cos_yaw * sin_roll) * scale.z,
                w: 0.0,
            },
            z: Vec4 {
                x: -sin_pitch * scale.x,
                y: (cos_pitch * sin_roll) * scale.y,
                z: (cos_pitch * cos_roll) * scale.z,
                w: 0.0,
            },
            w: Vec4 {
                x: position.x,
                y: position.y,
                z: position.z,
                w: 1.0,
            },
        }
    }

    pub fn data(self) -> Vec<u8> {
        let floats = [
            self.x.x, self.x.y, self.x.z, self.x.w, self.y.x, self.y.y, self.y.z, self.y.w,
            self.z.x, self.z.y, self.z.z, self.z.w, self.w.x, self.w.y, self.w.z, self.w.w,
        ];

        /// Resource: https://users.rust-lang.org/t/vec-f32-to-u8/21522/10
        fn vf_to_u8(v: &[f32]) -> &[u8] {
            unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
        }

        vf_to_u8(&floats).to_vec()
    }
}
