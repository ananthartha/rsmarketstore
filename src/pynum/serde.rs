use crate::proto::NumpyDataset;

use std::collections::BTreeMap;
use std::io::Cursor;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use serde::de::{self, DeserializeSeed, IntoDeserializer, MapAccess, Visitor};
use serde::forward_to_deserialize_any;

use super::error::Error;
use super::{NumPyType, Value};

pub fn from_dataset<T>(dataset: NumpyDataset) -> Result<Vec<T>, Error>
where
    T: de::DeserializeOwned,
{
    let mut items: Vec<T> = vec![];
    let mut deserializer = DatasetItr::new(dataset);

    loop {
        items.push(T::deserialize(&mut deserializer)?);
        if !deserializer.next() {
            break;
        }
    }

    return Ok(items);
}

pub struct Column(NumPyType, Cursor<Vec<u8>>);

pub fn as_column(numPyType: NumPyType, cursor: Cursor<Vec<u8>>) -> Column {
    return Column(numPyType, cursor);
}

impl Column {
    fn read(self: &mut Self) -> Value {
        match self.0 {
            NumPyType::i1 => self.1.read_i8().map(Value::I1).unwrap(),
            NumPyType::i2 => self.1.read_i16::<LittleEndian>().map(Value::I2).unwrap(),
            NumPyType::i4 => self.1.read_i32::<LittleEndian>().map(Value::I4).unwrap(),
            NumPyType::i8 => self.1.read_i64::<LittleEndian>().map(Value::I8).unwrap(),
            NumPyType::u1 => self.1.read_u8().map(Value::U1).unwrap(),
            NumPyType::u2 => self.1.read_u16::<LittleEndian>().map(Value::U2).unwrap(),
            NumPyType::u4 => self.1.read_u32::<LittleEndian>().map(Value::U4).unwrap(),
            NumPyType::u8 => self.1.read_u64::<LittleEndian>().map(Value::U8).unwrap(),
            // DType::f2 => self.1.read_f32::<LittleEndian>().map(Value::F2).unwrap(),
            NumPyType::f4 => self.1.read_f32::<LittleEndian>().map(Value::F4).unwrap(),
            NumPyType::f8 => self.1.read_f64::<LittleEndian>().map(Value::F8).unwrap(),
        }
    }
}

pub struct DatasetItr {
    pos: usize,
    length: usize,
    columns: BTreeMap<String, Column>,
}

impl DatasetItr {
    pub fn new(mut dataset: NumpyDataset) -> Self {
        let mut instance = Self {
            pos: 0,
            columns: BTreeMap::new(),
            length: dataset.length as usize,
        };

        while let Some(column_name) = dataset.column_names.pop() {
            instance.columns.insert(
                column_name,
                as_column(
                    NumPyType::try_from(dataset.column_types.pop().unwrap()).unwrap(),
                    Cursor::new(dataset.column_data.pop().unwrap()),
                ),
            );
        }

        instance
    }

    #[inline]
    pub fn has_columns(self: &Self, columns: Vec<&str>) -> Option<String> {
        for column in columns {
            if self.columns.get(column).is_none() {
                return Some(column.to_string());
            }
        }

        None
    }

    #[inline]
    pub fn next(self: &mut Self) -> bool {
        if self.pos < self.length {
            self.pos += 1;
        }
        return false;
    }

    #[inline]
    pub fn get(self: &mut Self, name: &str) -> Option<Value> {
        self.columns
            .get_mut(name.into())
            .map(|column| column.read())
    }
}

impl<'de> de::Deserializer<'de> for &'de mut DatasetItr {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map identifier ignored_any enum
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(missing_column) = self.has_columns(fields.to_vec()) {
            return Err(Error::ColumnNotFound {
                column: missing_column,
            });
        }

        visitor.visit_map(PopulateMap::new(self, fields.to_vec()))
    }
}

struct PopulateMap<'de> {
    de: &'de mut DatasetItr,
    fields: Vec<&'de str>,
    currentColumn: Option<String>,
}

impl<'de> PopulateMap<'de> {
    fn new(de: &'de mut DatasetItr, fields: Vec<&'de str>) -> Self {
        PopulateMap {
            de,
            fields,
            currentColumn: None,
        }
    }
}

impl<'de> MapAccess<'de> for PopulateMap<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Error>
    where
        K: DeserializeSeed<'de>,
    {
        let Some(column_name) = self.fields.pop() else {
            return Ok(None)
        };

        self.currentColumn = Option::Some(String::from(column_name));
        self.currentColumn
            .clone()
            .map(IntoDeserializer::into_deserializer)
            .map(|column| seed.deserialize(column).map(Some))
            .unwrap_or(Err(Error::UnableSeedColumnKey {
                column: String::from(column_name),
            }))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(IntoDeserializer::into_deserializer(
            self.de.get(self.currentColumn.as_ref().unwrap()).unwrap(),
        ))
    }
}

mod test {
    use serde::Deserialize;

    use crate::proto::NumpyDataset;

    use super::from_dataset;

    #[derive(Deserialize, Debug)]
    struct Candle {
        open: f64,
        heigh: f64,
        low: f64,
        close: f64,
        volume: i64,
    }

    #[test]
    fn test_from_dataset() {
        println!(
            "{:#?}",
            from_dataset::<Candle>(NumpyDataset {
                column_types: vec![
                    "f8".into(),
                    "f8".into(),
                    "f8".into(),
                    "f8".into(),
                    "i8".into(),
                ],
                column_names: vec![
                    "open".into(),
                    "heigh".into(),
                    "low".into(),
                    "close".into(),
                    "volume".into(),
                ],
                column_data: vec![
                    17500_f64.to_le_bytes().to_vec(),
                    17510_f64.to_le_bytes().to_vec(),
                    17490_f64.to_le_bytes().to_vec(),
                    17501_f64.to_le_bytes().to_vec(),
                    0_i64.to_le_bytes().to_vec(),
                    // 0.to_le_bytes().to_vec(),
                ],
                length: 1,
                // data_shapes: todo!(),
                ..Default::default()
            })
            .unwrap()
        );
    }
}
