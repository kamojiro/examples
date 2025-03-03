// This file is generated automatically using wasmcloud/weld-codegen 0.4.3

#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Borrow, borrow::Cow, io::Write, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    cbor::*,
    common::{
        deserialize, message_format, serialize, Context, Message, MessageDispatch, MessageFormat,
        SendOpts, Transport,
    },
    error::{RpcError, RpcResult},
    Timestamp,
};

#[allow(dead_code)]
pub const SMITHY_VERSION: &str = "1.0";

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AddPetRequest {
    #[serde(rename = "ownerId")]
    #[serde(default)]
    pub owner_id: u64,
    pub pet: Pet,
}

// Encode AddPetRequest as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_add_pet_request<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &AddPetRequest,
) -> RpcResult<()> {
    e.map(2)?;
    e.str("ownerId")?;
    e.u64(val.owner_id)?;
    e.str("pet")?;
    encode_pet(e, &val.pet)?;
    Ok(())
}

// Decode AddPetRequest from cbor input stream
#[doc(hidden)]
pub fn decode_add_pet_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<AddPetRequest, RpcError> {
    let __result = {
        let mut owner_id: Option<u64> = None;
        let mut pet: Option<Pet> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct AddPetRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => owner_id = Some(d.u64()?),
                    1 => {
                        pet = Some(decode_pet(d).map_err(|e| {
                            format!("decoding 'org.wasmcloud.examples.petclinic#Pet': {}", e)
                        })?)
                    }
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "ownerId" => owner_id = Some(d.u64()?),
                    "pet" => {
                        pet = Some(decode_pet(d).map_err(|e| {
                            format!("decoding 'org.wasmcloud.examples.petclinic#Pet': {}", e)
                        })?)
                    }
                    _ => d.skip()?,
                }
            }
        }
        AddPetRequest {
            owner_id: if let Some(__x) = owner_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field AddPetRequest.owner_id (#0)".to_string(),
                ));
            },

            pet: if let Some(__x) = pet {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field AddPetRequest.pet (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CreateOwnerReply {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub success: bool,
}

// Encode CreateOwnerReply as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_create_owner_reply<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &CreateOwnerReply,
) -> RpcResult<()> {
    e.map(2)?;
    e.str("id")?;
    e.u64(val.id)?;
    e.str("success")?;
    e.bool(val.success)?;
    Ok(())
}

// Decode CreateOwnerReply from cbor input stream
#[doc(hidden)]
pub fn decode_create_owner_reply(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<CreateOwnerReply, RpcError> {
    let __result = {
        let mut id: Option<u64> = None;
        let mut success: Option<bool> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct CreateOwnerReply, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => id = Some(d.u64()?),
                    1 => success = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "id" => id = Some(d.u64()?),
                    "success" => success = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        }
        CreateOwnerReply {
            id: if let Some(__x) = id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field CreateOwnerReply.id (#0)".to_string(),
                ));
            },

            success: if let Some(__x) = success {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field CreateOwnerReply.success (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Date {
    #[serde(default)]
    pub day: u8,
    #[serde(default)]
    pub month: u8,
    #[serde(default)]
    pub year: u16,
}

// Encode Date as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_date<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Date,
) -> RpcResult<()> {
    e.map(3)?;
    e.str("day")?;
    e.u8(val.day)?;
    e.str("month")?;
    e.u8(val.month)?;
    e.str("year")?;
    e.u16(val.year)?;
    Ok(())
}

// Decode Date from cbor input stream
#[doc(hidden)]
pub fn decode_date(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Date, RpcError> {
    let __result = {
        let mut day: Option<u8> = None;
        let mut month: Option<u8> = None;
        let mut year: Option<u16> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Date, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => day = Some(d.u8()?),
                    1 => month = Some(d.u8()?),
                    2 => year = Some(d.u16()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "day" => day = Some(d.u8()?),
                    "month" => month = Some(d.u8()?),
                    "year" => year = Some(d.u16()?),
                    _ => d.skip()?,
                }
            }
        }
        Date {
            day: if let Some(__x) = day {
                __x
            } else {
                return Err(RpcError::Deser("missing field Date.day (#0)".to_string()));
            },

            month: if let Some(__x) = month {
                __x
            } else {
                return Err(RpcError::Deser("missing field Date.month (#1)".to_string()));
            },

            year: if let Some(__x) = year {
                __x
            } else {
                return Err(RpcError::Deser("missing field Date.year (#2)".to_string()));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct FindOwnerReply {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
}

// Encode FindOwnerReply as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_find_owner_reply<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &FindOwnerReply,
) -> RpcResult<()> {
    e.map(1)?;
    if let Some(val) = val.owner.as_ref() {
        e.str("owner")?;
        encode_owner(e, val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode FindOwnerReply from cbor input stream
#[doc(hidden)]
pub fn decode_find_owner_reply(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<FindOwnerReply, RpcError> {
    let __result = {
        let mut owner: Option<Option<Owner>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct FindOwnerReply, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        owner = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(decode_owner(d).map_err(|e| {
                                format!("decoding 'org.wasmcloud.examples.petclinic#Owner': {}", e)
                            })?))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "owner" => {
                        owner = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(decode_owner(d).map_err(|e| {
                                format!("decoding 'org.wasmcloud.examples.petclinic#Owner': {}", e)
                            })?))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        FindOwnerReply {
            owner: owner.unwrap(),
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct FindPetReply {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pet: Option<Pet>,
}

// Encode FindPetReply as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_find_pet_reply<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &FindPetReply,
) -> RpcResult<()> {
    e.map(1)?;
    if let Some(val) = val.pet.as_ref() {
        e.str("pet")?;
        encode_pet(e, val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode FindPetReply from cbor input stream
#[doc(hidden)]
pub fn decode_find_pet_reply(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<FindPetReply, RpcError> {
    let __result = {
        let mut pet: Option<Option<Pet>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct FindPetReply, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        pet = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(decode_pet(d).map_err(|e| {
                                format!("decoding 'org.wasmcloud.examples.petclinic#Pet': {}", e)
                            })?))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "pet" => {
                        pet = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(decode_pet(d).map_err(|e| {
                                format!("decoding 'org.wasmcloud.examples.petclinic#Pet': {}", e)
                            })?))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        FindPetReply { pet: pet.unwrap() }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetAssetResponse {
    /// The raw asset as bytes
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub asset: Vec<u8>,
    /// Optionally hint to the caller what the content type is. Should be a valid MIME type
    #[serde(rename = "contentType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// True if the asset was found, false if request was successful, but asset was not found
    #[serde(default)]
    pub found: bool,
}

// Encode GetAssetResponse as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_get_asset_response<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &GetAssetResponse,
) -> RpcResult<()> {
    e.map(3)?;
    e.str("asset")?;
    e.bytes(&val.asset)?;
    if let Some(val) = val.content_type.as_ref() {
        e.str("contentType")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    e.str("found")?;
    e.bool(val.found)?;
    Ok(())
}

// Decode GetAssetResponse from cbor input stream
#[doc(hidden)]
pub fn decode_get_asset_response(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<GetAssetResponse, RpcError> {
    let __result = {
        let mut asset: Option<Vec<u8>> = None;
        let mut content_type: Option<Option<String>> = Some(None);
        let mut found: Option<bool> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct GetAssetResponse, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => asset = Some(d.bytes()?.to_vec()),
                    1 => {
                        content_type = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    2 => found = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "asset" => asset = Some(d.bytes()?.to_vec()),
                    "contentType" => {
                        content_type = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "found" => found = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        }
        GetAssetResponse {
            asset: if let Some(__x) = asset {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field GetAssetResponse.asset (#0)".to_string(),
                ));
            },
            content_type: content_type.unwrap(),

            found: if let Some(__x) = found {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field GetAssetResponse.found (#2)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// Request to list visits
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ListVisitsRequest {
    #[serde(rename = "ownerId")]
    #[serde(default)]
    pub owner_id: u64,
    #[serde(rename = "petIds")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pet_ids: Option<PetIdList>,
}

// Encode ListVisitsRequest as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_list_visits_request<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ListVisitsRequest,
) -> RpcResult<()> {
    e.map(2)?;
    e.str("ownerId")?;
    e.u64(val.owner_id)?;
    if let Some(val) = val.pet_ids.as_ref() {
        e.str("petIds")?;
        encode_pet_id_list(e, val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode ListVisitsRequest from cbor input stream
#[doc(hidden)]
pub fn decode_list_visits_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ListVisitsRequest, RpcError> {
    let __result = {
        let mut owner_id: Option<u64> = None;
        let mut pet_ids: Option<Option<PetIdList>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ListVisitsRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => owner_id = Some(d.u64()?),
                    1 => {
                        pet_ids = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(decode_pet_id_list(d).map_err(|e| {
                                format!(
                                    "decoding 'org.wasmcloud.examples.petclinic#PetIdList': {}",
                                    e
                                )
                            })?))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "ownerId" => owner_id = Some(d.u64()?),
                    "petIds" => {
                        pet_ids = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(decode_pet_id_list(d).map_err(|e| {
                                format!(
                                    "decoding 'org.wasmcloud.examples.petclinic#PetIdList': {}",
                                    e
                                )
                            })?))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        ListVisitsRequest {
            owner_id: if let Some(__x) = owner_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ListVisitsRequest.owner_id (#0)".to_string(),
                ));
            },
            pet_ids: pet_ids.unwrap(),
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Owner {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default)]
    pub email: String,
    #[serde(rename = "firstName")]
    #[serde(default)]
    pub first_name: String,
    #[serde(default)]
    pub id: u64,
    #[serde(rename = "lastName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
}

// Encode Owner as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_owner<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Owner,
) -> RpcResult<()> {
    e.map(7)?;
    if let Some(val) = val.address.as_ref() {
        e.str("address")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.city.as_ref() {
        e.str("city")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    e.str("email")?;
    e.str(&val.email)?;
    e.str("firstName")?;
    e.str(&val.first_name)?;
    e.str("id")?;
    e.u64(val.id)?;
    if let Some(val) = val.last_name.as_ref() {
        e.str("lastName")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.telephone.as_ref() {
        e.str("telephone")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode Owner from cbor input stream
#[doc(hidden)]
pub fn decode_owner(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Owner, RpcError> {
    let __result = {
        let mut address: Option<Option<String>> = Some(None);
        let mut city: Option<Option<String>> = Some(None);
        let mut email: Option<String> = None;
        let mut first_name: Option<String> = None;
        let mut id: Option<u64> = None;
        let mut last_name: Option<Option<String>> = Some(None);
        let mut telephone: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Owner, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        address = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    1 => {
                        city = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    2 => email = Some(d.str()?.to_string()),
                    3 => first_name = Some(d.str()?.to_string()),
                    4 => id = Some(d.u64()?),
                    5 => {
                        last_name = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    6 => {
                        telephone = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "address" => {
                        address = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "city" => {
                        city = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "email" => email = Some(d.str()?.to_string()),
                    "firstName" => first_name = Some(d.str()?.to_string()),
                    "id" => id = Some(d.u64()?),
                    "lastName" => {
                        last_name = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "telephone" => {
                        telephone = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        Owner {
            address: address.unwrap(),
            city: city.unwrap(),

            email: if let Some(__x) = email {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Owner.email (#2)".to_string(),
                ));
            },

            first_name: if let Some(__x) = first_name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Owner.first_name (#3)".to_string(),
                ));
            },

            id: if let Some(__x) = id {
                __x
            } else {
                return Err(RpcError::Deser("missing field Owner.id (#4)".to_string()));
            },
            last_name: last_name.unwrap(),
            telephone: telephone.unwrap(),
        }
    };
    Ok(__result)
}
pub type OwnersList = Vec<Owner>;

// Encode OwnersList as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_owners_list<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &OwnersList,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_owner(e, item)?;
    }
    Ok(())
}

// Decode OwnersList from cbor input stream
#[doc(hidden)]
pub fn decode_owners_list(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<OwnersList, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<Owner> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(decode_owner(d).map_err(|e| {
                    format!("decoding 'org.wasmcloud.examples.petclinic#Owner': {}", e)
                })?)
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<Owner> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(decode_owner(d).map_err(|e| {
                        format!("decoding 'org.wasmcloud.examples.petclinic#Owner': {}", e)
                    })?),
                }
            }
            arr
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Pet {
    pub birthdate: Date,
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "petType")]
    #[serde(default)]
    pub pet_type: u64,
}

// Encode Pet as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_pet<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Pet,
) -> RpcResult<()> {
    e.map(4)?;
    e.str("birthdate")?;
    encode_date(e, &val.birthdate)?;
    e.str("id")?;
    e.u64(val.id)?;
    e.str("name")?;
    e.str(&val.name)?;
    e.str("petType")?;
    e.u64(val.pet_type)?;
    Ok(())
}

// Decode Pet from cbor input stream
#[doc(hidden)]
pub fn decode_pet(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Pet, RpcError> {
    let __result = {
        let mut birthdate: Option<Date> = None;
        let mut id: Option<u64> = None;
        let mut name: Option<String> = None;
        let mut pet_type: Option<u64> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Pet, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        birthdate = Some(decode_date(d).map_err(|e| {
                            format!("decoding 'org.wasmcloud.examples.petclinic#Date': {}", e)
                        })?)
                    }
                    1 => id = Some(d.u64()?),
                    2 => name = Some(d.str()?.to_string()),
                    3 => pet_type = Some(d.u64()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "birthdate" => {
                        birthdate = Some(decode_date(d).map_err(|e| {
                            format!("decoding 'org.wasmcloud.examples.petclinic#Date': {}", e)
                        })?)
                    }
                    "id" => id = Some(d.u64()?),
                    "name" => name = Some(d.str()?.to_string()),
                    "petType" => pet_type = Some(d.u64()?),
                    _ => d.skip()?,
                }
            }
        }
        Pet {
            birthdate: if let Some(__x) = birthdate {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Pet.birthdate (#0)".to_string(),
                ));
            },

            id: if let Some(__x) = id {
                __x
            } else {
                return Err(RpcError::Deser("missing field Pet.id (#1)".to_string()));
            },

            name: if let Some(__x) = name {
                __x
            } else {
                return Err(RpcError::Deser("missing field Pet.name (#2)".to_string()));
            },

            pet_type: if let Some(__x) = pet_type {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Pet.pet_type (#3)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
pub type PetIdList = Vec<u64>;

// Encode PetIdList as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_pet_id_list<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &PetIdList,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        e.u64(*item)?;
    }
    Ok(())
}

// Decode PetIdList from cbor input stream
#[doc(hidden)]
pub fn decode_pet_id_list(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<PetIdList, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<u64> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(d.u64()?)
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<u64> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(d.u64()?),
                }
            }
            arr
        }
    };
    Ok(__result)
}
pub type PetList = Vec<Pet>;

// Encode PetList as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_pet_list<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &PetList,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_pet(e, item)?;
    }
    Ok(())
}

// Decode PetList from cbor input stream
#[doc(hidden)]
pub fn decode_pet_list(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<PetList, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<Pet> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(decode_pet(d).map_err(|e| {
                    format!("decoding 'org.wasmcloud.examples.petclinic#Pet': {}", e)
                })?)
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<Pet> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(decode_pet(d).map_err(|e| {
                        format!("decoding 'org.wasmcloud.examples.petclinic#Pet': {}", e)
                    })?),
                }
            }
            arr
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PetType {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub name: String,
}

// Encode PetType as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_pet_type<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &PetType,
) -> RpcResult<()> {
    e.map(2)?;
    e.str("id")?;
    e.u64(val.id)?;
    e.str("name")?;
    e.str(&val.name)?;
    Ok(())
}

// Decode PetType from cbor input stream
#[doc(hidden)]
pub fn decode_pet_type(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<PetType, RpcError> {
    let __result = {
        let mut id: Option<u64> = None;
        let mut name: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct PetType, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => id = Some(d.u64()?),
                    1 => name = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "id" => id = Some(d.u64()?),
                    "name" => name = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        PetType {
            id: if let Some(__x) = id {
                __x
            } else {
                return Err(RpcError::Deser("missing field PetType.id (#0)".to_string()));
            },

            name: if let Some(__x) = name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field PetType.name (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
pub type PetTypeList = Vec<PetType>;

// Encode PetTypeList as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_pet_type_list<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &PetTypeList,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_pet_type(e, item)?;
    }
    Ok(())
}

// Decode PetTypeList from cbor input stream
#[doc(hidden)]
pub fn decode_pet_type_list(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<PetTypeList, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<PetType> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(decode_pet_type(d).map_err(|e| {
                    format!("decoding 'org.wasmcloud.examples.petclinic#PetType': {}", e)
                })?)
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<PetType> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(decode_pet_type(d).map_err(|e| {
                        format!("decoding 'org.wasmcloud.examples.petclinic#PetType': {}", e)
                    })?),
                }
            }
            arr
        }
    };
    Ok(__result)
}
/// Request to record a visit
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RecordVisitRequest {
    #[serde(rename = "ownerId")]
    #[serde(default)]
    pub owner_id: u64,
    pub visit: Visit,
}

// Encode RecordVisitRequest as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_record_visit_request<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &RecordVisitRequest,
) -> RpcResult<()> {
    e.map(2)?;
    e.str("ownerId")?;
    e.u64(val.owner_id)?;
    e.str("visit")?;
    encode_visit(e, &val.visit)?;
    Ok(())
}

// Decode RecordVisitRequest from cbor input stream
#[doc(hidden)]
pub fn decode_record_visit_request(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<RecordVisitRequest, RpcError> {
    let __result = {
        let mut owner_id: Option<u64> = None;
        let mut visit: Option<Visit> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct RecordVisitRequest, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => owner_id = Some(d.u64()?),
                    1 => {
                        visit = Some(decode_visit(d).map_err(|e| {
                            format!("decoding 'org.wasmcloud.examples.petclinic#Visit': {}", e)
                        })?)
                    }
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "ownerId" => owner_id = Some(d.u64()?),
                    "visit" => {
                        visit = Some(decode_visit(d).map_err(|e| {
                            format!("decoding 'org.wasmcloud.examples.petclinic#Visit': {}", e)
                        })?)
                    }
                    _ => d.skip()?,
                }
            }
        }
        RecordVisitRequest {
            owner_id: if let Some(__x) = owner_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field RecordVisitRequest.owner_id (#0)".to_string(),
                ));
            },

            visit: if let Some(__x) = visit {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field RecordVisitRequest.visit (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
pub type SpecialtyList = Vec<String>;

// Encode SpecialtyList as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_specialty_list<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &SpecialtyList,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        e.str(item)?;
    }
    Ok(())
}

// Decode SpecialtyList from cbor input stream
#[doc(hidden)]
pub fn decode_specialty_list(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<SpecialtyList, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<String> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(d.str()?.to_string())
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<String> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(d.str()?.to_string()),
                }
            }
            arr
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Time {
    #[serde(default)]
    pub hour: u8,
    #[serde(default)]
    pub minute: u8,
}

// Encode Time as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_time<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Time,
) -> RpcResult<()> {
    e.map(2)?;
    e.str("hour")?;
    e.u8(val.hour)?;
    e.str("minute")?;
    e.u8(val.minute)?;
    Ok(())
}

// Decode Time from cbor input stream
#[doc(hidden)]
pub fn decode_time(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Time, RpcError> {
    let __result = {
        let mut hour: Option<u8> = None;
        let mut minute: Option<u8> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Time, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => hour = Some(d.u8()?),
                    1 => minute = Some(d.u8()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "hour" => hour = Some(d.u8()?),
                    "minute" => minute = Some(d.u8()?),
                    _ => d.skip()?,
                }
            }
        }
        Time {
            hour: if let Some(__x) = hour {
                __x
            } else {
                return Err(RpcError::Deser("missing field Time.hour (#0)".to_string()));
            },

            minute: if let Some(__x) = minute {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Time.minute (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct UpdateOwnerReply {
    #[serde(default)]
    pub success: bool,
}

// Encode UpdateOwnerReply as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_update_owner_reply<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &UpdateOwnerReply,
) -> RpcResult<()> {
    e.map(1)?;
    e.str("success")?;
    e.bool(val.success)?;
    Ok(())
}

// Decode UpdateOwnerReply from cbor input stream
#[doc(hidden)]
pub fn decode_update_owner_reply(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<UpdateOwnerReply, RpcError> {
    let __result = {
        let mut success: Option<bool> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct UpdateOwnerReply, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => success = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "success" => success = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        }
        UpdateOwnerReply {
            success: if let Some(__x) = success {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field UpdateOwnerReply.success (#0)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Vet {
    #[serde(rename = "firstName")]
    #[serde(default)]
    pub first_name: String,
    #[serde(default)]
    pub id: u64,
    #[serde(rename = "lastName")]
    #[serde(default)]
    pub last_name: String,
    pub specialties: SpecialtyList,
}

// Encode Vet as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_vet<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Vet,
) -> RpcResult<()> {
    e.map(4)?;
    e.str("firstName")?;
    e.str(&val.first_name)?;
    e.str("id")?;
    e.u64(val.id)?;
    e.str("lastName")?;
    e.str(&val.last_name)?;
    e.str("specialties")?;
    encode_specialty_list(e, &val.specialties)?;
    Ok(())
}

// Decode Vet from cbor input stream
#[doc(hidden)]
pub fn decode_vet(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Vet, RpcError> {
    let __result = {
        let mut first_name: Option<String> = None;
        let mut id: Option<u64> = None;
        let mut last_name: Option<String> = None;
        let mut specialties: Option<SpecialtyList> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Vet, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => first_name = Some(d.str()?.to_string()),
                    1 => id = Some(d.u64()?),
                    2 => last_name = Some(d.str()?.to_string()),
                    3 => {
                        specialties = Some(decode_specialty_list(d).map_err(|e| {
                            format!(
                                "decoding 'org.wasmcloud.examples.petclinic#SpecialtyList': {}",
                                e
                            )
                        })?)
                    }
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "firstName" => first_name = Some(d.str()?.to_string()),
                    "id" => id = Some(d.u64()?),
                    "lastName" => last_name = Some(d.str()?.to_string()),
                    "specialties" => {
                        specialties = Some(decode_specialty_list(d).map_err(|e| {
                            format!(
                                "decoding 'org.wasmcloud.examples.petclinic#SpecialtyList': {}",
                                e
                            )
                        })?)
                    }
                    _ => d.skip()?,
                }
            }
        }
        Vet {
            first_name: if let Some(__x) = first_name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Vet.first_name (#0)".to_string(),
                ));
            },

            id: if let Some(__x) = id {
                __x
            } else {
                return Err(RpcError::Deser("missing field Vet.id (#1)".to_string()));
            },

            last_name: if let Some(__x) = last_name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Vet.last_name (#2)".to_string(),
                ));
            },

            specialties: if let Some(__x) = specialties {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Vet.specialties (#3)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
pub type VetList = Vec<Vet>;

// Encode VetList as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_vet_list<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &VetList,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_vet(e, item)?;
    }
    Ok(())
}

// Decode VetList from cbor input stream
#[doc(hidden)]
pub fn decode_vet_list(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<VetList, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<Vet> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(decode_vet(d).map_err(|e| {
                    format!("decoding 'org.wasmcloud.examples.petclinic#Vet': {}", e)
                })?)
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<Vet> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(decode_vet(d).map_err(|e| {
                        format!("decoding 'org.wasmcloud.examples.petclinic#Vet': {}", e)
                    })?),
                }
            }
            arr
        }
    };
    Ok(__result)
}
/// The core metadata for a veterinarian visit
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Visit {
    /// The date the visit occurred
    pub date: Date,
    /// Description of the visit
    #[serde(default)]
    pub description: String,
    /// The ID of the pet involved in the visit
    #[serde(rename = "petId")]
    #[serde(default)]
    pub pet_id: u64,
    /// The time the visit occurred
    pub time: Time,
    /// ID of the veterinarian who saw the given pet on this visit
    #[serde(rename = "vetId")]
    #[serde(default)]
    pub vet_id: u64,
}

// Encode Visit as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_visit<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Visit,
) -> RpcResult<()> {
    e.map(5)?;
    e.str("date")?;
    encode_date(e, &val.date)?;
    e.str("description")?;
    e.str(&val.description)?;
    e.str("petId")?;
    e.u64(val.pet_id)?;
    e.str("time")?;
    encode_time(e, &val.time)?;
    e.str("vetId")?;
    e.u64(val.vet_id)?;
    Ok(())
}

// Decode Visit from cbor input stream
#[doc(hidden)]
pub fn decode_visit(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Visit, RpcError> {
    let __result = {
        let mut date: Option<Date> = None;
        let mut description: Option<String> = None;
        let mut pet_id: Option<u64> = None;
        let mut time: Option<Time> = None;
        let mut vet_id: Option<u64> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Visit, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        date = Some(decode_date(d).map_err(|e| {
                            format!("decoding 'org.wasmcloud.examples.petclinic#Date': {}", e)
                        })?)
                    }
                    1 => description = Some(d.str()?.to_string()),
                    2 => pet_id = Some(d.u64()?),
                    3 => {
                        time = Some(decode_time(d).map_err(|e| {
                            format!("decoding 'org.wasmcloud.examples.petclinic#Time': {}", e)
                        })?)
                    }
                    4 => vet_id = Some(d.u64()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "date" => {
                        date = Some(decode_date(d).map_err(|e| {
                            format!("decoding 'org.wasmcloud.examples.petclinic#Date': {}", e)
                        })?)
                    }
                    "description" => description = Some(d.str()?.to_string()),
                    "petId" => pet_id = Some(d.u64()?),
                    "time" => {
                        time = Some(decode_time(d).map_err(|e| {
                            format!("decoding 'org.wasmcloud.examples.petclinic#Time': {}", e)
                        })?)
                    }
                    "vetId" => vet_id = Some(d.u64()?),
                    _ => d.skip()?,
                }
            }
        }
        Visit {
            date: if let Some(__x) = date {
                __x
            } else {
                return Err(RpcError::Deser("missing field Visit.date (#0)".to_string()));
            },

            description: if let Some(__x) = description {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Visit.description (#1)".to_string(),
                ));
            },

            pet_id: if let Some(__x) = pet_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Visit.pet_id (#2)".to_string(),
                ));
            },

            time: if let Some(__x) = time {
                __x
            } else {
                return Err(RpcError::Deser("missing field Visit.time (#3)".to_string()));
            },

            vet_id: if let Some(__x) = vet_id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Visit.vet_id (#4)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
pub type VisitList = Vec<Visit>;

// Encode VisitList as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_visit_list<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &VisitList,
) -> RpcResult<()> {
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_visit(e, item)?;
    }
    Ok(())
}

// Decode VisitList from cbor input stream
#[doc(hidden)]
pub fn decode_visit_list(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<VisitList, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<Visit> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(decode_visit(d).map_err(|e| {
                    format!("decoding 'org.wasmcloud.examples.petclinic#Visit': {}", e)
                })?)
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<Visit> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(decode_visit(d).map_err(|e| {
                        format!("decoding 'org.wasmcloud.examples.petclinic#Visit': {}", e)
                    })?),
                }
            }
            arr
        }
    };
    Ok(__result)
}
/// wasmbus.actorReceive
#[async_trait]
pub trait Customers {
    async fn create_owner(&self, ctx: &Context, arg: &Owner) -> RpcResult<CreateOwnerReply>;
    async fn find_owner(&self, ctx: &Context, arg: &u64) -> RpcResult<FindOwnerReply>;
    async fn list_owners(&self, ctx: &Context) -> RpcResult<OwnersList>;
    async fn update_owner(&self, ctx: &Context, arg: &Owner) -> RpcResult<UpdateOwnerReply>;
    async fn list_pet_types(&self, ctx: &Context) -> RpcResult<PetTypeList>;
    async fn add_pet(&self, ctx: &Context, arg: &AddPetRequest) -> RpcResult<bool>;
    async fn remove_pet(&self, ctx: &Context, arg: &u64) -> RpcResult<bool>;
    async fn update_pet(&self, ctx: &Context, arg: &Pet) -> RpcResult<bool>;
    async fn list_pets(&self, ctx: &Context, arg: &u64) -> RpcResult<PetList>;
    async fn find_pet(&self, ctx: &Context, arg: &u64) -> RpcResult<FindPetReply>;
}

/// CustomersReceiver receives messages defined in the Customers service trait
#[doc(hidden)]
#[async_trait]
pub trait CustomersReceiver: MessageDispatch + Customers {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "CreateOwner" => {
                let value: Owner = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'Owner': {}", e)))?;
                let resp = Customers::create_owner(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "Customers.CreateOwner",
                    arg: Cow::Owned(buf),
                })
            }
            "FindOwner" => {
                let value: u64 = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'U64': {}", e)))?;
                let resp = Customers::find_owner(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "Customers.FindOwner",
                    arg: Cow::Owned(buf),
                })
            }
            "ListOwners" => {
                let resp = Customers::list_owners(self, ctx).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "Customers.ListOwners",
                    arg: Cow::Owned(buf),
                })
            }
            "UpdateOwner" => {
                let value: Owner = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'Owner': {}", e)))?;
                let resp = Customers::update_owner(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "Customers.UpdateOwner",
                    arg: Cow::Owned(buf),
                })
            }
            "ListPetTypes" => {
                let resp = Customers::list_pet_types(self, ctx).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "Customers.ListPetTypes",
                    arg: Cow::Owned(buf),
                })
            }
            "AddPet" => {
                let value: AddPetRequest = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'AddPetRequest': {}", e)))?;
                let resp = Customers::add_pet(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "Customers.AddPet",
                    arg: Cow::Owned(buf),
                })
            }
            "RemovePet" => {
                let value: u64 = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'U64': {}", e)))?;
                let resp = Customers::remove_pet(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "Customers.RemovePet",
                    arg: Cow::Owned(buf),
                })
            }
            "UpdatePet" => {
                let value: Pet = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'Pet': {}", e)))?;
                let resp = Customers::update_pet(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "Customers.UpdatePet",
                    arg: Cow::Owned(buf),
                })
            }
            "ListPets" => {
                let value: u64 = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'U64': {}", e)))?;
                let resp = Customers::list_pets(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "Customers.ListPets",
                    arg: Cow::Owned(buf),
                })
            }
            "FindPet" => {
                let value: u64 = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'U64': {}", e)))?;
                let resp = Customers::find_pet(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "Customers.FindPet",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Customers::{}",
                message.method
            ))),
        }
    }
}

