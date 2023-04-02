use crate::parsing::game_events::KeyData;
use crate::parsing::game_events::NameDataPair;
use polars::prelude::NamedFrom;
use polars::series::Series;

// Absolute mess of a file :D. Some sunny day make these the "proper" way.

#[derive(Debug, Clone)]
pub enum PropData {
    Bool(bool),
    U32(u32),
    I32(i32),
    F32(f32),
    U64(u64),
    String(String),
    VecXY([f32; 2]),
    VecXYZ([f32; 3]),
    Vec(Vec<i32>),
    FloatVec(Vec<f64>),
    FloatVec32(Vec<f32>),
}

#[derive(Debug, Clone)]
pub enum VarVec {
    U32(Vec<Option<u32>>),
    Bool(Vec<Option<bool>>),
    U64(Vec<Option<u64>>),
    F32(Vec<Option<f32>>),
    I32(Vec<Option<i32>>),
    String(Vec<Option<String>>),
}
#[derive(Debug, Clone)]
pub struct PropColumn {
    pub data: VarVec,
}
impl PropColumn {
    pub fn new(item: &PropData) -> Self {
        let typ = match item {
            PropData::Bool(_) => VarVec::Bool(vec![]),
            PropData::I32(_) => VarVec::I32(vec![]),
            PropData::F32(_) => VarVec::F32(vec![]),
            PropData::String(_) => VarVec::String(vec![]),
            PropData::U64(_) => VarVec::U64(vec![]),
            PropData::U32(_) => VarVec::U32(vec![]),
            _ => panic!("Tried to create propcolumns from: {:?}", item),
        };
        PropColumn { data: typ }
    }
    pub fn push(&mut self, item: Option<PropData>) {
        self.data.push_propdata(item.clone());
    }
}

impl VarVec {
    pub fn push_propdata(&mut self, item: Option<PropData>) {
        match item {
            Some(PropData::F32(p)) => match self {
                VarVec::F32(f) => f.push(Some(p)),
                _ => {
                    panic!("Tried to push a {:?} into a {:?} column", item, self);
                }
            },
            Some(PropData::I32(p)) => match self {
                VarVec::I32(f) => f.push(Some(p)),
                _ => {
                    panic!("Tried to push a {:?} into a {:?} column", item, self);
                }
            },

            Some(PropData::String(p)) => match self {
                VarVec::String(f) => f.push(Some(p)),
                _ => {
                    panic!("Tried to push a ? into a {:?} column", self);
                }
            },
            Some(PropData::U32(p)) => match self {
                VarVec::U32(f) => f.push(Some(p)),
                _ => {
                    panic!("Tried to push a {:?} into a {:?} column", item, self);
                }
            },
            Some(PropData::U64(p)) => match self {
                VarVec::U64(f) => f.push(Some(p)),
                _ => {
                    panic!("Tried to push a {:?} into a {:?} column", item, self);
                }
            },
            Some(PropData::Bool(p)) => match self {
                VarVec::Bool(f) => f.push(Some(p)),
                _ => {
                    panic!("Tried to push a {:?} into a {:?} column", item, self);
                }
            },
            None => self.push_none(),
            _ => panic!("bad type for prop: {:?}", item),
        }
    }
    pub fn push_string(&mut self, data: String) {
        if let VarVec::String(f) = self {
            f.push(Some(data))
        }
    }
    pub fn push_string_none(&mut self) {
        if let VarVec::String(f) = self {
            f.push(None)
        }
    }
    pub fn push_float_none(&mut self) {
        if let VarVec::F32(f) = self {
            f.push(None)
        }
    }
    pub fn push_i32_none(&mut self) {
        if let VarVec::I32(f) = self {
            f.push(None)
        }
    }
    pub fn push_none(&mut self) {
        match self {
            VarVec::I32(f) => f.push(None),
            VarVec::F32(f) => f.push(None),
            VarVec::String(f) => f.push(None),
            VarVec::U32(f) => f.push(None),
            VarVec::U64(f) => f.push(None),
            VarVec::Bool(f) => f.push(None),
            _ => panic!("unk col while pushing none"),
        }
    }
    pub fn push_u64(&mut self, data: u64) {
        match self {
            VarVec::U64(f) => f.push(Some(data)),
            _ => panic!("TRIED TO PUSH SMALLER TYPE TO U64"),
        }
    }
    pub fn push_i32(&mut self, data: i32) {
        match self {
            VarVec::I32(f) => f.push(Some(data)),
            _ => panic!("i32 push panic"),
        }
    }
    pub fn get_len(&self) -> usize {
        match self {
            VarVec::I32(v) => v.len(),
            VarVec::F32(v) => v.len(),
            VarVec::String(v) => v.len(),
            _ => panic!("bad len type"),
        }
    }
}

