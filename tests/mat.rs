use approx::assert_ulps_eq;
use cgmath;
use glam;
use mathbench::*;
use nalgebra;
use rand::{SeedableRng};
use rand_xoshiro::Xoshiro256Plus;

const NUM_ITERS: usize = 1024;

#[macro_export]
macro_rules! semi_implicit_euler {
    ($delta_secs: expr, $accel: expr, $vel: expr, $pos: expr) => {{
        for ((position, acceleration), velocity) in
            $pos.iter_mut().zip($accel.iter()).zip($vel.iter_mut())
        {
            *velocity += *acceleration * $delta_secs;
            *position += *velocity * $delta_secs;
        }
    }};
}

fn mat2_mul_compare() {
    let mut rng = Xoshiro256Plus::seed_from_u64(rand::random());
    let mint1 = random_mat2(&mut rng);
    let mint2 = random_mat2(&mut rng);

    let glam1: glam::Mat2 = mint1.into();
    let glam2: glam::Mat2 = mint2.into();
    let glam3 = glam1 * glam2;

    let mint1: mint::ColumnMatrix2<f32> = glam1.into();
    let mint2: mint::ColumnMatrix2<f32> = glam2.into();

    let nalg1: nalgebra::Matrix2<f32> = mint1.into();
    let nalg2: nalgebra::Matrix2<f32> = mint2.into();
    // column vector multiplication order is right to left
    let nalg3 = nalg1 * nalg2;

    let cgm1: cgmath::Matrix2<f32> = mint1.into();
    let cgm2: cgmath::Matrix2<f32> = mint2.into();
    let cgm3 = cgm1 * cgm2;

    // use nalgebra as assumed correct answer
    let mint3: mint::ColumnMatrix2<f32> = nalg3.into();

    assert_ulps_eq!(cgm3, mint3.into());
    assert_ulps_eq!(glam3, mint3.into());
}

fn mat3_mul_compare() {
    let mut rng = Xoshiro256Plus::seed_from_u64(rand::random());
    let mint1 = random_mat3(&mut rng);
    let mint2 = random_mat3(&mut rng);

    let glam1: glam::Mat3 = mint1.into();
    let glam2: glam::Mat3 = mint2.into();
    let glam3 = glam1 * glam2;

    let nalg1: nalgebra::Matrix3<f32> = mint1.into();
    let nalg2: nalgebra::Matrix3<f32> = mint2.into();
    // column vector multiplication order is right to left
    let nalg3 = nalg1 * nalg2;

    let cgm1: cgmath::Matrix3<f32> = mint1.into();
    let cgm2: cgmath::Matrix3<f32> = mint2.into();
    let cgm3 = cgm1 * cgm2;

    // use nalgebra as assumed correct answer
    let mint3: mint::ColumnMatrix3<f32> = nalg3.into();

    assert_ulps_eq!(cgm3, mint3.into());
    assert_ulps_eq!(glam3, mint3.into());
}

fn mat4_mul_compare() {
    let mut rng = Xoshiro256Plus::seed_from_u64(rand::random());
    let mint1 = random_mat4(&mut rng);
    let mint2 = random_mat4(&mut rng);

    let glam1: glam::Mat4 = mint1.into();
    let glam2: glam::Mat4 = mint2.into();
    let glam3 = glam1 * glam2;

    let nalg1: nalgebra::Matrix4<f32> = mint1.into();
    let nalg2: nalgebra::Matrix4<f32> = mint2.into();
    // column vector multiplication order is right to left
    let nalg3 = nalg1 * nalg2;

    let cgm1: cgmath::Matrix4<f32> = mint1.into();
    let cgm2: cgmath::Matrix4<f32> = mint2.into();
    let cgm3 = cgm1 * cgm2;

    // use nalgebra as assumed correct answer
    let mint3: mint::ColumnMatrix4<f32> = nalg3.into();

    assert_ulps_eq!(cgm3, mint3.into());
    assert_ulps_eq!(glam3, mint3.into());
}

