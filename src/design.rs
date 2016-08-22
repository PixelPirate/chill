use {ViewName, serde, std};

/// Container for a _map_ and optional _reduce_ function of a view.
///
/// `ViewFunction` is a convenience type for applications that work with view
/// functions in design documents. For more information about view functions and
/// design documents, please see the CouchDB documentation.
///
/// # Examples
///
/// ```
/// extern crate chill;
///
/// let view_function = chill::ViewFunction::new_with_reduce(
///     "function(doc) { emit(doc.key_thing, doc.value_thing); }",
///     "_sum");
///
/// assert_eq!("function(doc) { emit(doc.key_thing, doc.value_thing); }",
///            view_function.map);
/// assert_eq!(Some(String::from("_sum")), view_function.reduce);
/// ```
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ViewFunction {
    /// The view's map function.
    ///
    /// For more information about _map functions_, please see the CouchDB
    /// documentation.
    ///
    pub map: String,

    /// The view's reduce function, if available.
    ///
    /// For more information about _reduce functions_, please see the CouchDB
    /// documentation.
    ///
    pub reduce: Option<String>,

    // This field exists to prevent applications from directly constructing this
    // struct.
    _dummy: std::marker::PhantomData<()>,
}

impl ViewFunction {
    /// Constructs a new `ViewFunction` that has no _reduce function_.
    pub fn new<M: Into<String>>(map: M) -> Self {
        ViewFunction {
            map: map.into(),
            reduce: None,
            _dummy: std::marker::PhantomData,
        }
    }

    /// Constructs a new `ViewFunction` that has a _reduce_ function.
    pub fn new_with_reduce<M: Into<String>, R: Into<String>>(map: M, reduce: R) -> Self {
        ViewFunction {
            map: map.into(),
            reduce: Some(reduce.into()),
            _dummy: std::marker::PhantomData,
        }
    }
}

impl serde::Deserialize for ViewFunction {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: serde::Deserializer
    {
        enum Field {
            Map,
            Reduce,
        }

        impl serde::Deserialize for Field {
            fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
                where D: serde::Deserializer
            {
                struct Visitor;

                impl serde::de::Visitor for Visitor {
                    type Value = Field;

                    fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                        where E: serde::de::Error
                    {
                        match value {
                            "map" => Ok(Field::Map),
                            "reduce" => Ok(Field::Reduce),
                            _ => Err(E::unknown_field(value)),
                        }
                    }
                }

                deserializer.deserialize(Visitor)
            }
        }

        struct Visitor;

        impl serde::de::Visitor for Visitor {
            type Value = ViewFunction;

            fn visit_map<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
                where V: serde::de::MapVisitor
            {
                let mut map = None;
                let mut reduce = None;

                loop {
                    match try!(visitor.visit_key()) {
                        Some(Field::Map) => {
                            map = Some(try!(visitor.visit_value()));
                        }
                        Some(Field::Reduce) => {
                            reduce = Some(try!(visitor.visit_value()));
                        }
                        None => {
                            break;
                        }
                    }
                }

                try!(visitor.end());

                let map = match map {
                    Some(x) => x,
                    None => try!(visitor.missing_field("map")),
                };

                Ok(ViewFunction {
                    map: map,
                    reduce: reduce,
                    _dummy: std::marker::PhantomData,
                })
            }
        }

        static FIELDS: &'static [&'static str] = &["map", "reduce"];
        deserializer.deserialize_struct("SavedAttachment", FIELDS, Visitor)
    }
}

impl serde::Serialize for ViewFunction {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        let len = if self.reduce.is_some() { 2 } else { 1 };
        let mut state = try!(serializer.serialize_struct("ViewFunction", len));
        try!(serializer.serialize_struct_elt(&mut state, "map", &self.map));
        if let Some(ref reduce) = self.reduce {
            try!(serializer.serialize_struct_elt(&mut state, "reduce", reduce));
        }
        serializer.serialize_struct_end(state)
    }
}

/// Container for the content of a design document.
///
/// `Design` is a convenience type for applications that create, read, or update
/// design documents.
///
/// Currently, `Design` supports only the `views` field of a design document.
/// For more information about design documents, please see the CouchDB
/// documentation.
///
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Design {
    /// The view functions stored within the design document, if any.
    pub views: std::collections::HashMap<ViewName, ViewFunction>,

    // This field exists to prevent applications from directly constructing this
    // struct.
    _dummy: std::marker::PhantomData<()>,
}

