use wasm_bindgen::prelude::*;

use crate::geom::types::{Point, Vector};
use crate::geom::types::{PointActions, PointContainer};
use rstar::{RTreeObject, AABB};

// A very small number to be used in calculations to avoid some
// issues that come up with numerical stability. Set experimentally
// to the lowest order of magnitude where the issues disappear.
const EPSILON: f64 = 1e-14;

#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy)]
pub struct LineSegment {
    #[wasm_bindgen(skip)]
    pub c1: Point,
    #[wasm_bindgen(skip)]
    pub c2: Point,
}

impl PointContainer for LineSegment {
    fn apply(self, lambda: &dyn Fn(Point) -> Point) -> Self {
        LineSegment {
            c1: self.c1.apply(lambda),
            c2: self.c1.apply(lambda),
        }
    }
}

impl std::fmt::Debug for LineSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Line(({}, {}), ({}, {}))",
            self.c1.inner.x, self.c1.inner.y, self.c2.inner.x, self.c2.inner.y
        )
    }
}

impl Eq for LineSegment {}

impl PartialOrd for LineSegment {
    fn partial_cmp(&self, other: &LineSegment) -> Option<std::cmp::Ordering> {
        (
            self.c1.inner.x,
            self.c1.inner.y,
            self.c2.inner.x,
            self.c2.inner.y,
        )
            .partial_cmp(&(
                other.c1.inner.x,
                other.c1.inner.y,
                other.c2.inner.x,
                other.c2.inner.y,
            ))
    }
}

impl Ord for LineSegment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl RTreeObject for LineSegment {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(
            [self.c1.inner.x, self.c1.inner.y],
            [self.c2.inner.x, self.c2.inner.y],
        )
    }
}

impl LineSegment {
    pub fn new(c1: Point, c2: Point) -> LineSegment {
        LineSegment { c1, c2 }
    }

    pub fn reverse(&self) -> LineSegment {
        LineSegment {
            c1: self.c2,
            c2: self.c1,
        }
    }

    pub fn vector(&self) -> Vector {
        Vector {
            inner: self.c2.inner - self.c1.inner,
        }
    }

