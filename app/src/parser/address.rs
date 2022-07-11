/*******************************************************************************
*   (c) 2021 Zondax GmbH
*
*  Licensed under the Apache License, Version 2.0 (the "License");
*  you may not use this file except in compliance with the License.
*  You may obtain a copy of the License at
*
*      http://www.apache.org/licenses/LICENSE-2.0
*
*  Unless required by applicable law or agreed to in writing, software
*  distributed under the License is distributed on an "AS IS" BASIS,
*  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*  See the License for the specific language governing permissions and
*  limitations under the License.
********************************************************************************/
use core::{mem::MaybeUninit, ptr::addr_of_mut};
use nom::bytes::complete::take;

use crate::{
    handlers::handle_ui_message,
    parser::{DisplayableItem, FromBytes, ParserError},
};

use crate::sys::{bech32, hash::Ripemd160};
use zemu_sys::ViewError;

pub const ADDRESS_LEN: usize = Ripemd160::DIGEST_LEN;

// ripemd160(sha256(compress(secp256k1.publicKey()))
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct Address<'b>(&'b [u8; ADDRESS_LEN]);

impl<'b> FromBytes<'b> for Address<'b> {
    #[inline(never)]
    fn from_bytes_into(
        input: &'b [u8],
        out: &mut MaybeUninit<Self>,
    ) -> Result<&'b [u8], nom::Err<ParserError>> {
        let (rem, addr) = take(ADDRESS_LEN)(input)?;
        let addr = arrayref::array_ref!(addr, 0, ADDRESS_LEN);

        //good ptr and no uninit reads
        let out = out.as_mut_ptr();
        unsafe {
            addr_of_mut!((*out).0).write(addr);
        }

        Ok(rem)
    }
}

impl<'a> DisplayableItem for Address<'a> {
    fn num_items(&self) -> usize {
        1
    }

    #[inline(never)]
    fn render_item(
        &self,
        item_n: u8,
        title: &mut [u8],
        message: &mut [u8],
        page: u8,
    ) -> Result<u8, ViewError> {
        use bolos::{pic_str, PIC};

        if item_n != 0 {
            return Err(ViewError::NoData);
        }

        const MAX_SIZE: usize = bech32::estimate_size(0, Ripemd160::DIGEST_LEN);

        let mut addr = [0; MAX_SIZE];

        // TODO see https://github.com/Zondax/ledger-avalanche/issues/10
        let len = bech32::encode("", &self.0, &mut addr[..], bech32::Variant::Bech32).map_err(|_| ViewError::Unknown)?;

        let title_content = pic_str!(b"Address");
        title[..title_content.len()].copy_from_slice(title_content);

        handle_ui_message(&addr[..len], message, page)
    }
}
