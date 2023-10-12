// This file is generated by rust-protobuf 3.3.0. Do not edit
// .proto file is parsed by protoc 3.19.1
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

//! Generated file from `steammessages_helprequest.steamworkssdk.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

// @@protoc_insertion_point(message:CHelpRequestLogs_UploadUserApplicationLog_Request)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CHelpRequestLogs_UploadUserApplicationLog_Request {
    // message fields
    // @@protoc_insertion_point(field:CHelpRequestLogs_UploadUserApplicationLog_Request.appid)
    pub appid: ::std::option::Option<u32>,
    // @@protoc_insertion_point(field:CHelpRequestLogs_UploadUserApplicationLog_Request.log_type)
    pub log_type: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:CHelpRequestLogs_UploadUserApplicationLog_Request.version_string)
    pub version_string: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:CHelpRequestLogs_UploadUserApplicationLog_Request.log_contents)
    pub log_contents: ::std::option::Option<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:CHelpRequestLogs_UploadUserApplicationLog_Request.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CHelpRequestLogs_UploadUserApplicationLog_Request {
    fn default() -> &'a CHelpRequestLogs_UploadUserApplicationLog_Request {
        <CHelpRequestLogs_UploadUserApplicationLog_Request as ::protobuf::Message>::default_instance()
    }
}

impl CHelpRequestLogs_UploadUserApplicationLog_Request {
    pub fn new() -> CHelpRequestLogs_UploadUserApplicationLog_Request {
        ::std::default::Default::default()
    }

    // optional uint32 appid = 1;

    pub fn appid(&self) -> u32 {
        self.appid.unwrap_or(0)
    }

    pub fn clear_appid(&mut self) {
        self.appid = ::std::option::Option::None;
    }