/// CustomersSender sends messages to a Customers service
/// client for sending Customers messages
#[derive(Debug)]
pub struct CustomersSender<T: Transport> {
    transport: T,
}

impl<T: Transport> CustomersSender<T> {
    /// Constructs a CustomersSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> CustomersSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl CustomersSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Customers for CustomersSender<T> {
    #[allow(unused)]
    async fn create_owner(&self, ctx: &Context, arg: &Owner) -> RpcResult<CreateOwnerReply> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.CreateOwner",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: CreateOwnerReply = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': CreateOwnerReply", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn find_owner(&self, ctx: &Context, arg: &u64) -> RpcResult<FindOwnerReply> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.FindOwner",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: FindOwnerReply = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': FindOwnerReply", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn list_owners(&self, ctx: &Context) -> RpcResult<OwnersList> {
        let buf = *b"";
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.ListOwners",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: OwnersList = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': OwnersList", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn update_owner(&self, ctx: &Context, arg: &Owner) -> RpcResult<UpdateOwnerReply> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.UpdateOwner",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: UpdateOwnerReply = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': UpdateOwnerReply", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn list_pet_types(&self, ctx: &Context) -> RpcResult<PetTypeList> {
        let buf = *b"";
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.ListPetTypes",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: PetTypeList = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': PetTypeList", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn add_pet(&self, ctx: &Context, arg: &AddPetRequest) -> RpcResult<bool> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.AddPet",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: bool = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': Boolean", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn remove_pet(&self, ctx: &Context, arg: &u64) -> RpcResult<bool> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.RemovePet",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: bool = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': Boolean", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn update_pet(&self, ctx: &Context, arg: &Pet) -> RpcResult<bool> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.UpdatePet",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: bool = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': Boolean", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn list_pets(&self, ctx: &Context, arg: &u64) -> RpcResult<PetList> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.ListPets",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: PetList = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': PetList", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn find_pet(&self, ctx: &Context, arg: &u64) -> RpcResult<FindPetReply> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.FindPet",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: FindPetReply = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': FindPetReply", e)))?;
        Ok(value)
    }
}

