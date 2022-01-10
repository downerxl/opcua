// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    basic_types::*, date_time::DateTime, encoding::*, node_id::NodeId, node_ids::ObjectId,
    service_types::impls::MessageInfo,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ViewDescription {
    pub view_id: NodeId,
    pub timestamp: DateTime,
    pub view_version: u32,
}

impl MessageInfo for ViewDescription {
    fn object_id(&self) -> ObjectId {
        ObjectId::ViewDescription_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ViewDescription> for ViewDescription {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.view_id.byte_len();
        size += self.timestamp.byte_len();
        size += self.view_version.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.view_id.encode(stream)?;
        size += self.timestamp.encode(stream)?;
        size += self.view_version.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let view_id = NodeId::decode(stream, decoding_options)?;
        let timestamp = DateTime::decode(stream, decoding_options)?;
        let view_version = u32::decode(stream, decoding_options)?;
        Ok(ViewDescription {
            view_id,
            timestamp,
            view_version,
        })
    }
}