    pub fn intersect_segment(&self, other: &LineSegment) -> Option<(f64, bool)> {
        // If we were to extend this line out to infinity, would the other
        // line segment intersect it? If so, return the fraction along our
        // segment at which the intersection occurs and the direction.

        let v = self.vector().inner;

        // The algorithm assumes that the current line has finite slope (i.e. that
        // it is neither vertical nor a point), so we handle a few special cases:
        // - If it's a point, we treat it as if it doesn't intersect anything, so
        //   we return None.
        // - If it's a vertical line, we flip the coordinate space by swapping x and
        //   y and call ourselves recursively. If an intersection is found, we have
        //   to be careful to flip the direction as well.
        // For numerical stability, we actually do the flip if the line is not quite
        // vertical but close enough that floating point errors may cause issues.
        if v.y == 0. {
            if v.x == 0. {
                // This line segment is actually a point.
                return None;
            }
        } else {
            if (v.x / v.y).abs() < EPSILON {
                if let Some((frac, direction)) =
                    (*self).xy_flip().intersect_segment(&other.xy_flip())
                {
                    // Direction flips across x/y.
                    return Some((frac, !direction));
                } else {
                    return None;
                }
            }
        }

        let ov = other.vector().inner;

        // If we extend both lines to infinity, there are three possible cases:
        // 1. The lines are parallel and never intersect.
        // 2. The second line crosses the first from the left.
        // 3. The second line crosses the first from the right.
        //
        // Note that these are a function of the directions (vectors) of each line
        // and the actual positions of the lines in space has no bearing on the
        // outcome.
        //
        // This code determines which case we have, by computing the dot-product
        // between the first line's vector and the vector obtained by rotating the
        // second vector a quarter rotation counter-clockwise. If this is positive,
        // the second line crosses the first line from the right (from the first
        // line's vantage point); if it is negative, from the left; and if it is
        // zero, the lines are parallel.
        let direction = {
            let perp_dot = (v.x * ov.y) - (v.y * ov.x);

            if perp_dot == 0. {
                // The lines are parallel.
                return None;
            } else {
                perp_dot > 0.
            }
        };

        if ov.y != 0. && ((ov.x / ov.y).abs() < EPSILON) {
            // If the other line has (near) infinite slope, special case.
            // We know the lines cross at x = other.c1.x, so we just have to
            // find that value along x.
            let frac = (other.c1.inner.x - self.c1.inner.x) / (self.c2.inner.x - self.c1.inner.x);
            let y = self.c1.inner.y + v.y * frac;

            if (y - other.c1.inner.y) * (y - other.c2.inner.y) > 0. {
                None
            } else {
                Some((frac, direction))
            }
        } else {
            // Otherwise, we find the slope of both and subtract them.
            let other_slope = Vector{inner:ov}.slope();
            let net_slope = Vector{inner: v}.slope() - other_slope;
            let y_dist = other.c1.inner.y - self.c1.inner.y
                + (other_slope * (self.c1.inner.x - other.c1.inner.x));
            let x_delta = y_dist / net_slope;

            let frac = x_delta / v.x;
            let x = x_delta + self.c1.inner.x;

            if frac.is_nan() {
                // The lines may intersect but have the same slope.
                return None;
            }

            if (x - other.c1.inner.x) * (x - other.c2.inner.x) > EPSILON {
                None
            } else {
                Some((frac, direction))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect_vertical() {
        let l1 = LineSegment::new(
            Point {
                inner: [0., 5.].into(),
            },
            Point {
                inner: [10., 5.].into(),
            },
        );
        let l2 = LineSegment::new(
            Point {
                inner: [5., 0.].into(),
            },
            Point {
                inner: [5., 10.].into(),
            },
        );

        assert_eq!(Some((0.5, true)), l1.intersect_segment(&l2));
        assert_eq!(
            Some((1.5, true)),
            l1.intersect_segment(&l2.translate(Vector {
                inner: [10., 0.].into()
            }))
        );

        // The other line segment never intersects the current line.
        assert_eq!(
            None,
            l1.intersect_segment(&l2.translate(Vector {
                inner: [0., 10.].into()
            }))
        );
        assert_eq!(
            None,
            l1.intersect_segment(&l2.translate(Vector {
                inner: [10., 10.].into()
            }))
        );
    }

    #[test]
    fn test_intersect_horizontal() {
        let l1 = LineSegment::new(Point { inner: [0., 0.].into() }, Point { inner: [10., 10.].into() });
        let l2 = LineSegment::new(Point { inner: [0., 8.].into() }, Point { inner: [20., 8.].into() });
        assert_eq!(Some((0.8, false)), l1.intersect_segment(&l2));
    }

    #[test]
    fn test_intersect_regular() {
        let l1 = LineSegment::new(Point { inner: [3., 1.].into() }, Point { inner: [13., 6.].into() });
        let l2 = LineSegment::new(Point { inner: [10., 6.].into() }, Point { inner: [14., 2.].into() });
        assert_eq!(Some((0.8, false)), l1.intersect_segment(&l2));
        assert_eq!(
            Some((0.6, false)),
            l1.intersect_segment(&l2.translate(Vector {
                inner: [-2.0, -1.0].into()
            }))
        );
        assert_eq!(
            Some((1.2, false)),
            l1.intersect_segment(&l2.translate(Vector { inner: [4.0, 2.0].into() }))
        );
        assert_eq!(
            Some((-0.2, false)),
            l1.intersect_segment(&l2.translate(Vector {
                inner: [-10.0, -5.0].into()
            }))
        );

        assert_eq!(
            None,
            l1.intersect_segment(&l2.translate(Vector {
                inner: [-20.0, -5.0].into()
            }))
        );
    }

    #[test]
    fn test_intersect_direction() {
        let l1 = LineSegment::new(Point { inner: [3., 1.].into() }, Point { inner: [13., 6.].into() });
        let l2 = LineSegment::new(Point { inner: [10., 6.].into() }, Point { inner: [14., 2.].into() });
        assert_eq!(Some((0.8, false)), l1.intersect_segment(&l2));
        assert_eq!(Some((0.8, true)), l1.intersect_segment(&l2.reverse()));
        assert_eq!(Some((0.2, true)), l1.reverse().intersect_segment(&l2));
        assert_eq!(
            Some((0.2, false)),
            l1.reverse().intersect_segment(&l2.reverse())
        );
    }

    #[test]
    fn test_parallel_lines() {
        let l1 = LineSegment::new(
            Point {
                inner: [84.03137539808972, -50.69242864951529].into(),
            },
            Point {
                inner: [54.73012545327112, 11.113630310194111].into(),
            },
        );
        let l2 = LineSegment::new(
            Point {
                inner: [84.03137539808972, -50.69242864951529].into(),
            },
            Point {
                inner: [38.15776122775803, 46.07024555196011].into(),
            },
        );
        assert_eq!(None, l1.intersect_segment(&l2));
        assert_eq!(None, l1.intersect_segment(&l2.reverse()));
    }

    #[test]
    fn test_near_infinite_slope() {
        let l1 = LineSegment::new(
            Point {
                inner: [-0.000000000000000040274189285034336, 10.].into(),
            },
            Point {
                inner: [-0.00000000000000003900535185736784, 0.].into(),
            },
        );

        let l2 = LineSegment::new(
            Point {
                inner: [-10., 6.].into(),
            },
            Point {
                inner: [10., 6.].into(),
            },
        );

        assert_eq!(Some((0.4, true)), l1.intersect_segment(&l2));

        assert_eq!(Some((0.5, false)), l2.intersect_segment(&l1));
    }
}
