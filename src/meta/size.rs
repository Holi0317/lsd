use ansi_term::ANSIString;
use color::{Colors, Elem};
use std::fs::Metadata;

#[derive(Debug, PartialEq, Eq)]
pub enum Unit {
    Byte,
    Kilo,
    Mega,
    Giga,
    Tera,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Size {
    value: usize,
    unit: Unit,
}

impl<'a> From<&'a Metadata> for Size {
    fn from(meta: &Metadata) -> Self {
        let len = meta.len() as usize;

        if len < 1024 {
            Size {
                value: len * 1024,
                unit: Unit::Byte,
            }
        } else if len < 1024 * 1024 {
            Size {
                value: len,
                unit: Unit::Kilo,
            }
        } else if len < 1024 * 1024 * 1024 {
            Size {
                value: len / 1024,
                unit: Unit::Mega,
            }
        } else if len < 1024 * 1024 * 1024 * 1024 {
            Size {
                value: len / (1024 * 1024),
                unit: Unit::Giga,
            }
        } else {
            Size {
                value: len / (1024 * 1024 * 1024),
                unit: Unit::Tera,
            }
        }
    }
}

impl Size {
    pub fn render(&self, value_alignment: usize, unit_alignment: usize) -> ANSIString {
        let mut content = String::with_capacity(value_alignment + unit_alignment + 1);

        let value = self.render_value();
        let unit = self.render_unit();

        for _ in 0..(value_alignment - value.len()) {
            content.push(' ');
        }

        content += &self.render_value();
        content.push(' ');
        content += &self.render_unit();

        for _ in 0..(unit_alignment - unit.len()) {
            content.push(' ');
        }

        self.paint(content)
    }

    fn paint(&self, content: String) -> ANSIString {
        if self.unit == Unit::Byte || self.unit == Unit::Kilo {
            Colors[&Elem::FileSmall].paint(content)
        } else if self.unit == Unit::Mega {
            Colors[&Elem::FileMedium].paint(content)
        } else {
            Colors[&Elem::FileLarge].paint(content)
        }
    }

    pub fn render_value(&self) -> String {
        let size_str = (self.value as f32 / 1024.0).to_string();

        // Check if there is a fraction.
        if let Some(fraction_idx) = size_str.find('.') {
            // If the fraction start with 0 (like 32.01), the result is rounded
            // by removing the fraction.
            if size_str.chars().nth(fraction_idx + 1) == Some('0') {
                let (res, _) = size_str.split_at(fraction_idx); // Split before the fraction
                res.to_string()
            } else {
                //
                let (res, _) = size_str.split_at(fraction_idx + 2); // Split after the '.' and the first fraction digit.
                res.to_string()
            }
        } else {
            size_str
        }
    }

    pub fn render_unit(&self) -> String {
        match self.unit {
            Unit::Byte => String::from("B"),
            Unit::Kilo => String::from("KB"),
            Unit::Mega => String::from("MB"),
            Unit::Giga => String::from("GB"),
            Unit::Tera => String::from("TB"),
        }
    }
}
