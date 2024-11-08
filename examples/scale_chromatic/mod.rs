#[derive(Debug, Clone, Copy)]
pub struct ScaleOrdinal<const M: usize> {
    domain: [Color; M],
}

impl ScaleOrdinal<10> {
    pub fn _get(&self, i: usize) -> Option<Color> {
        self.domain.get(i).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = Color> + Clone + '_ {
        self.domain.iter().copied()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct SchemeCategory10;

/// `d3` stores its color palettes as strings of hexadecimal values (omitting the `#`).
///
/// Eventually, we should read / store this in a binary format, potentially using `const fn`.
const CATEGORY_10: &str = "1f77b4ff7f0e2ca02cd627289467bd8c564be377c27f7f7fbcbd2217becf";

impl From<SchemeCategory10> for ScaleOrdinal<10> {
    fn from(_: SchemeCategory10) -> ScaleOrdinal<10> {
        let mut colors = [Color { r: 0, g: 0, b: 0 }; 10];
        for (i, c) in CATEGORY_10.as_bytes().chunks(6).enumerate() {
            let r = u8::from_str_radix(std::str::from_utf8(&c[0..2]).unwrap(), 16).unwrap();
            let g = u8::from_str_radix(std::str::from_utf8(&c[2..4]).unwrap(), 16).unwrap();
            let b = u8::from_str_radix(std::str::from_utf8(&c[4..6]).unwrap(), 16).unwrap();
            colors[i] = Color { r, g, b };
        }

        ScaleOrdinal { domain: colors }
    }
}
