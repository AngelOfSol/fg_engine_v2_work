#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use std::sync::Arc;
use async_trait::async_trait;
pub enum JoinError {
    AlreadyConnected,
    Denied,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for JoinError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&JoinError::AlreadyConnected,) => {
                let mut debug_trait_builder = f.debug_tuple("AlreadyConnected");
                debug_trait_builder.finish()
            }
            (&JoinError::Denied,) => {
                let mut debug_trait_builder = f.debug_tuple("Denied");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for JoinError {
    #[inline]
    fn clone(&self) -> JoinError {
        {
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for JoinError {}
impl ::core::marker::StructuralPartialEq for JoinError {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for JoinError {
    #[inline]
    fn eq(&self, other: &JoinError) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    _ => true,
                }
            } else {
                false
            }
        }
    }
}
impl ::core::marker::StructuralEq for JoinError {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for JoinError {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialOrd for JoinError {
    #[inline]
    fn partial_cmp(&self, other: &JoinError) -> ::core::option::Option<::core::cmp::Ordering> {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    _ => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                }
            } else {
                __self_vi.partial_cmp(&__arg_1_vi)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Ord for JoinError {
    #[inline]
    fn cmp(&self, other: &JoinError) -> ::core::cmp::Ordering {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    _ => ::core::cmp::Ordering::Equal,
                }
            } else {
                __self_vi.cmp(&__arg_1_vi)
            }
        }
    }
}
pub enum HostError {
    AlreadyHosting,
}
pub trait NetworkingSubsytem {
    type LobbyId;
    type GameId;
    type PlayerId;
    #[must_use]
    fn request_host<'life0, 'async_trait>(
        &'life0 mut self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Self::LobbyId, HostError>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait;
    #[must_use]
    fn request_join<'life0, 'async_trait>(
        &'life0 mut self,
        lobby: Self::LobbyId,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Self::LobbyId, JoinError>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait;
    #[must_use]
    fn request_join2<'async_trait>(
        self: Arc<Self>,
        lobby: Self::LobbyId,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Self::LobbyId, JoinError>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        Self: 'async_trait;
}
