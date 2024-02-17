#[cfg(feature = "color_hints")]
use crate::color_hints::ColorHint;
use crate::Affinity;

#[derive(Debug, PartialEq)]
pub enum Element {
    Fire,
    Water,
    Ice,
    Grass,
    Electric,
    Ground,
    Flying,
    Normal,
}

impl Affinity for Element {
    fn weak_against(&self, other: &Self) -> bool {
        if self.eq(&Element::Normal) || other.eq(&Element::Normal) {
            return false;
        }

        match self {
            Element::Fire => matches!(other, Element::Water),
            Element::Water => matches!(other, Element::Grass) || matches!(other, Element::Ground),
            Element::Grass => matches!(other, Element::Fire),
            Element::Ice => matches!(other, Element::Fire) || matches!(other, Element::Grass),
            Element::Electric => {
                matches!(other, Element::Ground) || matches!(other, Element::Flying)
            }
            Element::Ground => matches!(other, Element::Flying),
            Element::Flying => matches!(other, Element::Electric) || matches!(other, Element::Ice),
            Element::Normal => false,
        }
    }

    fn strong_against(&self, other: &Self) -> bool {
        if self.eq(&Element::Normal) || other.eq(&Element::Normal) || self.eq(other) {
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
            Element::Ice => "Cyan".to_string(),
            Element::Electric => "Orange".to_string(),
            Element::Ground => "Brown".to_string(),
            Element::Flying => "White".to_string(),
            Element::Normal => "Gray".to_string(),
        }
    }
}
