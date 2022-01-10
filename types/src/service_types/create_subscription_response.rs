// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    basic_types::*, encoding::*, node_ids::ObjectId, response_header::ResponseHeader,
    service_types::impls::MessageInfo,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CreateSubscriptionResponse {
    pub response_header: ResponseHeader,
    pub subscription_id: u32,
    pub revised_publishing_interval: f64,
    pub revised_lifetime_count: u32,
    pub revised_max_keep_alive_count: u32,
}

impl MessageInfo for CreateSubscriptionResponse {
    fn object_id(&self) -> ObjectId {
        ObjectId::CreateSubscriptionResponse_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<CreateSubscriptionResponse> for CreateSubscriptionResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size += self.subscription_id.byte_len();
        size += self.revised_publishing_interval.byte_len();
        size += self.revised_lifetime_count.byte_len();
        size += self.revised_max_keep_alive_count.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        size += self.subscription_id.encode(stream)?;
        size += self.revised_publishing_interval.encode(stream)?;
        size += self.revised_lifetime_count.encode(stream)?;
        size += self.revised_max_keep_alive_count.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let response_header = ResponseHeader::decode(stream, decoding_options)?;
        let subscription_id = u32::decode(stream, decoding_options)?;
        let revised_publishing_interval = f64::decode(stream, decoding_options)?;
        let revised_lifetime_count = u32::decode(stream, decoding_options)?;
        let revised_max_keep_alive_count = u32::decode(stream, decoding_options)?;
        Ok(CreateSubscriptionResponse {
            response_header,
            subscription_id,
            revised_publishing_interval,
            revised_lifetime_count,
            revised_max_keep_alive_count,
        })
    }
}
