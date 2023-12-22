#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Container {
    #[prost(oneof = "container::Data", tags = "1, 2")]
    pub data: ::core::option::Option<container::Data>,
}
/// Nested message and enum types in `Container`.
pub mod container {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag = "1")]
        Foo(::prost::alloc::boxed::Box<super::Foo>),
        #[prost(message, tag = "2")]
        Bar(super::Bar),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Foo {
    #[prost(string, tag = "1")]
    pub foo: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bar {
    #[prost(message, required, boxed, tag = "1")]
    pub qux: ::prost::alloc::boxed::Box<Qux>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Qux {}