impl serde::Deserialize for Design {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: serde::Deserializer
    {
        enum Field {
            Views,
        }

        impl serde::Deserialize for Field {
            fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
                where D: serde::Deserializer
            {
                struct Visitor;

                impl serde::de::Visitor for Visitor {
                    type Value = Field;

                    fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                        where E: serde::de::Error
                    {
                        match value {
                            "views" => Ok(Field::Views),
                            _ => Err(E::unknown_field(value)),
                        }
                    }
                }

                deserializer.deserialize(Visitor)
            }
        }

        struct Visitor;

        impl serde::de::Visitor for Visitor {
            type Value = Design;

            fn visit_map<V>(&mut self, mut visitor: V) -> Result<Self::Value, V::Error>
                where V: serde::de::MapVisitor
            {
                let mut views = None;

                loop {
                    match try!(visitor.visit_key()) {
                        Some(Field::Views) => {
                            views = Some(try!(visitor.visit_value()));
                        }
                        None => {
                            break;
                        }
                    }
                }

                try!(visitor.end());

                let views = match views {
                    Some(x) => x,
                    None => std::collections::HashMap::new(),
                };

                Ok(Design {
                    views: views,
                    _dummy: std::marker::PhantomData,
                })
            }
        }

        static FIELDS: &'static [&'static str] = &["views"];
        deserializer.deserialize_struct("Design", FIELDS, Visitor)
    }
}

impl serde::Serialize for Design {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        let mut state = try!(serializer.serialize_struct("Design", 1));
        try!(serializer.serialize_struct_elt(&mut state, "views", &self.views));
        serializer.serialize_struct_end(state)
    }
}

/// Builder for a design document's content.
///
/// `Builder` is a convenience type for applications that create new design
/// documents. For more information about design documents, please see the
/// CouchDB documentation.
///
#[derive(Debug)]
pub struct DesignBuilder {
    inner: Design,
}

impl DesignBuilder {
    /// Constructs a new builder containing empty design document content.
    pub fn new() -> Self {
        DesignBuilder {
            inner: Design {
                views: std::collections::HashMap::new(),
                _dummy: std::marker::PhantomData,
            },
        }
    }

    /// Returns the builder's design document content.
    pub fn unwrap(self) -> Design {
        self.inner
    }

