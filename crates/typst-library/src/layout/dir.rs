use ecow::EcoString;

use crate::foundations::{func, scope, ty, Repr};
use crate::layout::{Axis, Side};

/// The four directions into which content can be laid out.
///
///  Possible values are:
/// - `{ltr}`: Left to right.
/// - `{rtl}`: Right to left.
/// - `{ttb}`: Top to bottom.
/// - `{btt}`: Bottom to top.
///
/// These values are available globally and
/// also in the direction type's scope, so you can write either of the following
/// two:
/// ```example
/// #stack(dir: rtl)[A][B][C]
/// #stack(dir: direction.rtl)[A][B][C]
/// ```
#[ty(scope, name = "direction")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Dir {
    /// Left to right.
    LTR,
    /// Right to left.
    RTL,
    /// Top to bottom.
    TTB,
    /// Bottom to top.
    BTT,
}

impl Dir {
    /// Whether this direction points into the positive coordinate direction.
    ///
    /// The positive directions are left-to-right and top-to-bottom.
    pub const fn is_positive(self) -> bool {
        match self {
            Self::LTR | Self::TTB => true,
            Self::RTL | Self::BTT => false,
        }
    }
}

#[scope]
impl Dir {
    pub const LTR: Self = Self::LTR;
    pub const RTL: Self = Self::RTL;
    pub const TTB: Self = Self::TTB;
    pub const BTT: Self = Self::BTT;

    /// The axis this direction belongs to, either `{"horizontal"}` or
    /// `{"vertical"}`.
    ///
    /// ```example
    /// #ltr.axis() \
    /// #ttb.axis()
    /// ```
    #[func]
    pub const fn axis(self) -> Axis {
        match self {
            Self::LTR | Self::RTL => Axis::X,
            Self::TTB | Self::BTT => Axis::Y,
        }
    }

    /// The start point of this direction, as an alignment.
    ///
    /// ```example
    /// #ltr.start() \
    /// #rtl.start() \
    /// #ttb.start() \
    /// #btt.start()
    /// ```
    #[func]
    pub const fn start(self) -> Side {
        match self {
            Self::LTR => Side::Left,
            Self::RTL => Side::Right,
            Self::TTB => Side::Top,
            Self::BTT => Side::Bottom,
        }
    }

    /// The end point of this direction, as an alignment.
    ///
    /// ```example
    /// #ltr.end() \
    /// #rtl.end() \
    /// #ttb.end() \
    /// #btt.end()
    /// ```
    #[func]
    pub const fn end(self) -> Side {
        match self {
            Self::LTR => Side::Right,
            Self::RTL => Side::Left,
            Self::TTB => Side::Bottom,
            Self::BTT => Side::Top,
        }
    }

    /// The inverse direction.
    ///
    /// ```example
    /// #ltr.inv() \
    /// #rtl.inv() \
    /// #ttb.inv() \
    /// #btt.inv()
    /// ```
    #[func(title = "Inverse")]
    pub const fn inv(self) -> Dir {
        match self {
            Self::LTR => Self::RTL,
            Self::RTL => Self::LTR,
            Self::TTB => Self::BTT,
            Self::BTT => Self::TTB,
        }
    }
}

impl Repr for Dir {
    fn repr(&self) -> EcoString {
        match self {
            Self::LTR => "ltr".into(),
            Self::RTL => "rtl".into(),
            Self::TTB => "ttb".into(),
            Self::BTT => "btt".into(),
        }
    }
}

/// The writing modes for block level content.
///
/// Possible values are:
/// - `{horizontal_tb}`: Horizontal top to bottom.
/// - `{vertical_rl}`: Vertical right to left.
/// - `{vertical_lr}`: Vertical left to right.
///
/// These values are available globally and also in the writing mode type's scope.
#[ty(scope, name = "writing_mode")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum WritingMode {
    /// Horizontal top to bottom.
    HorizontalTB,
    /// Vertical right to left.
    VerticalRL,
    /// Vertical left to right.
    VerticalLR,
}

impl WritingMode {
    /// Determine the text direction for the writing mode.
    pub const fn text_direction(self) -> Dir {
        match self {
            Self::HorizontalTB => Dir::LTR,
            Self::VerticalRL => Dir::TTB,
            Self::VerticalLR => Dir::TTB,
        }
    }

    /// Determine the rotation angle for the writing mode.
    pub const fn rotation_angle(self) -> f64 {
        match self {
            Self::HorizontalTB => 0.0,
            Self::VerticalRL => 90.0,
            Self::VerticalLR => -90.0,
        }
    }
}

#[scope]
impl WritingMode {
    pub const HORIZONTAL_TB: Self = Self::HorizontalTB;
    pub const VERTICAL_RL: Self = Self::VerticalRL;
    pub const VERTICAL_LR: Self = Self::VerticalLR;
}

impl Repr for WritingMode {
    fn repr(&self) -> EcoString {
        match self {
            Self::HorizontalTB => "horizontal_tb".into(),
            Self::VerticalRL => "vertical_rl".into(),
            Self::VerticalLR => "vertical_lr".into(),
        }
    }
}
