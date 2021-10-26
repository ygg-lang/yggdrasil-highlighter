use std::collections::BTreeSet;

use serde::{
    ser::{SerializeSeq, SerializeStruct},
    Serialize, Serializer,
};

use crate::ColorView;

impl Serialize for ColorView {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let set = BTreeSet::from_iter(self.into_iter().map(|v| v.color));
        let set = Vec::from_iter(set.into_iter());
        let bits: Vec<u8> = self
            .span
            .get_info()
            .iter()
            .map(|v| set.binary_search(&v.clone().unwrap_or_default()))
            .map(|v| v.unwrap_or_default() as u8)
            .collect();
        let mut ser = serializer.serialize_struct("ColorView", 3)?;
        ser.serialize_field("text", &self.text())?;
        ser.serialize_field("colors", &set)?;
        ser.serialize_field("bits", &bits)?;
        ser.end()
    }
}
