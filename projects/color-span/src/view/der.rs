use std::fmt::Formatter;

use serde::{
    de::{IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};

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
        formatter.write_str("Expect `[u32]`")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut out = ColorView { span: IndexedText::new(String::new()), characters: vec![] };
        while let Some(repr) = seq.next_element::<u32>()? {
            out.characters.push(Character::from(repr));
        }
        Ok(out)
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut text = String::new();
        let mut colors = vec![];
        let mut bits: Vec<u8> = vec![];

        while let Some(key) = map.next_key::<&str>() {
            match key {
                "text" => text = map.next_value()?,
                "colors" => colors = map.next_value()?,
                "bits" => bits = map.next_value()?,
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }

        Ok(ColorView { span: () })
    }
}
