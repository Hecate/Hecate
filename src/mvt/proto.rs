// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default,Debug)]
pub struct Tile {
    // message fields
    layers: ::protobuf::RepeatedField<Tile_Layer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Tile {}

impl Tile {
    pub fn new() -> Tile {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Tile {
        static mut instance: ::protobuf::lazy::Lazy<Tile> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Tile,
        };
        unsafe {
            instance.get(Tile::new)
        }
    }

    // repeated .vector_tile.Tile.Layer layers = 3;

    pub fn clear_layers(&mut self) {
        self.layers.clear();
    }

    // Param is passed by value, moved
    pub fn set_layers(&mut self, v: ::protobuf::RepeatedField<Tile_Layer>) {
        self.layers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_layers(&mut self) -> &mut ::protobuf::RepeatedField<Tile_Layer> {
        &mut self.layers
    }

    // Take field
    pub fn take_layers(&mut self) -> ::protobuf::RepeatedField<Tile_Layer> {
        ::std::mem::replace(&mut self.layers, ::protobuf::RepeatedField::new())
    }

    pub fn get_layers(&self) -> &[Tile_Layer] {
        &self.layers
    }

    fn get_layers_for_reflect(&self) -> &::protobuf::RepeatedField<Tile_Layer> {
        &self.layers
    }

    fn mut_layers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Tile_Layer> {
        &mut self.layers
    }
}

impl ::protobuf::Message for Tile {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.layers)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.layers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.layers {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Tile {
    fn new() -> Tile {
        Tile::new()
    }
}

impl ::protobuf::Clear for Tile {
    fn clear(&mut self) {
        self.clear_layers();
        self.unknown_fields.clear();
    }
}

#[derive(PartialEq,Clone,Default,Debug)]
pub struct Tile_Value {
    // message fields
    string_value: ::protobuf::SingularField<::std::string::String>,
    float_value: ::std::option::Option<f32>,
    double_value: ::std::option::Option<f64>,
    int_value: ::std::option::Option<i64>,
    uint_value: ::std::option::Option<u64>,
    sint_value: ::std::option::Option<i64>,
    bool_value: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Tile_Value {}

impl Tile_Value {
    pub fn new() -> Tile_Value {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Tile_Value {
        static mut instance: ::protobuf::lazy::Lazy<Tile_Value> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Tile_Value,
        };
        unsafe {
            instance.get(Tile_Value::new)
        }
    }

    // optional string string_value = 1;

    pub fn clear_string_value(&mut self) {
        self.string_value.clear();
    }

    pub fn has_string_value(&self) -> bool {
        self.string_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string_value(&mut self, v: ::std::string::String) {
        self.string_value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_value(&mut self) -> &mut ::std::string::String {
        if self.string_value.is_none() {
            self.string_value.set_default();
        };
        self.string_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_string_value(&mut self) -> ::std::string::String {
        self.string_value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_string_value(&self) -> &str {
        match self.string_value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_string_value_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.string_value
    }

    fn mut_string_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.string_value
    }

    // optional float float_value = 2;

    pub fn clear_float_value(&mut self) {
        self.float_value = ::std::option::Option::None;
    }

    pub fn has_float_value(&self) -> bool {
        self.float_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_float_value(&mut self, v: f32) {
        self.float_value = ::std::option::Option::Some(v);
    }

    pub fn get_float_value(&self) -> f32 {
        self.float_value.unwrap_or(0.)
    }

    fn get_float_value_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.float_value
    }

    fn mut_float_value_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.float_value
    }

    // optional double double_value = 3;

    pub fn clear_double_value(&mut self) {
        self.double_value = ::std::option::Option::None;
    }

    pub fn has_double_value(&self) -> bool {
        self.double_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_double_value(&mut self, v: f64) {
        self.double_value = ::std::option::Option::Some(v);
    }

    pub fn get_double_value(&self) -> f64 {
        self.double_value.unwrap_or(0.)
    }

    fn get_double_value_for_reflect(&self) -> &::std::option::Option<f64> {
        &self.double_value
    }

    fn mut_double_value_for_reflect(&mut self) -> &mut ::std::option::Option<f64> {
        &mut self.double_value
    }

    // optional int64 int_value = 4;

    pub fn clear_int_value(&mut self) {
        self.int_value = ::std::option::Option::None;
    }

    pub fn has_int_value(&self) -> bool {
        self.int_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_int_value(&mut self, v: i64) {
        self.int_value = ::std::option::Option::Some(v);
    }

    pub fn get_int_value(&self) -> i64 {
        self.int_value.unwrap_or(0)
    }

    fn get_int_value_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.int_value
    }

    fn mut_int_value_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.int_value
    }

    // optional uint64 uint_value = 5;

    pub fn clear_uint_value(&mut self) {
        self.uint_value = ::std::option::Option::None;
    }

    pub fn has_uint_value(&self) -> bool {
        self.uint_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uint_value(&mut self, v: u64) {
        self.uint_value = ::std::option::Option::Some(v);
    }

    pub fn get_uint_value(&self) -> u64 {
        self.uint_value.unwrap_or(0)
    }

    fn get_uint_value_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.uint_value
    }

    fn mut_uint_value_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.uint_value
    }

    // optional sint64 sint_value = 6;

    pub fn clear_sint_value(&mut self) {
        self.sint_value = ::std::option::Option::None;
    }

    pub fn has_sint_value(&self) -> bool {
        self.sint_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sint_value(&mut self, v: i64) {
        self.sint_value = ::std::option::Option::Some(v);
    }

    pub fn get_sint_value(&self) -> i64 {
        self.sint_value.unwrap_or(0)
    }

    fn get_sint_value_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.sint_value
    }

    fn mut_sint_value_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.sint_value
    }