pub fn filter_to_vec<Wanted>(v: impl IntoIterator<Item = impl TryInto<Wanted>>) -> Vec<Wanted> {
    v.into_iter().filter_map(|x| x.try_into().ok()).collect()
}
impl Default for KeyData {
    fn default() -> Self {
        KeyData::Bool(false)
    }
}

fn find_type_of_vals(pairs: &Vec<&NameDataPair>) -> i32 {
    // Need to find the correct type for outgoing series,
    // otherwise will default to something stupid most of
    // time, like strings for integers
    for pair in pairs {
        if pair.data.is_some() {
            return pair.data_type;
        }
    }
    return 0;
}

fn to_f32_series(pairs: &Vec<&NameDataPair>, name: &String) -> Series {
    let mut v = vec![];
    for pair in pairs {
        match &pair.data {
            Some(f) => match f {
                KeyData::Float(val) => v.push(Some(*val)),
                _ => v.push(None),
            },
            None => v.push(None),
        }
    }
    Series::new(name, v)
}
fn to_i32_series(pairs: &Vec<&NameDataPair>, name: &String) -> Series {
    let mut v = vec![];
    for pair in pairs {
        match &pair.data {
            Some(k) => match k {
                KeyData::I32(val) => v.push(Some(*val)),
                KeyData::Short(val) => v.push(Some((*val).into())),
                KeyData::Long(val) => v.push(Some(*val)),
                _ => v.push(None),
            },
            None => v.push(None),
        }
    }
    Series::new(name, v)
}
fn to_u64_series(pairs: &Vec<&NameDataPair>, name: &String) -> Series {
    let mut v = vec![];
    for pair in pairs {
        match &pair.data {
            Some(k) => match k {
                KeyData::Uint64(val) => v.push(Some(*val)),
                _ => v.push(None),
            },
            None => v.push(None),
        }
    }
    Series::new(name, v)
}
fn to_string_series(pairs: &Vec<&NameDataPair>, name: &String) -> Series {
    let mut v = vec![];
    for pair in pairs {
        match &pair.data {
            Some(k) => match k {
                KeyData::Str(val) => v.push(Some(val.to_owned())),
                _ => v.push(None),
            },
            None => v.push(None),
        }
    }
    Series::new(name, v)
}

fn to_bool_series(pairs: &Vec<&NameDataPair>, name: &String) -> Series {
    let mut v = vec![];
    for pair in pairs {
        match &pair.data {
            Some(k) => match k {
                KeyData::Bool(val) => v.push(Some(val.to_owned())),
                _ => v.push(None),
            },
            None => v.push(None),
        }
    }
    Series::new(name, v)
}

pub fn series_from_pairs(pairs: &Vec<&NameDataPair>, name: &String) -> Series {
    let only_data: Vec<Option<KeyData>> = pairs.iter().map(|x| x.data.clone()).collect();
    let field_type = find_type_of_vals(&pairs);
    let s = match field_type {
        1 => to_string_series(pairs, name),
        2 => to_f32_series(pairs, name),
        3 => to_i32_series(pairs, name),
        4 => to_i32_series(pairs, name),
        6 => to_bool_series(pairs, name),
        7 => to_u64_series(pairs, name),
        8 => to_i32_series(pairs, name),
        9 => to_i32_series(pairs, name),
        _ => panic!("unkown ge key: {:?}", field_type),
    };
    s
}

pub fn keydata_type_from_propdata(value: &Option<PropData>) -> i32 {
    match value {
        Some(PropData::String(_)) => 1,
        Some(PropData::F32(_)) => 2,
        Some(PropData::U32(_)) => 7,
        Some(PropData::I32(_)) => 4,
        Some(PropData::Bool(_)) => 6,
        None => 99,
        _ => panic!("Could not convert: {:?} into type", value),
    }
}
// 8 => Some(KeyData::Uint64(key.val_long().try_into().unwrap())),
// 9 => Some(KeyData::I32(key.val_short().try_into().unwrap())),

impl TryInto<i64> for KeyData {
    type Error = ();

    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Self::Long(l) => Ok(l as i64),
            Self::Byte(b) => Ok(b as i64),
            Self::Short(s) => Ok(s as i64),
            _ => Err(()),
        }
    }
}