/// Description of Petclinic service
/// wasmbus.actorReceive
#[async_trait]
pub trait Petclinic {
    /// Converts the input string to a result
    async fn convert<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<String>;
}

/// PetclinicReceiver receives messages defined in the Petclinic service trait
/// Description of Petclinic service
#[doc(hidden)]
#[async_trait]
pub trait PetclinicReceiver: MessageDispatch + Petclinic {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "Convert" => {
                let value: String = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'String': {}", e)))?;
                let resp = Petclinic::convert(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "Petclinic.Convert",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Petclinic::{}",
                message.method
            ))),
        }
    }
}

/// PetclinicSender sends messages to a Petclinic service
/// Description of Petclinic service
/// client for sending Petclinic messages
#[derive(Debug)]
pub struct PetclinicSender<T: Transport> {
    transport: T,
}

impl<T: Transport> PetclinicSender<T> {
    /// Constructs a PetclinicSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> PetclinicSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl PetclinicSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Petclinic for PetclinicSender<T> {
    #[allow(unused)]
    /// Converts the input string to a result
    async fn convert<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<String> {
        let buf = wasmbus_rpc::common::serialize(&arg.to_string())?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Petclinic.Convert",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: String = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': String", e)))?;
        Ok(value)
    }
}

/// wasmbus.actorReceive
#[async_trait]
pub trait Ui {
    /// Gets the asset with the given path. The input string should be the path part of a URL with the
    /// leading `/`
    async fn get_asset<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<GetAssetResponse>;
}

