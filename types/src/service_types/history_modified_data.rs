// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{basic_types::*, data_value::DataValue, encoding::*, service_types::ModificationInfo};

#[derive(Debug, Clone, PartialEq)]
pub struct HistoryModifiedData {
    pub data_values: Option<Vec<DataValue>>,
    pub modification_infos: Option<Vec<ModificationInfo>>,
}

impl BinaryEncoder<HistoryModifiedData> for HistoryModifiedData {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += byte_len_array(&self.data_values);
        size += byte_len_array(&self.modification_infos);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += write_array(stream, &self.data_values)?;
        size += write_array(stream, &self.modification_infos)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let data_values: Option<Vec<DataValue>> = read_array(stream, decoding_options)?;
        let modification_infos: Option<Vec<ModificationInfo>> =
            read_array(stream, decoding_options)?;
        Ok(HistoryModifiedData {
            data_values,
            modification_infos,
        })
    }
}
