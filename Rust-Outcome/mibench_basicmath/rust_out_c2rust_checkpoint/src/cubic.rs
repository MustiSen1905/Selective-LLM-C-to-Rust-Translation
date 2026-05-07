use ::f128;
use ::num_traits;
use num_traits::ToPrimitive;
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
    let mut a1: f128::f128 = f128::f128::new(b / a);
    let mut a2: f128::f128 = f128::f128::new(c / a);
    let mut a3: f128::f128 = f128::f128::new(d / a);
    let mut Q: f128::f128 = (a1 * a1 - f128::f128::new(3.0) * a2) / f128::f128::new(9.0);
    let mut R: f128::f128 = (f128::f128::new(2.0) * a1 * a1 * a1
        - f128::f128::new(9.0) * a1 * a2 + f128::f128::new(27.0) * a3)
        / f128::f128::new(54.0);
    let mut R2_Q3: core::ffi::c_double = (R * R - Q * Q * Q).to_f64().unwrap();
    let mut theta: core::ffi::c_double = 0.;
    if R2_Q3 <= 0 as core::ffi::c_int as core::ffi::c_double {
        *solutions = 3 as core::ffi::c_int;
        theta = acos(
            (R / f128::f128::new(sqrt((Q * Q * Q).to_f64().unwrap()))).to_f64().unwrap(),
        );
        *x.offset(0 as core::ffi::c_int as isize) = (f128::f128::new(
            -2.0f64 * sqrt(Q.to_f64().unwrap()) * cos(theta / 3.0f64),
        ) - a1 / f128::f128::new(3.0))
            .to_f64()
            .unwrap();
        *x.offset(1 as core::ffi::c_int as isize) = (f128::f128::new(
            -2.0f64 * sqrt(Q.to_f64().unwrap())
                * cos(
                    (theta
                        + 2.0f64
                            * (4 as core::ffi::c_int as core::ffi::c_double
                                * atan(1 as core::ffi::c_int as core::ffi::c_double)))
                        / 3.0f64,
                ),
        ) - a1 / f128::f128::new(3.0))
            .to_f64()
            .unwrap();
        *x.offset(2 as core::ffi::c_int as isize) = (f128::f128::new(
            -2.0f64 * sqrt(Q.to_f64().unwrap())
                * cos(
                    (theta
                        + 4.0f64
                            * (4 as core::ffi::c_int as core::ffi::c_double
                                * atan(1 as core::ffi::c_int as core::ffi::c_double)))
                        / 3.0f64,
                ),
        ) - a1 / f128::f128::new(3.0))
            .to_f64()
            .unwrap();
    } else {
        *solutions = 1 as core::ffi::c_int;
        *x.offset(0 as core::ffi::c_int as isize) = pow(
            sqrt(R2_Q3) + fabs(R.to_f64().unwrap()),
            1 as core::ffi::c_int as core::ffi::c_double / 3.0f64,
        );
        let ref mut fresh0 = *x.offset(0 as core::ffi::c_int as isize);
        *fresh0 = (f128::f128::from(*fresh0)
            + Q / f128::f128::new(*x.offset(0 as core::ffi::c_int as isize)))
            .to_f64()
            .unwrap();
        *x.offset(0 as core::ffi::c_int as isize)
            *= (if R < f128::f128::new(0.0) {
                1 as core::ffi::c_int
            } else {
                -(1 as core::ffi::c_int)
            }) as core::ffi::c_double;
        let ref mut fresh1 = *x.offset(0 as core::ffi::c_int as isize);
        *fresh1 = (f128::f128::from(*fresh1) - a1 / f128::f128::new(3.0))
            .to_f64()
            .unwrap();
    };
}
