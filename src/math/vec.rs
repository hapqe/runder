#[derive(Clone, Copy, Debug, Default)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    pub fn xyz(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }
    pub fn one() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Into<Vec3> for [f32; 3] {
    fn into(self) -> Vec3 {
        Vec3::new(self[0], self[1], self[2])
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn xyz(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn one() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }
    /// converting raw quaternion data (as stored in a Gltf to a tait-bryan-angle vector (Z -> Y -> X)) because I'm not smart enough for quaternions.
    /// <br>
    /// Reference: http://marc-b-reynolds.github.io/math/2017/04/18/TaitEuler.html#mjx-eqn:cos2_1
    pub fn euler_from_quaternion_data(q: [f32; 4]) -> Vec3 {
        let x = q[0];
        let y = q[1];
        let z = q[2];
        let w = q[3];

        let t0 = (x + z) * (x - z); // x^2 - z^2
        let t1 = (w + y) * (w - y); // w^2 - y^2
        let xx = 0.5 * (t0 + t1); // 1/2 x of x'
        let xy = x * y + w * z; // 1/2 y of x'
        let xz = w * y - x * z; // 1/2 z of x'
        let t = xx * xx + xy * xy; // cos(theta)^2
        let yz = 2.0 * (y * z + w * x); // z of y'

        let mut v = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        v.z = f32::atan2(xy, xx); // yaw   (psi)
        v.y = f32::atan2(xz, t.sqrt()); // pitch (theta)

        if t != 0.0 {
            v.x = f32::atan2(yz, t1 - t0);
        } else {
            v.x = 2.0 * f32::atan2(x, w) - xz.signum() * v.z;
        }

        v
    }
}
