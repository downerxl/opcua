// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{basic_types::*, date_time::DateTime, encoding::*};

#[derive(Debug, Clone, PartialEq)]
pub struct ReadAnnotationDataDetails {
    pub req_times: Option<Vec<DateTime>>,
}

impl BinaryEncoder<ReadAnnotationDataDetails> for ReadAnnotationDataDetails {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += byte_len_array(&self.req_times);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += write_array(stream, &self.req_times)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let req_times: Option<Vec<DateTime>> = read_array(stream, decoding_options)?;
        Ok(ReadAnnotationDataDetails { req_times })
    }
}
