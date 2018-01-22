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

#[derive(Debug, Clone, PartialEq)]
pub struct ContentFilterElement {
    pub filter_operator: FilterOperator,
    pub filter_operands: Option<Vec<ExtensionObject>>,
}

impl MessageInfo for ContentFilterElement {
    fn object_id(&self) -> ObjectId {
        ObjectId::ContentFilterElement_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ContentFilterElement> for ContentFilterElement {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.filter_operator.byte_len();
        size += byte_len_array(&self.filter_operands);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.filter_operator.encode(stream)?;
        size += write_array(stream, &self.filter_operands)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let filter_operator = FilterOperator::decode(stream)?;
        let filter_operands: Option<Vec<ExtensionObject>> = read_array(stream)?;
        Ok(ContentFilterElement {
            filter_operator,
            filter_operands,
        })
    }
}