// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{basic_types::*, encoding::*, node_ids::ObjectId, service_types::impls::MessageInfo};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ServerDiagnosticsSummaryDataType {
    pub server_view_count: u32,
    pub current_session_count: u32,
    pub cumulated_session_count: u32,
    pub security_rejected_session_count: u32,
    pub rejected_session_count: u32,
    pub session_timeout_count: u32,
    pub session_abort_count: u32,
    pub current_subscription_count: u32,
    pub cumulated_subscription_count: u32,
    pub publishing_interval_count: u32,
    pub security_rejected_requests_count: u32,
    pub rejected_requests_count: u32,
}

impl MessageInfo for ServerDiagnosticsSummaryDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::ServerDiagnosticsSummaryDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ServerDiagnosticsSummaryDataType> for ServerDiagnosticsSummaryDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.server_view_count.byte_len();
        size += self.current_session_count.byte_len();
        size += self.cumulated_session_count.byte_len();
        size += self.security_rejected_session_count.byte_len();
        size += self.rejected_session_count.byte_len();
        size += self.session_timeout_count.byte_len();
        size += self.session_abort_count.byte_len();
        size += self.current_subscription_count.byte_len();
        size += self.cumulated_subscription_count.byte_len();
        size += self.publishing_interval_count.byte_len();
        size += self.security_rejected_requests_count.byte_len();
        size += self.rejected_requests_count.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.server_view_count.encode(stream)?;
        size += self.current_session_count.encode(stream)?;
        size += self.cumulated_session_count.encode(stream)?;
        size += self.security_rejected_session_count.encode(stream)?;
        size += self.rejected_session_count.encode(stream)?;
        size += self.session_timeout_count.encode(stream)?;
        size += self.session_abort_count.encode(stream)?;
        size += self.current_subscription_count.encode(stream)?;
        size += self.cumulated_subscription_count.encode(stream)?;
        size += self.publishing_interval_count.encode(stream)?;
        size += self.security_rejected_requests_count.encode(stream)?;
        size += self.rejected_requests_count.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let server_view_count = u32::decode(stream, decoding_options)?;
        let current_session_count = u32::decode(stream, decoding_options)?;
        let cumulated_session_count = u32::decode(stream, decoding_options)?;
        let security_rejected_session_count = u32::decode(stream, decoding_options)?;
        let rejected_session_count = u32::decode(stream, decoding_options)?;
        let session_timeout_count = u32::decode(stream, decoding_options)?;
        let session_abort_count = u32::decode(stream, decoding_options)?;
        let current_subscription_count = u32::decode(stream, decoding_options)?;
        let cumulated_subscription_count = u32::decode(stream, decoding_options)?;
        let publishing_interval_count = u32::decode(stream, decoding_options)?;
        let security_rejected_requests_count = u32::decode(stream, decoding_options)?;
        let rejected_requests_count = u32::decode(stream, decoding_options)?;
        Ok(ServerDiagnosticsSummaryDataType {
            server_view_count,
            current_session_count,
            cumulated_session_count,
            security_rejected_session_count,
            rejected_session_count,
            session_timeout_count,
            session_abort_count,
            current_subscription_count,
            cumulated_subscription_count,
            publishing_interval_count,
            security_rejected_requests_count,
            rejected_requests_count,
        })
    }
}