    // optional bool bool_value = 7;

    pub fn clear_bool_value(&mut self) {
        self.bool_value = ::std::option::Option::None;
    }

    pub fn has_bool_value(&self) -> bool {
        self.bool_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bool_value(&mut self, v: bool) {
        self.bool_value = ::std::option::Option::Some(v);
    }

    pub fn get_bool_value(&self) -> bool {
        self.bool_value.unwrap_or(false)
    }

    fn get_bool_value_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.bool_value
    }

    fn mut_bool_value_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.bool_value
    }
}

impl ::protobuf::Message for Tile_Value {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.string_value)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.float_value = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_double()?;
                    self.double_value = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int64()?;
                    self.int_value = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.uint_value = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_sint64()?;
                    self.sint_value = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.bool_value = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.string_value.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.float_value {
            my_size += 5;
        };
        if let Some(v) = self.double_value {
            my_size += 9;
        };
        if let Some(v) = self.int_value {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.uint_value {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sint_value {
            my_size += ::protobuf::rt::value_varint_zigzag_size(6, v);
        };
        if let Some(v) = self.bool_value {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.string_value.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.float_value {
            os.write_float(2, v)?;
        };
        if let Some(v) = self.double_value {
            os.write_double(3, v)?;
        };
        if let Some(v) = self.int_value {
            os.write_int64(4, v)?;
        };
        if let Some(v) = self.uint_value {
            os.write_uint64(5, v)?;
        };
        if let Some(v) = self.sint_value {
            os.write_sint64(6, v)?;
        };
        if let Some(v) = self.bool_value {
            os.write_bool(7, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Tile_Value {
    fn new() -> Tile_Value {
        Tile_Value::new()
    }
}

impl ::protobuf::Clear for Tile_Value {
    fn clear(&mut self) {
        self.clear_string_value();
        self.clear_float_value();
        self.clear_double_value();
        self.clear_int_value();
        self.clear_uint_value();
        self.clear_sint_value();
        self.clear_bool_value();
        self.unknown_fields.clear();
    }
}

#[derive(PartialEq,Clone,Default,Debug)]
pub struct Tile_Feature {
    // message fields
    id: ::std::option::Option<u64>,
    tags: ::std::vec::Vec<u32>,
    field_type: ::std::option::Option<Tile_GeomType>,
    geometry: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Tile_Feature {}

impl Tile_Feature {
    pub fn new() -> Tile_Feature {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Tile_Feature {
        static mut instance: ::protobuf::lazy::Lazy<Tile_Feature> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Tile_Feature,
        };
        unsafe {
            instance.get(Tile_Feature::new)
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0u64)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // repeated uint32 tags = 2;

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_tags(&mut self, v: ::std::vec::Vec<u32>) {
        self.tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tags(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.tags
    }

    // Take field
    pub fn take_tags(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.tags, ::std::vec::Vec::new())
    }

    pub fn get_tags(&self) -> &[u32] {
        &self.tags
    }

    fn get_tags_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.tags
    }

    fn mut_tags_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.tags
    }

    // optional .vector_tile.Tile.GeomType type = 3;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Tile_GeomType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> Tile_GeomType {
        self.field_type.unwrap_or(Tile_GeomType::UNKNOWN)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<Tile_GeomType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<Tile_GeomType> {
        &mut self.field_type
    }

    // repeated uint32 geometry = 4;

    pub fn clear_geometry(&mut self) {
        self.geometry.clear();
    }

    // Param is passed by value, moved
    pub fn set_geometry(&mut self, v: ::std::vec::Vec<u32>) {
        self.geometry = v;
    }

    // Mutable pointer to the field.
    pub fn mut_geometry(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.geometry
    }

    // Take field
    pub fn take_geometry(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.geometry, ::std::vec::Vec::new())
    }

    pub fn get_geometry(&self) -> &[u32] {
        &self.geometry
    }

    fn get_geometry_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.geometry
    }

    fn mut_geometry_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.geometry
    }
}

impl ::protobuf::Message for Tile_Feature {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.tags)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.geometry)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.tags.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(2, &self.tags);
        };
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(3, v);
        };
        if !self.geometry.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(4, &self.geometry);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        };
        if !self.tags.is_empty() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.tags))?;
            for v in &self.tags {
                os.write_uint32_no_tag(*v)?;
            };
        };
        if let Some(v) = self.field_type {
            os.write_enum(3, v.value())?;
        };
        if !self.geometry.is_empty() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.geometry))?;
            for v in &self.geometry {
                os.write_uint32_no_tag(*v)?;
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Tile_Feature {
    fn new() -> Tile_Feature {
        Tile_Feature::new()
    }
}

impl ::protobuf::Clear for Tile_Feature {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_tags();
        self.clear_field_type();
        self.clear_geometry();
        self.unknown_fields.clear();
    }
}

