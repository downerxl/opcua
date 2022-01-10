// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    basic_types::*, data_value::DataValue, encoding::*, node_id::NodeId, node_ids::ObjectId,
    service_types::impls::MessageInfo, string::UAString,
};

#[derive(Debug, Clone, PartialEq)]
pub struct WriteValue {
    pub node_id: NodeId,
    pub attribute_id: u32,
    pub index_range: UAString,
    pub value: DataValue,
}

impl MessageInfo for WriteValue {
    fn object_id(&self) -> ObjectId {
        ObjectId::WriteValue_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<WriteValue> for WriteValue {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.node_id.byte_len();
        size += self.attribute_id.byte_len();
        size += self.index_range.byte_len();
        size += self.value.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.node_id.encode(stream)?;
        size += self.attribute_id.encode(stream)?;
        size += self.index_range.encode(stream)?;
        size += self.value.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let node_id = NodeId::decode(stream, decoding_options)?;
        let attribute_id = u32::decode(stream, decoding_options)?;
        let index_range = UAString::decode(stream, decoding_options)?;
        let value = DataValue::decode(stream, decoding_options)?;
        Ok(WriteValue {
            node_id,
            attribute_id,
            index_range,
            value,
        })
    }
}
