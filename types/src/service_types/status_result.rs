// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    basic_types::*, diagnostic_info::DiagnosticInfo, encoding::*, node_ids::ObjectId,
    service_types::impls::MessageInfo, status_codes::StatusCode,
};

#[derive(Debug, Clone, PartialEq)]
pub struct StatusResult {
    pub status_code: StatusCode,
    pub diagnostic_info: DiagnosticInfo,
}

impl MessageInfo for StatusResult {
    fn object_id(&self) -> ObjectId {
        ObjectId::StatusResult_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<StatusResult> for StatusResult {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.status_code.byte_len();
        size += self.diagnostic_info.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.status_code.encode(stream)?;
        size += self.diagnostic_info.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let status_code = StatusCode::decode(stream, decoding_options)?;
        let diagnostic_info = DiagnosticInfo::decode(stream, decoding_options)?;
        Ok(StatusResult {
            status_code,
            diagnostic_info,
        })
    }
}
