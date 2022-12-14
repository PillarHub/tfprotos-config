/// Dimensions of a tensor.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TensorShapeProto {
    /// Dimensions of the tensor, such as {"input", 30}, {"output", 40}
    /// for a 30 x 40 2D tensor.  If an entry has size -1, this
    /// corresponds to a dimension of unknown size. The names are
    /// optional.
    ///
    /// The order of entries in "dim" matters: It indicates the layout of the
    /// values in the tensor in-memory representation.
    ///
    /// The first entry in "dim" is the outermost dimension used to layout the
    /// values, the last entry is the innermost dimension.  This matches the
    /// in-memory layout of RowMajor Eigen tensors.
    ///
    /// If "dim.size()" > 0, "unknown_rank" must be false.
    #[prost(message, repeated, tag = "2")]
    pub dim: ::prost::alloc::vec::Vec<tensor_shape_proto::Dim>,
    /// If true, the number of dimensions in the shape is unknown.
    ///
    /// If true, "dim.size()" must be 0.
    #[prost(bool, tag = "3")]
    pub unknown_rank: bool,
}
/// Nested message and enum types in `TensorShapeProto`.
pub mod tensor_shape_proto {
    /// One dimension of the tensor.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Dim {
        /// Size of the tensor in that dimension.
        /// This value must be >= -1, but values of -1 are reserved for "unknown"
        /// shapes (values of -1 mean "unknown" dimension).  Certain wrappers
        /// that work with TensorShapeProto may fail at runtime when deserializing
        /// a TensorShapeProto containing a dim value of -1.
        #[prost(int64, tag = "1")]
        pub size: i64,
        /// Optional name of the tensor dimension.
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
    }
}
/// Represents a serialized tf.dtypes.Dtype
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerializedDType {
    #[prost(enumeration = "DataType", tag = "1")]
    pub datatype: i32,
}
/// (== suppress_warning documentation-presence ==)
/// LINT.IfChange
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataType {
    /// Not a legal value for DataType.  Used to indicate a DataType field
    /// has not been set.
    DtInvalid = 0,
    /// Data types that all computation devices are expected to be
    /// capable to support.
    DtFloat = 1,
    DtDouble = 2,
    DtInt32 = 3,
    DtUint8 = 4,
    DtInt16 = 5,
    DtInt8 = 6,
    DtString = 7,
    /// Single-precision complex
    DtComplex64 = 8,
    DtInt64 = 9,
    DtBool = 10,
    /// Quantized int8
    DtQint8 = 11,
    /// Quantized uint8
    DtQuint8 = 12,
    /// Quantized int32
    DtQint32 = 13,
    /// Float32 truncated to 16 bits.  Only for cast ops.
    DtBfloat16 = 14,
    /// Quantized int16
    DtQint16 = 15,
    /// Quantized uint16
    DtQuint16 = 16,
    DtUint16 = 17,
    /// Double-precision complex
    DtComplex128 = 18,
    DtHalf = 19,
    DtResource = 20,
    /// Arbitrary C++ data types
    DtVariant = 21,
    DtUint32 = 22,
    DtUint64 = 23,
    /// Do not use!  These are only for parameters.  Every enum above
    /// should have a corresponding value below (verified by types_test).
    DtFloatRef = 101,
    DtDoubleRef = 102,
    DtInt32Ref = 103,
    DtUint8Ref = 104,
    DtInt16Ref = 105,
    DtInt8Ref = 106,
    DtStringRef = 107,
    DtComplex64Ref = 108,
    DtInt64Ref = 109,
    DtBoolRef = 110,
    DtQint8Ref = 111,
    DtQuint8Ref = 112,
    DtQint32Ref = 113,
    DtBfloat16Ref = 114,
    DtQint16Ref = 115,
    DtQuint16Ref = 116,
    DtUint16Ref = 117,
    DtComplex128Ref = 118,
    DtHalfRef = 119,
    DtResourceRef = 120,
    DtVariantRef = 121,
    DtUint32Ref = 122,
    DtUint64Ref = 123,
}
impl DataType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DataType::DtInvalid => "DT_INVALID",
            DataType::DtFloat => "DT_FLOAT",
            DataType::DtDouble => "DT_DOUBLE",
            DataType::DtInt32 => "DT_INT32",
            DataType::DtUint8 => "DT_UINT8",
            DataType::DtInt16 => "DT_INT16",
            DataType::DtInt8 => "DT_INT8",
            DataType::DtString => "DT_STRING",
            DataType::DtComplex64 => "DT_COMPLEX64",
            DataType::DtInt64 => "DT_INT64",
            DataType::DtBool => "DT_BOOL",
            DataType::DtQint8 => "DT_QINT8",
            DataType::DtQuint8 => "DT_QUINT8",
            DataType::DtQint32 => "DT_QINT32",
            DataType::DtBfloat16 => "DT_BFLOAT16",
            DataType::DtQint16 => "DT_QINT16",
            DataType::DtQuint16 => "DT_QUINT16",
            DataType::DtUint16 => "DT_UINT16",
            DataType::DtComplex128 => "DT_COMPLEX128",
            DataType::DtHalf => "DT_HALF",
            DataType::DtResource => "DT_RESOURCE",
            DataType::DtVariant => "DT_VARIANT",
            DataType::DtUint32 => "DT_UINT32",
            DataType::DtUint64 => "DT_UINT64",
            DataType::DtFloatRef => "DT_FLOAT_REF",
            DataType::DtDoubleRef => "DT_DOUBLE_REF",
            DataType::DtInt32Ref => "DT_INT32_REF",
            DataType::DtUint8Ref => "DT_UINT8_REF",
            DataType::DtInt16Ref => "DT_INT16_REF",
            DataType::DtInt8Ref => "DT_INT8_REF",
            DataType::DtStringRef => "DT_STRING_REF",
            DataType::DtComplex64Ref => "DT_COMPLEX64_REF",
            DataType::DtInt64Ref => "DT_INT64_REF",
            DataType::DtBoolRef => "DT_BOOL_REF",
            DataType::DtQint8Ref => "DT_QINT8_REF",
            DataType::DtQuint8Ref => "DT_QUINT8_REF",
            DataType::DtQint32Ref => "DT_QINT32_REF",
            DataType::DtBfloat16Ref => "DT_BFLOAT16_REF",
            DataType::DtQint16Ref => "DT_QINT16_REF",
            DataType::DtQuint16Ref => "DT_QUINT16_REF",
            DataType::DtUint16Ref => "DT_UINT16_REF",
            DataType::DtComplex128Ref => "DT_COMPLEX128_REF",
            DataType::DtHalfRef => "DT_HALF_REF",
            DataType::DtResourceRef => "DT_RESOURCE_REF",
            DataType::DtVariantRef => "DT_VARIANT_REF",
            DataType::DtUint32Ref => "DT_UINT32_REF",
            DataType::DtUint64Ref => "DT_UINT64_REF",
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CostGraphDef {
    #[prost(message, repeated, tag = "1")]
    pub node: ::prost::alloc::vec::Vec<cost_graph_def::Node>,
    #[prost(message, repeated, tag = "2")]
    pub cost: ::prost::alloc::vec::Vec<cost_graph_def::AggregatedCost>,
}
/// Nested message and enum types in `CostGraphDef`.
pub mod cost_graph_def {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Node {
        /// The name of the node. Names are globally unique.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The device of the node. Can be empty if the node is mapped to the
        /// default partition or partitioning hasn't been run yet.
        #[prost(string, tag = "2")]
        pub device: ::prost::alloc::string::String,
        /// The id of the node. Node ids are only unique inside a partition.
        #[prost(int32, tag = "3")]
        pub id: i32,
        #[prost(message, repeated, tag = "4")]
        pub input_info: ::prost::alloc::vec::Vec<node::InputInfo>,
        #[prost(message, repeated, tag = "5")]
        pub output_info: ::prost::alloc::vec::Vec<node::OutputInfo>,
        /// Temporary memory used by this node.
        #[prost(int64, tag = "6")]
        pub temporary_memory_size: i64,
        /// Persistent memory used by this node.
        #[prost(int64, tag = "12")]
        pub persistent_memory_size: i64,
        #[deprecated]
        #[prost(int64, tag = "10")]
        pub host_temp_memory_size: i64,
        #[deprecated]
        #[prost(int64, tag = "11")]
        pub device_temp_memory_size: i64,
        #[deprecated]
        #[prost(int64, tag = "16")]
        pub device_persistent_memory_size: i64,
        /// Estimate of the computational cost of this node, in microseconds.
        #[prost(int64, tag = "9")]
        pub compute_cost: i64,
        /// Analytical estimate of the computational cost of this node, in
        /// microseconds.
        #[prost(int64, tag = "14")]
        pub compute_time: i64,
        /// Analytical estimate of the memory access cost of this node, in
        /// microseconds.
        #[prost(int64, tag = "15")]
        pub memory_time: i64,
        /// If true, the output is permanent: it can't be discarded, because this
        /// node is part of the "final output". Nodes may depend on final nodes.
        #[prost(bool, tag = "7")]
        pub is_final: bool,
        /// Ids of the control inputs for this node.
        #[prost(int32, repeated, tag = "8")]
        pub control_input: ::prost::alloc::vec::Vec<i32>,
        /// Are the costs inaccurate?
        #[prost(bool, tag = "17")]
        pub inaccurate: bool,
    }
    /// Nested message and enum types in `Node`.
    pub mod node {
        /// Inputs of this node. They must be executed before this node can be
        /// executed. An input is a particular output of another node, specified
        /// by the node id and the output index.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct InputInfo {
            #[prost(int32, tag = "1")]
            pub preceding_node: i32,
            #[prost(int32, tag = "2")]
            pub preceding_port: i32,
        }
        /// Outputs of this node.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct OutputInfo {
            #[prost(int64, tag = "1")]
            pub size: i64,
            /// If >= 0, the output is an alias of an input. Note that an alias input
            /// may itself be an alias. The algorithm will therefore need to follow
            /// those pointers.
            #[prost(int64, tag = "2")]
            pub alias_input_port: i64,
            #[prost(message, optional, tag = "3")]
            pub shape: ::core::option::Option<super::super::TensorShapeProto>,
            #[prost(enumeration = "super::super::DataType", tag = "4")]
            pub dtype: i32,
        }
    }
    /// Total cost of this graph, typically used for balancing decisions.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AggregatedCost {
        /// Aggregated cost value.
        #[prost(float, tag = "1")]
        pub cost: f32,
        /// Aggregated cost dimension (e.g. 'memory', 'compute', 'network').
        #[prost(string, tag = "2")]
        pub dimension: ::prost::alloc::string::String,
    }
}
/// Protocol buffer representing a handle to a tensorflow resource. Handles are
/// not valid across executions, but can be serialized back and forth from within
/// a single run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceHandleProto {
    /// Unique name for the device containing the resource.
    #[prost(string, tag = "1")]
    pub device: ::prost::alloc::string::String,
    /// Container in which this resource is placed.
    #[prost(string, tag = "2")]
    pub container: ::prost::alloc::string::String,
    /// Unique name of this resource.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Hash code for the type of the resource. Is only valid in the same device
    /// and in the same execution.
    #[prost(uint64, tag = "4")]
    pub hash_code: u64,
    /// For debug-only, the name of the type pointed to by this handle, if
    /// available.
    #[prost(string, tag = "5")]
    pub maybe_type_name: ::prost::alloc::string::String,
    /// Data types and shapes for the underlying resource.
    #[prost(message, repeated, tag = "6")]
    pub dtypes_and_shapes: ::prost::alloc::vec::Vec<
        resource_handle_proto::DtypeAndShape,
    >,
}
/// Nested message and enum types in `ResourceHandleProto`.
pub mod resource_handle_proto {
    /// Protocol buffer representing a pair of (data type, tensor shape).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DtypeAndShape {
        #[prost(enumeration = "super::DataType", tag = "1")]
        pub dtype: i32,
        #[prost(message, optional, tag = "2")]
        pub shape: ::core::option::Option<super::TensorShapeProto>,
    }
}
/// Protocol buffer representing a tensor.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TensorProto {
    #[prost(enumeration = "DataType", tag = "1")]
    pub dtype: i32,
    /// Shape of the tensor.  TODO(touts): sort out the 0-rank issues.
    #[prost(message, optional, tag = "2")]
    pub tensor_shape: ::core::option::Option<TensorShapeProto>,
    /// Version number.
    ///
    /// In version 0, if the "repeated xxx" representations contain only one
    /// element, that element is repeated to fill the shape.  This makes it easy
    /// to represent a constant Tensor with a single value.
    #[prost(int32, tag = "3")]
    pub version_number: i32,
    /// Serialized raw tensor content from either Tensor::AsProtoTensorContent or
    /// memcpy in tensorflow::grpc::EncodeTensorToByteBuffer. This representation
    /// can be used for all tensor types. The purpose of this representation is to
    /// reduce serialization overhead during RPC call by avoiding serialization of
    /// many repeated small items.
    #[prost(bytes = "vec", tag = "4")]
    pub tensor_content: ::prost::alloc::vec::Vec<u8>,
    /// DT_HALF, DT_BFLOAT16. Note that since protobuf has no int16 type, we'll
    /// have some pointless zero padding for each value here.
    #[prost(int32, repeated, tag = "13")]
    pub half_val: ::prost::alloc::vec::Vec<i32>,
    /// DT_FLOAT.
    #[prost(float, repeated, tag = "5")]
    pub float_val: ::prost::alloc::vec::Vec<f32>,
    /// DT_DOUBLE.
    #[prost(double, repeated, tag = "6")]
    pub double_val: ::prost::alloc::vec::Vec<f64>,
    /// DT_INT32, DT_INT16, DT_UINT16, DT_INT8, DT_UINT8.
    #[prost(int32, repeated, tag = "7")]
    pub int_val: ::prost::alloc::vec::Vec<i32>,
    /// DT_STRING
    #[prost(bytes = "vec", repeated, tag = "8")]
    pub string_val: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// DT_COMPLEX64. scomplex_val(2*i) and scomplex_val(2*i+1) are real
    /// and imaginary parts of i-th single precision complex.
    #[prost(float, repeated, tag = "9")]
    pub scomplex_val: ::prost::alloc::vec::Vec<f32>,
    /// DT_INT64
    #[prost(int64, repeated, tag = "10")]
    pub int64_val: ::prost::alloc::vec::Vec<i64>,
    /// DT_BOOL
    #[prost(bool, repeated, tag = "11")]
    pub bool_val: ::prost::alloc::vec::Vec<bool>,
    /// DT_COMPLEX128. dcomplex_val(2*i) and dcomplex_val(2*i+1) are real
    /// and imaginary parts of i-th double precision complex.
    #[prost(double, repeated, tag = "12")]
    pub dcomplex_val: ::prost::alloc::vec::Vec<f64>,
    /// DT_RESOURCE
    #[prost(message, repeated, tag = "14")]
    pub resource_handle_val: ::prost::alloc::vec::Vec<ResourceHandleProto>,
    /// DT_VARIANT
    #[prost(message, repeated, tag = "15")]
    pub variant_val: ::prost::alloc::vec::Vec<VariantTensorDataProto>,
    /// DT_UINT32
    #[prost(uint32, repeated, tag = "16")]
    pub uint32_val: ::prost::alloc::vec::Vec<u32>,
    /// DT_UINT64
    #[prost(uint64, repeated, tag = "17")]
    pub uint64_val: ::prost::alloc::vec::Vec<u64>,
}
/// Protocol buffer representing the serialization format of DT_VARIANT tensors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VariantTensorDataProto {
    /// Name of the type of objects being serialized.
    #[prost(string, tag = "1")]
    pub type_name: ::prost::alloc::string::String,
    /// Portions of the object that are not Tensors.
    #[prost(bytes = "vec", tag = "2")]
    pub metadata: ::prost::alloc::vec::Vec<u8>,
    /// Tensors contained within objects being serialized.
    #[prost(message, repeated, tag = "3")]
    pub tensors: ::prost::alloc::vec::Vec<TensorProto>,
}
/// Protocol buffer representing the value for an attr used to configure an Op.
/// Comment indicates the corresponding attr type.  Only the field matching the
/// attr type may be filled.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttrValue {
    #[prost(oneof = "attr_value::Value", tags = "2, 3, 4, 5, 6, 7, 8, 1, 10, 9")]
    pub value: ::core::option::Option<attr_value::Value>,
}
/// Nested message and enum types in `AttrValue`.
pub mod attr_value {
    /// LINT.IfChange
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ListValue {
        /// "list(string)"
        #[prost(bytes = "vec", repeated, tag = "2")]
        pub s: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
        /// "list(int)"
        #[prost(int64, repeated, tag = "3")]
        pub i: ::prost::alloc::vec::Vec<i64>,
        /// "list(float)"
        #[prost(float, repeated, tag = "4")]
        pub f: ::prost::alloc::vec::Vec<f32>,
        /// "list(bool)"
        #[prost(bool, repeated, tag = "5")]
        pub b: ::prost::alloc::vec::Vec<bool>,
        /// "list(type)"
        #[prost(enumeration = "super::DataType", repeated, tag = "6")]
        pub r#type: ::prost::alloc::vec::Vec<i32>,
        /// "list(shape)"
        #[prost(message, repeated, tag = "7")]
        pub shape: ::prost::alloc::vec::Vec<super::TensorShapeProto>,
        /// "list(tensor)"
        #[prost(message, repeated, tag = "8")]
        pub tensor: ::prost::alloc::vec::Vec<super::TensorProto>,
        /// "list(attr)"
        #[prost(message, repeated, tag = "9")]
        pub func: ::prost::alloc::vec::Vec<super::NameAttrList>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// "string"
        #[prost(bytes, tag = "2")]
        S(::prost::alloc::vec::Vec<u8>),
        /// "int"
        #[prost(int64, tag = "3")]
        I(i64),
        /// "float"
        #[prost(float, tag = "4")]
        F(f32),
        /// "bool"
        #[prost(bool, tag = "5")]
        B(bool),
        /// "type"
        #[prost(enumeration = "super::DataType", tag = "6")]
        Type(i32),
        /// "shape"
        #[prost(message, tag = "7")]
        Shape(super::TensorShapeProto),
        /// "tensor"
        #[prost(message, tag = "8")]
        Tensor(super::TensorProto),
        /// any "list(...)"
        #[prost(message, tag = "1")]
        List(ListValue),
        /// "func" represents a function. func.name is a function's name or
        /// a primitive op's name. func.attr.first is the name of an attr
        /// defined for that function. func.attr.second is the value for
        /// that attr in the instantiation.
        #[prost(message, tag = "10")]
        Func(super::NameAttrList),
        /// This is a placeholder only used in nodes defined inside a
        /// function.  It indicates the attr value will be supplied when
        /// the function is instantiated.  For example, let us suppose a
        /// node "N" in function "FN". "N" has an attr "A" with value
        /// placeholder = "foo". When FN is instantiated with attr "foo"
        /// set to "bar", the instantiated node N's attr A will have been
        /// given the value "bar".
        #[prost(string, tag = "9")]
        Placeholder(::prost::alloc::string::String),
    }
}
/// A list of attr names and their values. The whole list is attached
/// with a string name.  E.g., MatMul\\[T=float\\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NameAttrList {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(map = "string, message", tag = "2")]
    pub attr: ::std::collections::HashMap<::prost::alloc::string::String, AttrValue>,
}
/// Highly experimental and very likely to change.
/// This encoding uses tags instead of dedicated messages for regularity. In
/// particular the encoding imposes no restrictions on what the parameters of any
/// type should be, which in particular needs to be true for type symbols.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FullTypeDef {
    /// The principal type represented by this object. This may be a concrete type
    /// (Tensor, Dataset) a type variable (used for dependent types) a type
    /// symbol (Any, Union). See FullTypeId for details.
    #[prost(enumeration = "FullTypeId", tag = "1")]
    pub type_id: i32,
    #[prost(message, repeated, tag = "2")]
    pub args: ::prost::alloc::vec::Vec<FullTypeDef>,
    /// Literal values of this type object, if the type admits one.
    /// For example, a type variable admits a string attribute - its name.
    /// Shape-related types may admit int attributes - their static shape values.
    /// Fields for more data types to be added as needed.
    #[prost(oneof = "full_type_def::Attr", tags = "3, 4")]
    pub attr: ::core::option::Option<full_type_def::Attr>,
}
/// Nested message and enum types in `FullTypeDef`.
pub mod full_type_def {
    /// Literal values of this type object, if the type admits one.
    /// For example, a type variable admits a string attribute - its name.
    /// Shape-related types may admit int attributes - their static shape values.
    /// Fields for more data types to be added as needed.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Attr {
        #[prost(string, tag = "3")]
        S(::prost::alloc::string::String),
        /// TODO(mdan): list/tensor, map? Need to reconcile with TFT_RECORD, etc.
        #[prost(int64, tag = "4")]
        I(i64),
    }
}
/// LINT.IfChange
/// Experimental. Represents the complete type information of a TensorFlow value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FullTypeId {
    /// The default represents an uninitialized values.
    TftUnset = 0,
    /// Type variables may serve as placeholder for any other type ID in type
    /// templates.
    ///
    /// Examples:
    /// TFT_DATASET\\[TFT_VAR\["T"\]\\] is a Dataset returning a type indicated by "T".
    /// TFT_TENSOR\\[TFT_VAR\["T"\]\\] is a Tensor of n element type indicated by "T".
    /// TFT_TENSOR\\[TFT_VAR\["T"\]\\], TFT_TENSOR\\[TFT_VAR\["T"\]\\] are two tensors of
    /// identical element types.
    /// TFT_TENSOR\\[TFT_VAR\["P"\]\\], TFT_TENSOR\\[TFT_VAR\["Q"\]\\] are two tensors of
    /// independent element types.
    TftVar = 1,
    /// Wildcard type. Describes a parameter of unknown type. In TensorFlow, that
    /// can mean either a "Top" type (accepts any type), or a dynamically typed
    /// object whose type is unknown in context.
    /// Important: "unknown" does not necessarily mean undeterminable!
    TftAny = 2,
    /// The algebraic product type. This is an algebraic type that may be used just
    /// for logical grouping. Not to confused with TFT_TUPLE which describes a
    /// concrete object of several elements.
    ///
    /// Example:
    /// TFT_DATASET\\[TFT_PRODUCT\[TFT_TENSOR\[TFT_INT32\\], TFT_TENSOR\\[TFT_FLOAT64\]\]\\]
    /// is a Dataset producing two tensors, an integer one and a float one.
    TftProduct = 3,
    /// Represents a named field, with the name stored in the attribute.
    ///
    /// Parametrization:
    /// TFT_NAMED\\[<type>\\]{<name>}
    ///
    /// * <type> is the type of the field
    /// * <name> is the field name, as string (thpugh can theoretically be an int
    ///   as well)
    ///
    /// Example:
    /// TFT_RECORD\[
    /// TFT_NAMED\\[TFT_TENSOR\[TFT_INT32\]\\]{'foo'},
    /// TFT_NAMED\\[TFT_TENSOR\[TFT_FLOAT32\]\\]{'bar'},
    /// \]
    /// is a structure with two fields, an int tensor "foo" and a float tensor
    /// "bar".
    TftNamed = 4,
    /// Template definition. Expands the variables by repeating a template as
    /// arguments of container.
    ///
    /// Parametrization:
    /// TFT_FOR_EACH\[\<container_type>, <template>, <expansions>\]
    ///
    /// * \<container_type> is the type of the container that the template will be
    ///   expanded into
    /// * <template> is any type definition that potentially contains type
    ///   variables
    /// * <expansions> is a TFT_VAR and may include more types in the future
    ///
    /// Example:
    /// TFT_FOR_EACH\[
    /// TFT_PRODUCT,
    /// TFT_TENSOR\\[TFT_VAR\["t"\]\\],
    /// TFT_VAR\\["t"\\]
    /// \]
    /// will substitute a T = TFT_INT32 to TFT_PRODUCT\\[TFT_TENSOR\[TFT_INT32\]\\]
    /// and a T = (TFT_INT32, TFT_INT64) to
    /// TFT_PRODUCT\\[TFT_TENSOR\[TFT_INT32\\], TFT_TENSOR\\[TFT_INT64\]\\].
    TftForEach = 20,
    /// Callable types describe functions and ops.
    ///
    /// Parametrization:
    /// TFT_CALLABLE\[<arg type>, <return type>\]
    ///
    /// * <arg type> is the type of the arguments; TFT_PRODUCT represents
    ///   multiple
    ///   arguments.
    /// * <return type> is the return type; TFT_PRODUCT represents multiple
    ///   return values (that means that callables returning multiple things
    ///   don't necessarily return a single tuple).
    ///
    /// Example:
    /// TFT_CALLABLE\[
    /// TFT_ANY,
    /// TFT_PRODUCT\\[TFT_TENSOR\[TFT_INT32\\], TFT_TENSOR\\[TFT_FLOAT64\]\\],
    /// \]
    /// is a callable with unspecified (for now) input arguments, and
    /// two return values of type tensor.
    TftCallable = 100,
    /// The usual Tensor. This is a parametric type.
    ///
    /// Parametrization:
    /// TFT_TENSOR\[<element type>, <shape type>\]
    ///
    /// * <element type> is currently limited to one of the element types
    ///   defined below.
    /// * <shape type> is not yet defined, and may only be TFT_UNKNOWN for now.
    ///
    /// A TFT_SHAPE type will be defined in the future.
    ///
    /// Example:
    /// TFT_TENSOR\[TFT_INT32, TFT_UNKNOWN\]
    /// is a Tensor of int32 element type and unknown shape.
    ///
    /// TODO(mdan): Define TFT_SHAPE and add more examples.
    TftTensor = 1000,
    /// Array (or tensorflow::TensorList in the variant type registry).
    /// Note: this is not to be confused with the deprecated `TensorArray*` ops
    /// which are not supported by FullType.
    /// This type represents a random-access list whose elements can be
    /// described by a single type. Although immutable, Array is expected to
    /// support efficient mutation semantics (i.e. element update) in the
    /// user-facing API.
    /// The element type may be generic or even TFT_ANY for a heterogenous list.
    ///
    /// Parametrization:
    /// TFT_ARRAY\[<element type>\]
    ///
    /// * <element type> may be any concrete type.
    ///
    /// Examples:
    /// TFT_ARRAY\\[TFT_TENSOR\[TFT_INT32\]\\] is a TensorArray holding int32 Tensors
    /// of any shape.
    /// TFT_ARRAY\\[TFT_TENSOR\[TFT_UNKNOWN\]\\] is a TensorArray holding Tensors of
    /// mixed element types.
    /// TFT_ARRAY\\[TFT_UNKNOWN\\] is a TensorArray holding any element type.
    /// TFT_ARRAY\\[\\] is equivalent to TFT_ARRAY\\[TFT_UNKNOWN\\].
    /// TFT_ARRAY\\[TFT_ARRAY\[\]\\] is an array or arrays (of unknown types).
    TftArray = 1001,
    /// Optional (or tensorflow::OptionalVariant in the variant type registry).
    /// This type represents a value that may either hold an element of a single
    /// specified type, or nothing at all.
    ///
    /// Parametrization:
    /// TFT_OPTIONAL\[<element type>\]
    ///
    /// * <element type> may be any concrete type.
    ///
    /// Examples:
    /// TFT_OPTIONAL\\[TFT_TENSOR\[TFT_INT32\]\\] is an Optional holding an int32
    /// Tensor of any shape.
    TftOptional = 1002,
    /// Literal types describe compile-time constant values.
    /// Literal types may also participate in dependent types.
    ///
    /// Parametrization:
    /// TFT_LITERAL\[<value type>\]{<value>}
    ///
    /// * <value type> may be any concrete type compatible that can hold <value>
    /// * <value> is the type's attribute, and holds the actual literal value
    ///
    /// Examples:
    /// TFT_LITERAL\\[TFT_INT32\\]{1} is the compile-time constant 1.
    TftLiteral = 1003,
    /// Encoding types describe a value of a certain type, encoded as a different
    /// type.
    ///
    /// Parametrization:
    /// TFT_ENCODED\[<encoded type>, <encoding type>\]
    ///
    /// * <encoded type> may be any type
    /// * <encoding type> may be any type
    ///
    /// Examples:
    /// TFT_ENCODING\[TFT_INT32, TFT_STRING\] is an integer encoded as string.
    TftEncoded = 1004,
    /// The bool element type.
    /// TODO(mdan): Quantized types, legacy representations (e.g. ref)
    TftBool = 200,
    /// Integer element types.
    TftUint8 = 201,
    TftUint16 = 202,
    TftUint32 = 203,
    TftUint64 = 204,
    TftInt8 = 205,
    TftInt16 = 206,
    TftInt32 = 207,
    TftInt64 = 208,
    /// Floating-point element types.
    TftHalf = 209,
    TftFloat = 210,
    TftDouble = 211,
    TftBfloat16 = 215,
    /// Complex element types.
    /// TODO(mdan): Represent as TFT_COMPLEX\\[TFT_DOUBLE\\] instead?
    TftComplex64 = 212,
    TftComplex128 = 213,
    /// The string element type.
    TftString = 214,
    /// Datasets created by tf.data ops and APIs. Datasets have generator/iterable
    /// semantics, that is, one can construct an iterator from them. Like
    /// Array, they are considered to return elements that can be described
    /// by a single type. Unlike Array, they do not support random access or
    /// mutation, and can potentially produce an infinite number of elements.
    /// A datasets can produce logical structures (e.g. multiple elements). This
    /// is expressed using TFT_PRODUCT.
    ///
    /// Parametrization: TFT_DATASET\[<element type>\].
    ///
    /// * <element type> may be a concrete type or a type symbol. It represents
    ///   the data type of the elements produced by the dataset.
    ///
    /// Examples:
    /// TFT_DATSET\\[TFT_TENSOR\[TFT_INT32\]\\] is a Dataset producing single int32
    /// Tensors of unknown shape.
    /// TFT_DATSET\\[TFT_PRODUCT\[TFT_TENSOR\[TFT_INT32\\], TFT_TENSOR\\[TFT_FLOAT32\]\\] is
    /// a Dataset producing pairs of Tensors, one integer and one float.
    /// Note: The high ID number is to prepare for the eventuality that Datasets
    /// will be supported by user types in the future.
    TftDataset = 10102,
    /// A ragged tensor created by tf.ragged ops and APIs.
    ///
    /// Parametrization: TFT_RAGGED\\[\<element_type>\\].
    TftRagged = 10103,
    /// Iterators created by tf.data ops and APIs. Very similar to Datasets, except
    /// they are mutable.
    ///
    /// Parametrization: TFT_ITERATOR\[<element type>\].
    ///
    /// * <element type> may be a concrete type or a type symbol. It represents
    ///   the data type of the elements produced by the dataset.
    TftIterator = 10104,
    /// A mutex lock tensor, produced by tf.raw_ops.MutexLock.
    /// Unlike strict execution models, where ownership of a lock is denoted by
    /// "running after the lock has been acquired", in non-strict mode, lock
    /// ownership is in the true sense: "the op argument representing the lock is
    /// available".
    /// Mutex locks are the dynamic counterpart of control dependencies.
    /// TODO(mdan): Properly document this thing.
    ///
    /// Parametrization: TFT_MUTEX_LOCK\\[\\].
    TftMutexLock = 10202,
    /// The equivalent of a Tensor with DT_VARIANT dtype, kept here to simplify
    /// translation. This type should not normally appear after type inference.
    /// Note that LEGACY_VARIANT != ANY: TENSOR\\[INT32\\] is a subtype of ANY, but is
    /// not a subtype of LEGACY_VARIANT.
    TftLegacyVariant = 10203,
}
impl FullTypeId {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FullTypeId::TftUnset => "TFT_UNSET",
            FullTypeId::TftVar => "TFT_VAR",
            FullTypeId::TftAny => "TFT_ANY",
            FullTypeId::TftProduct => "TFT_PRODUCT",
            FullTypeId::TftNamed => "TFT_NAMED",
            FullTypeId::TftForEach => "TFT_FOR_EACH",
            FullTypeId::TftCallable => "TFT_CALLABLE",
            FullTypeId::TftTensor => "TFT_TENSOR",
            FullTypeId::TftArray => "TFT_ARRAY",
            FullTypeId::TftOptional => "TFT_OPTIONAL",
            FullTypeId::TftLiteral => "TFT_LITERAL",
            FullTypeId::TftEncoded => "TFT_ENCODED",
            FullTypeId::TftBool => "TFT_BOOL",
            FullTypeId::TftUint8 => "TFT_UINT8",
            FullTypeId::TftUint16 => "TFT_UINT16",
            FullTypeId::TftUint32 => "TFT_UINT32",
            FullTypeId::TftUint64 => "TFT_UINT64",
            FullTypeId::TftInt8 => "TFT_INT8",
            FullTypeId::TftInt16 => "TFT_INT16",
            FullTypeId::TftInt32 => "TFT_INT32",
            FullTypeId::TftInt64 => "TFT_INT64",
            FullTypeId::TftHalf => "TFT_HALF",
            FullTypeId::TftFloat => "TFT_FLOAT",
            FullTypeId::TftDouble => "TFT_DOUBLE",
            FullTypeId::TftBfloat16 => "TFT_BFLOAT16",
            FullTypeId::TftComplex64 => "TFT_COMPLEX64",
            FullTypeId::TftComplex128 => "TFT_COMPLEX128",
            FullTypeId::TftString => "TFT_STRING",
            FullTypeId::TftDataset => "TFT_DATASET",
            FullTypeId::TftRagged => "TFT_RAGGED",
            FullTypeId::TftIterator => "TFT_ITERATOR",
            FullTypeId::TftMutexLock => "TFT_MUTEX_LOCK",
            FullTypeId::TftLegacyVariant => "TFT_LEGACY_VARIANT",
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeDef {
    /// The name given to this operator. Used for naming inputs,
    /// logging, visualization, etc.  Unique within a single GraphDef.
    /// Must match the regexp "\\[A-Za-z0-9.\]\[A-Za-z0-9\_\>./\\]\*".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The operation name.  There may be custom parameters in attrs.
    /// Op names starting with an underscore are reserved for internal use.
    #[prost(string, tag = "2")]
    pub op: ::prost::alloc::string::String,
    /// Each input is "node:src_output" with "node" being a string name and
    /// "src_output" indicating which output tensor to use from "node". If
    /// "src_output" is 0 the ":0" suffix can be omitted.  Regular inputs
    /// may optionally be followed by control inputs that have the format
    /// "^node".
    #[prost(string, repeated, tag = "3")]
    pub input: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A (possibly partial) specification for the device on which this
    /// node should be placed.
    /// The expected syntax for this string is as follows:
    ///
    /// DEVICE_SPEC ::= PARTIAL_SPEC
    ///
    /// PARTIAL_SPEC ::= ("/" CONSTRAINT) *
    /// CONSTRAINT ::= ("job:" JOB_NAME)
    /// \| ("replica:" \\[1-9\]\[0-9\\]*)
    /// \| ("task:" \\[1-9\]\[0-9\\]*)
    /// \| ("device:" \\[A-Za-z\\]\* ":" (\\[1-9\]\[0-9\\]\* | "\*") )
    ///
    /// Valid values for this string include:
    ///
    /// * "/job:worker/replica:0/task:1/device:GPU:3"  (full specification)
    /// * "/job:worker/device:GPU:3"                   (partial specification)
    /// * ""                                    (no specification)
    ///
    /// If the constraints do not resolve to a single device (or if this
    /// field is empty or not present), the runtime will attempt to
    /// choose a device automatically.
    #[prost(string, tag = "4")]
    pub device: ::prost::alloc::string::String,
    /// Operation-specific graph-construction-time configuration.
    /// Note that this should include all attrs defined in the
    /// corresponding OpDef, including those with a value matching
    /// the default -- this allows the default to change and makes
    /// NodeDefs easier to interpret on their own.  However, if
    /// an attr with a default is not specified in this list, the
    /// default will be used.
    /// The "names" (keys) must match the regexp "\\[a-z\]\[a-z0-9\_\\]+" (and
    /// one of the names from the corresponding OpDef's attr field).
    /// The values must have a type matching the corresponding OpDef
    /// attr's type field.
    /// TODO(josh11b): Add some examples here showing best practices.
    #[prost(map = "string, message", tag = "5")]
    pub attr: ::std::collections::HashMap<::prost::alloc::string::String, AttrValue>,
    /// This stores debug information associated with the node.
    #[prost(message, optional, tag = "6")]
    pub experimental_debug_info: ::core::option::Option<node_def::ExperimentalDebugInfo>,
    /// The complete type of this node. Experimental and subject to change.
    /// Currently, the field only contains the return types of the node. That will
    /// extend in the future to contain the entire signature of the node, as a
    /// function type.
    #[prost(message, optional, tag = "7")]
    pub experimental_type: ::core::option::Option<FullTypeDef>,
}
/// Nested message and enum types in `NodeDef`.
pub mod node_def {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExperimentalDebugInfo {
        /// Opaque string inserted into error messages created by the runtime.
        ///
        /// This is intended to store the list of names of the nodes from the
        /// original graph that this node was derived. For example if this node, say
        /// C, was result of a fusion of 2 nodes A and B, then 'original_node' would
        /// be {A, B}. This information can be used to map errors originating at the
        /// current node to some top level source code.
        #[prost(string, repeated, tag = "1")]
        pub original_node_names: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// This is intended to store the list of names of the functions from the
        /// original graph that this node was derived. For example if this node, say
        /// C, was result of a fusion of node A in function FA and node B in function
        /// FB, then `original_funcs` would be {FA, FB}. If the node is in the top
        /// level graph, the `original_func` is empty. This information, with the
        /// `original_node_names` can be used to map errors originating at the
        /// current ndoe to some top level source code.
        #[prost(string, repeated, tag = "2")]
        pub original_func_names: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
}
/// Defines an operation. A NodeDef in a GraphDef specifies an Op by
/// using the "op" field which should match the name of a OpDef.
/// LINT.IfChange
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpDef {
    /// Op names starting with an underscore are reserved for internal use.
    /// Names should be CamelCase and match the regexp "\\[A-Z\]\[a-zA-Z0-9>\_\\]\*".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Description of the input(s).
    #[prost(message, repeated, tag = "2")]
    pub input_arg: ::prost::alloc::vec::Vec<op_def::ArgDef>,
    /// Description of the output(s).
    #[prost(message, repeated, tag = "3")]
    pub output_arg: ::prost::alloc::vec::Vec<op_def::ArgDef>,
    /// Named control outputs for this operation. Useful only for composite
    /// operations (i.e. functions) which want to name different control outputs.
    #[prost(string, repeated, tag = "20")]
    pub control_output: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub attr: ::prost::alloc::vec::Vec<op_def::AttrDef>,
    /// Optional deprecation based on GraphDef versions.
    #[prost(message, optional, tag = "8")]
    pub deprecation: ::core::option::Option<OpDeprecation>,
    /// One-line human-readable description of what the Op does.
    #[prost(string, tag = "5")]
    pub summary: ::prost::alloc::string::String,
    /// Additional, longer human-readable description of what the Op does.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    /// True if the operation is commutative ("op(a,b) == op(b,a)" for all inputs)
    #[prost(bool, tag = "18")]
    pub is_commutative: bool,
    /// If is_aggregate is true, then this operation accepts N >= 2
    /// inputs and produces 1 output all of the same type.  Should be
    /// associative and commutative, and produce output with the same
    /// shape as the input.  The optimizer may replace an aggregate op
    /// taking input from multiple devices with a tree of aggregate ops
    /// that aggregate locally within each device (and possibly within
    /// groups of nearby devices) before communicating.
    /// TODO(josh11b): Implement that optimization.
    ///
    /// for things like add
    #[prost(bool, tag = "16")]
    pub is_aggregate: bool,
    /// Ops are marked as stateful if their behavior depends on some state beyond
    /// their input tensors (e.g. variable reading op) or if they have
    /// a side-effect (e.g. printing or asserting ops). Equivalently, stateless ops
    /// must always produce the same output for the same input and have
    /// no side-effects.
    ///
    /// By default Ops may be moved between devices.  Stateful ops should
    /// either not be moved, or should only be moved if that state can also
    /// be moved (e.g. via some sort of save / restore).
    /// Stateful ops are guaranteed to never be optimized away by Common
    /// Subexpression Elimination (CSE).
    ///
    /// for things like variables, queue
    #[prost(bool, tag = "17")]
    pub is_stateful: bool,
    /// By default, all inputs to an Op must be initialized Tensors.  Ops
    /// that may initialize tensors for the first time should set this
    /// field to true, to allow the Op to take an uninitialized Tensor as
    /// input.
    ///
    /// for Assign, etc.
    #[prost(bool, tag = "19")]
    pub allows_uninitialized_input: bool,
    /// Indicates whether the op implementation uses distributed communication.
    /// If True, the op is allowed to return errors for network disconnection and
    /// trigger TF network failure handling logics.
    #[prost(bool, tag = "21")]
    pub is_distributed_communication: bool,
}
/// Nested message and enum types in `OpDef`.
pub mod op_def {
    /// For describing inputs and outputs.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ArgDef {
        /// Name for the input/output.  Should match the regexp "\\[a-z\]\[a-z0-9\_\\]\*".
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Human readable description.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
        /// Describes the type of one or more tensors that are accepted/produced
        /// by this input/output arg.  The only legal combinations are:
        ///
        /// * For a single tensor: either the "type" field is set or the
        ///   "type_attr" field is set to the name of an attr with type "type".
        /// * For a sequence of tensors with the same type: the "number_attr"
        ///   field will be set to the name of an attr with type "int", and
        ///   either the "type" or "type_attr" field will be set as for
        ///   single tensors.
        /// * For a sequence of tensors, the "type_list_attr" field will be set
        ///   to the name of an attr with type "list(type)".
        #[prost(enumeration = "super::DataType", tag = "3")]
        pub r#type: i32,
        /// if specified, attr must have type "type"
        #[prost(string, tag = "4")]
        pub type_attr: ::prost::alloc::string::String,
        /// if specified, attr must have type "int"
        #[prost(string, tag = "5")]
        pub number_attr: ::prost::alloc::string::String,
        /// If specified, attr must have type "list(type)", and none of
        /// type, type_attr, and number_attr may be specified.
        #[prost(string, tag = "6")]
        pub type_list_attr: ::prost::alloc::string::String,
        /// The handle data for resource inputs.
        #[prost(message, repeated, tag = "7")]
        pub handle_data: ::prost::alloc::vec::Vec<
            super::resource_handle_proto::DtypeAndShape,
        >,
        /// For inputs: if true, the inputs are required to be refs.
        /// By default, inputs can be either refs or non-refs.
        /// For outputs: if true, outputs are refs, otherwise they are not.
        #[prost(bool, tag = "16")]
        pub is_ref: bool,
        /// Experimental. Full type declaration for this argument.
        /// The full type specification combines type, type_attr, type_list_attr,
        /// etc. into a unified representation.
        /// This declaration may contain non-concrete types (for example,
        /// Tensor\<TypeVar\<'T'>> is a valid type declaration.
        ///
        /// Note: this is a transient field. The long-term aim is to represent the
        /// entire OpDef as a single type: a callable. In that context, this field is
        /// just the type of a single argument.
        #[prost(message, optional, tag = "17")]
        pub experimental_full_type: ::core::option::Option<super::FullTypeDef>,
    }
    /// Description of the graph-construction-time configuration of this
    /// Op.  That is to say, this describes the attr fields that will
    /// be specified in the NodeDef.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AttrDef {
        /// A descriptive name for the argument.  May be used, e.g. by the
        /// Python client, as a keyword argument name, and so should match
        /// the regexp "\\[a-z\]\[a-z0-9\_\\]+".
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// One of the type names from attr_value.proto ("string", "list(string)",
        /// "int", etc.).
        #[prost(string, tag = "2")]
        pub r#type: ::prost::alloc::string::String,
        /// A reasonable default for this attribute if the user does not supply
        /// a value.  If not specified, the user must supply a value.
        #[prost(message, optional, tag = "3")]
        pub default_value: ::core::option::Option<super::AttrValue>,
        /// Human-readable description.
        #[prost(string, tag = "4")]
        pub description: ::prost::alloc::string::String,
        /// For type == "int", this is a minimum value.  For "list(\_\_\_)"
        /// types, this is the minimum length.
        #[prost(bool, tag = "5")]
        pub has_minimum: bool,
        #[prost(int64, tag = "6")]
        pub minimum: i64,
        /// The set of allowed values.  Has type that is the "list" version
        /// of the "type" field above (uses the "list" field of AttrValue).
        /// If type == "type" or "list(type)" above, then the "type" field
        /// of "allowed_values.list" has the set of allowed DataTypes.
        /// If type == "string" or "list(string)", then the "s" field of
        /// "allowed_values.list" has the set of allowed strings.
        #[prost(message, optional, tag = "7")]
        pub allowed_values: ::core::option::Option<super::AttrValue>,
    }
}
/// Information about version-dependent deprecation of an op
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpDeprecation {
    /// First GraphDef version at which the op is disallowed.
    #[prost(int32, tag = "1")]
    pub version: i32,
    /// Explanation of why it was deprecated and what to use instead.
    #[prost(string, tag = "2")]
    pub explanation: ::prost::alloc::string::String,
}
/// A collection of OpDefs
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpList {
    #[prost(message, repeated, tag = "1")]
    pub op: ::prost::alloc::vec::Vec<OpDef>,
}
/// A library is a set of named functions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionDefLibrary {
    #[prost(message, repeated, tag = "1")]
    pub function: ::prost::alloc::vec::Vec<FunctionDef>,
    #[prost(message, repeated, tag = "2")]
    pub gradient: ::prost::alloc::vec::Vec<GradientDef>,
    #[prost(message, repeated, tag = "3")]
    pub registered_gradients: ::prost::alloc::vec::Vec<RegisteredGradient>,
}
/// A function can be instantiated when the runtime can bind every attr
/// with a value. When a GraphDef has a call to a function, it must
/// have binding for every attr defined in the signature.
///
/// TODO(zhifengc):
///
/// * device spec, etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionDef {
    /// The definition of the function's name, arguments, return values,
    /// attrs etc.
    #[prost(message, optional, tag = "1")]
    pub signature: ::core::option::Option<OpDef>,
    /// Attributes specific to this function definition.
    #[prost(map = "string, message", tag = "5")]
    pub attr: ::std::collections::HashMap<::prost::alloc::string::String, AttrValue>,
    #[prost(map = "uint32, message", tag = "7")]
    pub arg_attr: ::std::collections::HashMap<u32, function_def::ArgAttrs>,
    /// Unique IDs for each resource argument, used to track aliasing resources. If
    /// Argument A and Argument B alias each other, then
    /// resource_arg_unique_ids\\[A.index\\] == resource_arg_unique_ids\\[B.index\\].
    ///
    /// If this field is empty, none of the arguments could alias; otherwise, every
    /// resource argument should have an entry in this field.
    ///
    /// When instantiated, the unique IDs will be attached to the \_Arg nodes'
    /// "\_resource_arg_unique_id" attribute.
    #[prost(map = "uint32, uint32", tag = "8")]
    pub resource_arg_unique_id: ::std::collections::HashMap<u32, u32>,
    /// By convention, "op" in node_def is resolved by consulting with a
    /// user-defined library first. If not resolved, "func" is assumed to
    /// be a builtin op.
    #[prost(message, repeated, tag = "3")]
    pub node_def: ::prost::alloc::vec::Vec<NodeDef>,
    /// A mapping from the output arg names from `signature` to the
    /// outputs from `node_def` that should be returned by the function.
    #[prost(map = "string, string", tag = "4")]
    pub ret: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// A mapping from control output names from `signature` to node names in
    /// `node_def` which should be control outputs of this function.
    #[prost(map = "string, string", tag = "6")]
    pub control_ret: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `FunctionDef`.
