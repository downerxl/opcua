// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{basic_types::*, encoding::*, service_types::RolePermissionType, string::UAString};

#[derive(Debug, Clone, PartialEq)]
pub struct SubscribedDataSetMirrorDataType {
    pub parent_node_name: UAString,
    pub role_permissions: Option<Vec<RolePermissionType>>,
}

impl BinaryEncoder<SubscribedDataSetMirrorDataType> for SubscribedDataSetMirrorDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.parent_node_name.byte_len();
        size += byte_len_array(&self.role_permissions);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.parent_node_name.encode(stream)?;
        size += write_array(stream, &self.role_permissions)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let parent_node_name = UAString::decode(stream, decoding_options)?;
        let role_permissions: Option<Vec<RolePermissionType>> =
            read_array(stream, decoding_options)?;
        Ok(SubscribedDataSetMirrorDataType {
            parent_node_name,
            role_permissions,
        })
    }
}
