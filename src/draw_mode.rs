#[derive(Clone)]
pub struct DrawMode {
    pub pen: Option<usize>,
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

pub fn pen(pen: usize) -> DrawMode {
    DrawMode {
        pen: Some(pen),
        fill: true,
    }
}
