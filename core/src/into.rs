use std::collections::{
    HashMap,
    HashSet,
    BTreeMap,
    BTreeSet,
};
use crate::value::{
    Value,
    Collection,
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

impl IntoGeneric for String {
    fn into_generic(&self) -> Value {
        Value::String(self.clone())
    }
}


///////////////////////////////////////////////////////////////////////////////
// BASIC IMPLEMENTATIONS - COLLECTIONS
///////////////////////////////////////////////////////////////////////////////

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
