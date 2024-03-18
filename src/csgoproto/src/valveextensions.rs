// This file is generated by rust-protobuf 3.3.0. Do not edit
// .proto file is parsed by protoc 3.12.4
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `valveextensions.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

/// Extension fields
pub mod exts {

    pub const valve_map_field: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(61000, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const valve_map_key: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(61001, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const diff_encode_field: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, i32> = ::protobuf::ext::ExtFieldOptional::new(61002, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_INT32);

    pub const delta_ignore: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(61003, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const steamml_max_entries: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, u32> = ::protobuf::ext::ExtFieldOptional::new(61004, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_UINT32);

    pub const steamml_is_timestamp: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(61005, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const steamlearn_count: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, u32> = ::protobuf::ext::ExtFieldOptional::new(61006, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_UINT32);

    pub const schema_friendly_name: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::EnumValueOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(1000, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);

    pub const schema_description: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::EnumValueOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(1001, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);

    pub const schema_suppress_enumerator: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::EnumValueOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(1002, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15valveextensions.proto\x1a\x20google/protobuf/descriptor.proto:N\n\
    \x0fvalve_map_field\x18\xc8\xdc\x03\x20\x01(\x08\x12\x1d.google.protobuf\
    .FieldOptions:\x05falseR\rvalveMapField:J\n\rvalve_map_key\x18\xc9\xdc\
    \x03\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptions:\x05falseR\x0bval\
    veMapKey:N\n\x11diff_encode_field\x18\xca\xdc\x03\x20\x01(\x05\x12\x1d.g\
    oogle.protobuf.FieldOptions:\x010R\x0fdiffEncodeField:I\n\x0cdelta_ignor\
    e\x18\xcb\xdc\x03\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptions:\x05\
    falseR\x0bdeltaIgnore:R\n\x13steamml_max_entries\x18\xcc\xdc\x03\x20\x01\
    (\r\x12\x1d.google.protobuf.FieldOptions:\x010R\x11steammlMaxEntries:X\n\
    \x14steamml_is_timestamp\x18\xcd\xdc\x03\x20\x01(\x08\x12\x1d.google.pro\
    tobuf.FieldOptions:\x05falseR\x12steammlIsTimestamp:M\n\x10steamlearn_co\
    unt\x18\xce\xdc\x03\x20\x01(\r\x12\x1d.google.protobuf.FieldOptions:\x01\
    0R\x0fsteamlearnCount:T\n\x14schema_friendly_name\x18\xe8\x07\x20\x01(\t\
    \x12!.google.protobuf.EnumValueOptionsR\x12schemaFriendlyName:Q\n\x12sch\
    ema_description\x18\xe9\x07\x20\x01(\t\x12!.google.protobuf.EnumValueOpt\
    ionsR\x11schemaDescription:`\n\x1aschema_suppress_enumerator\x18\xea\x07\
    \x20\x01(\x08\x12!.google.protobuf.EnumValueOptionsR\x18schemaSuppressEn\
    umerator\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(::protobuf::descriptor::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
