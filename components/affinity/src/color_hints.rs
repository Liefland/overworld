use std::fmt::{Display, Formatter};

pub trait ColorHint {
    fn color(&self) -> String;
    fn rgb(&self) -> (u8, u8, u8) {
        match self.color().to_lowercase().as_str() {
            "red" => (255, 0, 0),
            "blue" => (0, 0, 255),
            "green" => (0, 255, 0),
            "cyan" => (0, 255, 255),
            "yellow" => (255, 255, 0),
            "brown" => (139, 69, 19),
            "white" => (255, 255, 255),
            "gray" | "grey" => (128, 128, 128),
            _ => (0, 0, 0),
        }
    }

    fn hex(&self) -> u32 {
        let (r, g, b) = self.rgb();

        (r as u32) << 16 | (g as u32) << 8 | b as u32
    }

    fn hex_string(&self) -> String {
        let (r, g, b) = self.rgb();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
}

impl Display for dyn ColorHint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.color())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    enum Foo {
        Bar,
    }

    impl ColorHint for Foo {
        fn color(&self) -> String {
            match self {
                Foo::Bar => "Brown".to_string(),
            }
        }
    }

    #[test]
    fn test_color() {
        assert_eq!("Brown", Foo::Bar.color());
    }

    #[test]
    fn test_rgb() {
        assert_eq!((139, 69, 19), Foo::Bar.rgb());
    }

    #[test]
    fn test_hex() {
        assert_eq!(0x8B4513, Foo::Bar.hex());
    }

    #[test]
    fn test_hex_string() {
        assert_eq!("#8B4513", Foo::Bar.hex_string());
    }
}
