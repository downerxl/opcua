// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    basic_types::*, encoding::*, node_id::NodeId, node_ids::ObjectId,
    qualified_name::QualifiedName, service_types::impls::MessageInfo,
};

#[derive(Debug, Clone, PartialEq)]
pub struct RelativePathElement {
    pub reference_type_id: NodeId,
    pub is_inverse: bool,
    pub include_subtypes: bool,
    pub target_name: QualifiedName,
}

impl MessageInfo for RelativePathElement {
    fn object_id(&self) -> ObjectId {
        ObjectId::RelativePathElement_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<RelativePathElement> for RelativePathElement {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.reference_type_id.byte_len();
        size += self.is_inverse.byte_len();
        size += self.include_subtypes.byte_len();
        size += self.target_name.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.reference_type_id.encode(stream)?;
        size += self.is_inverse.encode(stream)?;
        size += self.include_subtypes.encode(stream)?;
        size += self.target_name.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let reference_type_id = NodeId::decode(stream, decoding_options)?;
        let is_inverse = bool::decode(stream, decoding_options)?;
        let include_subtypes = bool::decode(stream, decoding_options)?;
        let target_name = QualifiedName::decode(stream, decoding_options)?;
        Ok(RelativePathElement {
            reference_type_id,
            is_inverse,
            include_subtypes,
            target_name,
        })
    }
}
