// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrabRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrabResponse {
    #[prost(bool, tag="1")]
    pub success: bool,
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {
    /// Name of a gripper
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopResponse {
}
/// Encoded file descriptor set for the `viam.component.gripper.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xa6, 0x11, 0x0a, 0x22, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x67,
    0x72, 0x69, 0x70, 0x70, 0x65, 0x72, 0x2f, 0x76, 0x31, 0x2f, 0x67, 0x72, 0x69, 0x70, 0x70, 0x65,
    0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x19, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f,
    0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x67, 0x72, 0x69, 0x70, 0x70, 0x65, 0x72, 0x2e,
    0x76, 0x31, 0x1a, 0x1c, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x61,
    0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x1a, 0x1c, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
    0x66, 0x2f, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x50,
    0x0a, 0x0b, 0x4f, 0x70, 0x65, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x12, 0x2d, 0x0a, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x18, 0x63, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x17, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62,
    0x75, 0x66, 0x2e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x52, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61,
    0x22, 0x0e, 0x0a, 0x0c, 0x4f, 0x70, 0x65, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x22, 0x50, 0x0a, 0x0b, 0x47, 0x72, 0x61, 0x62, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x12, 0x2d, 0x0a, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x18, 0x63, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x17, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x62, 0x75, 0x66, 0x2e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x52, 0x05, 0x65, 0x78, 0x74,
    0x72, 0x61, 0x22, 0x57, 0x0a, 0x0c, 0x47, 0x72, 0x61, 0x62, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x12, 0x2d, 0x0a, 0x05,
    0x65, 0x78, 0x74, 0x72, 0x61, 0x18, 0x63, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x67, 0x6f,
    0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x53, 0x74,
    0x72, 0x75, 0x63, 0x74, 0x52, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x22, 0x50, 0x0a, 0x0b, 0x53,
    0x74, 0x6f, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x2d,
    0x0a, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x18, 0x63, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e,
    0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e,
    0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x52, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x22, 0x0e, 0x0a,
    0x0c, 0x53, 0x74, 0x6f, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x32, 0xba, 0x03,
    0x0a, 0x0e, 0x47, 0x72, 0x69, 0x70, 0x70, 0x65, 0x72, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x12, 0x8b, 0x01, 0x0a, 0x04, 0x4f, 0x70, 0x65, 0x6e, 0x12, 0x26, 0x2e, 0x76, 0x69, 0x61, 0x6d,
    0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x67, 0x72, 0x69, 0x70, 0x70,
    0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x4f, 0x70, 0x65, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x1a, 0x27, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65,
    0x6e, 0x74, 0x2e, 0x67, 0x72, 0x69, 0x70, 0x70, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x4f, 0x70,
    0x65, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x32, 0x82, 0xd3, 0xe4, 0x93,
    0x02, 0x2c, 0x1a, 0x2a, 0x2f, 0x76, 0x69, 0x61, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31,
    0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x67, 0x72, 0x69, 0x70, 0x70,
    0x65, 0x72, 0x2f, 0x7b, 0x6e, 0x61, 0x6d, 0x65, 0x7d, 0x2f, 0x6f, 0x70, 0x65, 0x6e, 0x12, 0x8b,
    0x01, 0x0a, 0x04, 0x47, 0x72, 0x61, 0x62, 0x12, 0x26, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63,
    0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x67, 0x72, 0x69, 0x70, 0x70, 0x65, 0x72,
    0x2e, 0x76, 0x31, 0x2e, 0x47, 0x72, 0x61, 0x62, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a,
    0x27, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74,
    0x2e, 0x67, 0x72, 0x69, 0x70, 0x70, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x72, 0x61, 0x62,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x32, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x2c,
    0x1a, 0x2a, 0x2f, 0x76, 0x69, 0x61, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63,
    0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x67, 0x72, 0x69, 0x70, 0x70, 0x65, 0x72,
    0x2f, 0x7b, 0x6e, 0x61, 0x6d, 0x65, 0x7d, 0x2f, 0x67, 0x72, 0x61, 0x62, 0x12, 0x8b, 0x01, 0x0a,
    0x04, 0x53, 0x74, 0x6f, 0x70, 0x12, 0x26, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d,
    0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x67, 0x72, 0x69, 0x70, 0x70, 0x65, 0x72, 0x2e, 0x76,
    0x31, 0x2e, 0x53, 0x74, 0x6f, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x27, 0x2e,
    0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x67,
    0x72, 0x69, 0x70, 0x70, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x74, 0x6f, 0x70, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x32, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x2c, 0x22, 0x2a,
    0x2f, 0x76, 0x69, 0x61, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d,
    0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x67, 0x72, 0x69, 0x70, 0x70, 0x65, 0x72, 0x2f, 0x7b,
    0x6e, 0x61, 0x6d, 0x65, 0x7d, 0x2f, 0x73, 0x74, 0x6f, 0x70, 0x42, 0x45, 0x0a, 0x1d, 0x63, 0x6f,
    0x6d, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74,
    0x2e, 0x67, 0x72, 0x69, 0x70, 0x70, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x5a, 0x24, 0x67, 0x6f, 0x2e,
    0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x63, 0x6f, 0x6d,
    0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x67, 0x72, 0x69, 0x70, 0x70, 0x65, 0x72, 0x2f, 0x76,
    0x31, 0x4a, 0xad, 0x09, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x39, 0x17, 0x0a, 0x08, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x22,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x26, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x05, 0x00, 0x26, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x3b,
    0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x07, 0x00, 0x3b, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x08, 0x00, 0x36, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x08, 0x00, 0x36,
    0x0a, 0x4d, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x0b, 0x00, 0x20, 0x01, 0x1a, 0x41, 0x20, 0x41,
    0x6e, 0x20, 0x47, 0x72, 0x69, 0x70, 0x70, 0x65, 0x72, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x73, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x67, 0x72,
    0x69, 0x70, 0x70, 0x65, 0x72, 0x73, 0x20, 0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65,
    0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x61, 0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x16, 0x0a, 0x3d, 0x0a, 0x04, 0x06,
    0x00, 0x02, 0x00, 0x12, 0x04, 0x0d, 0x02, 0x11, 0x03, 0x1a, 0x2f, 0x20, 0x4f, 0x70, 0x65, 0x6e,
    0x20, 0x6f, 0x70, 0x65, 0x6e, 0x73, 0x20, 0x61, 0x20, 0x67, 0x72, 0x69, 0x70, 0x70, 0x65, 0x72,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x6c, 0x79, 0x69,
    0x6e, 0x67, 0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x06, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x03, 0x0d, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x0d, 0x21, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0e,
    0x04, 0x10, 0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x00, 0x04, 0xb0, 0xca, 0xbc, 0x22,
    0x12, 0x04, 0x0e, 0x04, 0x10, 0x06, 0x0a, 0x48, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x04,
    0x14, 0x02, 0x18, 0x03, 0x1a, 0x3a, 0x20, 0x47, 0x72, 0x61, 0x62, 0x20, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x73, 0x20, 0x61, 0x20, 0x67, 0x72, 0x69, 0x70, 0x70, 0x65, 0x72, 0x20, 0x6f,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x6c, 0x79, 0x69, 0x6e, 0x67,
    0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x67, 0x72, 0x61, 0x62, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x14, 0x06, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x14, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x14, 0x21, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x15, 0x04, 0x17, 0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00, 0x02,
    0x01, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x12, 0x04, 0x15, 0x04, 0x17, 0x06, 0x0a, 0x2c, 0x0a, 0x04,
    0x06, 0x00, 0x02, 0x02, 0x12, 0x04, 0x1b, 0x02, 0x1f, 0x03, 0x1a, 0x1e, 0x20, 0x53, 0x74, 0x6f,
    0x70, 0x20, 0x73, 0x74, 0x6f, 0x70, 0x73, 0x20, 0x61, 0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x27,
    0x73, 0x20, 0x67, 0x72, 0x69, 0x70, 0x70, 0x65, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x06, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x1b, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x1b, 0x21, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x1c,
    0x04, 0x1e, 0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x02, 0x04, 0xb0, 0xca, 0xbc, 0x22,
    0x12, 0x04, 0x1c, 0x04, 0x1e, 0x06, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x22, 0x00,
    0x25, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x22, 0x08, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x23, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x23, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x23, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x23, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x24,
    0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x24, 0x02, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x24, 0x19, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x24, 0x21, 0x23, 0x0a, 0x09, 0x0a, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x27, 0x00, 0x17, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x27, 0x08, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x29, 0x00, 0x2c, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x29, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x00, 0x12, 0x03, 0x2a, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x2a, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x2a, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2a,
    0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x02, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x2b, 0x02, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2b, 0x19, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2b, 0x21, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12,
    0x04, 0x2e, 0x00, 0x31, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x2e, 0x08,
    0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x2f, 0x02, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2f, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2f, 0x07, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x2f, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01,
    0x12, 0x03, 0x30, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x30, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x30, 0x19,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x30, 0x21, 0x23, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x33, 0x00, 0x37, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x04, 0x01, 0x12, 0x03, 0x33, 0x08, 0x13, 0x0a, 0x20, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12,
    0x03, 0x35, 0x02, 0x12, 0x1a, 0x13, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x61,
    0x20, 0x67, 0x72, 0x69, 0x70, 0x70, 0x65, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x35, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x35, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x35, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x36, 0x02, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x36, 0x02, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x36, 0x19, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x36, 0x21, 0x23, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x05,
    0x12, 0x03, 0x39, 0x00, 0x17, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x39, 0x08,
    0x14, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("viam.component.gripper.v1.tonic.rs");
// @@protoc_insertion_point(module)