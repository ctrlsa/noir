use std::embedded_curve_ops::{EmbeddedCurvePoint, EmbeddedCurveScalar, multi_scalar_mul};

#[test]

fn test_infinite_point() {
    let zero = EmbeddedCurvePoint::point_at_infinity();
    let g1 = EmbeddedCurvePoint::generator();
    let g2 = g1 + g1;

    let s1 = EmbeddedCurveScalar { lo: 1, hi: 0 };
    let a = multi_scalar_mul([g1], [s1]);
    assert(!a.is_infinite);
    assert(g1 + zero == g1);
    assert(g1 - g1 == zero);
    assert(g1 - zero == g1);
    assert(zero + zero == zero);
    assert(multi_scalar_mul([g1], [s1]) == g1);
    assert(multi_scalar_mul([g1, g1], [s1, s1]) == g2);
    assert(
        multi_scalar_mul(
            [g1, zero],
            [EmbeddedCurveScalar { lo: 2, hi: 0 }, EmbeddedCurveScalar { lo: 42, hi: 25 }],
        )
            == g2,
    );
    assert(
        multi_scalar_mul(
            [g1, g1, zero],
            [s1, s1, EmbeddedCurveScalar { lo: 42, hi: 25 }],
        )
            == g2,
    );
}