fn mat2_det_compare() {
    use cgmath::prelude::*;

    let mut rng = Xoshiro256Plus::seed_from_u64(rand::random());
    let mm1 = random_mat2(&mut rng);

    let gm1: glam::Mat2 = mm1.into();
    let gmd = gm1.determinant();


    let nm1: nalgebra::Matrix2<f32> = mm1.into();
    let nmd = nm1.determinant();

    let cm1: cgmath::Matrix2<f32> = mm1.into();
    let cmd = cm1.determinant();

    // use nalgebra as assumed correct answer
    assert_ulps_eq!(cmd, nmd);
    assert_ulps_eq!(gmd, nmd);
}

fn mat2_inv_compare() {
    use cgmath::prelude::*;

    let mut rng = Xoshiro256Plus::seed_from_u64(rand::random());
    let mm1 = random_invertible_mat2(&mut rng);

    let gm1: glam::Mat2 = mm1.into();
    let gmi = gm1.inverse();

    let nm1: nalgebra::Matrix2<f32> = mm1.into();
    let nmi = nm1.try_inverse();
    assert!(nmi.is_some());

    let cm1: cgmath::Matrix2<f32> = mm1.into();
    let cmi = cm1.invert();
    assert!(cmi.is_some());

    // use nalgebra as assumed correct answer
    let mmi: mint::ColumnMatrix2<f32> = nmi.unwrap().into();

    assert_ulps_eq!(cmi.unwrap(), mmi.into());
    assert_ulps_eq!(gmi, mmi.into());
}

fn mat3_inv_compare() {
    use cgmath::prelude::*;

    let mut rng = Xoshiro256Plus::seed_from_u64(rand::random());
    let mm1 = random_invertible_mat3(&mut rng);

    let gm1: glam::Mat3 = mm1.into();
    let gmi = gm1.inverse();

    let nm1: nalgebra::Matrix3<f32> = mm1.into();
    let nmi = nm1.try_inverse();
    assert!(nmi.is_some());

    let cm1: cgmath::Matrix3<f32> = mm1.into();
    let cmi = cm1.invert();
    assert!(cmi.is_some());

    // use nalgebra as assumed correct answer
    let mmi: mint::ColumnMatrix3<f32> = nmi.unwrap().into();

    assert_ulps_eq!(cmi.unwrap(), mmi.into());
    assert_ulps_eq!(gmi, mmi.into());
}

fn mat4_inv_compare() {
    use cgmath::prelude::*;

    let mut rng = Xoshiro256Plus::seed_from_u64(rand::random());
    let mm1 = random_invertible_mat4(&mut rng);

    let gm1: glam::Mat4 = mm1.into();
    let gmi = gm1.inverse();

    let nm1: nalgebra::Matrix4<f32> = mm1.into();
    let nmi = nm1.try_inverse();
    assert!(nmi.is_some());

    let cm1: cgmath::Matrix4<f32> = mm1.into();
    let cmi = cm1.invert();
    assert!(cmi.is_some());

    // use nalgebra as assumed correct answer
    let mmi: mint::ColumnMatrix4<f32> = nmi.unwrap().into();

    assert_ulps_eq!(cmi.unwrap(), mmi.into(), epsilon = 0.0001);
    assert_ulps_eq!(gmi, mmi.into(), epsilon = 0.0001);
}

#[test]
fn test_mat2_mul() {
    for _ in 0..NUM_ITERS {
        mat2_mul_compare();
    }
}

#[test]
fn test_mat3_mul() {
    for _ in 0..NUM_ITERS {
        mat3_mul_compare();
    }
}

#[test]
fn test_mat4_mul() {
    for _ in 0..NUM_ITERS {
        mat4_mul_compare();
    }
}

#[test]
fn test_mat2_det() {
    for _ in 0..NUM_ITERS {
        mat2_det_compare();
    }
}

#[test]
fn test_mat2_inverse() {
    for _ in 0..NUM_ITERS {
        mat2_inv_compare();
    }
}

#[test]
fn test_mat3_inverse() {
    for _ in 0..NUM_ITERS {
        mat3_inv_compare();
    }
}

#[test]
fn test_mat4_inverse() {
    for _ in 0..NUM_ITERS {
        mat4_inv_compare();
    }
}