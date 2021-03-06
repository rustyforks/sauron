use crate::{Callback, Value};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute<ATT, EVENT, MSG>
where
    ATT: Clone,
{
    pub name: ATT,
    pub value: AttribValue<EVENT, MSG>,
    pub namespace: Option<&'static str>,
}

impl<ATT, EVENT, MSG> Attribute<ATT, EVENT, MSG>
where
    ATT: Clone,
{
    pub fn from_callback(name: ATT, cb: Callback<EVENT, MSG>) -> Self {
        Attribute {
            name,
            value: cb.into(),
            namespace: None,
        }
    }

    pub fn from_value(name: ATT, value: Value) -> Self {
        Attribute {
            name,
            value: value.into(),
            namespace: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttribValue<EVENT, MSG> {
    /// normal attribute value
    Value(Value),
    /// function call such as value, checked, innerHTML
    FuncCall(Value),
    /// callback such as used in oninput, onclick
    Callback(Callback<EVENT, MSG>),
}

impl<ATT, EVENT, MSG> Attribute<ATT, EVENT, MSG>
where
    MSG: 'static,
    EVENT: 'static,
    ATT: PartialEq + Ord + ToString + Clone,
{
    pub(super) fn map_callback<MSG2>(
        self,
        cb: Callback<MSG, MSG2>,
    ) -> Attribute<ATT, EVENT, MSG2>
    where
        MSG2: 'static,
    {
        Attribute {
            name: self.name,
            value: self.value.map_callback(cb),
            namespace: self.namespace,
        }
    }

    pub fn is_event(&self) -> bool {
        self.value.is_event()
    }

    pub fn is_value(&self) -> bool {
        self.value.is_value()
    }
    pub fn is_func_call(&self) -> bool {
        self.value.is_func_call()
    }

    pub fn reform<F, EVENT2>(self, func: F) -> Attribute<ATT, EVENT2, MSG>
    where
        F: Fn(EVENT2) -> EVENT + 'static,
        EVENT2: 'static,
    {
        Attribute {
            name: self.name,
            value: self.value.reform(func),
            namespace: self.namespace,
        }
    }

    pub fn get_value(&self) -> Option<&Value> {
        self.value.get_value()
    }

    pub fn get_callback(&self) -> Option<&Callback<EVENT, MSG>> {
        self.value.get_callback()
    }

    pub fn take_callback(self) -> Option<Callback<EVENT, MSG>> {
        self.value.take_callback()
    }

    pub fn to_pretty_string(&self) -> String
    where
        ATT: ToString,
    {
        let mut buffer = String::new();
        if self.is_value() {
            if let Some(_ns) = self.namespace {
                buffer += &format!(
                    r#"xlink:{}="{}""#,
                    self.name.to_string(),
                    self.value
                );
            } else {
                buffer +=
                    &format!(r#"{}="{}""#, self.name.to_string(), self.value);
            }
        }
        buffer
    }
}

impl<EVENT, MSG> AttribValue<EVENT, MSG>
where
    MSG: 'static,
    EVENT: 'static,
{
    fn map_callback<MSG2>(
        self,
        cb: Callback<MSG, MSG2>,
    ) -> AttribValue<EVENT, MSG2>
    where
        MSG2: 'static,
    {
        match self {
            AttribValue::Value(value) => AttribValue::Value(value),
            AttribValue::FuncCall(value) => AttribValue::FuncCall(value),
            AttribValue::Callback(existing) => {
                AttribValue::Callback(existing.map_callback(cb))
            }
        }
    }

    fn reform<F, EVENT2>(self, func: F) -> AttribValue<EVENT2, MSG>
    where
        F: Fn(EVENT2) -> EVENT + 'static,
        EVENT2: 'static,
    {
        match self {
            AttribValue::Value(value) => AttribValue::Value(value),
            AttribValue::FuncCall(value) => AttribValue::FuncCall(value),
            AttribValue::Callback(cb) => AttribValue::Callback(cb.reform(func)),
        }
    }

    fn is_value(&self) -> bool {
        match self {
            AttribValue::Value(_) => true,
            _ => false,
        }
    }
    fn is_event(&self) -> bool {
        match self {
            AttribValue::Callback(_) => true,
            _ => false,
        }
    }
    fn is_func_call(&self) -> bool {
        match self {
            AttribValue::FuncCall(_) => true,
            _ => false,
        }
    }

    pub fn get_callback(&self) -> Option<&Callback<EVENT, MSG>> {
        match self {
            AttribValue::Value(_) => None,
            AttribValue::FuncCall(_) => None,
            AttribValue::Callback(cb) => Some(cb),
        }
    }

    pub fn take_callback(self) -> Option<Callback<EVENT, MSG>> {
        match self {
            AttribValue::Value(_) => None,
            AttribValue::FuncCall(_) => None,
            AttribValue::Callback(cb) => Some(cb),
        }
    }

    pub fn get_value(&self) -> Option<&Value> {
        match self {
            AttribValue::Value(value) => Some(value),
            AttribValue::FuncCall(value) => Some(value),
            AttribValue::Callback(_) => None,
        }
    }
}

impl<EVENT, MSG> fmt::Display for AttribValue<EVENT, MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttribValue::Value(value) => write!(f, "{}", value),
            AttribValue::FuncCall(_) => write!(f, ""),
            AttribValue::Callback(_) => write!(f, ""),
        }
    }
}

impl<EVENT, MSG> From<Callback<EVENT, MSG>> for AttribValue<EVENT, MSG> {
    fn from(cb: Callback<EVENT, MSG>) -> Self {
        AttribValue::Callback(cb)
    }
}

impl<EVENT, MSG> From<Value> for AttribValue<EVENT, MSG> {
    fn from(value: Value) -> Self {
        AttribValue::Value(value)
    }
}
