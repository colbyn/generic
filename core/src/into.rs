use std::collections::{
    HashMap,
    HashSet,
    BTreeMap,
    BTreeSet,
};
use crate::value::{
    Value,
    Collection,
    Numeric,
};

///////////////////////////////////////////////////////////////////////////////
// GENERIC INTERFACE
///////////////////////////////////////////////////////////////////////////////

pub trait IntoGeneric where Self: Sized {
    fn into_generic(&self) -> Value;
}


///////////////////////////////////////////////////////////////////////////////
// BASIC IMPLEMENTATIONS - MISC
///////////////////////////////////////////////////////////////////////////////

impl IntoGeneric for uuid::Uuid {
    fn into_generic(&self) -> Value {
        Value::String(self.to_string())
    }
}


///////////////////////////////////////////////////////////////////////////////
// BASIC IMPLEMENTATIONS - STD
///////////////////////////////////////////////////////////////////////////////

impl IntoGeneric for f32 {
    fn into_generic(&self) -> Value {
        Value::Numeric(Numeric::F64(*self as f64))
    }
}
impl IntoGeneric for f64 {
    fn into_generic(&self) -> Value {
        Value::Numeric(Numeric::F64(*self))
    }
}
impl IntoGeneric for i8 {
    fn into_generic(&self) -> Value {
        Value::Numeric(Numeric::I64(*self as i64))
    }
}
impl IntoGeneric for i16 {
    fn into_generic(&self) -> Value {
        Value::Numeric(Numeric::I64(*self as i64))
    }
}
impl IntoGeneric for i32 {
    fn into_generic(&self) -> Value {
        Value::Numeric(Numeric::I64(*self as i64))
    }
}
impl IntoGeneric for i64 {
    fn into_generic(&self) -> Value {
        Value::Numeric(Numeric::I64(*self))
    }
}
impl IntoGeneric for i128 {
    fn into_generic(&self) -> Value {
        Value::Numeric(Numeric::I128(*self))
    }
}
impl IntoGeneric for u8 {
    fn into_generic(&self) -> Value {
        Value::Numeric(Numeric::U64(*self as u64))
    }
}
impl IntoGeneric for u16 {
    fn into_generic(&self) -> Value {
        Value::Numeric(Numeric::U64(*self as u64))
    }
}
impl IntoGeneric for u32 {
    fn into_generic(&self) -> Value {
        Value::Numeric(Numeric::U64(*self as u64))
    }
}
impl IntoGeneric for u64 {
    fn into_generic(&self) -> Value {
        Value::Numeric(Numeric::U64(*self))
    }
}
impl IntoGeneric for u128 {
    fn into_generic(&self) -> Value {
        Value::Numeric(Numeric::U128(*self))
    }
}

// impl IntoGeneric for usize {
//     fn into_generic(&self) -> Value {
//         Value::Numeric(unimplemented!())
//     }
// }
// impl IntoGeneric for isize {
//     fn into_generic(&self) -> Value {
//         Value::Numeric(unimplemented!())
//     }
// }

impl IntoGeneric for () {
    fn into_generic(&self) -> Value {
        Value::Unit
    }
}

impl IntoGeneric for bool {
    fn into_generic(&self) -> Value {
        Value::Bool(*self)
    }
}

impl IntoGeneric for char {
    fn into_generic(&self) -> Value {
        let mut x = String::new();
        x.push(*self);
        Value::String(x)
    }
}

impl IntoGeneric for String {
    fn into_generic(&self) -> Value {
        Value::String(self.clone())
    }
}

impl<V: IntoGeneric> IntoGeneric for Vec<V> {
    fn into_generic(&self) -> Value {
        let xs = self
            .iter()
            .map(|(v)| v.into_generic())
            .collect::<Vec<_>>();
        Value::Collection(Collection::Vec(xs))
    }
}


impl<V: IntoGeneric> IntoGeneric for HashMap<String, V> {
    fn into_generic(&self) -> Value {
        let xs = self
            .iter()
            .map(|(k, v)| (k.clone(), v.into_generic()))
            .collect::<HashMap<String, Value>>();
        Value::Collection(Collection::Map(xs))
    }
}


///////////////////////////////////////////////////////////////////////////////
// BASIC IMPLEMENTATIONS - TUPLES
///////////////////////////////////////////////////////////////////////////////

impl<T1, T2> IntoGeneric for (T1, T2)
where
    T1: IntoGeneric,
    T2: IntoGeneric,
{
    fn into_generic(&self) -> Value {
        Value::Tuple(vec![
            self.0.into_generic(),
            self.1.into_generic(),
        ])
    }
}

impl<T1, T2, T3> IntoGeneric for (T1, T2, T3)
where
    T1: IntoGeneric,
    T2: IntoGeneric,
    T3: IntoGeneric,
{
    fn into_generic(&self) -> Value {
        Value::Tuple(vec![
            self.0.into_generic(),
            self.1.into_generic(),
            self.2.into_generic(),
        ])
    }
}

impl<T1, T2, T3, T4> IntoGeneric for (T1, T2, T3, T4)
where
    T1: IntoGeneric,
    T2: IntoGeneric,
    T3: IntoGeneric,
    T4: IntoGeneric,
{
    fn into_generic(&self) -> Value {
        Value::Tuple(vec![
            self.0.into_generic(),
            self.1.into_generic(),
            self.2.into_generic(),
            self.3.into_generic(),
        ])
    }
}