/// UiReceiver receives messages defined in the Ui service trait
#[doc(hidden)]
#[async_trait]
pub trait UiReceiver: MessageDispatch + Ui {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "GetAsset" => {
                let value: String = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'String': {}", e)))?;
                let resp = Ui::get_asset(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "Ui.GetAsset",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Ui::{}",
                message.method
            ))),
        }
    }
}

/// UiSender sends messages to a Ui service
/// client for sending Ui messages
#[derive(Debug)]
pub struct UiSender<T: Transport> {
    transport: T,
}

impl<T: Transport> UiSender<T> {
    /// Constructs a UiSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> UiSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl UiSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Ui for UiSender<T> {
    #[allow(unused)]
    /// Gets the asset with the given path. The input string should be the path part of a URL with the
    /// leading `/`
    async fn get_asset<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<GetAssetResponse> {
        let buf = wasmbus_rpc::common::serialize(&arg.to_string())?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Ui.GetAsset",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: GetAssetResponse = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': GetAssetResponse", e)))?;
        Ok(value)
    }
}

/// wasmbus.actorReceive
#[async_trait]
pub trait Vets {
    async fn list_vets(&self, ctx: &Context) -> RpcResult<VetList>;
}

/// VetsReceiver receives messages defined in the Vets service trait
#[doc(hidden)]
#[async_trait]
pub trait VetsReceiver: MessageDispatch + Vets {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "ListVets" => {
                let resp = Vets::list_vets(self, ctx).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "Vets.ListVets",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Vets::{}",
                message.method
            ))),
        }
    }
}

