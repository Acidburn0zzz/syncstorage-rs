// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, you can obtain one at https://mozilla.org/MPL/2.0/.

//! Parameter types for database methods.

macro_rules! data {
    ($name:ident {$($property:ident: $type:ty,)*}) => {
        #[derive(Debug)]
        pub struct $name {
            $(pub $property: $type,)*
        }
    }
}

macro_rules! uid_data {
    ($($name:ident,)+) => ($(
        data! {
            $name {
                user_id: String,
            }
        }
    )+)
}

macro_rules! collection_data {
    ($($name:ident {$($property:ident: $type:ty,)*},)+) => ($(
        data! {
            $name {
                user_id: String,
                collection: String,
                $($property: $type,)*
            }
        }
    )+)
}

macro_rules! bso_data {
    ($($name:ident {$($property:ident: $type:ty,)*},)+) => ($(
        data! {
            $name {
                user_id: String,
                collection: String,
                bso_id: String,
                $($property: $type,)*
            }
        }
    )+)
}

uid_data! {
    GetCollections,
    GetCollectionCounts,
    GetCollectionUsage,
    GetQuota,
    DeleteAll,
}

collection_data! {
    DeleteCollection {
        bso_ids: Vec<String>,
    },
    GetCollection {},
    PostCollection {
        bsos: Vec<PostCollectionBso>,
    },
}

bso_data! {
    DeleteBso {},
    GetBso {},
    PutBso {
        sortindex: Option<i32>,
        payload: Option<String>,
        ttl: Option<u32>,
    },
}

#[derive(Debug)]
pub struct PostCollectionBso {
    pub bso_id: String,
    pub sortindex: Option<i32>,
    pub payload: Option<String>,
    pub ttl: Option<u32>,
}
