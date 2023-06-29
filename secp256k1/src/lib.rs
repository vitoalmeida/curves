use curv::{
    arithmetic::{traits::Integer, Converter, Modulo, Samplable},
    BigInt,
};

#[derive(Debug, Clone)]
enum Point {
    Affine(i32, i32),
    AtInfinity,
}

#[derive(Debug, Clone)]
pub struct PointElpCurve {
    point: Point,
}

#[derive(Debug, Clone)]
pub struct EllipticCurve {
    pub a: i32,
    pub b: i32,
    pub p: i32,
    pub generator: PointElpCurve,
    pub order: i32,
}

impl EllipticCurve {
    pub fn new(a: i32, b: i32, p: i32, generator_x: i32, generator_y: i32, order: i32) -> Self {
        let generator = PointElpCurve::new_point(&generator_x, &generator_y);

        // if (BigInt::mod_mul(&BigInt::from(4), &BigInt::mod_pow(&a, 3, &p), &p)) {
        if ((4 * (&a ^ 3)) + 27 * (&b ^ 2)).rem_euclid(p) != 0 {
            EllipticCurve {
                a,
                b,
                p,
                generator,
                order,
            }
        } else {
            panic!("Failed on the discrimant condition")
        }
    }
}

impl PointElpCurve {
    pub fn new_point(x: &i32, y: &i32) -> Self {
        let point = Point::Affine(x.clone(), y.clone());
        PointElpCurve { point }
    }

    pub fn new_point_at_infinity() -> Self {
        PointElpCurve {
            point: Point::AtInfinity,
        }
    }

    // pub fn double(&self) -> Self {
    //     match &self.point {
    //         Point::AtInfinity => PointElpCurve::new_point_at_infinity(),
    //         Point::Affine(x, y) => {
    //             let slope = {
    //                 let numerator = x.mod_mul(&BigInt::from(3), &self.get_curve_order());
    //                 let denominator = y.mod_mul(&BigInt::from(2), &self.get_curve_order());
    //                 denominator
    //                     .mod_inverse(&self.get_curve_order())
    //                     .expect("Inverse not found!")
    //                     .mod_mul(&numerator, &self.get_curve_order())
    //             };

    //             let x_result = {
    //                 let slope_squared = slope.mod_mul(&slope, &self.get_curve_order());
    //                 slope_squared.mod_sub(
    //                     x.mod_mul(&BigInt::from(2), &self.get_curve_order()),
    //                     &self.get_curve_order(),
    //                 )
    //             };

    //             let y_result = {
    //                 let x_minus_x_result = x.mod_sub(&x_result, &self.get_curve_order());
    //                 slope
    //                     .mod_mul(&x_minus_x_result, &self.get_curve_order())
    //                     .mod_sub(y, &self.get_curve_order())
    //             };

    //             PointElpCurve::new_point(&x_result, &y_result)
    //         }
    //     }
    // }

    // pub fn plus(&self, other: &PointElpCurve) -> Self {
    //     match (&self.point, &other.point) {
    //         (Point::AtInfinity, _) => other.clone(),
    //         (_, Point::AtInfinity) => self.clone(),
    //         (Point::Affine(x1, y1), Point::Affine(x2, y2)) => {
    //             if x1 == x2 && y1 != y2 {
    //                 PointElpCurve::new_point_at_infinity()
    //             } else {
    //                 let slope = {
    //                     let numerator = y2.mod_sub(y1, &self.get_curve_order());
    //                     let denominator = x2.mod_sub(x1, &self.get_curve_order());
    //                     denominator
    //                         .mod_inverse(&self.get_curve_order())
    //                         .expect("Inverse not found!")
    //                         .mod_mul(&numerator, &self.get_curve_order())
    //                 };

    //                 let x_result = {
    //                     let slope_squared = slope.mod_mul(&slope, &self.get_curve_order());
    //                     let x_sum = x1.mod_add(x2, &self.get_curve_order());
    //                     x_sum.mod_sub(&slope_squared, &self.get_curve_order())
    //                 };

    //                 let y_result = {
    //                     let x_minus_x_result = x1.mod_sub(&x_result, &self.get_curve_order());
    //                     slope
    //                         .mod_mul(&x_minus_x_result, &self.get_curve_order())
    //                         .mod_sub(y1, &self.get_curve_order())
    //                 };

    //                 PointElpCurve::new_point(&x_result, &y_result)
    //             }
    //         }
    //     }
    // }

    // pub fn multiply(&self, scalar: &BigInt) -> Self {
    //     let mut result = PointElpCurve::new_point_at_infinity();
    //     let mut addend = self.clone();
    //     let bits = scalar.to_bits_be();
    //     for bit in bits {
    //         if bit {
    //             result = result.plus(&addend);
    //         }
    //         addend = addend.double();
    //     }
    //     result
    // }

    // pub fn get_curve_order(&self) -> BigInt {
    //     self.get_elliptic_curve().order.clone()
    // }

    // fn get_elliptic_curve(&self) -> &EllipticCurve {
    //     match &self.point {
    //         Point::AtInfinity => panic!("Invalid operation on Point at Infinity"),
    //         Point::Affine(_, _) => {
    //             let elliptic_curve = &self.get_domain_parameters().elliptic_curve;
    //             match elliptic_curve {
    //                 Some(curve) => curve,
    //                 None => panic!("Elliptic curve not set for Point on Elliptic Curve"),
    //             }
    //         }
    //     }
    // }

    // fn get_domain_parameters(&self) -> &DomainParameters {
    //     let elliptic_curve = &self.get_elliptic_curve();
    //     &elliptic_curve.domain_parameters
    // }
}

// fn generate_private_key(order: &BigInt) -> BigInt {
//     let mut rng = curv::arithmetic::traits::Samplable::new();
//     rng.sample_range(
//         &BigInt::from(1),
//         &BigInt::mod_sub(order, &BigInt::from(1), order),
//     )
// }

// fn generate_public_key(private_key: &BigInt, elliptic_curve: &EllipticCurve) -> PointElpCurve {
//     let generator = &elliptic_curve.generator;
//     generator.multiply(private_key)
// }

// fn generate_signature(
//     message: &str,
//     private_key: &BigInt,
//     elliptic_curve: &EllipticCurve,
// ) -> (BigInt, BigInt) {
//     let generator = &elliptic_curve.generator;
//     let public_key = generate_public_key(private_key, elliptic_curve);

//     let message_hash = BigInt::from_str_radix(&message, 16).unwrap();
//     let nonce = BigInt::from_str_radix(&private_key.to_string(), 16).unwrap();

//     let r = generator.multiply(&nonce).point;
//     let rx = match r {
//         Point::Affine(x, _) => x,
//         _ => panic!("Invalid r"),
//     };

//     let s = BigInt::mod_mul(
//         BigInt::mod_add(
//             &message_hash,
//             BigInt::mod_mul(private_key, &rx, &elliptic_curve.order),
//             &elliptic_curve.order,
//         ),
//         nonce
//             .mod_inverse(&elliptic_curve.order)
//             .expect("Inverse not found!"),
//         &elliptic_curve.order,
//     );

//     (rx, s)
// }
