// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{basic_types::*, encoding::*};

#[derive(Debug, Clone, PartialEq)]
pub struct ThreeDCartesianCoordinates {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl BinaryEncoder<ThreeDCartesianCoordinates> for ThreeDCartesianCoordinates {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.x.byte_len();
        size += self.y.byte_len();
        size += self.z.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.x.encode(stream)?;
        size += self.y.encode(stream)?;
        size += self.z.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let x = f64::decode(stream, decoding_options)?;
        let y = f64::decode(stream, decoding_options)?;
        let z = f64::decode(stream, decoding_options)?;
        Ok(ThreeDCartesianCoordinates { x, y, z })
    }
}
