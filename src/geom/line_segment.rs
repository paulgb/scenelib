use crate::geom::traits::{Translate, XYFlip, Slope};
use crate::geom::types::{Point2f, Vec2f};
use rstar::{RTreeObject, AABB};

#[derive(PartialEq, Clone, Copy)]
pub struct LineSegment {
    pub c1: Point2f,
    pub c2: Point2f,
}

impl std::fmt::Debug for LineSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line(({}, {}), ({}, {}))", self.c1.x, self.c1.y, self.c2.x, self.c2.y)
    }
}

impl Eq for LineSegment {

}

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

impl RTreeObject for LineSegment
{
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope
    {
        AABB::from_corners([self.c1.x, self.c1.y], [self.c2.x, self.c2.y])
    }
}


impl XYFlip for LineSegment {
    fn xy_flip(&self) -> LineSegment {
        LineSegment::new(self.c1.xy_flip(), self.c2.xy_flip())
    }
}

impl Translate for LineSegment {
    fn translate(&self, dist: Vec2f) -> LineSegment {
        LineSegment::new(self.c1.translate(dist), self.c2.translate(dist))
    }
}

impl LineSegment {
    pub fn new(c1: Point2f, c2: Point2f) -> LineSegment {
        LineSegment { c1, c2 }
    }

    pub fn reverse(&self) -> LineSegment {
        LineSegment { c1: self.c2, c2: self.c1 }
    }

    pub fn vector(&self) -> Vec2f {
        self.c2 - self.c1
    }

    pub fn intersect_segment(&self, other: &LineSegment) -> Option<(f64, bool)> {
        // If we were to extend this line out to infinity, would the other
        // line segment intersect it? If so, return the fraction along our
        // segment at which the intersection occurs and the direction.

        let v = self.vector();

        // If this line has infinite slope, flip it.
        if v.x == 0. {
            if v.y != 0. {
                return self.xy_flip().intersect_segment(&other.xy_flip());
            } else {
                // This line segment is actually a point.
                return None;
            }
        }

        let ov = other.vector();

        let perp_dot = (v.x * ov.y) - (v.y * ov.x);

        if perp_dot == 0. {
            return None
        }

        let direction = perp_dot > 0.;

        if ov.x == 0. {
            // If the other line has infinite slope, special case.
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

            if (x - other.c1.x) * (x - other.c2.x) > 0. {
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
        let l1 = LineSegment::new(Point2f::new(0., 5.), Point2f::new(10., 5.));
        let l2 = LineSegment::new(Point2f::new(5., 0.), Point2f::new(5., 10.));

        assert_eq!(Some((0.5, true)), l1.intersect_segment(&l2));
        assert_eq!(Some((1.5, true)), l1.intersect_segment(&l2.translate(Vec2f::new(10., 0.))));

        // The other line segment never intersects the current line.
        assert_eq!(None, l1.intersect_segment(&l2.translate(Vec2f::new(0., 10.))));
        assert_eq!(None, l1.intersect_segment(&l2.translate(Vec2f::new(10., 10.))));
    }

    #[test]
    fn test_intersect_horizontal() {
        let l1 = LineSegment::new(Point2f::new(0., 0.), Point2f::new(10., 10.));
        let l2 = LineSegment::new(Point2f::new(0., 8.), Point2f::new(20., 8.));
        assert_eq!(Some((0.8, false)), l1.intersect_segment(&l2));
    }

    #[test]
    fn test_intersect_regular() {
        let l1 = LineSegment::new(Point2f::new(3., 1.), Point2f::new(13., 6.));
        let l2 = LineSegment::new(Point2f::new(10., 6.), Point2f::new(14., 2.));
        assert_eq!(Some((0.8, false)), l1.intersect_segment(&l2));
        assert_eq!(Some((0.6, false)), l1.intersect_segment(&l2.translate(Vec2f::new(-2.0, -1.0))));
        assert_eq!(Some((1.2, false)), l1.intersect_segment(&l2.translate(Vec2f::new(4.0, 2.0))));
        assert_eq!(Some((-0.2, false)), l1.intersect_segment(&l2.translate(Vec2f::new(-10.0, -5.0))));

        assert_eq!(None, l1.intersect_segment(&l2.translate(Vec2f::new(-20.0, -5.0))));
    }

    #[test]
    fn test_intersect_direction() {
        let l1 = LineSegment::new(Point2f::new(3., 1.), Point2f::new(13., 6.));
        let l2 = LineSegment::new(Point2f::new(10., 6.), Point2f::new(14., 2.));
        assert_eq!(Some((0.8, false)), l1.intersect_segment(&l2));
        assert_eq!(Some((0.8, true)), l1.intersect_segment(&l2.reverse()));
        assert_eq!(Some((0.2, true)), l1.reverse().intersect_segment(&l2));
        assert_eq!(Some((0.2, false)), l1.reverse().intersect_segment(&l2.reverse()));
    }

}