#[derive(PartialEq,Clone,Default,Debug)]
pub struct Tile_Layer {
    // message fields
    version: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    features: ::protobuf::RepeatedField<Tile_Feature>,
    keys: ::protobuf::RepeatedField<::std::string::String>,
    values: ::protobuf::RepeatedField<Tile_Value>,
    extent: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Tile_Layer {}

impl Tile_Layer {
    pub fn new() -> Tile_Layer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Tile_Layer {
        static mut instance: ::protobuf::lazy::Lazy<Tile_Layer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Tile_Layer,
        };
        unsafe {
            instance.get(Tile_Layer::new)
        }
    }

    // required uint32 version = 15;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u32) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u32 {
        self.version.unwrap_or(1u32)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.version
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // repeated .vector_tile.Tile.Feature features = 2;

    pub fn clear_features(&mut self) {
        self.features.clear();
    }

    // Param is passed by value, moved
    pub fn set_features(&mut self, v: ::protobuf::RepeatedField<Tile_Feature>) {
        self.features = v;
    }

    // Mutable pointer to the field.
    pub fn mut_features(&mut self) -> &mut ::protobuf::RepeatedField<Tile_Feature> {
        &mut self.features
    }

    // Take field
    pub fn take_features(&mut self) -> ::protobuf::RepeatedField<Tile_Feature> {
        ::std::mem::replace(&mut self.features, ::protobuf::RepeatedField::new())
    }

    pub fn get_features(&self) -> &[Tile_Feature] {
        &self.features
    }

    fn get_features_for_reflect(&self) -> &::protobuf::RepeatedField<Tile_Feature> {
        &self.features
    }

    fn mut_features_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Tile_Feature> {
        &mut self.features
    }

    // repeated string keys = 3;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[::std::string::String] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.keys
    }

    // repeated .vector_tile.Tile.Value values = 4;

    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    // Param is passed by value, moved
    pub fn set_values(&mut self, v: ::protobuf::RepeatedField<Tile_Value>) {
        self.values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_values(&mut self) -> &mut ::protobuf::RepeatedField<Tile_Value> {
        &mut self.values
    }

    // Take field
    pub fn take_values(&mut self) -> ::protobuf::RepeatedField<Tile_Value> {
        ::std::mem::replace(&mut self.values, ::protobuf::RepeatedField::new())
    }

    pub fn get_values(&self) -> &[Tile_Value] {
        &self.values
    }

    fn get_values_for_reflect(&self) -> &::protobuf::RepeatedField<Tile_Value> {
        &self.values
    }

    fn mut_values_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Tile_Value> {
        &mut self.values
    }

    // optional uint32 extent = 5;

    pub fn clear_extent(&mut self) {
        self.extent = ::std::option::Option::None;
    }

    pub fn has_extent(&self) -> bool {
        self.extent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_extent(&mut self, v: u32) {
        self.extent = ::std::option::Option::Some(v);
    }

    pub fn get_extent(&self) -> u32 {
        self.extent.unwrap_or(4096u32)
    }

    fn get_extent_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.extent
    }

    fn mut_extent_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.extent
    }
}

impl ::protobuf::Message for Tile_Layer {
    fn is_initialized(&self) -> bool {
        if self.version.is_none() {
            return false;
        };
        if self.name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.features)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.keys)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.values)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.extent = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        for value in &self.features {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.keys {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.values {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.extent {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            os.write_uint32(15, v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        };
        for v in &self.features {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.keys {
            os.write_string(3, &v)?;
        };
        for v in &self.values {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.extent {
            os.write_uint32(5, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Tile_Layer {
    fn new() -> Tile_Layer {
        Tile_Layer::new()
    }
}

impl ::protobuf::Clear for Tile_Layer {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_name();
        self.clear_features();
        self.clear_keys();
        self.clear_values();
        self.clear_extent();
        self.unknown_fields.clear();
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Tile_GeomType {
    UNKNOWN = 0,
    POINT = 1,
    LINESTRING = 2,
    POLYGON = 3,
}

impl ::protobuf::ProtobufEnum for Tile_GeomType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Tile_GeomType> {
        match value {
            0 => ::std::option::Option::Some(Tile_GeomType::UNKNOWN),
            1 => ::std::option::Option::Some(Tile_GeomType::POINT),
            2 => ::std::option::Option::Some(Tile_GeomType::LINESTRING),
            3 => ::std::option::Option::Some(Tile_GeomType::POLYGON),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Tile_GeomType] = &[
            Tile_GeomType::UNKNOWN,
            Tile_GeomType::POINT,
            Tile_GeomType::LINESTRING,
            Tile_GeomType::POLYGON,
        ];
        values
    }
}

impl ::std::marker::Copy for Tile_GeomType {
}
