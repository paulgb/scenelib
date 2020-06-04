/// Represents a mode for drawing a polygon to a plot.
#[derive(Clone)]
pub struct DrawMode {
    /// The pen to use to outline the polygon. If not given, no outline is drawn.
    pub pen: Option<usize>,
    /// Whether to simulate fill, by erasing lines that the polygon overlaps.
    pub fill: bool,
}

impl std::default::Default for DrawMode {
    fn default() -> Self {
        DrawMode {
            pen: Some(0),
            fill: true,
        }
    }
}

/// Helper for generating a `DrawMode` for the given pen that *does not*
/// use fill.
pub fn stroke(pen: usize) -> DrawMode {
    DrawMode {
        pen: Some(pen),
        fill: false,
    }
}

/// Helper for generating a `DrawMode` that fills but does not stroke.
pub fn fill_only() -> DrawMode {
    DrawMode {
        pen: None,
        fill: true,
    }
}

/// Helper for generating a `DrawMode` that fills and uses a pen.
pub fn pen(pen: usize) -> DrawMode {
    DrawMode {
        pen: Some(pen),
        fill: true,
    }
}