pub mod function_def {
    /// Attributes for function arguments. These attributes are the same set of
    /// valid attributes as to \_Arg nodes.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ArgAttrs {
        #[prost(map = "string, message", tag = "1")]
        pub attr: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            super::AttrValue,
        >,
    }
}
/// GradientDef defines the gradient function of a function defined in
/// a function library.
///
/// A gradient function g (specified by gradient_func) for a function f
/// (specified by function_name) must follow the following:
///
/// The function 'f' must be a numerical function which takes N inputs
/// and produces M outputs. Its gradient function 'g', which is a
/// function taking N + M inputs and produces N outputs.
///
/// I.e. if we have
/// (y1, y2, ..., y_M) = f(x1, x2, ..., x_N),
/// then, g is
/// (dL/dx1, dL/dx2, ..., dL/dx_N) = g(x1, x2, ..., x_N,
/// dL/dy1, dL/dy2, ..., dL/dy_M),
/// where L is a scalar-value function of (x1, x2, ..., xN) (e.g., the
/// loss function). dL/dx_i is the partial derivative of L with respect
/// to x_i.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GradientDef {
    /// The function name.
    #[prost(string, tag = "1")]
    pub function_name: ::prost::alloc::string::String,
    /// The gradient function's name.
    #[prost(string, tag = "2")]
    pub gradient_func: ::prost::alloc::string::String,
}
/// RegisteredGradient stores a gradient function that is registered in the
/// gradients library and used in the ops of a function in the function library.
/// Unlike GradientDef, these gradients are identified by op type, and not
/// directly linked to any function.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisteredGradient {
    /// The gradient function's name.
    #[prost(string, tag = "1")]
    pub gradient_func: ::prost::alloc::string::String,
    /// The gradient function's registered op type.
    #[prost(string, tag = "2")]
    pub registered_op_type: ::prost::alloc::string::String,
}
/// Version information for a piece of serialized data
///
/// There are different types of versions for each type of data
/// (GraphDef, etc.), but they all have the same common shape
/// described here.
///
/// Each consumer has "consumer" and "min_producer" versions (specified
/// elsewhere).  A consumer is allowed to consume this data if
///
/// producer >= min_producer
/// consumer >= min_consumer
/// consumer not in bad_consumers
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionDef {
    /// The version of the code that produced this data.
    #[prost(int32, tag = "1")]
    pub producer: i32,
    /// Any consumer below this version is not allowed to consume this data.
    #[prost(int32, tag = "2")]
    pub min_consumer: i32,
    /// Specific consumer versions which are disallowed (e.g. due to bugs).
    #[prost(int32, repeated, tag = "3")]
    pub bad_consumers: ::prost::alloc::vec::Vec<i32>,
}
/// Represents the graph of operations
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GraphDef {
    #[prost(message, repeated, tag = "1")]
    pub node: ::prost::alloc::vec::Vec<NodeDef>,
    /// Compatibility versions of the graph.  See core/public/version.h for version
    /// history.  The GraphDef version is distinct from the TensorFlow version, and
    /// each release of TensorFlow will support a range of GraphDef versions.
    #[prost(message, optional, tag = "4")]
    pub versions: ::core::option::Option<VersionDef>,
    /// Deprecated single version field; use versions above instead.  Since all
    /// GraphDef changes before "versions" was introduced were forward
    /// compatible, this field is entirely ignored.
    #[deprecated]
    #[prost(int32, tag = "3")]
    pub version: i32,
    /// "library" provides user-defined functions.
    ///
    /// Naming:
    ///
    /// * library.function.name are in a flat namespace.
    ///   NOTE: We may need to change it to be hierarchical to support
    ///   different orgs. E.g.,
    ///   { "/google/nn", { ... }},
    ///   { "/google/vision", { ... }}
    ///   { "/org_foo/module_bar", { ... }}
    ///   map\<string, FunctionDefLib> named_lib;
    /// * If node\\[i\\].op is the name of one function in "library",
    ///   node\\[i\\] is deemed as a function call. Otherwise, node\\[i\\].op
    ///   must be a primitive operation supported by the runtime.
    ///
    /// Function call semantics:
    ///
    /// * The callee may start execution as soon as some of its inputs
    ///   are ready. The caller may want to use Tuple() mechanism to
    ///   ensure all inputs are ready in the same time.
    ///
    /// * The consumer of return values may start executing as soon as
    ///   the return values the consumer depends on are ready.  The
    ///   consumer may want to use Tuple() mechanism to ensure the
    ///   consumer does not start until all return values of the callee
    ///   function are ready.
    #[prost(message, optional, tag = "2")]
    pub library: ::core::option::Option<FunctionDefLibrary>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocationDescription {
    /// Total number of bytes requested
    #[prost(int64, tag = "1")]
    pub requested_bytes: i64,
    /// Total number of bytes allocated if known
    #[prost(int64, tag = "2")]
    pub allocated_bytes: i64,
    /// Name of the allocator used
    #[prost(string, tag = "3")]
    pub allocator_name: ::prost::alloc::string::String,
    /// Identifier of the allocated buffer if known
    #[prost(int64, tag = "4")]
    pub allocation_id: i64,
    /// Set if this tensor only has one remaining reference
    #[prost(bool, tag = "5")]
    pub has_single_reference: bool,
    /// Address of the allocation.
    #[prost(uint64, tag = "6")]
    pub ptr: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TensorDescription {
    /// Data type of tensor elements
    #[prost(enumeration = "DataType", tag = "1")]
    pub dtype: i32,
    /// Shape of the tensor.
    #[prost(message, optional, tag = "2")]
    pub shape: ::core::option::Option<TensorShapeProto>,
    /// Information about the size and allocator used for the data
    #[prost(message, optional, tag = "4")]
    pub allocation_description: ::core::option::Option<AllocationDescription>,
}
/// An allocation/de-allocation operation performed by the allocator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocationRecord {
    /// The timestamp of the operation.
    #[prost(int64, tag = "1")]
    pub alloc_micros: i64,
    /// Number of bytes allocated, or de-allocated if negative.
    #[prost(int64, tag = "2")]
    pub alloc_bytes: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocatorMemoryUsed {
    #[prost(string, tag = "1")]
    pub allocator_name: ::prost::alloc::string::String,
    /// These are per-node allocator memory stats.
    #[prost(int64, tag = "2")]
    pub total_bytes: i64,
    #[prost(int64, tag = "3")]
    pub peak_bytes: i64,
    /// The bytes that are not deallocated.
    #[prost(int64, tag = "4")]
    pub live_bytes: i64,
    /// The allocation and deallocation timeline.
    #[prost(message, repeated, tag = "6")]
    pub allocation_records: ::prost::alloc::vec::Vec<AllocationRecord>,
    /// These are snapshots of the overall allocator memory stats.
    /// The number of live bytes currently allocated by the allocator.
    #[prost(int64, tag = "5")]
    pub allocator_bytes_in_use: i64,
}
/// Output sizes recorded for a single execution of a graph node.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeOutput {
    #[prost(int32, tag = "1")]
    pub slot: i32,
    #[prost(message, optional, tag = "3")]
    pub tensor_description: ::core::option::Option<TensorDescription>,
}
/// For memory tracking.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemoryStats {
    #[prost(int64, tag = "1")]
    pub temp_memory_size: i64,
    #[prost(int64, tag = "3")]
    pub persistent_memory_size: i64,
    #[prost(int64, repeated, tag = "5")]
    pub persistent_tensor_alloc_ids: ::prost::alloc::vec::Vec<i64>,
    #[deprecated]
    #[prost(int64, tag = "2")]
    pub device_temp_memory_size: i64,
    #[deprecated]
    #[prost(int64, tag = "4")]
    pub device_persistent_memory_size: i64,
    #[deprecated]
    #[prost(int64, repeated, packed = "false", tag = "6")]
    pub device_persistent_tensor_alloc_ids: ::prost::alloc::vec::Vec<i64>,
}
/// Time/size stats recorded for a single execution of a graph node.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeExecStats {
    /// TODO(tucker): Use some more compact form of node identity than
    /// the full string name.  Either all processes should agree on a
    /// global id (cost_id?) for each node, or we should use a hash of
    /// the name.
    #[prost(string, tag = "1")]
    pub node_name: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub all_start_micros: i64,
    #[prost(int64, tag = "3")]
    pub op_start_rel_micros: i64,
    #[prost(int64, tag = "4")]
    pub op_end_rel_micros: i64,
    #[prost(int64, tag = "5")]
    pub all_end_rel_micros: i64,
    #[prost(message, repeated, tag = "6")]
    pub memory: ::prost::alloc::vec::Vec<AllocatorMemoryUsed>,
    #[prost(message, repeated, tag = "7")]
    pub output: ::prost::alloc::vec::Vec<NodeOutput>,
    #[prost(string, tag = "8")]
    pub timeline_label: ::prost::alloc::string::String,
    #[prost(int64, tag = "9")]
    pub scheduled_micros: i64,
    #[prost(uint32, tag = "10")]
    pub thread_id: u32,
    #[prost(message, repeated, tag = "11")]
    pub referenced_tensor: ::prost::alloc::vec::Vec<AllocationDescription>,
    #[prost(message, optional, tag = "12")]
    pub memory_stats: ::core::option::Option<MemoryStats>,
    #[prost(int64, tag = "13")]
    pub all_start_nanos: i64,
    #[prost(int64, tag = "14")]
    pub op_start_rel_nanos: i64,
    #[prost(int64, tag = "15")]
    pub op_end_rel_nanos: i64,
    #[prost(int64, tag = "16")]
    pub all_end_rel_nanos: i64,
    #[prost(int64, tag = "17")]
    pub scheduled_nanos: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceStepStats {
    #[prost(string, tag = "1")]
    pub device: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub node_stats: ::prost::alloc::vec::Vec<NodeExecStats>,
    /// Its key is thread id.
    #[prost(map = "uint32, string", tag = "3")]
    pub thread_names: ::std::collections::HashMap<u32, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepStats {
    #[prost(message, repeated, tag = "1")]
    pub dev_stats: ::prost::alloc::vec::Vec<DeviceStepStats>,
}
/// Defines a single job in a TensorFlow cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobDef {
    /// The name of this job.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Mapping from task ID to "hostname:port" string.
    ///
    /// If the `name` field contains "worker", and the `tasks` map contains a
    /// mapping from 7 to "example.org:2222", then the device prefix
    /// "/job:worker/task:7" will be assigned to "example.org:2222".
    #[prost(map = "int32, string", tag = "2")]
    pub tasks: ::std::collections::HashMap<i32, ::prost::alloc::string::String>,
}
/// Defines a TensorFlow cluster as a set of jobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterDef {
    /// The jobs that comprise the cluster.
    #[prost(message, repeated, tag = "1")]
    pub job: ::prost::alloc::vec::Vec<JobDef>,
}
/// Coordination service configuration parameters.
/// The system picks appropriate values for fields that are not set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoordinationServiceConfig {
    /// Type of coordination service implementation to enable.
    /// For example, setting the service type as "standalone" starts a service
    /// instance on the leader task to provide the coordination services such as
    /// heartbeats and consistent key-value store.
    #[prost(string, tag = "1")]
    pub service_type: ::prost::alloc::string::String,
    /// Address where the coordination service instance is hosted.
    #[prost(string, tag = "2")]
    pub service_leader: ::prost::alloc::string::String,
    /// Whether to enable the health check mechanism.
    #[prost(bool, tag = "3")]
    pub enable_health_check: bool,
    /// Maximum wait time for all members in the cluster to be registered.
    #[prost(int64, tag = "4")]
    pub cluster_register_timeout_in_ms: i64,
    /// Heartbeat timeout, if a task does not record heartbeat in this time
    /// window, it will be considered disconnected.
    /// Note: This is also used as a grace period to accept any heartbeats after
    /// the agent has disconnected, to account for the lag time between the service
    /// recording the state change and the agent stopping heartbeats.
    #[prost(int64, tag = "5")]
    pub heartbeat_timeout_in_ms: i64,
    /// The list of jobs that partipate in the coordination service. If empty, all
    /// jobs will be included in the coordination service by default.
    #[prost(string, repeated, tag = "6")]
    pub coordinated_jobs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Denotes how long to wait for all coordination agents to reach the barriers
    /// (after the first shutdown request) before disconnecting together. If
    /// set to 0, no barrier is imposed upon shutdown and each worker can
    /// disconnect individually.
    #[prost(int64, tag = "7")]
    pub shutdown_barrier_timeout_in_ms: i64,
    /// If set, agents do not make an explicit Shutdown() call. Service will only
    /// find out about the disconnecte agent via stale heartbeats. Used for
    /// testing.
    #[prost(bool, tag = "8")]
    pub agent_destruction_without_shutdown: bool,
}
/// Option for watching a node in TensorFlow Debugger (tfdbg).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugTensorWatch {
    /// Name of the node to watch.
    /// Use "\*" for wildcard. But note: currently, regex is not supported in
    /// general.
    #[prost(string, tag = "1")]
    pub node_name: ::prost::alloc::string::String,
    /// Output slot to watch.
    /// The semantics of output_slot == -1 is that all outputs of the node
    /// will be watched (i.e., a wildcard).
    /// Other negative values of output_slot are invalid and will lead to
    /// errors currently.
    #[prost(int32, tag = "2")]
    pub output_slot: i32,
    /// Name(s) of the debugging op(s).
    /// One or more than one probes on a tensor.
    /// e.g., {"DebugIdentity", "DebugNanCount"}
    #[prost(string, repeated, tag = "3")]
    pub debug_ops: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// URL(s) for debug targets(s).
    ///
    /// Supported URL formats are:
    ///
    /// * file:///foo/tfdbg_dump: Writes out Event content to file
    ///   /foo/tfdbg_dump.  Assumes all directories can be created if they don't
    ///   already exist.
    /// * grpc://localhost:11011: Sends an RPC request to an EventListener
    ///   service running at localhost:11011 with the event.
    /// * memcbk:///event_key: Routes tensors to clients using the
    ///   callback registered with the DebugCallbackRegistry for event_key.
    ///
    /// Each debug op listed in debug_ops will publish its output tensor (debug
    /// signal) to all URLs in debug_urls.
    ///
    /// N.B. Session::Run() supports concurrent invocations of the same inputs
    /// (feed keys), outputs and target nodes. If such concurrent invocations
    /// are to be debugged, the callers of Session::Run() must use distinct
    /// debug_urls to make sure that the streamed or dumped events do not overlap
    /// among the invocations.
    /// TODO(cais): More visible documentation of this in g3docs.
    #[prost(string, repeated, tag = "4")]
    pub debug_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Do not error out if debug op creation fails (e.g., due to dtype
    /// incompatibility). Instead, just log the failure.
    #[prost(bool, tag = "5")]
    pub tolerate_debug_op_creation_failures: bool,
}
/// Options for initializing DebuggerState in TensorFlow Debugger (tfdbg).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugOptions {
    /// Debugging options
    #[prost(message, repeated, tag = "4")]
    pub debug_tensor_watch_opts: ::prost::alloc::vec::Vec<DebugTensorWatch>,
    /// Caller-specified global step count.
    /// Note that this is distinct from the session run count and the executor
    /// step count.
    #[prost(int64, tag = "10")]
    pub global_step: i64,
    /// Whether the total disk usage of tfdbg is to be reset to zero
    /// in this Session.run call. This is used by wrappers and hooks
    /// such as the local CLI ones to indicate that the dumped tensors
    /// are cleaned up from the disk after each Session.run.
    #[prost(bool, tag = "11")]
    pub reset_disk_byte_usage: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebuggedSourceFile {
    /// The host name on which a source code file is located.
    #[prost(string, tag = "1")]
    pub host: ::prost::alloc::string::String,
    /// Path to the source code file.
    #[prost(string, tag = "2")]
    pub file_path: ::prost::alloc::string::String,
    /// The timestamp at which the source code file is last modified.
    #[prost(int64, tag = "3")]
    pub last_modified: i64,
    /// Byte size of the file.
    #[prost(int64, tag = "4")]
    pub bytes: i64,
    /// Line-by-line content of the source code file.
    #[prost(string, repeated, tag = "5")]
    pub lines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebuggedSourceFiles {
    /// A collection of source code files.
    #[prost(message, repeated, tag = "1")]
    pub source_files: ::prost::alloc::vec::Vec<DebuggedSourceFile>,
}
/// The config for graph verifiers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifierConfig {
    /// Deadline for completion of all verification i.e. all the Toggle ON
    /// verifiers must complete execution within this time.
    #[prost(int64, tag = "1")]
    pub verification_timeout_in_ms: i64,
    /// Perform structural validation on a tensorflow graph. Default is OFF.
    #[prost(enumeration = "verifier_config::Toggle", tag = "2")]
    pub structure_verifier: i32,
}
/// Nested message and enum types in `VerifierConfig`.
pub mod verifier_config {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Toggle {
        Default = 0,
        On = 1,
        Off = 2,
    }
    impl Toggle {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Toggle::Default => "DEFAULT",
                Toggle::On => "ON",
                Toggle::Off => "OFF",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoParallelOptions {
    #[prost(bool, tag = "1")]
    pub enable: bool,
    #[prost(int32, tag = "2")]
    pub num_replicas: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopedAllocatorOptions {
    /// If present, only perform optimization for these ops.
    #[prost(string, repeated, tag = "1")]
    pub enable_op: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Graph rewriting is experimental and subject to change, not covered by any
/// API stability guarantees.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewriterConfig {
    /// CPU Conversion settings between NHCW and NCHW.
    #[prost(enumeration = "rewriter_config::CpuLayout", tag = "50")]
    pub cpu_layout_conversion: i32,
    /// Optimize tensor layouts (default is ON)
    /// e.g. This will try to use NCHW layout on GPU which is faster.
    #[prost(enumeration = "rewriter_config::Toggle", tag = "1")]
    pub layout_optimizer: i32,
    /// Fold constants (default is ON)
    /// Statically infer the value of tensors when possible, and materialize the
    /// result using constants.
    #[prost(enumeration = "rewriter_config::Toggle", tag = "3")]
    pub constant_folding: i32,
    /// Shape optimizations (default is ON)
    /// Simplify computations made on shapes.
    #[prost(enumeration = "rewriter_config::Toggle", tag = "13")]
    pub shape_optimization: i32,
    /// Remapping (default is ON)
    /// Remap subgraphs onto more efficient implementations.
    #[prost(enumeration = "rewriter_config::Toggle", tag = "14")]
    pub remapping: i32,
    /// Common subgraph elimination (default is ON)
    /// e.g. Simplify arithmetic ops; merge ops with same value (like constants).
    #[prost(enumeration = "rewriter_config::Toggle", tag = "24")]
    pub common_subgraph_elimination: i32,
    /// Arithmetic optimizations (default is ON)
    /// e.g. Simplify arithmetic ops; merge ops with same value (like constants).
    #[prost(enumeration = "rewriter_config::Toggle", tag = "7")]
    pub arithmetic_optimization: i32,
    /// Control dependency optimizations (default is ON).
    /// Remove redundant control dependencies, which may enable other optimization.
    #[prost(enumeration = "rewriter_config::Toggle", tag = "8")]
    pub dependency_optimization: i32,
    /// Loop optimizations (default is ON).
    #[prost(enumeration = "rewriter_config::Toggle", tag = "9")]
    pub loop_optimization: i32,
    /// Function optimizations (default is ON).
    #[prost(enumeration = "rewriter_config::Toggle", tag = "10")]
    pub function_optimization: i32,
    /// Strips debug-related nodes from the graph (off by default).
    #[prost(enumeration = "rewriter_config::Toggle", tag = "11")]
    pub debug_stripper: i32,
    /// If true, don't remove unnecessary ops from the graph
    #[prost(bool, tag = "2")]
    pub disable_model_pruning: bool,
    /// Try to allocate some independent Op outputs contiguously in order to
    /// merge or eliminate downstream Ops (off by default).
    #[prost(enumeration = "rewriter_config::Toggle", tag = "15")]
    pub scoped_allocator_optimization: i32,
    /// Force small ops onto the CPU (default is OFF).
    #[prost(enumeration = "rewriter_config::Toggle", tag = "18")]
    pub pin_to_host_optimization: i32,
    /// Enable the swap of kernel implementations based on the device placement
    /// (default is ON).
    #[prost(enumeration = "rewriter_config::Toggle", tag = "22")]
    pub implementation_selector: i32,
    /// Optimize data types for CUDA (default is OFF).
    /// This will try to use float16 on GPU which is faster.
    /// Note that this can change the numerical stability of the graph and may
    /// require the use of loss scaling to maintain model convergence.
    #[prost(enumeration = "rewriter_config::Toggle", tag = "23")]
    pub auto_mixed_precision: i32,
    /// Optimize data types for oneDNN (default is OFF).
    /// This will try to use bfloat16 on CPUs, which is faster.
    /// Note that this can change the numerical stability of the graph.
    /// Note: this is deprecated.
    /// It is replaced by auto_mixed_precision_onednn_bfloat16
    #[prost(enumeration = "rewriter_config::Toggle", tag = "25")]
    pub auto_mixed_precision_mkl: i32,
    /// Optimize data types for oneDNN (default is OFF).
    /// This will try to use bfloat16 on CPUs, which is faster.
    /// Note that this can change the numerical stability of the graph.
    /// Note: this is equivalent to the deprecated option auto_mixed_precision_mkl
    #[prost(enumeration = "rewriter_config::Toggle", tag = "31")]
    pub auto_mixed_precision_onednn_bfloat16: i32,
    /// Emulate a model using data type float16 on CPU (default is OFF).
    /// This will try to emulate the float16 inputs and outputs of an operator
    /// on CPU to have better correlation with float16 on GPU; however the
    /// computation in the operator is based on float32.
    /// Note that this can change the numerical stability of the graph.
    #[prost(enumeration = "rewriter_config::Toggle", tag = "29")]
    pub auto_mixed_precision_cpu: i32,
    /// Disable the entire meta optimizer (off by default).
    #[prost(bool, tag = "19")]
    pub disable_meta_optimizer: bool,
    /// Optimizers registered by plugin (default is ON)
    #[prost(enumeration = "rewriter_config::Toggle", tag = "28")]
    pub use_plugin_optimizers: i32,
    /// Conditional code motion (default is ON).
    #[prost(enumeration = "rewriter_config::Toggle", tag = "30")]
    pub experimental_conditional_code_motion: i32,
    /// Controls how many times we run the optimizers in meta optimizer (default
    /// is once).
    #[prost(enumeration = "rewriter_config::NumIterationsType", tag = "12")]
    pub meta_optimizer_iterations: i32,
    /// The minimum number of nodes in a graph to optimizer. For smaller graphs,
    /// optimization is skipped.
    /// 0 means the system picks an appropriate number.
    /// \< 0 means do not skip optimization.
    #[prost(int32, tag = "17")]
    pub min_graph_nodes: i32,
    /// Disable optimizations that assume compressed tensors. Note that this flag
    /// is experimental and may be removed in the future.
    #[prost(bool, tag = "26")]
    pub experimental_disable_compressed_tensor_optimization: bool,
    /// Disable folding quantization emulation ops such as FakeQuantWithMinMax\* and
    /// QuantizeAndDequantize\*. Some compilers (e.g. the TF-to-tflite converter)
    /// have to extract quantization configs (e.g. min/max range, number of bits,
    /// and per-channel) from the quantization emulation ops. Note that this flag
    /// is experimental and may be removed in the future. See b/174138564 for more
    /// details.
    #[prost(bool, tag = "27")]
    pub experimental_disable_folding_quantization_emulation: bool,
    /// Configures memory optimization passes through the meta-optimizer. Has no
    /// effect on manually requested memory optimization passes in the optimizers
    /// field.
    #[prost(enumeration = "rewriter_config::MemOptType", tag = "4")]
    pub memory_optimization: i32,
    /// A node name scope for node names which are valid outputs of recomputations.
    /// Inputs to nodes that match this scope may be recomputed (subject either to
    /// manual annotation of those input nodes or to manual annotation and
    /// heuristics depending on memory_optimization), but the nodes themselves will
    /// not be recomputed. This matches any sub-scopes as well, meaning the scope
    /// can appear not just as a top-level scope. For example, if the value is
    /// "gradients/", the default, it will match node name "gradients/foo",
    /// "foo/gradients/bar", but not "foo_gradients/"
    #[prost(string, tag = "6")]
    pub memory_optimizer_target_node_name_scope: ::prost::alloc::string::String,
    /// Maximum number of milliseconds to spend optimizing a single graph before
    /// timing out. If less than or equal to 0 (default value) the optimizer will
    /// never time out.
    #[prost(int64, tag = "20")]
    pub meta_optimizer_timeout_ms: i64,
    /// Configures AutoParallel optimization passes either through the
    /// meta-optimizer or when manually specified through the optimizers field.
    #[prost(message, optional, tag = "5")]
    pub auto_parallel: ::core::option::Option<AutoParallelOptions>,
    /// If true, any optimization pass failing will cause the MetaOptimizer to
    /// stop with an error. By default - or when set to false, failing passes are
    /// skipped silently.
    #[prost(bool, tag = "21")]
    pub fail_on_optimizer_errors: bool,
    #[prost(message, optional, tag = "16")]
    pub scoped_allocator_opts: ::core::option::Option<ScopedAllocatorOptions>,
    /// If non-empty, will use this as an alternative way to specify a list of
    /// optimizations to turn on and the order of the optimizations (replacing the
    /// meta-optimizer).
    ///
    /// Of the RewriterConfig options, only the AutoParallel configuration options
    /// (the auto_parallel field) apply to manually requested optimization passes
    /// ("autoparallel"). Memory optimization passes ("memory") invoked here are
    /// not configurable (in contrast to memory optimization passes through the
    /// meta-optimizer) and act only on manual op annotations.
    ///
    /// Custom optimizers (see custom_optimizers) that are not part of this
    /// schedule will be run after - in the order that they were specified.
    #[prost(string, repeated, tag = "100")]
    pub optimizers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// list of CustomGraphOptimizers to apply.
    #[prost(message, repeated, tag = "200")]
    pub custom_optimizers: ::prost::alloc::vec::Vec<
        rewriter_config::CustomGraphOptimizer,
    >,
    /// VerifierConfig specifying the verifiers to be run after every optimizer.
    #[prost(message, optional, tag = "300")]
    pub inter_optimizer_verifier_config: ::core::option::Option<VerifierConfig>,
    /// VerifierConfig specifying the verifiers to be run at the end, after all
    /// optimizers have run.
    #[prost(message, optional, tag = "301")]
    pub post_optimization_verifier_config: ::core::option::Option<VerifierConfig>,
}
/// Nested message and enum types in `RewriterConfig`.
pub mod rewriter_config {
    /// Message to describe custom graph optimizer and its parameters
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomGraphOptimizer {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(map = "string, message", tag = "2")]
        pub parameter_map: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            super::AttrValue,
        >,
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Toggle {
        Default = 0,
        On = 1,
        Off = 2,
        /// Enable some aggressive optimizations that use assumptions that TF graphs
        /// may break. For example, assume the shape of a placeholder matches its
        /// actual feed.
        Aggressive = 3,
        /// Run MLIR pass if there's one implemented in TFG, do nothing otherwise.
        /// I.e., if there's no corresponding TFG pass, it's an OFF. This is supposed
        /// to be mapped with `ON` and there's no `AGGRESSIVE` in MLIR pass now.
        ExperimentalMlir = 4,
        /// Run both MLIR and Grappler passes consecutively and MLIR pass will come
        /// first.
        ExperimentalBoth = 5,
    }
    impl Toggle {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Toggle::Default => "DEFAULT",
                Toggle::On => "ON",
                Toggle::Off => "OFF",
                Toggle::Aggressive => "AGGRESSIVE",
                Toggle::ExperimentalMlir => "EXPERIMENTAL_MLIR",
                Toggle::ExperimentalBoth => "EXPERIMENTAL_BOTH",
            }
        }
    }
    /// Enum for layout conversion between NCHW and NHWC on CPU. Default is OFF.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum CpuLayout {
        NoConversionOnCpu = 0,
        NchwToNhwc = 1,
        NhwcToNchw = 2,
    }
    impl CpuLayout {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CpuLayout::NoConversionOnCpu => "NO_CONVERSION_ON_CPU",
                CpuLayout::NchwToNhwc => "NCHW_TO_NHWC",
                CpuLayout::NhwcToNchw => "NHWC_TO_NCHW",
            }
        }
    }
    /// Enum controlling the number of times to run optimizers. The default is to
    /// run them twice.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum NumIterationsType {
        DefaultNumIters = 0,
        One = 1,
        Two = 2,
    }
    impl NumIterationsType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                NumIterationsType::DefaultNumIters => "DEFAULT_NUM_ITERS",
                NumIterationsType::One => "ONE",
                NumIterationsType::Two => "TWO",
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum MemOptType {
        /// The default setting (SCHEDULING and SWAPPING HEURISTICS only)
        DefaultMemOpt = 0,
        /// Disabled in the meta-optimizer.
        NoMemOpt = 1,
        /// Driven by manual op-level annotations.
        Manual = 2,
        /// Swapping heuristic will move a tensor from the GPU to the CPU and move
        /// it back when needed to reduce peak memory usage.
        SwappingHeuristics = 4,
        /// Recomputation heuristics will recompute ops (such as Relu activation)
        /// during backprop instead of storing them, reducing peak memory usage.
        RecomputationHeuristics = 5,
        /// Scheduling will split big ops such as AddN and try to enforce a schedule
        /// of the new computations that decreases peak memory usage.
        SchedulingHeuristics = 6,
        /// Use any combination of swapping and recomputation heuristics.
        Heuristics = 3,
    }
    impl MemOptType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MemOptType::DefaultMemOpt => "DEFAULT_MEM_OPT",
                MemOptType::NoMemOpt => "NO_MEM_OPT",
                MemOptType::Manual => "MANUAL",
                MemOptType::SwappingHeuristics => "SWAPPING_HEURISTICS",
                MemOptType::RecomputationHeuristics => "RECOMPUTATION_HEURISTICS",
                MemOptType::SchedulingHeuristics => "SCHEDULING_HEURISTICS",
                MemOptType::Heuristics => "HEURISTICS",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GpuOptions {
    /// Fraction of the available GPU memory to allocate for each process.
    /// 1 means to allocate all of the GPU memory, 0.5 means the process
    /// allocates up to ~50% of the available GPU memory.
    ///
    /// GPU memory is pre-allocated unless the allow_growth option is enabled.
    ///
    /// If greater than 1.0, uses CUDA unified memory to potentially oversubscribe
    /// the amount of memory available on the GPU device by using host memory as a
    /// swap space. Accessing memory not available on the device will be
    /// significantly slower as that would require memory transfer between the host
    /// and the device. Options to reduce the memory requirement should be
    /// considered before enabling this option as this may come with a negative
    /// performance impact. Oversubscription using the unified memory requires
    /// Pascal class or newer GPUs and it is currently only supported on the Linux
    /// operating system. See
    /// <https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#um-requirements>
    /// for the detailed requirements.
    #[prost(double, tag = "1")]
    pub per_process_gpu_memory_fraction: f64,
    /// If true, the allocator does not pre-allocate the entire specified
    /// GPU memory region, instead starting small and growing as needed.
    #[prost(bool, tag = "4")]
    pub allow_growth: bool,
    /// The type of GPU allocation strategy to use.
    ///
    /// Allowed values:
    /// "": The empty string (default) uses a system-chosen default
    /// which may change over time.
    ///
    /// "BFC": A "Best-fit with coalescing" algorithm, simplified from a
    /// version of dlmalloc.
    #[prost(string, tag = "2")]
    pub allocator_type: ::prost::alloc::string::String,
    /// Delay deletion of up to this many bytes to reduce the number of
    /// interactions with gpu driver code.  If 0, the system chooses
    /// a reasonable default (several MBs).
    #[prost(int64, tag = "3")]
    pub deferred_deletion_bytes: i64,
    /// A comma-separated list of GPU ids that determines the 'visible'
    /// to 'virtual' mapping of GPU devices.  For example, if TensorFlow
    /// can see 8 GPU devices in the process, and one wanted to map
    /// visible GPU devices 5 and 3 as "/device:GPU:0", and "/device:GPU:1",
    /// then one would specify this field as "5,3".  This field is similar in
    /// spirit to the CUDA_VISIBLE_DEVICES environment variable, except
    /// it applies to the visible GPU devices in the process.
    ///
    /// NOTE:
    ///
    /// 1. The GPU driver provides the process with the visible GPUs
    ///    in an order which is not guaranteed to have any correlation to
    ///    the *physical* GPU id in the machine.  This field is used for
    ///    remapping "visible" to "virtual", which means this operates only
    ///    after the process starts.  Users are required to use vendor
    ///    specific mechanisms (e.g., CUDA_VISIBLE_DEVICES) to control the
    ///    physical to visible device mapping prior to invoking TensorFlow.
    /// 1. In the code, the ids in this list are also called "platform GPU id"s,
    ///    and the 'virtual' ids of GPU devices (i.e. the ids in the device
    ///    name "/device:GPU:<id>") are also called "TF GPU id"s. Please
    ///    refer to third_party/tensorflow/core/common_runtime/gpu/gpu_id.h
    ///    for more information.
    #[prost(string, tag = "5")]
    pub visible_device_list: ::prost::alloc::string::String,
    /// In the event polling loop sleep this many microseconds between
    /// PollEvents calls, when the queue is not empty.  If value is not
    /// set or set to 0, gets set to a non-zero default.
    #[prost(int32, tag = "6")]
    pub polling_active_delay_usecs: i32,
    /// This field is deprecated and ignored.
    #[prost(int32, tag = "7")]
    pub polling_inactive_delay_msecs: i32,
    /// Force all tensors to be gpu_compatible. On a GPU-enabled TensorFlow,
    /// enabling this option forces all CPU tensors to be allocated with Cuda
    /// pinned memory. Normally, TensorFlow will infer which tensors should be
    /// allocated as the pinned memory. But in case where the inference is
    /// incomplete, this option can significantly speed up the cross-device memory
    /// copy performance as long as it fits the memory.
    /// Note that this option is not something that should be
    /// enabled by default for unknown or very large models, since all Cuda pinned
    /// memory is unpageable, having too much pinned memory might negatively impact
    /// the overall host system performance.
    #[prost(bool, tag = "8")]
    pub force_gpu_compatible: bool,
    /// Everything inside experimental is subject to change and is not subject
    /// to API stability guarantees in
    /// <https://www.tensorflow.org/guide/version_compat.>
    #[prost(message, optional, tag = "9")]
    pub experimental: ::core::option::Option<gpu_options::Experimental>,
}
/// Nested message and enum types in `GPUOptions`.
pub mod gpu_options {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Experimental {
        /// The multi virtual device settings. If empty (not set), it will create
        /// single virtual device on each visible GPU, according to the settings
        /// in "visible_device_list" above. Otherwise, the number of elements in the
        /// list must be the same as the number of visible GPUs (after
        /// "visible_device_list" filtering if it is set), and the string represented
        /// device names (e.g. /device:GPU:<id>) will refer to the virtual
        /// devices and have the <id> field assigned sequentially starting from 0,
        /// according to the order of the virtual devices determined by
        /// device_ordinal and the location in the virtual device list.
        ///
        /// For example,
        /// visible_device_list = "1,0"
        /// virtual_devices { memory_limit: 1GB memory_limit: 2GB }
        /// virtual_devices { memory_limit: 3GB memory_limit: 4GB }
        /// will create 4 virtual devices as:
        /// /device:GPU:0 -> visible GPU 1 with 1GB memory
        /// /device:GPU:1 -> visible GPU 1 with 2GB memory
        /// /device:GPU:2 -> visible GPU 0 with 3GB memory
        /// /device:GPU:3 -> visible GPU 0 with 4GB memory
        ///
        /// but
        /// visible_device_list = "1,0"
        /// virtual_devices { memory_limit: 1GB memory_limit: 2GB
        /// device_ordinal: 10 device_ordinal: 20}
        /// virtual_devices { memory_limit: 3GB memory_limit: 4GB
        /// device_ordinal: 10 device_ordinal: 20}
        /// will create 4 virtual devices as:
        /// /device:GPU:0 -> visible GPU 1 with 1GB memory  (ordinal 10)
        /// /device:GPU:1 -> visible GPU 0 with 3GB memory  (ordinal 10)
        /// /device:GPU:2 -> visible GPU 1 with 2GB memory  (ordinal 20)
        /// /device:GPU:3 -> visible GPU 0 with 4GB memory  (ordinal 20)
        ///
        /// NOTE:
        ///
        /// 1. It's invalid to set both this and "per_process_gpu_memory_fraction"
        ///    at the same time.
        /// 1. Currently this setting is per-process, not per-session. Using
        ///    different settings in different sessions within same process will
        ///    result in undefined behavior.
        #[prost(message, repeated, tag = "1")]
        pub virtual_devices: ::prost::alloc::vec::Vec<experimental::VirtualDevices>,
        /// If true, uses CUDA unified memory for memory allocations. If
        /// per_process_gpu_memory_fraction option is greater than 1.0, then unified
        /// memory is used regardless of the value for this field. See comments for
        /// per_process_gpu_memory_fraction field for more details and requirements
        /// of the unified memory. This option is useful to oversubscribe memory if
        /// multiple processes are sharing a single GPU while individually using less
        /// than 1.0 per process memory fraction.
        #[prost(bool, tag = "2")]
        pub use_unified_memory: bool,
        /// If > 1, the number of device-to-device copy streams to create
        /// for each GPUDevice.  Default value is 0, which is automatically
        /// converted to 1.
        #[prost(int32, tag = "3")]
        pub num_dev_to_dev_copy_streams: i32,
        /// If non-empty, defines a good GPU ring order on a single worker based on
        /// device interconnect.  This assumes that all workers have the same GPU
        /// topology.  Specify as a comma-separated string, e.g. "3,2,1,0,7,6,5,4".
        /// This ring order is used by the RingReducer implementation of
        /// CollectiveReduce, and serves as an override to automatic ring order
        /// generation in OrderTaskDeviceMap() during CollectiveParam resolution.
        #[prost(string, tag = "4")]
        pub collective_ring_order: ::prost::alloc::string::String,
        /// If true then extra work is done by GPUDevice and GPUBFCAllocator to
        /// keep track of when GPU memory is freed and when kernels actually
        /// complete so that we can know when a nominally free memory chunk
        /// is really not subject to pending use.
        #[prost(bool, tag = "5")]
        pub timestamped_allocator: bool,
        /// Parameters for GPUKernelTracker.  By default no kernel tracking is done.
        /// Note that timestamped_allocator is only effective if some tracking is
        /// specified.
        ///
        /// If kernel_tracker_max_interval = n > 0, then a tracking event
        /// is inserted after every n kernels without an event.
        #[prost(int32, tag = "7")]
        pub kernel_tracker_max_interval: i32,
        /// If kernel_tracker_max_bytes = n > 0, then a tracking event is
        /// inserted after every series of kernels allocating a sum of
        /// memory >= n.  If one kernel allocates b * n bytes, then one
        /// event will be inserted after it, but it will count as b against
        /// the pending limit.
        #[prost(int32, tag = "8")]
        pub kernel_tracker_max_bytes: i32,
        /// If kernel_tracker_max_pending > 0 then no more than this many
        /// tracking events can be outstanding at a time.  An attempt to
        /// launch an additional kernel will stall until an event
        /// completes.
        #[prost(int32, tag = "9")]
        pub kernel_tracker_max_pending: i32,
        /// BFC Allocator can return an allocated chunk of memory upto 2x the
        /// requested size. For virtual devices with tight memory constraints, and
        /// proportionately large allocation requests, this can lead to a significant
        /// reduction in available memory. The threshold below controls when a chunk
        /// should be split if the chunk size exceeds requested memory size. It is
        /// expressed as a fraction of total available memory for the tf device. For
        /// example setting it to 0.05 would imply a chunk needs to be split if its
        /// size exceeds the requested memory by 5% of the total virtual device/gpu
        /// memory size.
        #[prost(double, tag = "10")]
        pub internal_fragmentation_fraction: f64,
        /// When true, use CUDA cudaMallocAsync API instead of TF gpu allocator.
        #[prost(bool, tag = "11")]
        pub use_cuda_malloc_async: bool,
        /// By default, BFCAllocator may sleep when it runs out of memory, in the
        /// hopes that another thread will free up memory in the meantime.  Setting
        /// this to true disables the sleep; instead we'll OOM immediately.
        #[prost(bool, tag = "12")]
        pub disallow_retry_on_allocation_failure: bool,
    }
    /// Nested message and enum types in `Experimental`.
    pub mod experimental {
        /// Configuration for breaking down a visible GPU into multiple "virtual"
        /// devices.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct VirtualDevices {
            /// Per "virtual" device memory limit, in MB. The number of elements in
            /// the list is the number of virtual devices to create on the
            /// corresponding visible GPU (see "virtual_devices" below).
            /// If empty, it will create single virtual device taking all available
            /// memory from the device.
            ///
            /// For the concept of "visible" and "virtual" GPU, see the comments for
            /// "visible_device_list" above for more information.
            #[prost(float, repeated, tag = "1")]
            pub memory_limit_mb: ::prost::alloc::vec::Vec<f32>,
            /// Priority values to use with the virtual devices. Use the cuda function
            /// cudaDeviceGetStreamPriorityRange to query for valid range of values for
            /// priority.
            ///
            /// On a P4000 GPU with cuda 10.1, the priority range reported was 0 for
            /// least priority and -1 for greatest priority.
            ///
            /// If this field is not specified, then the virtual devices will be
            /// created with the default. If this field has values set, then the size
            /// of this must match with the above memory_limit_mb.
            #[prost(int32, repeated, tag = "2")]
            pub priority: ::prost::alloc::vec::Vec<i32>,
            /// Virtual Device ordinal number determines the device ID of the device.
            /// A Virtual device with a lower ordinal number always receives the a
            /// smaller device id. The phyiscal device id and location in the
            /// virtual device list is used to break ties.
            #[prost(int32, repeated, tag = "3")]
            pub device_ordinal: ::prost::alloc::vec::Vec<i32>,
        }
    }
}
/// Options passed to the graph optimizer
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptimizerOptions {
    /// If true, optimize the graph using common subexpression elimination.
    /// Note: the optimization Level L1 will override this setting to true. So in
    /// order to disable common subexpression elimination the opt_level has to be
    /// set to L0.
    #[prost(bool, tag = "1")]
    pub do_common_subexpression_elimination: bool,
    /// If true, perform constant folding optimization on the graph.
    /// Note: the optimization Level L1 will override this setting to true. So in
    /// order to disable constant folding the opt_level has to be set to L0.
    #[prost(bool, tag = "2")]
    pub do_constant_folding: bool,
    /// Constant folding optimization replaces tensors whose values can be
    /// predetermined, with constant nodes. To avoid inserting too large constants,
    /// the size of each constant created can be limited. If this value is zero, a
    /// default limit of 10 MiB will be applied. If constant folding optimization
    /// is disabled, this value is ignored.
    #[prost(int64, tag = "6")]
    pub max_folded_constant_in_bytes: i64,
    /// If true, perform function inlining on the graph.
    #[prost(bool, tag = "4")]
    pub do_function_inlining: bool,
    /// Overall optimization level. The actual optimizations applied will be the
    /// logical OR of the flags that this level implies and any flags already set.
    #[prost(enumeration = "optimizer_options::Level", tag = "3")]
    pub opt_level: i32,
    #[prost(enumeration = "optimizer_options::GlobalJitLevel", tag = "5")]
    pub global_jit_level: i32,
    /// CPU code will be autoclustered only if global_jit_level >= ON_1 and either:
    ///
    /// * this flag is true, or
    /// * TF_XLA_FLAGS contains --tf_xla_cpu_global_jit=true.
    #[prost(bool, tag = "7")]
    pub cpu_global_jit: bool,
}
/// Nested message and enum types in `OptimizerOptions`.
pub mod optimizer_options {
    /// Optimization level
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Level {
        /// L1 is the default level.
        /// Optimization performed at L1 :
        ///
        /// 1. Common subexpression elimination
        /// 1. Constant folding
        L1 = 0,
        /// No optimizations
        L0 = -1,
    }
    impl Level {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Level::L1 => "L1",
                Level::L0 => "L0",
            }
        }
    }
    /// Control the use of the compiler/jit.  Experimental.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum GlobalJitLevel {
        /// Default setting ("off" now, but later expected to be "on")
        Default = 0,
        Off = -1,
        /// The following settings turn on compilation, with higher values being
        /// more aggressive.  Higher values may reduce opportunities for parallelism
        /// and may use more memory.  (At present, there is no distinction, but this
        /// is expected to change.)
        On1 = 1,
        On2 = 2,
    }
    impl GlobalJitLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GlobalJitLevel::Default => "DEFAULT",
                GlobalJitLevel::Off => "OFF",
                GlobalJitLevel::On1 => "ON_1",
                GlobalJitLevel::On2 => "ON_2",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GraphOptions {
    /// If true, use control flow to schedule the activation of Recv nodes.
    /// (Currently ignored.)
    #[prost(bool, tag = "2")]
    pub enable_recv_scheduling: bool,
    /// Options controlling how graph is optimized.
    #[prost(message, optional, tag = "3")]
    pub optimizer_options: ::core::option::Option<OptimizerOptions>,
    /// The number of steps to run before returning a cost model detailing
    /// the memory usage and performance of each node of the graph. 0 means
    /// no cost model.
    #[prost(int64, tag = "4")]
    pub build_cost_model: i64,
    /// The number of steps to skip before collecting statistics for the
    /// cost model.
    #[prost(int64, tag = "9")]
    pub build_cost_model_after: i64,
    /// Annotate each Node with Op output shape data, to the extent it can
    /// be statically inferred.
    #[prost(bool, tag = "5")]
    pub infer_shapes: bool,
    /// Only place the subgraphs that are run, rather than the entire graph.
    ///
    /// This is useful for interactive graph building, where one might
    /// produce graphs that cannot be placed during the debugging
    /// process.  In particular, it allows the client to continue work in
    /// a session after adding a node to a graph whose placement
    /// constraints are unsatisfiable.
    #[prost(bool, tag = "6")]
    pub place_pruned_graph: bool,
    /// If true, transfer float values between processes as bfloat16.
    #[prost(bool, tag = "7")]
    pub enable_bfloat16_sendrecv: bool,
    /// If > 0, record a timeline every this many steps.
    /// EXPERIMENTAL: This currently has no effect in MasterSession.
    #[prost(int32, tag = "8")]
    pub timeline_step: i32,
    /// Options that control the type and amount of graph rewriting.
    /// Not currently configurable via the public Python API (i.e. there is no API
    /// stability guarantee if you import RewriterConfig explicitly).
    #[prost(message, optional, tag = "10")]
    pub rewrite_options: ::core::option::Option<RewriterConfig>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreadPoolOptionProto {
    /// The number of threads in the pool.
    ///
    /// 0 means the system picks a value based on where this option proto is used
    /// (see the declaration of the specific field for more info).
    #[prost(int32, tag = "1")]
    pub num_threads: i32,
    /// The global name of the threadpool.
    ///
    /// If empty, then the threadpool is made and used according to the scope it's
    /// in - e.g., for a session threadpool, it is used by that session only.
    ///
    /// If non-empty, then:
    ///
    /// * a global threadpool associated with this name is looked
    ///   up or created. This allows, for example, sharing one threadpool across
    ///   many sessions (e.g., like the default behavior, if
    ///   inter_op_parallelism_threads is not configured), but still partitioning
    ///   into a large and small pool.
    /// * if the threadpool for this global_name already exists, then it is an
    ///   error if the existing pool was created using a different num_threads
    ///   value as is specified on this call.
    /// * threadpools created this way are never garbage collected.
    #[prost(string, tag = "2")]
    pub global_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpcOptions {
    /// If true, always use RPC to contact the session target.
    ///
    /// If false (the default option), TensorFlow may use an optimized
    /// transport for client-master communication that avoids the RPC
    /// stack. This option is primarily for used testing the RPC stack.
    #[prost(bool, tag = "1")]
    pub use_rpc_for_inprocess_master: bool,
    /// The compression algorithm to be used. One of "deflate", "gzip".
    #[prost(string, tag = "2")]
    pub compression_algorithm: ::prost::alloc::string::String,
    /// If compression_algorithm is set, the compression level to be used.
    /// From 0 (no compression), up to 3.
    #[prost(int32, tag = "3")]
    pub compression_level: i32,
    /// Setting cache_rpc_response to true will enable sender side caching of
    /// response for RecvTensorAsync and RecvBufAsync to allow receiver to retry
    /// requests . This is only necessary when the network fabric is experiencing a
    /// significant error rate.  Without it we'll fail a step on an network error,
    /// while with it we'll be able to complete long steps (like complex
    /// initializations) in the face of some network errors during RecvTensor.
    #[prost(bool, tag = "4")]
    pub cache_rpc_response: bool,
    /// Disables TCP connection sharing when opening a new RPC channel.
    #[prost(bool, tag = "5")]
    pub disable_session_connection_sharing: bool,
    /// Setting num_channels_per_target > 0 allows uses of multiple channels to
    /// communicate to the same target. This can be used to improve the aggregate
    /// throughput on high speed links (e.g 100G) where single connection is not
    /// sufficient to maximize link utilization. Note that a single RPC only goes
    /// on a single channel, this only helps in situations where there are multiple
    /// transfers to the same target overlapping in time.
    #[prost(int32, tag = "6")]
    pub num_channels_per_target: i32,
}
/// Metadata about the session.
///
/// This can be used by the runtime and the Ops for debugging, monitoring, etc.
///
/// The (name, version) tuple is expected to be a unique identifier for
/// sessions within the same process.
///
/// NOTE: This is currently used and propagated only by the direct session.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionMetadata {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The version is optional. If set, needs to be >= 0.
    #[prost(int64, tag = "2")]
    pub version: i64,
}
/// Session configuration parameters.
/// The system picks appropriate values for fields that are not set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigProto {
    /// Map from device type name (e.g., "CPU" or "GPU" ) to maximum
    /// number of devices of that type to use.  If a particular device
    /// type is not found in the map, the system picks an appropriate
    /// number.
    #[prost(map = "string, int32", tag = "1")]
    pub device_count: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
    /// The execution of an individual op (for some op types) can be
    /// parallelized on a pool of intra_op_parallelism_threads.
    /// 0 means the system picks an appropriate number.
    ///
    /// If you create an ordinary session, e.g., from Python or C++,
    /// then there is exactly one intra op thread pool per process.
    /// The first session created determines the number of threads in this pool.
    /// All subsequent sessions reuse/share this one global pool.
    ///
    /// There are notable exceptions to the default behavior described above:
    ///
    /// 1. There is an environment variable  for overriding this thread pool,
    ///    named TF_OVERRIDE_GLOBAL_THREADPOOL.
    /// 1. When connecting to a server, such as a remote `tf.train.Server`
    ///    instance, then this option will be ignored altogether.
    #[prost(int32, tag = "2")]
    pub intra_op_parallelism_threads: i32,
    /// Nodes that perform blocking operations are enqueued on a pool of
    /// inter_op_parallelism_threads available in each process.
    ///
    /// 0 means the system picks an appropriate number.
    /// Negative means all operations are performed in caller's thread.
    ///
    /// Note that the first Session created in the process sets the
    /// number of threads for all future sessions unless use_per_session_threads is
    /// true or session_inter_op_thread_pool is configured.
    #[prost(int32, tag = "5")]
    pub inter_op_parallelism_threads: i32,
    /// If true, use a new set of threads for this session rather than the global
    /// pool of threads. Only supported by direct sessions.
    ///
    /// If false, use the global threads created by the first session, or the
    /// per-session thread pools configured by session_inter_op_thread_pool.
    ///
    /// This option is deprecated. The same effect can be achieved by setting
    /// session_inter_op_thread_pool to have one element, whose num_threads equals
    /// inter_op_parallelism_threads.
    #[prost(bool, tag = "9")]
    pub use_per_session_threads: bool,
    /// This option is experimental - it may be replaced with a different mechanism
    /// in the future.
    ///
    /// Configures session thread pools. If this is configured, then RunOptions for
    /// a Run call can select the thread pool to use.
    ///
    /// The intended use is for when some session invocations need to run in a
    /// background pool limited to a small number of threads:
    ///
    /// * For example, a session may be configured to have one large pool (for
    ///   regular compute) and one small pool (for periodic, low priority work);
    ///   using the small pool is currently the mechanism for limiting the inter-op
    ///   parallelism of the low priority work.  Note that it does not limit the
    ///   parallelism of work spawned by a single op kernel implementation.
    /// * Using this setting is normally not needed in training, but may help some
    ///   serving use cases.
    /// * It is also generally recommended to set the global_name field of this
    ///   proto, to avoid creating multiple large pools. It is typically better to
    ///   run the non-low-priority work, even across sessions, in a single large
    ///   pool.
    #[prost(message, repeated, tag = "12")]
    pub session_inter_op_thread_pool: ::prost::alloc::vec::Vec<ThreadPoolOptionProto>,
    /// Assignment of Nodes to Devices is recomputed every placement_period
    /// steps until the system warms up (at which point the recomputation
    /// typically slows down automatically).
    #[prost(int32, tag = "3")]
    pub placement_period: i32,
    /// When any filters are present sessions will ignore all devices which do not
    /// match the filters. Each filter can be partially specified, e.g. "/job:ps"
    /// "/job:worker/replica:3", etc.
    #[prost(string, repeated, tag = "4")]
    pub device_filters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Options that apply to all GPUs.
    #[prost(message, optional, tag = "6")]
    pub gpu_options: ::core::option::Option<GpuOptions>,
    /// Whether soft placement is allowed. If allow_soft_placement is true,
    /// an op will be placed on CPU if
    ///
    /// 1. there's no GPU implementation for the OP
    ///    or
    /// 1. no GPU devices are known or registered
    ///    or
    /// 1. need to co-locate with reftype input(s) which are from CPU.
    #[prost(bool, tag = "7")]
    pub allow_soft_placement: bool,
    /// Whether device placements should be logged.
    #[prost(bool, tag = "8")]
    pub log_device_placement: bool,
    /// Options that apply to all graphs.
    #[prost(message, optional, tag = "10")]
    pub graph_options: ::core::option::Option<GraphOptions>,
    /// Global timeout for all blocking operations in this session.  If non-zero,
    /// and not overridden on a per-operation basis, this value will be used as the
    /// deadline for all blocking operations.
    #[prost(int64, tag = "11")]
    pub operation_timeout_in_ms: i64,
    /// Options that apply when this session uses the distributed runtime.
    #[prost(message, optional, tag = "13")]
    pub rpc_options: ::core::option::Option<RpcOptions>,
    /// Optional list of all workers to use in this session.
    #[prost(message, optional, tag = "14")]
    pub cluster_def: ::core::option::Option<ClusterDef>,
    /// If true, any resources such as Variables used in the session will not be
    /// shared with other sessions. However, when clusterspec propagation is
    /// enabled, this field is ignored and sessions are always isolated.
    #[prost(bool, tag = "15")]
    pub isolate_session_state: bool,
    /// When true, WorkerSessions are created with device attributes from the
    /// full cluster.
    /// This is helpful when a worker wants to partition a graph
    /// (for example during a PartitionedCallOp).
    #[prost(bool, tag = "17")]
    pub share_cluster_devices_in_session: bool,
    #[prost(message, optional, tag = "16")]
    pub experimental: ::core::option::Option<config_proto::Experimental>,
}
/// Nested message and enum types in `ConfigProto`.
pub mod config_proto {
    /// Everything inside Experimental is subject to change and is not subject
    /// to API stability guarantees in
    /// <https://www.tensorflow.org/guide/version_compat.>
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Experimental {
        /// Task name for group resolution.
        #[prost(string, tag = "1")]
        pub collective_group_leader: ::prost::alloc::string::String,
        /// Which executor to use, the default executor will be used
        /// if it is an empty string or "DEFAULT"
        #[prost(string, tag = "3")]
        pub executor_type: ::prost::alloc::string::String,
        /// Guidance to formatting of large RecvBuf fields for transfer.
        /// Any positive value sets the max chunk size.  0 defaults to 4096.
        /// Any negative value indicates no max, i.e. one chunk only.
        #[prost(int32, tag = "4")]
        pub recv_buf_max_chunk: i32,
        /// If true, and supported by the platform, the runtime will attempt to
        /// use NUMA affinity where applicable.  One consequence will be the
        /// existence of as many CPU devices as there are available NUMA nodes.
        #[prost(bool, tag = "5")]
        pub use_numa_affinity: bool,
        /// If true, make collective op execution order sequential and deterministic
        /// for potentially concurrent collective instances.
        #[prost(bool, tag = "6")]
        pub collective_deterministic_sequential_execution: bool,
        /// If true, use NCCL for CollectiveOps.  This feature is highly
        /// experimental.
        #[prost(bool, tag = "7")]
        pub collective_nccl: bool,
        /// In the following, session state means the value of a variable, elements
        /// in a hash table, or any other resource, accessible by worker sessions
        /// held by a TF server.
        ///
        /// When ClusterSpec propagation is enabled, the value of
        /// isolate_session_state is ignored when deciding whether to share session
        /// states in a TF server (for backwards compatibility reasons).
        ///
        /// * If share_session_state_in_clusterspec_propagation is true, the session
        ///   states are shared.
        /// * If share_session_state_in_clusterspec_propagation is false, session
        ///   states are isolated.
        ///
        /// When clusterspec propagation is not used, the value of
        /// share_session_state_in_clusterspec_propagation is ignored when deciding
        /// whether to share session states in a TF server.
        ///
        /// * If isolate_session_state is true, session states are isolated.
        /// * If isolate_session_state is false, session states are shared.
        ///
        /// TODO(b/129330037): Add a single API that consistently treats
        /// isolate_session_state and ClusterSpec propagation.
        #[prost(bool, tag = "8")]
        pub share_session_state_in_clusterspec_propagation: bool,
        /// If using a direct session, disable spinning while waiting for work in
        /// the thread pool. This may result in higher latency for completing ops,
        /// but in the case where there is a lot of spinning may result in lower
        /// CPU usage.
        #[prost(bool, tag = "9")]
        pub disable_thread_spinning: bool,
        /// This was promoted to a non-experimental API. Please use
        /// ConfigProto.share_cluster_devices_in_session instead.
        #[prost(bool, tag = "10")]
        pub share_cluster_devices_in_session: bool,
        /// Metadata about the session.
        ///
        /// If set, this can be used by the runtime and the Ops for debugging,
        /// monitoring, etc.
        ///
        /// NOTE: This is currently used and propagated only by the direct session.
        #[prost(message, optional, tag = "11")]
        pub session_metadata: ::core::option::Option<super::SessionMetadata>,
        /// If true, the session may treat the graph as being static for optimization
        /// purposes.
        ///
        /// If this option is set to true when a session is created, the full
        /// GraphDef must be passed in a single call to Session::Create(), and
        /// Session::Extend() may not be supported.
        #[prost(bool, tag = "12")]
        pub optimize_for_static_graph: bool,
        /// This field will eventually be deprecated and replaced by
        /// mlir_bridge_rollout (b/166038521).
        ///
        /// Whether to enable the MLIR-based TF->XLA bridge.
        ///
        /// This is a replacement to the existing bridge, and not ready for
        /// production usage yet.
        /// If this option is set to true when a session is created, MLIR is used to
        /// perform the set of graph transformations to put the graph in a form that
        /// can be executed with delegation of some computations to an accelerator.
        /// This builds on the model of XLA where a subset of the graph is
        /// encapsulated and attached to a "compile" operation, whose result is fed
        /// to an "execute" operation. The kernel for these operations is responsible
        /// to lower the encapsulated graph to a particular device.
        #[prost(bool, tag = "13")]
        pub enable_mlir_bridge: bool,
        /// This field is underdevelopment, for now use enable_mlir_bridge
        /// (b/166038521).
        ///
        /// Whether to enable the MLIR-based TF->XLA bridge.
        #[prost(enumeration = "experimental::MlirBridgeRollout", tag = "17")]
        pub mlir_bridge_rollout: i32,
        /// Whether to enable the MLIR-based Graph optimizations.
        ///
        /// This will become a part of standard Tensorflow graph optimization
        /// pipeline, currently this is only used for gradual migration and testing
        /// new passes that are replacing existing optimizations in Grappler.
        #[prost(bool, tag = "16")]
        pub enable_mlir_graph_optimization: bool,
        /// If true, the session will not store an additional copy of the graph for
        /// each subgraph.
        ///
        /// If this option is set to true when a session is created, the
        /// `RunOptions.output_partition_graphs` options must not be set.
        #[prost(bool, tag = "14")]
        pub disable_output_partition_graphs: bool,
        /// Minimum number of batches run through the XLA graph before XLA fusion
        /// autotuner is enabled. Default value of zero disables the autotuner.
        ///
        /// The XLA fusion autotuner can improve performance by executing a heuristic
        /// search on the compiler parameters.
        #[prost(int64, tag = "15")]
        pub xla_fusion_autotuner_thresh: i64,
        /// Whether runtime execution uses TFRT.
        #[prost(bool, tag = "18")]
        pub use_tfrt: bool,
        /// Whether functional control flow op lowering should be disabled. This is
        /// useful when executing within a portable runtime where control flow op
        /// kernels may not be loaded due to selective registration.
        #[prost(bool, tag = "21")]
        pub disable_functional_ops_lowering: bool,
        /// Provides a hint to XLA auto clustering to prefer forming a single large
        /// cluster that encompases most of the graph.
        #[prost(bool, tag = "22")]
        pub xla_prefer_single_graph_cluster: bool,
        /// Distributed coordination service configurations.
        #[prost(message, optional, tag = "23")]
        pub coordination_config: ::core::option::Option<
            super::CoordinationServiceConfig,
        >,
    }
    /// Nested message and enum types in `Experimental`.
    pub mod experimental {
        /// An enum that describes the state of the MLIR bridge rollout.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum MlirBridgeRollout {
            /// If this field is left unspecified, the MLIR bridge may be selectively
            /// enabled on a per graph basis.
            Unspecified = 0,
            /// Enabling the MLIR bridge enables it for all graphs in this session.
            Enabled = 1,
            /// Disabling the MLIR bridge disables it for all graphs in this session.
            Disabled = 2,
            /// Enable the MLIR bridge on a per graph basis based on an analysis of
            /// the features used in the graph. If the features used by the graph are
            /// supported by the MLIR bridge, the MLIR bridge will be used to run the
            /// graph.
            SafeModeEnabled = 3,
            /// Enable the MLIR bridge in a fallback mode on a per graph basis based
            /// on an analysis of the features used in the graph.
            /// Running the MLIR bridge in the fallback mode means that it is
            /// executed and it commits all the changes to the TF graph in case
            /// of success. And it does not in case of failures and let the old bridge
            /// to process the TF graph.
            SafeModeFallbackEnabled = 4,
        }
        impl MlirBridgeRollout {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    MlirBridgeRollout::Unspecified => "MLIR_BRIDGE_ROLLOUT_UNSPECIFIED",
                    MlirBridgeRollout::Enabled => "MLIR_BRIDGE_ROLLOUT_ENABLED",
                    MlirBridgeRollout::Disabled => "MLIR_BRIDGE_ROLLOUT_DISABLED",
                    MlirBridgeRollout::SafeModeEnabled => {
                        "MLIR_BRIDGE_ROLLOUT_SAFE_MODE_ENABLED"
                    }
                    MlirBridgeRollout::SafeModeFallbackEnabled => {
                        "MLIR_BRIDGE_ROLLOUT_SAFE_MODE_FALLBACK_ENABLED"
                    }
                }
            }
        }
    }
}
/// Options for a single Run() call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunOptions {
    #[prost(enumeration = "run_options::TraceLevel", tag = "1")]
    pub trace_level: i32,
    /// Time to wait for operation to complete in milliseconds.
    #[prost(int64, tag = "2")]
    pub timeout_in_ms: i64,
    /// The thread pool to use, if session_inter_op_thread_pool is configured.
    /// To use the caller thread set this to -1 - this uses the caller thread
    /// to execute Session::Run() and thus avoids a context switch. Using the
    /// caller thread to execute Session::Run() should be done ONLY for simple
    /// graphs, where the overhead of an additional context switch is
    /// comparable with the overhead of Session::Run().
    #[prost(int32, tag = "3")]
    pub inter_op_thread_pool: i32,
    /// Whether the partition graph(s) executed by the executor(s) should be
    /// outputted via RunMetadata.
    #[prost(bool, tag = "5")]
    pub output_partition_graphs: bool,
    /// EXPERIMENTAL.  Options used to initialize DebuggerState, if enabled.
    #[prost(message, optional, tag = "6")]
    pub debug_options: ::core::option::Option<DebugOptions>,
    /// When enabled, causes tensor allocation information to be included in
    /// the error message when the Run() call fails because the allocator ran
    /// out of memory (OOM).
    ///
    /// Enabling this option can slow down the Run() call.
    #[prost(bool, tag = "7")]
    pub report_tensor_allocations_upon_oom: bool,
    #[prost(message, optional, tag = "8")]
    pub experimental: ::core::option::Option<run_options::Experimental>,
}
/// Nested message and enum types in `RunOptions`.
pub mod run_options {
    /// Everything inside Experimental is subject to change and is not subject
    /// to API stability guarantees in
    /// <https://www.tensorflow.org/guide/version_compat.>
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Experimental {
        /// If non-zero, declares that this graph is going to use collective
        /// ops and must synchronize step_ids with any other graph with this
        /// same group_key value (in a distributed computation where tasks
        /// run disjoint graphs).
        #[prost(int64, tag = "1")]
        pub collective_graph_key: i64,
        /// If true, then operations (using the inter-op pool) across all
        /// session::run() calls will be centrally scheduled, optimizing for (median
        /// and tail) latency.
        /// Consider using this option for CPU-bound workloads like inference.
        #[prost(bool, tag = "2")]
        pub use_run_handler_pool: bool,
        #[prost(message, optional, tag = "3")]
        pub run_handler_pool_options: ::core::option::Option<
            experimental::RunHandlerPoolOptions,
        >,
    }
    /// Nested message and enum types in `Experimental`.
    pub mod experimental {
        /// Options for run handler thread pool.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RunHandlerPoolOptions {
            /// Priority of the request. The run handler thread pool will schedule ops
            /// based on the priority number. The larger number means higher priority.
            #[prost(int64, tag = "1")]
            pub priority: i64,
        }
    }
    /// TODO(pbar) Turn this into a TraceOptions proto which allows
    /// tracing to be controlled in a more orthogonal manner?
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum TraceLevel {
        NoTrace = 0,
        SoftwareTrace = 1,
        HardwareTrace = 2,
        FullTrace = 3,
    }
    impl TraceLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TraceLevel::NoTrace => "NO_TRACE",
                TraceLevel::SoftwareTrace => "SOFTWARE_TRACE",
                TraceLevel::HardwareTrace => "HARDWARE_TRACE",
                TraceLevel::FullTrace => "FULL_TRACE",
            }
        }
    }
}
/// Metadata output (i.e., non-Tensor) for a single Run() call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunMetadata {
    /// Statistics traced for this step. Populated if tracing is turned on via the
    /// "RunOptions" proto.
    /// EXPERIMENTAL: The format and set of events may change in future versions.
    #[prost(message, optional, tag = "1")]
    pub step_stats: ::core::option::Option<StepStats>,
    /// The cost graph for the computation defined by the run call.
    #[prost(message, optional, tag = "2")]
    pub cost_graph: ::core::option::Option<CostGraphDef>,
    /// Graphs of the partitions executed by executors.
    #[prost(message, repeated, tag = "3")]
    pub partition_graphs: ::prost::alloc::vec::Vec<GraphDef>,
    /// This is only populated for graphs that are run as functions in TensorFlow
    /// V2. There will be an entry below for each function that is traced.
    /// The main use cases of the post_optimization_graph and the partition_graphs
    /// is to give the caller insight into the graphs that were actually run by the
    /// runtime. Additional information (such as those in step_stats) will match
    /// these graphs.
    /// We also include the pre_optimization_graph since it is usually easier to
    /// read, and is helpful in situations where the caller wants to get a high
    /// level idea of what the built graph looks like (since the various graph
    /// optimization passes might change the structure of the graph significantly).
    #[prost(message, repeated, tag = "4")]
    pub function_graphs: ::prost::alloc::vec::Vec<run_metadata::FunctionGraphs>,
    /// Metadata about the session.
    #[prost(message, optional, tag = "5")]
    pub session_metadata: ::core::option::Option<SessionMetadata>,
}
/// Nested message and enum types in `RunMetadata`.
pub mod run_metadata {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FunctionGraphs {
        /// TODO(nareshmodi): Include some sort of function/cache-key identifier?
        #[prost(message, repeated, tag = "1")]
        pub partition_graphs: ::prost::alloc::vec::Vec<super::GraphDef>,
        #[prost(message, optional, tag = "2")]
        pub pre_optimization_graph: ::core::option::Option<super::GraphDef>,
        #[prost(message, optional, tag = "3")]
        pub post_optimization_graph: ::core::option::Option<super::GraphDef>,
    }
}
/// Defines a connection between two tensors in a `GraphDef`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TensorConnection {
    /// A tensor name. The value of this tensor will be substituted for
    /// the tensor named in `to_tensor`.
    #[prost(string, tag = "1")]
    pub from_tensor: ::prost::alloc::string::String,
    /// A tensor name. The value of this tensor will be bound to the
    /// value of the tensor named in `from_tensor`.
    #[prost(string, tag = "2")]
    pub to_tensor: ::prost::alloc::string::String,
}
/// Defines a subgraph in another `GraphDef` as a set of feed points and nodes
/// to be fetched or executed.
///
/// Compare with the arguments to `Session::Run()`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallableOptions {
    /// Tensors to be fed in the callable. Each feed is the name of a tensor.
    #[prost(string, repeated, tag = "1")]
    pub feed: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Fetches. A list of tensor names. The caller of the callable expects a
    /// tensor to be returned for each fetch\\[i\\] (see RunStepResponse.tensor). The
    /// order of specified fetches does not change the execution order.
    #[prost(string, repeated, tag = "2")]
    pub fetch: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Target Nodes. A list of node names. The named nodes will be run by the
    /// callable but their outputs will not be returned.
    #[prost(string, repeated, tag = "3")]
    pub target: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Options that will be applied to each run.
    #[prost(message, optional, tag = "4")]
    pub run_options: ::core::option::Option<RunOptions>,
    /// Tensors to be connected in the callable. Each TensorConnection denotes
    /// a pair of tensors in the graph, between which an edge will be created
    /// in the callable.
    #[prost(message, repeated, tag = "5")]
    pub tensor_connection: ::prost::alloc::vec::Vec<TensorConnection>,
    /// The Tensor objects fed in the callable and fetched from the callable
    /// are expected to be backed by host (CPU) memory by default.
    ///
    /// The options below allow changing that - feeding tensors backed by
    /// device memory, or returning tensors that are backed by device memory.
    ///
    /// The maps below map the name of a feed/fetch tensor (which appears in
    /// 'feed' or 'fetch' fields above), to the fully qualified name of the device
    /// owning the memory backing the contents of the tensor.
    ///
    /// For example, creating a callable with the following options:
    ///
    /// CallableOptions {
    /// feed: "a:0"
    /// feed: "b:0"
    ///
    /// fetch: "x:0"
    /// fetch: "y:0"
    ///
    /// feed_devices: {
    /// "a:0": "/job:localhost/replica:0/task:0/device:GPU:0"
    /// }
    ///
    /// fetch_devices: {
    /// "y:0": "/job:localhost/replica:0/task:0/device:GPU:0"
    /// }
    /// }
    ///
    /// means that the Callable expects:
    ///
    /// * The first argument ("a:0") is a Tensor backed by GPU memory.
    /// * The second argument ("b:0") is a Tensor backed by host memory.
    ///   and of its return values:
    /// * The first output ("x:0") will be backed by host memory.
    /// * The second output ("y:0") will be backed by GPU memory.
    ///
    /// FEEDS:
    /// It is the responsibility of the caller to ensure that the memory of the fed
    /// tensors will be correctly initialized and synchronized before it is
    /// accessed by operations executed during the call to Session::RunCallable().
    ///
    /// This is typically ensured by using the TensorFlow memory allocators
    /// (Device::GetAllocator()) to create the Tensor to be fed.
    ///
    /// Alternatively, for CUDA-enabled GPU devices, this typically means that the
    /// operation that produced the contents of the tensor has completed, i.e., the
    /// CUDA stream has been synchronized (e.g., via cuCtxSynchronize() or
    /// cuStreamSynchronize()).
    #[prost(map = "string, string", tag = "6")]
    pub feed_devices: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(map = "string, string", tag = "7")]
    pub fetch_devices: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// By default, RunCallable() will synchronize the GPU stream before returning
    /// fetched tensors on a GPU device, to ensure that the values in those tensors
    /// have been produced. This simplifies interacting with the tensors, but
    /// potentially incurs a performance hit.
    ///
    /// If this options is set to true, the caller is responsible for ensuring
    /// that the values in the fetched tensors have been produced before they are
    /// used. The caller can do this by invoking `Device::Sync()` on the underlying
    /// device(s), or by feeding the tensors back to the same Session using
    /// `feed_devices` with the same corresponding device name.
    #[prost(bool, tag = "8")]
    pub fetch_skip_sync: bool,
}
