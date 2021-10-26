use std::fmt::Formatter;

use internship::IStr;
use serde::{
    de::{IgnoredAny, MapAccess, Visitor},
    Deserialize, Deserializer,
};

use code_span::CodeView;

use crate::ColorView;

struct ColorViewMap {}

impl<'de> Deserialize<'de> for ColorView {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ColorViewMap {})
    }
}

impl<'de> Visitor<'de> for ColorViewMap {
    type Value = ColorView;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Expect `ColorView` object")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut text = String::new();
        let mut colors = vec![];
        let mut bits = vec![];

        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "text" => text = map.next_value()?,
                "colors" => colors = map.next_value::<Vec<String>>()?,
                "bits" => bits = map.next_value::<Vec<u8>>()?,
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let mut info = Vec::with_capacity(bits.len());
        for bit in bits {
            info.push(colors.get(bit as usize).map(|v| IStr::new(v)));
        }

        Ok(ColorView {
            span: CodeView::new(text, bits.into_iter().map(|v| colors.get(v as usize).map(|v| IStr::new(v))).collect()),
        })
    }
}