    pub fn has_appid(&self) -> bool {
        self.appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_appid(&mut self, v: u32) {
        self.appid = ::std::option::Option::Some(v);
    }

    // optional string log_type = 2;

    pub fn log_type(&self) -> &str {
        match self.log_type.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_log_type(&mut self) {
        self.log_type = ::std::option::Option::None;
    }

    pub fn has_log_type(&self) -> bool {
        self.log_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_log_type(&mut self, v: ::std::string::String) {
        self.log_type = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_log_type(&mut self) -> &mut ::std::string::String {
        if self.log_type.is_none() {
            self.log_type = ::std::option::Option::Some(::std::string::String::new());
        }
        self.log_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_log_type(&mut self) -> ::std::string::String {
        self.log_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string version_string = 3;

    pub fn version_string(&self) -> &str {
        match self.version_string.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_version_string(&mut self) {
        self.version_string = ::std::option::Option::None;
    }

    pub fn has_version_string(&self) -> bool {
        self.version_string.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version_string(&mut self, v: ::std::string::String) {
        self.version_string = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version_string(&mut self) -> &mut ::std::string::String {
        if self.version_string.is_none() {
            self.version_string = ::std::option::Option::Some(::std::string::String::new());
        }
        self.version_string.as_mut().unwrap()
    }

    // Take field
    pub fn take_version_string(&mut self) -> ::std::string::String {
        self.version_string.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional string log_contents = 4;

    pub fn log_contents(&self) -> &str {
        match self.log_contents.as_ref() {
            Some(v) => v,
            None => "",
        }
    }

    pub fn clear_log_contents(&mut self) {
        self.log_contents = ::std::option::Option::None;
    }

    pub fn has_log_contents(&self) -> bool {
        self.log_contents.is_some()
    }

    // Param is passed by value, moved
    pub fn set_log_contents(&mut self, v: ::std::string::String) {
        self.log_contents = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_log_contents(&mut self) -> &mut ::std::string::String {
        if self.log_contents.is_none() {
            self.log_contents = ::std::option::Option::Some(::std::string::String::new());
        }
        self.log_contents.as_mut().unwrap()
    }

    // Take field
    pub fn take_log_contents(&mut self) -> ::std::string::String {
        self.log_contents.take().unwrap_or_else(|| ::std::string::String::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "appid",
            |m: &CHelpRequestLogs_UploadUserApplicationLog_Request| { &m.appid },
            |m: &mut CHelpRequestLogs_UploadUserApplicationLog_Request| { &mut m.appid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "log_type",
            |m: &CHelpRequestLogs_UploadUserApplicationLog_Request| { &m.log_type },
            |m: &mut CHelpRequestLogs_UploadUserApplicationLog_Request| { &mut m.log_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "version_string",
            |m: &CHelpRequestLogs_UploadUserApplicationLog_Request| { &m.version_string },
            |m: &mut CHelpRequestLogs_UploadUserApplicationLog_Request| { &mut m.version_string },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "log_contents",
            |m: &CHelpRequestLogs_UploadUserApplicationLog_Request| { &m.log_contents },
            |m: &mut CHelpRequestLogs_UploadUserApplicationLog_Request| { &mut m.log_contents },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CHelpRequestLogs_UploadUserApplicationLog_Request>(
            "CHelpRequestLogs_UploadUserApplicationLog_Request",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CHelpRequestLogs_UploadUserApplicationLog_Request {
    const NAME: &'static str = "CHelpRequestLogs_UploadUserApplicationLog_Request";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.appid = ::std::option::Option::Some(is.read_uint32()?);
                },
                18 => {
                    self.log_type = ::std::option::Option::Some(is.read_string()?);
                },
                26 => {
                    self.version_string = ::std::option::Option::Some(is.read_string()?);
                },
                34 => {
                    self.log_contents = ::std::option::Option::Some(is.read_string()?);
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
        if let Some(v) = self.appid {
            my_size += ::protobuf::rt::uint32_size(1, v);
        }
        if let Some(v) = self.log_type.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.version_string.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.log_contents.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.appid {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.log_type.as_ref() {
            os.write_string(2, v)?;
        }
        if let Some(v) = self.version_string.as_ref() {
            os.write_string(3, v)?;
        }
        if let Some(v) = self.log_contents.as_ref() {
            os.write_string(4, v)?;
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

    fn new() -> CHelpRequestLogs_UploadUserApplicationLog_Request {
        CHelpRequestLogs_UploadUserApplicationLog_Request::new()
    }

    fn clear(&mut self) {
        self.appid = ::std::option::Option::None;
        self.log_type = ::std::option::Option::None;
        self.version_string = ::std::option::Option::None;
        self.log_contents = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CHelpRequestLogs_UploadUserApplicationLog_Request {
        static instance: CHelpRequestLogs_UploadUserApplicationLog_Request = CHelpRequestLogs_UploadUserApplicationLog_Request {
            appid: ::std::option::Option::None,
            log_type: ::std::option::Option::None,
            version_string: ::std::option::Option::None,
            log_contents: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CHelpRequestLogs_UploadUserApplicationLog_Request {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CHelpRequestLogs_UploadUserApplicationLog_Request").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CHelpRequestLogs_UploadUserApplicationLog_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CHelpRequestLogs_UploadUserApplicationLog_Request {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:CHelpRequestLogs_UploadUserApplicationLog_Response)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CHelpRequestLogs_UploadUserApplicationLog_Response {
    // message fields
    // @@protoc_insertion_point(field:CHelpRequestLogs_UploadUserApplicationLog_Response.id)
    pub id: ::std::option::Option<u64>,
    // special fields
    // @@protoc_insertion_point(special_field:CHelpRequestLogs_UploadUserApplicationLog_Response.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CHelpRequestLogs_UploadUserApplicationLog_Response {
    fn default() -> &'a CHelpRequestLogs_UploadUserApplicationLog_Response {
        <CHelpRequestLogs_UploadUserApplicationLog_Response as ::protobuf::Message>::default_instance()
    }
}

impl CHelpRequestLogs_UploadUserApplicationLog_Response {
    pub fn new() -> CHelpRequestLogs_UploadUserApplicationLog_Response {
        ::std::default::Default::default()
    }

    // optional uint64 id = 1;

    pub fn id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

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

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "id",
            |m: &CHelpRequestLogs_UploadUserApplicationLog_Response| { &m.id },
            |m: &mut CHelpRequestLogs_UploadUserApplicationLog_Response| { &mut m.id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CHelpRequestLogs_UploadUserApplicationLog_Response>(
            "CHelpRequestLogs_UploadUserApplicationLog_Response",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CHelpRequestLogs_UploadUserApplicationLog_Response {
    const NAME: &'static str = "CHelpRequestLogs_UploadUserApplicationLog_Response";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.id = ::std::option::Option::Some(is.read_uint64()?);
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::uint64_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
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

    fn new() -> CHelpRequestLogs_UploadUserApplicationLog_Response {
        CHelpRequestLogs_UploadUserApplicationLog_Response::new()
    }

    fn clear(&mut self) {
        self.id = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CHelpRequestLogs_UploadUserApplicationLog_Response {
        static instance: CHelpRequestLogs_UploadUserApplicationLog_Response = CHelpRequestLogs_UploadUserApplicationLog_Response {
            id: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CHelpRequestLogs_UploadUserApplicationLog_Response {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CHelpRequestLogs_UploadUserApplicationLog_Response").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CHelpRequestLogs_UploadUserApplicationLog_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CHelpRequestLogs_UploadUserApplicationLog_Response {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n-steammessages_helprequest.steamworkssdk.proto\x1a.steammessages_unifi\
    ed_base.steamworkssdk.proto\"\xae\x01\n1CHelpRequestLogs_UploadUserAppli\
    cationLog_Request\x12\x14\n\x05appid\x18\x01\x20\x01(\rR\x05appid\x12\
    \x19\n\x08log_type\x18\x02\x20\x01(\tR\x07logType\x12%\n\x0eversion_stri\
    ng\x18\x03\x20\x01(\tR\rversionString\x12!\n\x0clog_contents\x18\x04\x20\
    \x01(\tR\x0blogContents\"D\n2CHelpRequestLogs_UploadUserApplicationLog_R\
    esponse\x12\x0e\n\x02id\x18\x01\x20\x01(\x04R\x02id2\xee\x01\n\x0fHelpRe\
    questLogs\x12\xa8\x01\n\x18UploadUserApplicationLog\x122.CHelpRequestLog\
    s_UploadUserApplicationLog_Request\x1a3.CHelpRequestLogs_UploadUserAppli\
    cationLog_Response\"#\x82\xb5\x18\x1fUser\x20uploading\x20application\
    \x20logs\x1a0\x82\xb5\x18,Service\x20for\x20dealing\x20with\x20user-subm\
    itted\x20logsB\x03\x80\x01\x01\
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
            deps.push(super::steammessages_unified_base_steamworkssdk::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(CHelpRequestLogs_UploadUserApplicationLog_Request::generated_message_descriptor_data());
            messages.push(CHelpRequestLogs_UploadUserApplicationLog_Response::generated_message_descriptor_data());
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