    /// Inserts a view into the design document content.
    pub fn insert_view<V>(mut self, view_name: V, view_function: ViewFunction) -> Self
        where V: Into<ViewName>
    {
        self.inner.views.insert(view_name.into(), view_function);
        self
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use {serde_json, std};

    #[test]
    fn view_function_new() {

        let map_function = r#"function(doc) { emit(doc.key, doc.value); }"#;

        let expected = ViewFunction {
            map: String::from(map_function),
            reduce: None,
            _dummy: std::marker::PhantomData,
        };

        let got = ViewFunction::new(map_function);

        assert_eq!(expected, got);
    }

    #[test]
    fn view_function_new_with_reduce() {

        let map_function = r#"function(doc) { emit(doc.key, doc.value); }"#;
        let reduce_function = r#"_count"#;

        let expected = ViewFunction {
            map: String::from(map_function),
            reduce: Some(String::from(reduce_function)),
            _dummy: std::marker::PhantomData,
        };

        let got = ViewFunction::new_with_reduce(map_function, reduce_function);

        assert_eq!(expected, got);
    }

    #[test]
    fn view_function_serialize_without_reduce() {

        let view_function = ViewFunction::new("function(doc) { emit(doc.key_thing, doc.value_thing); }");

        let encoded = serde_json::to_string(&view_function).unwrap();

        let expected = serde_json::builder::ObjectBuilder::new()
            .insert("map", &view_function.map)
            .build();

        let got = serde_json::from_str(&encoded).unwrap();
        assert_eq!(expected, got);
    }

    #[test]
    fn view_function_serialize_with_reduce() {

        let view_function = ViewFunction::new_with_reduce("function(doc) { emit(doc.key_thing, doc.value_thing); }",
                                                          "_sum");

        let encoded = serde_json::to_string(&view_function).unwrap();

        let expected = serde_json::builder::ObjectBuilder::new()
            .insert("map", &view_function.map)
            .insert("reduce", &view_function.reduce)
            .build();

        let got = serde_json::from_str(&encoded).unwrap();
        assert_eq!(expected, got);
    }

    #[test]
    fn view_function_deserialize_ok_without_reduce() {

        let expected = ViewFunction::new("function(doc) { emit(doc.key_thing, doc.value_thing); }");

        let source = serde_json::builder::ObjectBuilder::new()
            .insert("map",
                    "function(doc) { emit(doc.key_thing, doc.value_thing); }")
            .build();

        let source = serde_json::to_string(&source).unwrap();
        let got = serde_json::from_str(&source).unwrap();
        assert_eq!(expected, got);
    }

    #[test]
    fn view_function_deserialize_ok_with_reduce() {

        let expected = ViewFunction::new_with_reduce("function(doc) { emit(doc.key_thing, doc.value_thing); }",
                                                     "_sum");

        let source = serde_json::builder::ObjectBuilder::new()
            .insert("map",
                    "function(doc) { emit(doc.key_thing, doc.value_thing); }")
            .insert("reduce", "_sum")
            .build();

        let source = serde_json::to_string(&source).unwrap();
        let got = serde_json::from_str(&source).unwrap();
        assert_eq!(expected, got);
    }

    #[test]
    fn view_function_deserialize_nok_missing_map() {

        let source = serde_json::builder::ObjectBuilder::new()
            .insert("reduce", "_sum")
            .build();

        let source = serde_json::to_string(&source).unwrap();
        let got = serde_json::from_str::<ViewFunction>(&source);
        expect_json_error_missing_field!(got, "map");
    }

    #[test]
    fn design_serialize() {

        let design = DesignBuilder::new()
            .insert_view("alpha",
                         ViewFunction::new("function(doc) { emit(doc.key_thing, doc.value_thing); }"))
            .insert_view("bravo",
                         ViewFunction::new_with_reduce("function(doc) { emit(doc.key_thing_2, doc.value_thing_2); }",
                                                       "_sum"))
            .unwrap();

        let encoded = serde_json::to_string(&design).unwrap();

        let expected = serde_json::builder::ObjectBuilder::new()
            .insert_object("views", |x| {
                x.insert_object("alpha", |x| {
                        x.insert("map",
                                 "function(doc) { emit(doc.key_thing, doc.value_thing); }")
                    })
                    .insert_object("bravo", |x| {
                        x.insert("map",
                                    "function(doc) { emit(doc.key_thing_2, doc.value_thing_2); }")
                            .insert("reduce", "_sum")
                    })
            })
            .build();

        let got = serde_json::from_str(&encoded).unwrap();
        assert_eq!(expected, got);
    }

    #[test]
    fn design_deserialize_ok_empty() {
        let expected = DesignBuilder::new().unwrap();
        let source = serde_json::builder::ObjectBuilder::new().build();
        let source = serde_json::to_string(&source).unwrap();
        let got = serde_json::from_str(&source).unwrap();
        assert_eq!(expected, got);
    }

    #[test]
    fn design_deserialize_ok_with_views() {

        let expected = DesignBuilder::new()
            .insert_view("alpha",
                         ViewFunction::new("function(doc) { emit(doc.key_thing, doc.value_thing); }"))
            .insert_view("bravo",
                         ViewFunction::new_with_reduce("function(doc) { emit(doc.key_thing_2, doc.value_thing_2); }",
                                                       "_sum"))
            .unwrap();

        let source = serde_json::builder::ObjectBuilder::new()
            .insert_object("views", |x| {
                x.insert_object("alpha", |x| {
                        x.insert("map",
                                 "function(doc) { emit(doc.key_thing, doc.value_thing); }")
                    })
                    .insert_object("bravo", |x| {
                        x.insert("map",
                                    "function(doc) { emit(doc.key_thing_2, doc.value_thing_2); }")
                            .insert("reduce", "_sum")
                    })
            })
            .build();

        let source = serde_json::to_string(&source).unwrap();
        let got = serde_json::from_str(&source).unwrap();
        assert_eq!(expected, got);
    }
}