/// VetsSender sends messages to a Vets service
/// client for sending Vets messages
#[derive(Debug)]
pub struct VetsSender<T: Transport> {
    transport: T,
}

impl<T: Transport> VetsSender<T> {
    /// Constructs a VetsSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> VetsSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl VetsSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Vets for VetsSender<T> {
    #[allow(unused)]
    async fn list_vets(&self, ctx: &Context) -> RpcResult<VetList> {
        let buf = *b"";
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Vets.ListVets",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: VetList = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': VetList", e)))?;
        Ok(value)
    }
}

/// wasmbus.actorReceive
#[async_trait]
pub trait Visits {
    /// Retrieve a list of visits for a given owner and an optional
    /// list of pet IDs
    async fn list_visits(&self, ctx: &Context, arg: &ListVisitsRequest) -> RpcResult<VisitList>;
    /// Records a new visit
    async fn record_visit(&self, ctx: &Context, arg: &RecordVisitRequest) -> RpcResult<bool>;
}

/// VisitsReceiver receives messages defined in the Visits service trait
#[doc(hidden)]
#[async_trait]
pub trait VisitsReceiver: MessageDispatch + Visits {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "ListVisits" => {
                let value: ListVisitsRequest = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'ListVisitsRequest': {}", e)))?;
                let resp = Visits::list_visits(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "Visits.ListVisits",
                    arg: Cow::Owned(buf),
                })
            }
            "RecordVisit" => {
                let value: RecordVisitRequest = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'RecordVisitRequest': {}", e)))?;
                let resp = Visits::record_visit(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;
                Ok(Message {
                    method: "Visits.RecordVisit",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Visits::{}",
                message.method
            ))),
        }
    }
}

/// VisitsSender sends messages to a Visits service
/// client for sending Visits messages
#[derive(Debug)]
pub struct VisitsSender<T: Transport> {
    transport: T,
}

impl<T: Transport> VisitsSender<T> {
    /// Constructs a VisitsSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> VisitsSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl VisitsSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Visits for VisitsSender<T> {
    #[allow(unused)]
    /// Retrieve a list of visits for a given owner and an optional
    /// list of pet IDs
    async fn list_visits(&self, ctx: &Context, arg: &ListVisitsRequest) -> RpcResult<VisitList> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Visits.ListVisits",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: VisitList = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': VisitList", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// Records a new visit
    async fn record_visit(&self, ctx: &Context, arg: &RecordVisitRequest) -> RpcResult<bool> {
        let buf = wasmbus_rpc::common::serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Visits.RecordVisit",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: bool = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': Boolean", e)))?;
        Ok(value)
    }
}
