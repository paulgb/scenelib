//! Basic representation of line segments, which are the most
//! basic unit of a `Scene`. This is also where the heart of
//! the intersection code lives (`LineSegment::intersect_segment`).

use crate::types::{Point, PointActions, PointContainer, Vector, VectorExtension};
use rstar::{RTreeObject, AABB};

// A very small number to be used in calculations to avoid some
// issues that come up with numerical stability. Set experimentally
// to the lowest order of magnitude where the issues disappear.
const EPSILON: f64 = 1e-14;

/// Represents a two dimensional line segment, defined in terms of
/// its endpoints.
#[derive(PartialEq, Clone, Copy)]
pub struct LineSegment {
    /// The first endpoint of the line segment.
    pub c1: Point,
    /// The second endpoint of the line segment.
    pub c2: Point,
    /// The pen to use to draw this line segment.
    pub pen: usize,
}

impl PointContainer for LineSegment {
    fn apply(self, lambda: &dyn Fn(Point) -> Point) -> Self {
        LineSegment {
            c1: self.c1.apply(lambda),
            c2: self.c2.apply(lambda),
            pen: self.pen,
        }
    }
}

impl std::fmt::Debug for LineSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Line(({}, {}), ({}, {}))",
            self.c1.x, self.c1.y, self.c2.x, self.c2.y
        )
    }
}

impl Eq for LineSegment {}

impl PartialOrd for LineSegment {
    fn partial_cmp(&self, other: &LineSegment) -> Option<std::cmp::Ordering> {
        (self.c1.x, self.c1.y, self.c2.x, self.c2.y)
            .partial_cmp(&(other.c1.x, other.c1.y, other.c2.x, other.c2.y))
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
        AABB::from_corners([self.c1.x, self.c1.y], [self.c2.x, self.c2.y])
    }
}

impl LineSegment {
    /// Create a new line segment with the default pen.
    pub fn new(c1: Point, c2: Point) -> LineSegment {
        LineSegment { c1, c2, pen: 0 }
    }

    /// Create a new line segment with a given pen.
    pub fn new_with_pen(c1: Point, c2: Point, pen: usize) -> LineSegment {
        LineSegment { c1, c2, pen }
    }

    #[cfg(test)]
    fn reverse(&self) -> LineSegment {
        LineSegment {
            c1: self.c2,
            c2: self.c1,
            pen: self.pen,
        }
    }

    /// Construct a vector matching the direction and length of the line segment.
    pub fn vector(&self) -> Vector {
        self.c2 - self.c1
    }

    /// Determines whether two line segments intersect, and returns a value accordingly:
    /// - If the line segments intersect, returns the fraction along *this* line segment
    ///   at which they intersect.
    /// - Otherwise, if this line segment *extended to infinity* intersects the other line
    ///   segment, return the location along this line in terms of the length of the line
    ///   segment at which they intersect, i.e. a number less than 0 or greater than 1.
    /// - Otherwise, return `None`.
    pub fn intersect_segment(&self, other: &LineSegment) -> Option<(f64, bool)> {
        let v = self.vector();

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
                if let Some((frac, direction)) = self.xy_flip().intersect_segment(&other.xy_flip())
                {
                    // Direction flips across x/y.
                    return Some((frac, !direction));
                } else {
                    return None;
                }
            }
        }

        let ov = other.vector();

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
            let frac = (other.c1.x - self.c1.x) / (self.c2.x - self.c1.x);
            let y = self.c1.y + v.y * frac;

            if (y - other.c1.y) * (y - other.c2.y) > 0. {
                None
            } else {
                Some((frac, direction))
            }
        } else {
            // Otherwise, we find the slope of both and subtract them.
            let other_slope = ov.slope();
            let net_slope = v.slope() - other_slope;
            let y_dist = other.c1.y - self.c1.y + (other_slope * (self.c1.x - other.c1.x));
            let x_delta = y_dist / net_slope;

            let frac = x_delta / v.x;
            let x = x_delta + self.c1.x;

            if frac.is_nan() {
                // The lines may intersect but have the same slope.
                return None;
            }

            if (x - other.c1.x) * (x - other.c2.x) > EPSILON {
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
    use crate::types::{pt, vec};

    #[test]
    fn test_intersect_vertical() {
        let l1 = LineSegment::new(pt(0., 5.), pt(10., 5.));
        let l2 = LineSegment::new(pt(5., 0.), pt(5., 10.));

        assert_eq!(Some((0.5, true)), l1.intersect_segment(&l2));
        assert_eq!(
            Some((1.5, true)),
            l1.intersect_segment(&l2.translate(vec(10., 0.)))
        );

        // The other line segment never intersects the current line.
        assert_eq!(None, l1.intersect_segment(&l2.translate(vec(0., 10.))));
        assert_eq!(None, l1.intersect_segment(&l2.translate(vec(10., 10.))));
    }

    #[test]
    fn test_intersect_horizontal() {
        let l1 = LineSegment::new(pt(0., 0.), pt(10., 10.));
        let l2 = LineSegment::new(pt(0., 8.), pt(20., 8.));
        assert_eq!(Some((0.8, false)), l1.intersect_segment(&l2));
    }

    #[test]
    fn test_intersect_regular() {
        let l1 = LineSegment::new(pt(3., 1.), pt(13., 6.));
        let l2 = LineSegment::new(pt(10., 6.), pt(14., 2.));
        assert_eq!(Some((0.8, false)), l1.intersect_segment(&l2));
        assert_eq!(
            Some((0.6, false)),
            l1.intersect_segment(&l2.translate(vec(-2.0, -1.0)))
        );
        assert_eq!(
            Some((1.2, false)),
            l1.intersect_segment(&l2.translate(vec(4.0, 2.0)))
        );
        assert_eq!(
            Some((-0.2, false)),
            l1.intersect_segment(&l2.translate(vec(-10.0, -5.0)))
        );

        assert_eq!(None, l1.intersect_segment(&l2.translate(vec(-20.0, -5.0))));
    }

    #[test]
    fn test_intersect_direction() {
        let l1 = LineSegment::new(pt(3., 1.), pt(13., 6.));
        let l2 = LineSegment::new(pt(10., 6.), pt(14., 2.));
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
            pt(84.03137539808972, -50.69242864951529),
            pt(54.73012545327112, 11.113630310194111),
        );
        let l2 = LineSegment::new(
            pt(84.03137539808972, -50.69242864951529),
            pt(38.15776122775803, 46.07024555196011),
        );
        assert_eq!(None, l1.intersect_segment(&l2));
        assert_eq!(None, l1.intersect_segment(&l2.reverse()));
    }

    #[test]
    fn test_near_infinite_slope() {
        let l1 = LineSegment::new(
            pt(-0.000000000000000040274189285034336, 10.),
            pt(-0.00000000000000003900535185736784, 0.),
        );

        let l2 = LineSegment::new(pt(-10., 6.), pt(10., 6.));

        assert_eq!(Some((0.4, true)), l1.intersect_segment(&l2));

        assert_eq!(Some((0.5, false)), l2.intersect_segment(&l1));
    }
}
