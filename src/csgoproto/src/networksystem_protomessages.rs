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

//! Generated file from `networksystem_protomessages.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

// @@protoc_insertion_point(message:NetMessageSplitscreenUserChanged)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NetMessageSplitscreenUserChanged {
    // message fields
    // @@protoc_insertion_point(field:NetMessageSplitscreenUserChanged.slot)
    pub slot: ::std::option::Option<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:NetMessageSplitscreenUserChanged.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NetMessageSplitscreenUserChanged {
    fn default() -> &'a NetMessageSplitscreenUserChanged {
        <NetMessageSplitscreenUserChanged as ::protobuf::Message>::default_instance()
    }
}

impl NetMessageSplitscreenUserChanged {
    pub fn new() -> NetMessageSplitscreenUserChanged {
        ::std::default::Default::default()
    }

    // optional uint32 slot = 1;

    pub fn slot(&self) -> u32 {
        self.slot.unwrap_or(0)
    }

    pub fn clear_slot(&mut self) {
        self.slot = ::std::option::Option::None;
    }

    pub fn has_slot(&self) -> bool {
        self.slot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slot(&mut self, v: u32) {
        self.slot = ::std::option::Option::Some(v);
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "slot",
            |m: &NetMessageSplitscreenUserChanged| { &m.slot },
            |m: &mut NetMessageSplitscreenUserChanged| { &mut m.slot },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NetMessageSplitscreenUserChanged>(
            "NetMessageSplitscreenUserChanged",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NetMessageSplitscreenUserChanged {
    const NAME: &'static str = "NetMessageSplitscreenUserChanged";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.slot = ::std::option::Option::Some(is.read_uint32()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.slot {
            my_size += ::protobuf::rt::uint32_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.slot {
            os.write_uint32(1, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> NetMessageSplitscreenUserChanged {
        NetMessageSplitscreenUserChanged::new()
    }

    fn clear(&mut self) {
        self.slot = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NetMessageSplitscreenUserChanged {
        static instance: NetMessageSplitscreenUserChanged = NetMessageSplitscreenUserChanged {
            slot: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NetMessageSplitscreenUserChanged {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NetMessageSplitscreenUserChanged").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NetMessageSplitscreenUserChanged {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NetMessageSplitscreenUserChanged {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:NetMessageConnectionClosed)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NetMessageConnectionClosed {
    // message fields
    // @@protoc_insertion_point(field:NetMessageConnectionClosed.reason)
    pub reason: ::std::option::Option<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:NetMessageConnectionClosed.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NetMessageConnectionClosed {
    fn default() -> &'a NetMessageConnectionClosed {
        <NetMessageConnectionClosed as ::protobuf::Message>::default_instance()
    }
}

impl NetMessageConnectionClosed {
    pub fn new() -> NetMessageConnectionClosed {
        ::std::default::Default::default()
    }

    // optional uint32 reason = 1;

    pub fn reason(&self) -> u32 {
        self.reason.unwrap_or(0)
    }

    pub fn clear_reason(&mut self) {
        self.reason = ::std::option::Option::None;
    }

    pub fn has_reason(&self) -> bool {
        self.reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: u32) {
        self.reason = ::std::option::Option::Some(v);
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "reason",
            |m: &NetMessageConnectionClosed| { &m.reason },
            |m: &mut NetMessageConnectionClosed| { &mut m.reason },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NetMessageConnectionClosed>(
            "NetMessageConnectionClosed",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NetMessageConnectionClosed {
    const NAME: &'static str = "NetMessageConnectionClosed";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.reason = ::std::option::Option::Some(is.read_uint32()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.reason {
            my_size += ::protobuf::rt::uint32_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.reason {
            os.write_uint32(1, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> NetMessageConnectionClosed {
        NetMessageConnectionClosed::new()
    }

    fn clear(&mut self) {
        self.reason = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NetMessageConnectionClosed {
        static instance: NetMessageConnectionClosed = NetMessageConnectionClosed {
            reason: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NetMessageConnectionClosed {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NetMessageConnectionClosed").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NetMessageConnectionClosed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NetMessageConnectionClosed {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:NetMessageConnectionCrashed)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NetMessageConnectionCrashed {
    // message fields
    // @@protoc_insertion_point(field:NetMessageConnectionCrashed.reason)
    pub reason: ::std::option::Option<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:NetMessageConnectionCrashed.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NetMessageConnectionCrashed {
    fn default() -> &'a NetMessageConnectionCrashed {
        <NetMessageConnectionCrashed as ::protobuf::Message>::default_instance()
    }
}

impl NetMessageConnectionCrashed {
    pub fn new() -> NetMessageConnectionCrashed {
        ::std::default::Default::default()
    }

    // optional uint32 reason = 1;

    pub fn reason(&self) -> u32 {
        self.reason.unwrap_or(0)
    }

    pub fn clear_reason(&mut self) {
        self.reason = ::std::option::Option::None;
    }

    pub fn has_reason(&self) -> bool {
        self.reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: u32) {
        self.reason = ::std::option::Option::Some(v);
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "reason",
            |m: &NetMessageConnectionCrashed| { &m.reason },
            |m: &mut NetMessageConnectionCrashed| { &mut m.reason },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NetMessageConnectionCrashed>(
            "NetMessageConnectionCrashed",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NetMessageConnectionCrashed {
    const NAME: &'static str = "NetMessageConnectionCrashed";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.reason = ::std::option::Option::Some(is.read_uint32()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.reason {
            my_size += ::protobuf::rt::uint32_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.reason {
            os.write_uint32(1, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> NetMessageConnectionCrashed {
        NetMessageConnectionCrashed::new()
    }

    fn clear(&mut self) {
        self.reason = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NetMessageConnectionCrashed {
        static instance: NetMessageConnectionCrashed = NetMessageConnectionCrashed {
            reason: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NetMessageConnectionCrashed {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NetMessageConnectionCrashed").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NetMessageConnectionCrashed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NetMessageConnectionCrashed {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:NetMessagePacketStart)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NetMessagePacketStart {
    // special fields
    // @@protoc_insertion_point(special_field:NetMessagePacketStart.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NetMessagePacketStart {
    fn default() -> &'a NetMessagePacketStart {
        <NetMessagePacketStart as ::protobuf::Message>::default_instance()
    }
}

impl NetMessagePacketStart {
    pub fn new() -> NetMessagePacketStart {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(0);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NetMessagePacketStart>(
            "NetMessagePacketStart",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NetMessagePacketStart {
    const NAME: &'static str = "NetMessagePacketStart";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> NetMessagePacketStart {
        NetMessagePacketStart::new()
    }

    fn clear(&mut self) {
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NetMessagePacketStart {
        static instance: NetMessagePacketStart = NetMessagePacketStart {
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NetMessagePacketStart {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NetMessagePacketStart").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NetMessagePacketStart {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NetMessagePacketStart {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:NetMessagePacketEnd)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NetMessagePacketEnd {
    // special fields
    // @@protoc_insertion_point(special_field:NetMessagePacketEnd.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NetMessagePacketEnd {
    fn default() -> &'a NetMessagePacketEnd {
        <NetMessagePacketEnd as ::protobuf::Message>::default_instance()
    }
}

impl NetMessagePacketEnd {
    pub fn new() -> NetMessagePacketEnd {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(0);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NetMessagePacketEnd>(
            "NetMessagePacketEnd",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NetMessagePacketEnd {
    const NAME: &'static str = "NetMessagePacketEnd";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> NetMessagePacketEnd {
        NetMessagePacketEnd::new()
    }

    fn clear(&mut self) {
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NetMessagePacketEnd {
        static instance: NetMessagePacketEnd = NetMessagePacketEnd {
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NetMessagePacketEnd {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NetMessagePacketEnd").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NetMessagePacketEnd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NetMessagePacketEnd {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!networksystem_protomessages.proto\"6\n\x20NetMessageSplitscreenUserCh\
    anged\x12\x12\n\x04slot\x18\x01\x20\x01(\rR\x04slot\"4\n\x1aNetMessageCo\
    nnectionClosed\x12\x16\n\x06reason\x18\x01\x20\x01(\rR\x06reason\"5\n\
    \x1bNetMessageConnectionCrashed\x12\x16\n\x06reason\x18\x01\x20\x01(\rR\
    \x06reason\"\x17\n\x15NetMessagePacketStart\"\x15\n\x13NetMessagePacketE\
    nd\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(5);
            messages.push(NetMessageSplitscreenUserChanged::generated_message_descriptor_data());
            messages.push(NetMessageConnectionClosed::generated_message_descriptor_data());
            messages.push(NetMessageConnectionCrashed::generated_message_descriptor_data());
            messages.push(NetMessagePacketStart::generated_message_descriptor_data());
            messages.push(NetMessagePacketEnd::generated_message_descriptor_data());
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
