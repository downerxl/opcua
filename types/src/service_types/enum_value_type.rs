// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    basic_types::*, encoding::*, localized_text::LocalizedText, node_ids::ObjectId,
    service_types::impls::MessageInfo,
};

#[derive(Debug, Clone, PartialEq)]
pub struct EnumValueType {
    pub value: i64,
    pub display_name: LocalizedText,
    pub description: LocalizedText,
}

impl MessageInfo for EnumValueType {
    fn object_id(&self) -> ObjectId {
        ObjectId::EnumValueType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<EnumValueType> for EnumValueType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.value.byte_len();
        size += self.display_name.byte_len();
        size += self.description.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.value.encode(stream)?;
        size += self.display_name.encode(stream)?;
        size += self.description.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let value = i64::decode(stream, decoding_options)?;
        let display_name = LocalizedText::decode(stream, decoding_options)?;
        let description = LocalizedText::decode(stream, decoding_options)?;
        Ok(EnumValueType {
            value,
            display_name,
            description,
        })
    }
}
