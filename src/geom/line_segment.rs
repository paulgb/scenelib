//! Basic representation of line segments, which are the most
//! basic unit of a `Scene`. This is also where the heart of
//! the intersection code lives (`LineSegment::intersect_segment`).

use crate::types::{Point, PointContainer, Vector};
use rstar::{RTreeObject, AABB};


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
        match self.intersect_lines(other) {
            Some((f1, f2)) if (0. <= f2 && f2 <= 1.) => {
                let direction = {
                    let v = self.c2 - self.c1;
                    let ov = other.c2 - other.c1;
                    let perp_dot = (v.x * ov.y) - (v.y * ov.x);
        
                    if perp_dot == 0. {
                        // The lines are parallel.
                        return None;
                    } else {
                        perp_dot > 0.
                    }
                };
                Some((f1, direction))
            },
            _ => None
        }
        

    }

    /// Returns the location at which two lines (extended to infinity) intersect, relative
    /// to each line segment.
    pub fn intersect_lines(&self, other: &LineSegment) -> Option<(f64, f64)> {
        let ground = other.c1 - self.c1;
        let ground_len = ground.norm();
        
        if ground_len == 0. {
            // Lines start from the same position.
            return Some((0., 0.))
        }
        
        let ground_norm = ground / ground.norm();
        let ground_perp = Vector::new(-ground_norm.y, ground_norm.x);
        
        let self_vec = self.c2 - self.c1;
        let self_vec_norm = self_vec / self_vec.norm();
        
        let other_vec = other.c2 - other.c1;
        let other_vec_norm = other_vec / other_vec.norm();
        
        let other_run = other_vec_norm.dot(&ground_norm);
        let other_rise = other_vec_norm.dot(&ground_perp);
        if other_rise == 0. {
            // First line starts on second line.
            return Some((0., ground_len / other_vec.norm()));

        }
        
        let other_slope = other_run / other_rise;
        
        let self_run = self_vec_norm.dot(&ground_norm);
        let self_rise = self_vec_norm.dot(&ground_perp);
        if self_rise == 0. {
            // Second line starts on first line.
            return Some((ground_len / self_vec.norm(), 0.))
        }

        let self_slope = self_run / self_rise;

        let net_slope = self_slope - other_slope;
        
        if net_slope == 0. {
            // Lines are parallel.
            return None
        }

        let f = ground_len / net_slope;
        return Some((
            f / (self_rise * self_vec.norm()),
            f / (other_rise * other_vec.norm())
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{pt, vec};
    use crate::types::{PointActions};

    macro_rules! assert_close {
        ( $expected:expr, $actual:expr ) => {
            {
                match ($expected, $actual) {
                    (Some((f1, d1)), Some((f2, d2))) => {
                        assert_eq!(d1, d2);
                        assert!((f1 - f2).abs() < 1e10);
                    },
                    (None, None) => (),
                    _ => assert_eq!($expected, $actual)
                }
            }
        };
    }

    #[test]
    fn test_intersect_vertical() {
        let l1 = LineSegment::new(pt(0., 5.), pt(10., 5.));
        let l2 = LineSegment::new(pt(5., 0.), pt(5., 10.));

        assert_close!(Some((0.5, true)), l1.intersect_segment(&l2));
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
        assert_close!(
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
    fn test_near_infinite_slope() {
        let l1 = LineSegment::new(
            pt(-0.000000000000000040274189285034336, 10.),
            pt(-0.00000000000000003900535185736784, 0.),
        );

        let l2 = LineSegment::new(pt(-10., 6.), pt(10., 6.));

        assert_close!(Some((0.4, true)), l1.intersect_segment(&l2));

        assert_close!(Some((0.5, false)), l2.intersect_segment(&l1));
    }
}
