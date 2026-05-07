extern "C" {
    fn acos(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn atan(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn cos(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn pow(__x: core::ffi::c_double, __y: core::ffi::c_double) -> core::ffi::c_double;
    fn sqrt(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn fabs(__x: core::ffi::c_double) -> core::ffi::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn SolveCubic(
    mut a: core::ffi::c_double,
    mut b: core::ffi::c_double,
    mut c: core::ffi::c_double,
    mut d: core::ffi::c_double,
    mut solutions: *mut core::ffi::c_int,
    mut x: *mut core::ffi::c_double,
) {
    let solutions = unsafe { &mut *solutions };
    let x: &mut [f64; 3] = unsafe { &mut *(x as *mut [f64; 3]) };

    let a1 = b / a;
    let a2 = c / a;
    let a3 = d / a;
    let q = (a1 * a1 - 3.0 * a2) / 9.0;
    let r = (2.0 * a1 * a1 * a1 - 9.0 * a1 * a2 + 27.0 * a3) / 54.0;
    let r2_q3 = r * r - q * q * q;

    if r2_q3 <= 0.0 {
        *solutions = 3;
        let theta = (r / (q * q * q).sqrt()).acos();
        x[0] = -2.0 * q.sqrt() * theta.cos() / 3.0 - a1 / 3.0;
        x[1] = -2.0 * q.sqrt() * (theta + 2.0 * std::f64::consts::PI).cos() / 3.0 - a1 / 3.0;
        x[2] = -2.0 * q.sqrt() * (theta + 4.0 * std::f64::consts::PI).cos() / 3.0 - a1 / 3.0;
    } else {
        *solutions = 1;
        x[0] = (r2_q3.abs().sqrt() + r) / 3.0;
        x[0] += q / x[0];
        x[0] *= if r < 0.0 { 1.0 } else { -1.0 };
        x[0] -= a1 / 3.0;
    }
}