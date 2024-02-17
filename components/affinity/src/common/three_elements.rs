#[cfg(feature = "color_hints")]
use crate::color_hints::ColorHint;
use crate::Affinity;

#[derive(Debug, PartialEq)]
pub enum Element {
    Fire,
    Water,
    Grass,
}

impl Affinity for Element {
    fn weak_against(&self, other: &Self) -> bool {
        match self {
            Element::Fire => matches!(other, Element::Water),
            Element::Water => matches!(other, Element::Grass),
            Element::Grass => matches!(other, Element::Fire),
        }
    }

    fn strong_against(&self, other: &Self) -> bool {
        if self.eq(&other) {
            return false;
        }

        !self.weak_against(other)
    }
}

#[cfg(feature = "color_hints")]
impl ColorHint for Element {
    fn color(&self) -> String {
        match self {
            Element::Fire => "Red".to_string(),
            Element::Water => "Blue".to_string(),
            Element::Grass => "Green".to_string(),
        }
    }
}
