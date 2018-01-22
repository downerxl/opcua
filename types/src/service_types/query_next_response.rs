// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
#[allow(unused_imports)]
use string::*;
#[allow(unused_imports)]
use byte_string::ByteString;
#[allow(unused_imports)]
use data_types::*;
#[allow(unused_imports)]
use data_value::*;
#[allow(unused_imports)]
use date_time::*;
#[allow(unused_imports)]
use node_id::*;
#[allow(unused_imports)]
use service_types::enums::*;
#[allow(unused_imports)]
use variant::*;
#[allow(unused_imports)]
use service_types::impls::*;
#[allow(unused_imports)]
use node_ids::ObjectId;
#[allow(unused_imports)]
use status_codes::StatusCode;
use service_types::QueryDataSet;

#[derive(Debug, Clone, PartialEq)]
pub struct QueryNextResponse {
    pub response_header: ResponseHeader,
    pub query_data_sets: Option<Vec<QueryDataSet>>,
    pub revised_continuation_point: ByteString,
}

impl MessageInfo for QueryNextResponse {
    fn object_id(&self) -> ObjectId {
        ObjectId::QueryNextResponse_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<QueryNextResponse> for QueryNextResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size += byte_len_array(&self.query_data_sets);
        size += self.revised_continuation_point.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        size += write_array(stream, &self.query_data_sets)?;
        size += self.revised_continuation_point.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let response_header = ResponseHeader::decode(stream)?;
        let query_data_sets: Option<Vec<QueryDataSet>> = read_array(stream)?;
        let revised_continuation_point = ByteString::decode(stream)?;
        Ok(QueryNextResponse {
            response_header,
            query_data_sets,
            revised_continuation_point,
        })
    }
}