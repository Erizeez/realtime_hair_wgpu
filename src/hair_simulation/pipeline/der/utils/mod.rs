use bevy::math::Vec3;

pub fn parallel_transport(former: Vec3, latter: Vec3) -> (Vec3, Vec3) {
    let b = former.cross(latter);
    let n0 = former.cross(b);
    let n1 = latter.cross(b);

    (n0.normalize(), n1.normalize())
}
