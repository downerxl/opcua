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
    service_types::impls::MessageInfo, service_types::Argument, service_types::StatusResult,
    string::UAString,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ProgramDiagnosticDataType {
    pub create_session_id: NodeId,
    pub create_client_name: UAString,
    pub invocation_creation_time: DateTime,
    pub last_transition_time: DateTime,
    pub last_method_call: UAString,
    pub last_method_session_id: NodeId,
    pub last_method_input_arguments: Option<Vec<Argument>>,
    pub last_method_output_arguments: Option<Vec<Argument>>,
    pub last_method_call_time: DateTime,
    pub last_method_return_status: StatusResult,
}

impl MessageInfo for ProgramDiagnosticDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::ProgramDiagnosticDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ProgramDiagnosticDataType> for ProgramDiagnosticDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.create_session_id.byte_len();
        size += self.create_client_name.byte_len();
        size += self.invocation_creation_time.byte_len();
        size += self.last_transition_time.byte_len();
        size += self.last_method_call.byte_len();
        size += self.last_method_session_id.byte_len();
        size += byte_len_array(&self.last_method_input_arguments);
        size += byte_len_array(&self.last_method_output_arguments);
        size += self.last_method_call_time.byte_len();
        size += self.last_method_return_status.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.create_session_id.encode(stream)?;
        size += self.create_client_name.encode(stream)?;
        size += self.invocation_creation_time.encode(stream)?;
        size += self.last_transition_time.encode(stream)?;
        size += self.last_method_call.encode(stream)?;
        size += self.last_method_session_id.encode(stream)?;
        size += write_array(stream, &self.last_method_input_arguments)?;
        size += write_array(stream, &self.last_method_output_arguments)?;
        size += self.last_method_call_time.encode(stream)?;
        size += self.last_method_return_status.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let create_session_id = NodeId::decode(stream, decoding_options)?;
        let create_client_name = UAString::decode(stream, decoding_options)?;
        let invocation_creation_time = DateTime::decode(stream, decoding_options)?;
        let last_transition_time = DateTime::decode(stream, decoding_options)?;
        let last_method_call = UAString::decode(stream, decoding_options)?;
        let last_method_session_id = NodeId::decode(stream, decoding_options)?;
        let last_method_input_arguments: Option<Vec<Argument>> =
            read_array(stream, decoding_options)?;
        let last_method_output_arguments: Option<Vec<Argument>> =
            read_array(stream, decoding_options)?;
        let last_method_call_time = DateTime::decode(stream, decoding_options)?;
        let last_method_return_status = StatusResult::decode(stream, decoding_options)?;
        Ok(ProgramDiagnosticDataType {
            create_session_id,
            create_client_name,
            invocation_creation_time,
            last_transition_time,
            last_method_call,
            last_method_session_id,
            last_method_input_arguments,
            last_method_output_arguments,
            last_method_call_time,
            last_method_return_status,
        })
    }
}
