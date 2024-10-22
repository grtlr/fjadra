#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Extent {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

impl From<Extent> for [[f64; 2]; 2] {
    fn from(extent: Extent) -> Self {
        [[extent.x0, extent.y0], [extent.x1, extent.y1]]
    }
}

impl From<[[f64; 2]; 2]> for Extent {
    fn from(extent: [[f64; 2]; 2]) -> Self {
        Extent {
            x0: extent[0][0],
            y0: extent[0][1],
            x1: extent[1][0],
            y1: extent[1][1],
        }
    }
}

impl From<[f64; 4]> for Extent {
    fn from(extent: [f64; 4]) -> Self {
        Extent {
            x0: extent[0],
            y0: extent[1],
            x1: extent[2],
            y1: extent[3],
        }
    }
}

impl From<Extent> for [f64; 4] {
    fn from(extent: Extent) -> Self {
        [extent.x0, extent.y0, extent.x1, extent.y1]
    }
}
