use std::collections::HashMap;

///////////////////////////////////////////////////////////////////////////////
// VALUES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub enum Value {
    Unit,
    Bool(bool),
    Numeric(Numeric),
    String(String),
    Collection(Collection),
    Option(Box<Option<Value>>),
    Tuple(Vec<Value>),
    Struct(Struct),
    TupleStruct(TupleStruct),
    Variant(Variant),
}


#[derive(Clone, Debug)]
pub enum Numeric {
    I64(i64),
    I128(i128),
    U64(u64),
    U128(u128),
    F64(f64),
}


#[derive(Clone, Debug)]
pub enum Collection {
    Map(HashMap<String, Value>),
    Vec(Vec<Value>),
}


#[derive(Clone, Debug)]
pub struct Struct {
    pub type_name: String,
    pub data: HashMap<String, Value>,
}

#[derive(Clone, Debug)]
pub struct TupleStruct {
    pub type_name: String,
    pub data: Vec<Value>,
}

#[derive(Clone, Debug)]
pub enum Variant {
    TupleVariant {
        type_name: String,
        variant_name: String,
        data: Vec<Value>
    },
    StructVariant {
        type_name: String,
        variant_name: String,
        data: HashMap<String, Value>
    },
    UnitVariant {
        type_name: String,
        variant_name: String,
    }
}

