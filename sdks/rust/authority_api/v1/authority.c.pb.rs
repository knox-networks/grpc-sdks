extern crate protobuf_cpp as __pb;
extern crate std as __std;
#[path="../../business_api/v1/business.c.pb.rs"]
#[allow(non_snake_case)]
pub mod business__api_sv1_sbusiness;
#[path="../../common/v1/amqp.c.pb.rs"]
#[allow(non_snake_case)]
pub mod common_sv1_samqp;
#[path="../../common/v1/common.c.pb.rs"]
#[allow(non_snake_case)]
pub mod common_sv1_scommon;
#[path="../../common/v1/packet.c.pb.rs"]
#[allow(non_snake_case)]
pub mod common_sv1_spacket;
#[path="../../gateway/v1/gateway.c.pb.rs"]
#[allow(non_snake_case)]
pub mod gateway_sv1_sgateway;
#[path="../../google/api/annotations.c.pb.rs"]
#[allow(non_snake_case)]
pub mod google_sapi_sannotations;
#[path="../../google/api/http.c.pb.rs"]
#[allow(non_snake_case)]
pub mod google_sapi_shttp;
#[path="../../google/protobuf/struct.c.pb.rs"]
#[allow(non_snake_case)]
pub mod google_sprotobuf_sstruct;
#[path="../../google/protobuf/timestamp.c.pb.rs"]
#[allow(non_snake_case)]
pub mod google_sprotobuf_stimestamp;
#[path="../../issuer_api/v1/issuer.c.pb.rs"]
#[allow(non_snake_case)]
pub mod issuer__api_sv1_sissuer;
#[path="../../registry_api/v1/registry.c.pb.rs"]
#[allow(non_snake_case)]
pub mod registry__api_sv1_sregistry;
#[path="../../transaction_manager_api/v1/transaction_manager.c.pb.rs"]
#[allow(non_snake_case)]
pub mod transaction__manager__api_sv1_stransaction__manager;
#[path="../../treasury/v1/treasury.c.pb.rs"]
#[allow(non_snake_case)]
pub mod treasury_sv1_streasury;
#[path="../../user_api/v1/user.c.pb.rs"]
#[allow(non_snake_case)]
pub mod user__api_sv1_suser;
#[path="../../vc_api/v1/vc.c.pb.rs"]
#[allow(non_snake_case)]
pub mod vc__api_sv1_svc;
#[path="../../wallet_api/v1/wallet.c.pb.rs"]
#[allow(non_snake_case)]
pub mod wallet__api_sv1_swallet;
#[path="../../webhook_api/v1/webhook.c.pb.rs"]
#[allow(non_snake_case)]
pub mod webhook__api_sv1_swebhook;
#[allow(unused_imports)]
pub use crate::business__api_sv1_sbusiness::*;
#[allow(unused_imports)]
pub use crate::common_sv1_samqp::*;
#[allow(unused_imports)]
pub use crate::common_sv1_scommon::*;
#[allow(unused_imports)]
pub use crate::common_sv1_spacket::*;
#[allow(unused_imports)]
pub use crate::gateway_sv1_sgateway::*;
#[allow(unused_imports)]
pub use crate::google_sapi_sannotations::*;
#[allow(unused_imports)]
pub use crate::google_sapi_shttp::*;
#[allow(unused_imports)]
pub use crate::google_sprotobuf_sstruct::*;
#[allow(unused_imports)]
pub use crate::google_sprotobuf_stimestamp::*;
#[allow(unused_imports)]
pub use crate::issuer__api_sv1_sissuer::*;
#[allow(unused_imports)]
pub use crate::registry__api_sv1_sregistry::*;
#[allow(unused_imports)]
pub use crate::transaction__manager__api_sv1_stransaction__manager::*;
#[allow(unused_imports)]
pub use crate::treasury_sv1_streasury::*;
#[allow(unused_imports)]
pub use crate::user__api_sv1_suser::*;
#[allow(unused_imports)]
pub use crate::vc__api_sv1_svc::*;
#[allow(unused_imports)]
pub use crate::wallet__api_sv1_swallet::*;
#[allow(unused_imports)]
pub use crate::webhook__api_sv1_swebhook::*;
#[allow(non_camel_case_types)]
pub struct SetIssuerLimitRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for SetIssuerLimitRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for SetIssuerLimitRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `SetIssuerLimitRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `SetIssuerLimitRequestMut`.
unsafe impl Sync for SetIssuerLimitRequest {}

// SAFETY:
// - `SetIssuerLimitRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SetIssuerLimitRequest {}

impl ::__pb::Proxied for SetIssuerLimitRequest {
  type View<'msg> = SetIssuerLimitRequestView<'msg>;
  type Mut<'msg> = SetIssuerLimitRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SetIssuerLimitRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for SetIssuerLimitRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> SetIssuerLimitRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_serialize(self.raw_msg()) }
  }

  // limit: optional message common.Amount
  pub fn limit(self) -> crate::AmountView<'msg> {
                  let submsg = unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_get_limit(self.raw_msg()) };
    crate::AmountView::new(::__pb::__internal::Private, submsg)
  }
  pub fn limit_opt(self) ->
  ::__pb::Optional<crate::AmountView<'msg>> {
    let view = self.limit();
    ::__pb::Optional::new(view, self.has_limit())
  }
  pub fn has_limit(self) -> bool {
    unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_has_limit(self.raw_msg()) }
  }

  // issuer_public_key: optional string
  pub fn issuer_public_key(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_get_issuer_public_key(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

}

// SAFETY:
// - `SetIssuerLimitRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for SetIssuerLimitRequestView<'_> {}

// SAFETY:
// - `SetIssuerLimitRequestView` is `Send` because while its alive a `SetIssuerLimitRequestMut` cannot.
// - `SetIssuerLimitRequestView` does not use thread-local data.
unsafe impl Send for SetIssuerLimitRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for SetIssuerLimitRequestView<'msg> {
  type Proxied = SetIssuerLimitRequest;

  fn as_view(&self) -> ::__pb::View<'msg, SetIssuerLimitRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, SetIssuerLimitRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for SetIssuerLimitRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    SetIssuerLimitRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    SetIssuerLimitRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for SetIssuerLimitRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for SetIssuerLimitRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, SetIssuerLimitRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, SetIssuerLimitRequest>;

  fn clear_present_field(present_mutator: Self::PresentMutData<'_>)
     -> Self::AbsentMutData<'_> {
     // SAFETY: The raw ptr msg_ref is valid
    unsafe {
      (present_mutator.optional_vtable().clearer)(present_mutator.msg_ref().msg());

     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       present_mutator.msg_ref(),
       present_mutator.optional_vtable())
    }
  }

  fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>)
     -> Self::PresentMutData<'_> {
   unsafe {
     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       absent_mutator.msg_ref(),
       absent_mutator.optional_vtable())
   }
  }
}

impl<'msg> ::__pb::SettableValue<SetIssuerLimitRequest> for SetIssuerLimitRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, SetIssuerLimitRequest>)
    where SetIssuerLimitRequest: 'dst {
    unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<SetIssuerLimitRequest> for SetIssuerLimitRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, SetIssuerLimitRequest>)
    where SetIssuerLimitRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for SetIssuerLimitRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: ::__pb::View<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_copy_from(
        __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
        v.raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_copy_from(new_elem, v.raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    mut dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for SetIssuerLimitRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetIssuerLimitRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> ::__pb::MapIter<'_, i32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i32, Self>) -> Option<(::__pb::View<'a, i32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetIssuerLimitRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for SetIssuerLimitRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetIssuerLimitRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> ::__pb::MapIter<'_, u32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u32, Self>) -> Option<(::__pb::View<'a, u32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetIssuerLimitRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for SetIssuerLimitRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetIssuerLimitRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> ::__pb::MapIter<'_, i64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i64, Self>) -> Option<(::__pb::View<'a, i64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetIssuerLimitRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for SetIssuerLimitRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetIssuerLimitRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> ::__pb::MapIter<'_, u64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u64, Self>) -> Option<(::__pb::View<'a, u64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetIssuerLimitRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for SetIssuerLimitRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetIssuerLimitRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> ::__pb::MapIter<'_, bool, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, bool, Self>) -> Option<(::__pb::View<'a, bool>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<bool, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetIssuerLimitRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for SetIssuerLimitRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetIssuerLimitRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> ::__pb::MapIter<'_, ::__pb::ProtoStr, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, ::__pb::ProtoStr, Self>) -> Option<(::__pb::View<'a, ::__pb::ProtoStr>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<::__pb::ProtoStr, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| SetIssuerLimitRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SetIssuerLimitRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for SetIssuerLimitRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> SetIssuerLimitRequestMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__runtime::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(
               ::__pb::__internal::Private, parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(_private, msg) }
  }

  #[deprecated = "This .or_default() is a no-op, usages can be safely removed"]
  pub fn or_default(self) -> Self { self }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg()
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    ::__pb::ViewProxy::as_view(self).serialize()
  }


  // limit: optional message common.Amount
  pub fn limit(&self) -> crate::AmountView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_get_limit(self.raw_msg()) };
    crate::AmountView::new(::__pb::__internal::Private, submsg)
  }
  pub fn limit_mut(&mut self) -> crate::AmountMut<'_> {
    self.limit_entry().or_default()
  }
  fn limit_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::Amount> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_get_limit,
                                __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_get_mut_limit,
                                __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_clear_limit);
    unsafe {
      let has = self.has_limit();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn limit_opt(&self) ->
  ::__pb::Optional<crate::AmountView<'_>> {
    let view = self.limit();
    ::__pb::Optional::new(view, self.has_limit())
  }
  pub fn set_limit(&mut self, val: impl ::__pb::SettableValue<crate::Amount>) {
    self.limit_entry().set(val);
  }
  pub fn has_limit(&self) -> bool {
    unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_has_limit(self.raw_msg()) }
  }
  pub fn clear_limit(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_clear_limit(self.raw_msg()) }
  }

  // issuer_public_key: optional string
  pub fn issuer_public_key(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_get_issuer_public_key(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_issuer_public_key(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.issuer_public_key_mut().set(val);
  }
  fn issuer_public_key_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          SetIssuerLimitRequest::__ISSUER_PUBLIC_KEY_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `SetIssuerLimitRequestMut` does not perform any shared mutation.
// - `SetIssuerLimitRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for SetIssuerLimitRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for SetIssuerLimitRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, SetIssuerLimitRequest> {
    SetIssuerLimitRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, SetIssuerLimitRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for SetIssuerLimitRequestMut<'msg> {
  type Proxied = SetIssuerLimitRequest;
  fn as_view(&self) -> ::__pb::View<'_, SetIssuerLimitRequest> {
    SetIssuerLimitRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, SetIssuerLimitRequest> where 'msg: 'shorter {
    SetIssuerLimitRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl SetIssuerLimitRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_new() } } }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(::__pb::__internal::Private, &mut self.inner)
  }


  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    self.as_view().serialize()
  }
  #[deprecated = "Prefer Msg::parse(), or use the new name 'clear_and_parse' to parse into a pre-existing message."]
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    self.clear_and_parse(data)
  }
  pub fn clear_and_parse(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::__pb::__runtime::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> SetIssuerLimitRequestView {
    SetIssuerLimitRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> SetIssuerLimitRequestMut {
    SetIssuerLimitRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // limit: optional message common.Amount
  pub fn limit(&self) -> crate::AmountView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_get_limit(self.raw_msg()) };
    crate::AmountView::new(::__pb::__internal::Private, submsg)
  }
  pub fn limit_mut(&mut self) -> crate::AmountMut<'_> {
    self.limit_entry().or_default()
  }
  fn limit_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::Amount> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_get_limit,
                                __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_get_mut_limit,
                                __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_clear_limit);
    unsafe {
      let has = self.has_limit();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn limit_opt(&self) ->
  ::__pb::Optional<crate::AmountView<'_>> {
    let view = self.limit();
    ::__pb::Optional::new(view, self.has_limit())
  }
  pub fn set_limit(&mut self, val: impl ::__pb::SettableValue<crate::Amount>) {
    self.limit_entry().set(val);
  }
  pub fn has_limit(&self) -> bool {
    unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_has_limit(self.raw_msg()) }
  }
  pub fn clear_limit(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_clear_limit(self.raw_msg()) }
  }

  // issuer_public_key: optional string
  pub fn issuer_public_key(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_get_issuer_public_key(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_issuer_public_key(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.issuer_public_key_mut().set(val);
  }
  const __ISSUER_PUBLIC_KEY_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_get_issuer_public_key,
      __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_set_issuer_public_key,
    );
  fn issuer_public_key_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          SetIssuerLimitRequest::__ISSUER_PUBLIC_KEY_VTABLE,
        )
      )
    }
  }

}  // impl SetIssuerLimitRequest

impl ::__std::ops::Drop for SetIssuerLimitRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_get_limit(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_get_mut_limit(raw_msg: ::__pb::__runtime::RawMessage)
     -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_clear_limit(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_has_limit(raw_msg: ::__pb::__runtime::RawMessage) -> bool;

  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_get_issuer_public_key(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_set_issuer_public_key(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for SetIssuerLimitRequest


impl SetIssuerLimitRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> SetIssuerLimitRequestMut<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut ::__pb::__runtime::RawMessage) -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_raw_msg(::__pb::__internal::Private, msg)
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.raw_msg()
  }
}

impl<'a> SetIssuerLimitRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct SetIssuerLimitResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for SetIssuerLimitResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for SetIssuerLimitResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `SetIssuerLimitResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `SetIssuerLimitResponseMut`.
unsafe impl Sync for SetIssuerLimitResponse {}

// SAFETY:
// - `SetIssuerLimitResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SetIssuerLimitResponse {}

impl ::__pb::Proxied for SetIssuerLimitResponse {
  type View<'msg> = SetIssuerLimitResponseView<'msg>;
  type Mut<'msg> = SetIssuerLimitResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SetIssuerLimitResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for SetIssuerLimitResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> SetIssuerLimitResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_serialize(self.raw_msg()) }
  }

  // limit: optional message common.Amount
  pub fn limit(self) -> crate::AmountView<'msg> {
                  let submsg = unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_get_limit(self.raw_msg()) };
    crate::AmountView::new(::__pb::__internal::Private, submsg)
  }
  pub fn limit_opt(self) ->
  ::__pb::Optional<crate::AmountView<'msg>> {
    let view = self.limit();
    ::__pb::Optional::new(view, self.has_limit())
  }
  pub fn has_limit(self) -> bool {
    unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_has_limit(self.raw_msg()) }
  }

  // signature: optional bytes
  pub fn signature(self) -> &'msg [u8] {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_get_signature(self.raw_msg()).as_ref() };
    view
  }

}

// SAFETY:
// - `SetIssuerLimitResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for SetIssuerLimitResponseView<'_> {}

// SAFETY:
// - `SetIssuerLimitResponseView` is `Send` because while its alive a `SetIssuerLimitResponseMut` cannot.
// - `SetIssuerLimitResponseView` does not use thread-local data.
unsafe impl Send for SetIssuerLimitResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for SetIssuerLimitResponseView<'msg> {
  type Proxied = SetIssuerLimitResponse;

  fn as_view(&self) -> ::__pb::View<'msg, SetIssuerLimitResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, SetIssuerLimitResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for SetIssuerLimitResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    SetIssuerLimitResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    SetIssuerLimitResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for SetIssuerLimitResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for SetIssuerLimitResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, SetIssuerLimitResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, SetIssuerLimitResponse>;

  fn clear_present_field(present_mutator: Self::PresentMutData<'_>)
     -> Self::AbsentMutData<'_> {
     // SAFETY: The raw ptr msg_ref is valid
    unsafe {
      (present_mutator.optional_vtable().clearer)(present_mutator.msg_ref().msg());

     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       present_mutator.msg_ref(),
       present_mutator.optional_vtable())
    }
  }

  fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>)
     -> Self::PresentMutData<'_> {
   unsafe {
     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       absent_mutator.msg_ref(),
       absent_mutator.optional_vtable())
   }
  }
}

impl<'msg> ::__pb::SettableValue<SetIssuerLimitResponse> for SetIssuerLimitResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, SetIssuerLimitResponse>)
    where SetIssuerLimitResponse: 'dst {
    unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<SetIssuerLimitResponse> for SetIssuerLimitResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, SetIssuerLimitResponse>)
    where SetIssuerLimitResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for SetIssuerLimitResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: ::__pb::View<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_copy_from(
        __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
        v.raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_copy_from(new_elem, v.raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    mut dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for SetIssuerLimitResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetIssuerLimitResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> ::__pb::MapIter<'_, i32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i32, Self>) -> Option<(::__pb::View<'a, i32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetIssuerLimitResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for SetIssuerLimitResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetIssuerLimitResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> ::__pb::MapIter<'_, u32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u32, Self>) -> Option<(::__pb::View<'a, u32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetIssuerLimitResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for SetIssuerLimitResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetIssuerLimitResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> ::__pb::MapIter<'_, i64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i64, Self>) -> Option<(::__pb::View<'a, i64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetIssuerLimitResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for SetIssuerLimitResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetIssuerLimitResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> ::__pb::MapIter<'_, u64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u64, Self>) -> Option<(::__pb::View<'a, u64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetIssuerLimitResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for SetIssuerLimitResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetIssuerLimitResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> ::__pb::MapIter<'_, bool, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, bool, Self>) -> Option<(::__pb::View<'a, bool>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<bool, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetIssuerLimitResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for SetIssuerLimitResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetIssuerLimitResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> ::__pb::MapIter<'_, ::__pb::ProtoStr, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, ::__pb::ProtoStr, Self>) -> Option<(::__pb::View<'a, ::__pb::ProtoStr>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<::__pb::ProtoStr, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| SetIssuerLimitResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SetIssuerLimitResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for SetIssuerLimitResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> SetIssuerLimitResponseMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__runtime::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(
               ::__pb::__internal::Private, parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(_private, msg) }
  }

  #[deprecated = "This .or_default() is a no-op, usages can be safely removed"]
  pub fn or_default(self) -> Self { self }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg()
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    ::__pb::ViewProxy::as_view(self).serialize()
  }


  // limit: optional message common.Amount
  pub fn limit(&self) -> crate::AmountView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_get_limit(self.raw_msg()) };
    crate::AmountView::new(::__pb::__internal::Private, submsg)
  }
  pub fn limit_mut(&mut self) -> crate::AmountMut<'_> {
    self.limit_entry().or_default()
  }
  fn limit_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::Amount> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_get_limit,
                                __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_get_mut_limit,
                                __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_clear_limit);
    unsafe {
      let has = self.has_limit();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn limit_opt(&self) ->
  ::__pb::Optional<crate::AmountView<'_>> {
    let view = self.limit();
    ::__pb::Optional::new(view, self.has_limit())
  }
  pub fn set_limit(&mut self, val: impl ::__pb::SettableValue<crate::Amount>) {
    self.limit_entry().set(val);
  }
  pub fn has_limit(&self) -> bool {
    unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_has_limit(self.raw_msg()) }
  }
  pub fn clear_limit(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_clear_limit(self.raw_msg()) }
  }

  // signature: optional bytes
  pub fn signature(&self) -> &'_ [u8] {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_get_signature(self.raw_msg()).as_ref() };
    view
  }
  pub fn set_signature(&mut self, val: impl ::__pb::SettableValue<[u8]>) {
    self.signature_mut().set(val);
  }
  fn signature_mut(&mut self) -> ::__pb::Mut<'_, [u8]> {
    unsafe {
      <::__pb::Mut<[u8]>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          SetIssuerLimitResponse::__SIGNATURE_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `SetIssuerLimitResponseMut` does not perform any shared mutation.
// - `SetIssuerLimitResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for SetIssuerLimitResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for SetIssuerLimitResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, SetIssuerLimitResponse> {
    SetIssuerLimitResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, SetIssuerLimitResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for SetIssuerLimitResponseMut<'msg> {
  type Proxied = SetIssuerLimitResponse;
  fn as_view(&self) -> ::__pb::View<'_, SetIssuerLimitResponse> {
    SetIssuerLimitResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, SetIssuerLimitResponse> where 'msg: 'shorter {
    SetIssuerLimitResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl SetIssuerLimitResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_new() } } }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(::__pb::__internal::Private, &mut self.inner)
  }


  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    self.as_view().serialize()
  }
  #[deprecated = "Prefer Msg::parse(), or use the new name 'clear_and_parse' to parse into a pre-existing message."]
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    self.clear_and_parse(data)
  }
  pub fn clear_and_parse(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::__pb::__runtime::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> SetIssuerLimitResponseView {
    SetIssuerLimitResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> SetIssuerLimitResponseMut {
    SetIssuerLimitResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // limit: optional message common.Amount
  pub fn limit(&self) -> crate::AmountView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_get_limit(self.raw_msg()) };
    crate::AmountView::new(::__pb::__internal::Private, submsg)
  }
  pub fn limit_mut(&mut self) -> crate::AmountMut<'_> {
    self.limit_entry().or_default()
  }
  fn limit_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::Amount> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_get_limit,
                                __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_get_mut_limit,
                                __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_clear_limit);
    unsafe {
      let has = self.has_limit();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn limit_opt(&self) ->
  ::__pb::Optional<crate::AmountView<'_>> {
    let view = self.limit();
    ::__pb::Optional::new(view, self.has_limit())
  }
  pub fn set_limit(&mut self, val: impl ::__pb::SettableValue<crate::Amount>) {
    self.limit_entry().set(val);
  }
  pub fn has_limit(&self) -> bool {
    unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_has_limit(self.raw_msg()) }
  }
  pub fn clear_limit(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_clear_limit(self.raw_msg()) }
  }

  // signature: optional bytes
  pub fn signature(&self) -> &'_ [u8] {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_get_signature(self.raw_msg()).as_ref() };
    view
  }
  pub fn set_signature(&mut self, val: impl ::__pb::SettableValue<[u8]>) {
    self.signature_mut().set(val);
  }
  const __SIGNATURE_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_get_signature,
      __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_set_signature,
    );
  fn signature_mut(&mut self) -> ::__pb::Mut<'_, [u8]> {
    unsafe {
      <::__pb::Mut<[u8]>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          SetIssuerLimitResponse::__SIGNATURE_VTABLE,
        )
      )
    }
  }

}  // impl SetIssuerLimitResponse

impl ::__std::ops::Drop for SetIssuerLimitResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_get_limit(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_get_mut_limit(raw_msg: ::__pb::__runtime::RawMessage)
     -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_clear_limit(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_has_limit(raw_msg: ::__pb::__runtime::RawMessage) -> bool;

  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_get_signature(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_set_signature(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for SetIssuerLimitResponse


impl SetIssuerLimitResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> SetIssuerLimitResponseMut<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut ::__pb::__runtime::RawMessage) -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_raw_msg(::__pb::__internal::Private, msg)
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.raw_msg()
  }
}

impl<'a> SetIssuerLimitResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct GetIssuerLimitsRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for GetIssuerLimitsRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for GetIssuerLimitsRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `GetIssuerLimitsRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetIssuerLimitsRequestMut`.
unsafe impl Sync for GetIssuerLimitsRequest {}

// SAFETY:
// - `GetIssuerLimitsRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetIssuerLimitsRequest {}

impl ::__pb::Proxied for GetIssuerLimitsRequest {
  type View<'msg> = GetIssuerLimitsRequestView<'msg>;
  type Mut<'msg> = GetIssuerLimitsRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetIssuerLimitsRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for GetIssuerLimitsRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> GetIssuerLimitsRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_serialize(self.raw_msg()) }
  }

  // issuer_public_key: optional string
  pub fn issuer_public_key(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_get_issuer_public_key(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

}

// SAFETY:
// - `GetIssuerLimitsRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetIssuerLimitsRequestView<'_> {}

// SAFETY:
// - `GetIssuerLimitsRequestView` is `Send` because while its alive a `GetIssuerLimitsRequestMut` cannot.
// - `GetIssuerLimitsRequestView` does not use thread-local data.
unsafe impl Send for GetIssuerLimitsRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for GetIssuerLimitsRequestView<'msg> {
  type Proxied = GetIssuerLimitsRequest;

  fn as_view(&self) -> ::__pb::View<'msg, GetIssuerLimitsRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, GetIssuerLimitsRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for GetIssuerLimitsRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    GetIssuerLimitsRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    GetIssuerLimitsRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for GetIssuerLimitsRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for GetIssuerLimitsRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, GetIssuerLimitsRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, GetIssuerLimitsRequest>;

  fn clear_present_field(present_mutator: Self::PresentMutData<'_>)
     -> Self::AbsentMutData<'_> {
     // SAFETY: The raw ptr msg_ref is valid
    unsafe {
      (present_mutator.optional_vtable().clearer)(present_mutator.msg_ref().msg());

     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       present_mutator.msg_ref(),
       present_mutator.optional_vtable())
    }
  }

  fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>)
     -> Self::PresentMutData<'_> {
   unsafe {
     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       absent_mutator.msg_ref(),
       absent_mutator.optional_vtable())
   }
  }
}

impl<'msg> ::__pb::SettableValue<GetIssuerLimitsRequest> for GetIssuerLimitsRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, GetIssuerLimitsRequest>)
    where GetIssuerLimitsRequest: 'dst {
    unsafe { __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<GetIssuerLimitsRequest> for GetIssuerLimitsRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, GetIssuerLimitsRequest>)
    where GetIssuerLimitsRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for GetIssuerLimitsRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: ::__pb::View<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_copy_from(
        __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
        v.raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_copy_from(new_elem, v.raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    mut dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for GetIssuerLimitsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetIssuerLimitsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> ::__pb::MapIter<'_, i32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i32, Self>) -> Option<(::__pb::View<'a, i32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetIssuerLimitsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for GetIssuerLimitsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetIssuerLimitsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> ::__pb::MapIter<'_, u32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u32, Self>) -> Option<(::__pb::View<'a, u32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetIssuerLimitsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for GetIssuerLimitsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetIssuerLimitsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> ::__pb::MapIter<'_, i64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i64, Self>) -> Option<(::__pb::View<'a, i64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetIssuerLimitsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for GetIssuerLimitsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetIssuerLimitsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> ::__pb::MapIter<'_, u64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u64, Self>) -> Option<(::__pb::View<'a, u64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetIssuerLimitsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for GetIssuerLimitsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetIssuerLimitsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> ::__pb::MapIter<'_, bool, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, bool, Self>) -> Option<(::__pb::View<'a, bool>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<bool, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetIssuerLimitsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for GetIssuerLimitsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetIssuerLimitsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> ::__pb::MapIter<'_, ::__pb::ProtoStr, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, ::__pb::ProtoStr, Self>) -> Option<(::__pb::View<'a, ::__pb::ProtoStr>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<::__pb::ProtoStr, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| GetIssuerLimitsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetIssuerLimitsRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for GetIssuerLimitsRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> GetIssuerLimitsRequestMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__runtime::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(
               ::__pb::__internal::Private, parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(_private, msg) }
  }

  #[deprecated = "This .or_default() is a no-op, usages can be safely removed"]
  pub fn or_default(self) -> Self { self }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg()
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    ::__pb::ViewProxy::as_view(self).serialize()
  }


  // issuer_public_key: optional string
  pub fn issuer_public_key(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_get_issuer_public_key(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_issuer_public_key(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.issuer_public_key_mut().set(val);
  }
  fn issuer_public_key_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          GetIssuerLimitsRequest::__ISSUER_PUBLIC_KEY_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `GetIssuerLimitsRequestMut` does not perform any shared mutation.
// - `GetIssuerLimitsRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetIssuerLimitsRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for GetIssuerLimitsRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, GetIssuerLimitsRequest> {
    GetIssuerLimitsRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, GetIssuerLimitsRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for GetIssuerLimitsRequestMut<'msg> {
  type Proxied = GetIssuerLimitsRequest;
  fn as_view(&self) -> ::__pb::View<'_, GetIssuerLimitsRequest> {
    GetIssuerLimitsRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, GetIssuerLimitsRequest> where 'msg: 'shorter {
    GetIssuerLimitsRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl GetIssuerLimitsRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_new() } } }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(::__pb::__internal::Private, &mut self.inner)
  }


  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    self.as_view().serialize()
  }
  #[deprecated = "Prefer Msg::parse(), or use the new name 'clear_and_parse' to parse into a pre-existing message."]
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    self.clear_and_parse(data)
  }
  pub fn clear_and_parse(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::__pb::__runtime::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> GetIssuerLimitsRequestView {
    GetIssuerLimitsRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetIssuerLimitsRequestMut {
    GetIssuerLimitsRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // issuer_public_key: optional string
  pub fn issuer_public_key(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_get_issuer_public_key(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_issuer_public_key(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.issuer_public_key_mut().set(val);
  }
  const __ISSUER_PUBLIC_KEY_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_get_issuer_public_key,
      __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_set_issuer_public_key,
    );
  fn issuer_public_key_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          GetIssuerLimitsRequest::__ISSUER_PUBLIC_KEY_VTABLE,
        )
      )
    }
  }

}  // impl GetIssuerLimitsRequest

impl ::__std::ops::Drop for GetIssuerLimitsRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_get_issuer_public_key(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_set_issuer_public_key(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for GetIssuerLimitsRequest


impl GetIssuerLimitsRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> GetIssuerLimitsRequestMut<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut ::__pb::__runtime::RawMessage) -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_raw_msg(::__pb::__internal::Private, msg)
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.raw_msg()
  }
}

impl<'a> GetIssuerLimitsRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct GetIssuerLimitsResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for GetIssuerLimitsResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for GetIssuerLimitsResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `GetIssuerLimitsResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GetIssuerLimitsResponseMut`.
unsafe impl Sync for GetIssuerLimitsResponse {}

// SAFETY:
// - `GetIssuerLimitsResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetIssuerLimitsResponse {}

impl ::__pb::Proxied for GetIssuerLimitsResponse {
  type View<'msg> = GetIssuerLimitsResponseView<'msg>;
  type Mut<'msg> = GetIssuerLimitsResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetIssuerLimitsResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for GetIssuerLimitsResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> GetIssuerLimitsResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_serialize(self.raw_msg()) }
  }

  // limit: repeated message common.Amount
  pub fn limit(self) -> ::__pb::RepeatedView<'msg, crate::Amount> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_get_limit(self.raw_msg()),
      )
    }
  }

  // issuer_public_key: optional string
  pub fn issuer_public_key(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_get_issuer_public_key(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

}

// SAFETY:
// - `GetIssuerLimitsResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetIssuerLimitsResponseView<'_> {}

// SAFETY:
// - `GetIssuerLimitsResponseView` is `Send` because while its alive a `GetIssuerLimitsResponseMut` cannot.
// - `GetIssuerLimitsResponseView` does not use thread-local data.
unsafe impl Send for GetIssuerLimitsResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for GetIssuerLimitsResponseView<'msg> {
  type Proxied = GetIssuerLimitsResponse;

  fn as_view(&self) -> ::__pb::View<'msg, GetIssuerLimitsResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, GetIssuerLimitsResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for GetIssuerLimitsResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    GetIssuerLimitsResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    GetIssuerLimitsResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for GetIssuerLimitsResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for GetIssuerLimitsResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, GetIssuerLimitsResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, GetIssuerLimitsResponse>;

  fn clear_present_field(present_mutator: Self::PresentMutData<'_>)
     -> Self::AbsentMutData<'_> {
     // SAFETY: The raw ptr msg_ref is valid
    unsafe {
      (present_mutator.optional_vtable().clearer)(present_mutator.msg_ref().msg());

     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       present_mutator.msg_ref(),
       present_mutator.optional_vtable())
    }
  }

  fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>)
     -> Self::PresentMutData<'_> {
   unsafe {
     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       absent_mutator.msg_ref(),
       absent_mutator.optional_vtable())
   }
  }
}

impl<'msg> ::__pb::SettableValue<GetIssuerLimitsResponse> for GetIssuerLimitsResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, GetIssuerLimitsResponse>)
    where GetIssuerLimitsResponse: 'dst {
    unsafe { __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<GetIssuerLimitsResponse> for GetIssuerLimitsResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, GetIssuerLimitsResponse>)
    where GetIssuerLimitsResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for GetIssuerLimitsResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: ::__pb::View<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_copy_from(
        __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
        v.raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_copy_from(new_elem, v.raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    mut dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for GetIssuerLimitsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetIssuerLimitsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> ::__pb::MapIter<'_, i32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i32, Self>) -> Option<(::__pb::View<'a, i32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetIssuerLimitsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for GetIssuerLimitsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetIssuerLimitsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> ::__pb::MapIter<'_, u32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u32, Self>) -> Option<(::__pb::View<'a, u32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetIssuerLimitsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for GetIssuerLimitsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetIssuerLimitsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> ::__pb::MapIter<'_, i64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i64, Self>) -> Option<(::__pb::View<'a, i64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetIssuerLimitsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for GetIssuerLimitsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetIssuerLimitsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> ::__pb::MapIter<'_, u64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u64, Self>) -> Option<(::__pb::View<'a, u64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetIssuerLimitsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for GetIssuerLimitsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetIssuerLimitsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> ::__pb::MapIter<'_, bool, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, bool, Self>) -> Option<(::__pb::View<'a, bool>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<bool, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetIssuerLimitsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for GetIssuerLimitsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetIssuerLimitsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> ::__pb::MapIter<'_, ::__pb::ProtoStr, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, ::__pb::ProtoStr, Self>) -> Option<(::__pb::View<'a, ::__pb::ProtoStr>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<::__pb::ProtoStr, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| GetIssuerLimitsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetIssuerLimitsResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for GetIssuerLimitsResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> GetIssuerLimitsResponseMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__runtime::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(
               ::__pb::__internal::Private, parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(_private, msg) }
  }

  #[deprecated = "This .or_default() is a no-op, usages can be safely removed"]
  pub fn or_default(self) -> Self { self }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg()
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    ::__pb::ViewProxy::as_view(self).serialize()
  }


  // limit: repeated message common.Amount
  pub fn limit(&self) -> ::__pb::RepeatedView<'_, crate::Amount> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_get_limit(self.raw_msg()),
      )
    }
  }
  pub fn limit_mut(&mut self) -> ::__pb::RepeatedMut<'_, crate::Amount> {
    unsafe {
      ::__pb::RepeatedMut::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__runtime::InnerRepeatedMut::new(
          ::__pb::__internal::Private,
          __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_get_mut_limit(self.raw_msg()),
        ),
      )
    }
  }

  // issuer_public_key: optional string
  pub fn issuer_public_key(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_get_issuer_public_key(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_issuer_public_key(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.issuer_public_key_mut().set(val);
  }
  fn issuer_public_key_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          GetIssuerLimitsResponse::__ISSUER_PUBLIC_KEY_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `GetIssuerLimitsResponseMut` does not perform any shared mutation.
// - `GetIssuerLimitsResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetIssuerLimitsResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for GetIssuerLimitsResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, GetIssuerLimitsResponse> {
    GetIssuerLimitsResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, GetIssuerLimitsResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for GetIssuerLimitsResponseMut<'msg> {
  type Proxied = GetIssuerLimitsResponse;
  fn as_view(&self) -> ::__pb::View<'_, GetIssuerLimitsResponse> {
    GetIssuerLimitsResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, GetIssuerLimitsResponse> where 'msg: 'shorter {
    GetIssuerLimitsResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl GetIssuerLimitsResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_new() } } }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(::__pb::__internal::Private, &mut self.inner)
  }


  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    self.as_view().serialize()
  }
  #[deprecated = "Prefer Msg::parse(), or use the new name 'clear_and_parse' to parse into a pre-existing message."]
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    self.clear_and_parse(data)
  }
  pub fn clear_and_parse(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::__pb::__runtime::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> GetIssuerLimitsResponseView {
    GetIssuerLimitsResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetIssuerLimitsResponseMut {
    GetIssuerLimitsResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // limit: repeated message common.Amount
  pub fn limit(&self) -> ::__pb::RepeatedView<'_, crate::Amount> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_get_limit(self.raw_msg()),
      )
    }
  }
  pub fn limit_mut(&mut self) -> ::__pb::RepeatedMut<'_, crate::Amount> {
    unsafe {
      ::__pb::RepeatedMut::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__runtime::InnerRepeatedMut::new(
          ::__pb::__internal::Private,
          __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_get_mut_limit(self.raw_msg()),
        ),
      )
    }
  }

  // issuer_public_key: optional string
  pub fn issuer_public_key(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_get_issuer_public_key(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_issuer_public_key(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.issuer_public_key_mut().set(val);
  }
  const __ISSUER_PUBLIC_KEY_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_get_issuer_public_key,
      __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_set_issuer_public_key,
    );
  fn issuer_public_key_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          GetIssuerLimitsResponse::__ISSUER_PUBLIC_KEY_VTABLE,
        )
      )
    }
  }

}  // impl GetIssuerLimitsResponse

impl ::__std::ops::Drop for GetIssuerLimitsResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_clear_limit(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_get_mut_limit(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawRepeatedField;
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_get_limit(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawRepeatedField;

  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_get_issuer_public_key(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_set_issuer_public_key(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for GetIssuerLimitsResponse


impl GetIssuerLimitsResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> GetIssuerLimitsResponseMut<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut ::__pb::__runtime::RawMessage) -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_raw_msg(::__pb::__internal::Private, msg)
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.raw_msg()
  }
}

impl<'a> GetIssuerLimitsResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct AuthorizeRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for AuthorizeRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for AuthorizeRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `AuthorizeRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `AuthorizeRequestMut`.
unsafe impl Sync for AuthorizeRequest {}

// SAFETY:
// - `AuthorizeRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for AuthorizeRequest {}

impl ::__pb::Proxied for AuthorizeRequest {
  type View<'msg> = AuthorizeRequestView<'msg>;
  type Mut<'msg> = AuthorizeRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AuthorizeRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for AuthorizeRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> AuthorizeRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__authority_api_v1_AuthorizeRequest_serialize(self.raw_msg()) }
  }

  // promissory_file: optional bytes
  pub fn promissory_file(self) -> &'msg [u8] {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_AuthorizeRequest_get_promissory_file(self.raw_msg()).as_ref() };
    view
  }

}

// SAFETY:
// - `AuthorizeRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for AuthorizeRequestView<'_> {}

// SAFETY:
// - `AuthorizeRequestView` is `Send` because while its alive a `AuthorizeRequestMut` cannot.
// - `AuthorizeRequestView` does not use thread-local data.
unsafe impl Send for AuthorizeRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for AuthorizeRequestView<'msg> {
  type Proxied = AuthorizeRequest;

  fn as_view(&self) -> ::__pb::View<'msg, AuthorizeRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, AuthorizeRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for AuthorizeRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    AuthorizeRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    AuthorizeRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for AuthorizeRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for AuthorizeRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, AuthorizeRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, AuthorizeRequest>;

  fn clear_present_field(present_mutator: Self::PresentMutData<'_>)
     -> Self::AbsentMutData<'_> {
     // SAFETY: The raw ptr msg_ref is valid
    unsafe {
      (present_mutator.optional_vtable().clearer)(present_mutator.msg_ref().msg());

     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       present_mutator.msg_ref(),
       present_mutator.optional_vtable())
    }
  }

  fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>)
     -> Self::PresentMutData<'_> {
   unsafe {
     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       absent_mutator.msg_ref(),
       absent_mutator.optional_vtable())
   }
  }
}

impl<'msg> ::__pb::SettableValue<AuthorizeRequest> for AuthorizeRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, AuthorizeRequest>)
    where AuthorizeRequest: 'dst {
    unsafe { __rust_proto_thunk__authority_api_v1_AuthorizeRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<AuthorizeRequest> for AuthorizeRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, AuthorizeRequest>)
    where AuthorizeRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for AuthorizeRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_AuthorizeRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: ::__pb::View<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_AuthorizeRequest_copy_from(
        __rust_proto_thunk__authority_api_v1_AuthorizeRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
        v.raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { __rust_proto_thunk__authority_api_v1_AuthorizeRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_AuthorizeRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__authority_api_v1_AuthorizeRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__authority_api_v1_AuthorizeRequest_copy_from(new_elem, v.raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    mut dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_AuthorizeRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for AuthorizeRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthorizeRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> ::__pb::MapIter<'_, i32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i32, Self>) -> Option<(::__pb::View<'a, i32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AuthorizeRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for AuthorizeRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthorizeRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> ::__pb::MapIter<'_, u32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u32, Self>) -> Option<(::__pb::View<'a, u32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AuthorizeRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for AuthorizeRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthorizeRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> ::__pb::MapIter<'_, i64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i64, Self>) -> Option<(::__pb::View<'a, i64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AuthorizeRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for AuthorizeRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthorizeRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> ::__pb::MapIter<'_, u64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u64, Self>) -> Option<(::__pb::View<'a, u64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AuthorizeRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for AuthorizeRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthorizeRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> ::__pb::MapIter<'_, bool, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, bool, Self>) -> Option<(::__pb::View<'a, bool>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<bool, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AuthorizeRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for AuthorizeRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthorizeRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> ::__pb::MapIter<'_, ::__pb::ProtoStr, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, ::__pb::ProtoStr, Self>) -> Option<(::__pb::View<'a, ::__pb::ProtoStr>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<::__pb::ProtoStr, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| AuthorizeRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AuthorizeRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for AuthorizeRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> AuthorizeRequestMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__runtime::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(
               ::__pb::__internal::Private, parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(_private, msg) }
  }

  #[deprecated = "This .or_default() is a no-op, usages can be safely removed"]
  pub fn or_default(self) -> Self { self }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg()
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    ::__pb::ViewProxy::as_view(self).serialize()
  }


  // promissory_file: optional bytes
  pub fn promissory_file(&self) -> &'_ [u8] {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_AuthorizeRequest_get_promissory_file(self.raw_msg()).as_ref() };
    view
  }
  pub fn set_promissory_file(&mut self, val: impl ::__pb::SettableValue<[u8]>) {
    self.promissory_file_mut().set(val);
  }
  fn promissory_file_mut(&mut self) -> ::__pb::Mut<'_, [u8]> {
    unsafe {
      <::__pb::Mut<[u8]>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          AuthorizeRequest::__PROMISSORY_FILE_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `AuthorizeRequestMut` does not perform any shared mutation.
// - `AuthorizeRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for AuthorizeRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for AuthorizeRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, AuthorizeRequest> {
    AuthorizeRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, AuthorizeRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for AuthorizeRequestMut<'msg> {
  type Proxied = AuthorizeRequest;
  fn as_view(&self) -> ::__pb::View<'_, AuthorizeRequest> {
    AuthorizeRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, AuthorizeRequest> where 'msg: 'shorter {
    AuthorizeRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl AuthorizeRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__authority_api_v1_AuthorizeRequest_new() } } }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(::__pb::__internal::Private, &mut self.inner)
  }


  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    self.as_view().serialize()
  }
  #[deprecated = "Prefer Msg::parse(), or use the new name 'clear_and_parse' to parse into a pre-existing message."]
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    self.clear_and_parse(data)
  }
  pub fn clear_and_parse(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::__pb::__runtime::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__authority_api_v1_AuthorizeRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> AuthorizeRequestView {
    AuthorizeRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> AuthorizeRequestMut {
    AuthorizeRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // promissory_file: optional bytes
  pub fn promissory_file(&self) -> &'_ [u8] {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_AuthorizeRequest_get_promissory_file(self.raw_msg()).as_ref() };
    view
  }
  pub fn set_promissory_file(&mut self, val: impl ::__pb::SettableValue<[u8]>) {
    self.promissory_file_mut().set(val);
  }
  const __PROMISSORY_FILE_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__authority_api_v1_AuthorizeRequest_get_promissory_file,
      __rust_proto_thunk__authority_api_v1_AuthorizeRequest_set_promissory_file,
    );
  fn promissory_file_mut(&mut self) -> ::__pb::Mut<'_, [u8]> {
    unsafe {
      <::__pb::Mut<[u8]>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          AuthorizeRequest::__PROMISSORY_FILE_VTABLE,
        )
      )
    }
  }

}  // impl AuthorizeRequest

impl ::__std::ops::Drop for AuthorizeRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_AuthorizeRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__authority_api_v1_AuthorizeRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_AuthorizeRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_AuthorizeRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__authority_api_v1_AuthorizeRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__authority_api_v1_AuthorizeRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_AuthorizeRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__authority_api_v1_AuthorizeRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_AuthorizeRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_AuthorizeRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_AuthorizeRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__authority_api_v1_AuthorizeRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__authority_api_v1_AuthorizeRequest_get_promissory_file(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__authority_api_v1_AuthorizeRequest_set_promissory_file(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for AuthorizeRequest


impl AuthorizeRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> AuthorizeRequestMut<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut ::__pb::__runtime::RawMessage) -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_raw_msg(::__pb::__internal::Private, msg)
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.raw_msg()
  }
}

impl<'a> AuthorizeRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct AuthorizeResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for AuthorizeResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for AuthorizeResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `AuthorizeResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `AuthorizeResponseMut`.
unsafe impl Sync for AuthorizeResponse {}

// SAFETY:
// - `AuthorizeResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for AuthorizeResponse {}

impl ::__pb::Proxied for AuthorizeResponse {
  type View<'msg> = AuthorizeResponseView<'msg>;
  type Mut<'msg> = AuthorizeResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AuthorizeResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for AuthorizeResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> AuthorizeResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__authority_api_v1_AuthorizeResponse_serialize(self.raw_msg()) }
  }

  // promissory_file: optional bytes
  pub fn promissory_file(self) -> &'msg [u8] {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_AuthorizeResponse_get_promissory_file(self.raw_msg()).as_ref() };
    view
  }

}

// SAFETY:
// - `AuthorizeResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for AuthorizeResponseView<'_> {}

// SAFETY:
// - `AuthorizeResponseView` is `Send` because while its alive a `AuthorizeResponseMut` cannot.
// - `AuthorizeResponseView` does not use thread-local data.
unsafe impl Send for AuthorizeResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for AuthorizeResponseView<'msg> {
  type Proxied = AuthorizeResponse;

  fn as_view(&self) -> ::__pb::View<'msg, AuthorizeResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, AuthorizeResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for AuthorizeResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    AuthorizeResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    AuthorizeResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for AuthorizeResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for AuthorizeResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, AuthorizeResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, AuthorizeResponse>;

  fn clear_present_field(present_mutator: Self::PresentMutData<'_>)
     -> Self::AbsentMutData<'_> {
     // SAFETY: The raw ptr msg_ref is valid
    unsafe {
      (present_mutator.optional_vtable().clearer)(present_mutator.msg_ref().msg());

     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       present_mutator.msg_ref(),
       present_mutator.optional_vtable())
    }
  }

  fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>)
     -> Self::PresentMutData<'_> {
   unsafe {
     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       absent_mutator.msg_ref(),
       absent_mutator.optional_vtable())
   }
  }
}

impl<'msg> ::__pb::SettableValue<AuthorizeResponse> for AuthorizeResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, AuthorizeResponse>)
    where AuthorizeResponse: 'dst {
    unsafe { __rust_proto_thunk__authority_api_v1_AuthorizeResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<AuthorizeResponse> for AuthorizeResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, AuthorizeResponse>)
    where AuthorizeResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for AuthorizeResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_AuthorizeResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: ::__pb::View<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_AuthorizeResponse_copy_from(
        __rust_proto_thunk__authority_api_v1_AuthorizeResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
        v.raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { __rust_proto_thunk__authority_api_v1_AuthorizeResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_AuthorizeResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__authority_api_v1_AuthorizeResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__authority_api_v1_AuthorizeResponse_copy_from(new_elem, v.raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    mut dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_AuthorizeResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for AuthorizeResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthorizeResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> ::__pb::MapIter<'_, i32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i32, Self>) -> Option<(::__pb::View<'a, i32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AuthorizeResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for AuthorizeResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthorizeResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> ::__pb::MapIter<'_, u32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u32, Self>) -> Option<(::__pb::View<'a, u32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AuthorizeResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for AuthorizeResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthorizeResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> ::__pb::MapIter<'_, i64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i64, Self>) -> Option<(::__pb::View<'a, i64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AuthorizeResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for AuthorizeResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthorizeResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> ::__pb::MapIter<'_, u64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u64, Self>) -> Option<(::__pb::View<'a, u64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AuthorizeResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for AuthorizeResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthorizeResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> ::__pb::MapIter<'_, bool, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, bool, Self>) -> Option<(::__pb::View<'a, bool>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<bool, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AuthorizeResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for AuthorizeResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthorizeResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> ::__pb::MapIter<'_, ::__pb::ProtoStr, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, ::__pb::ProtoStr, Self>) -> Option<(::__pb::View<'a, ::__pb::ProtoStr>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<::__pb::ProtoStr, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| AuthorizeResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AuthorizeResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for AuthorizeResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> AuthorizeResponseMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__runtime::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(
               ::__pb::__internal::Private, parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(_private, msg) }
  }

  #[deprecated = "This .or_default() is a no-op, usages can be safely removed"]
  pub fn or_default(self) -> Self { self }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg()
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    ::__pb::ViewProxy::as_view(self).serialize()
  }


  // promissory_file: optional bytes
  pub fn promissory_file(&self) -> &'_ [u8] {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_AuthorizeResponse_get_promissory_file(self.raw_msg()).as_ref() };
    view
  }
  pub fn set_promissory_file(&mut self, val: impl ::__pb::SettableValue<[u8]>) {
    self.promissory_file_mut().set(val);
  }
  fn promissory_file_mut(&mut self) -> ::__pb::Mut<'_, [u8]> {
    unsafe {
      <::__pb::Mut<[u8]>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          AuthorizeResponse::__PROMISSORY_FILE_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `AuthorizeResponseMut` does not perform any shared mutation.
// - `AuthorizeResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for AuthorizeResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for AuthorizeResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, AuthorizeResponse> {
    AuthorizeResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, AuthorizeResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for AuthorizeResponseMut<'msg> {
  type Proxied = AuthorizeResponse;
  fn as_view(&self) -> ::__pb::View<'_, AuthorizeResponse> {
    AuthorizeResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, AuthorizeResponse> where 'msg: 'shorter {
    AuthorizeResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl AuthorizeResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__authority_api_v1_AuthorizeResponse_new() } } }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(::__pb::__internal::Private, &mut self.inner)
  }


  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    self.as_view().serialize()
  }
  #[deprecated = "Prefer Msg::parse(), or use the new name 'clear_and_parse' to parse into a pre-existing message."]
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    self.clear_and_parse(data)
  }
  pub fn clear_and_parse(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::__pb::__runtime::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__authority_api_v1_AuthorizeResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> AuthorizeResponseView {
    AuthorizeResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> AuthorizeResponseMut {
    AuthorizeResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // promissory_file: optional bytes
  pub fn promissory_file(&self) -> &'_ [u8] {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_AuthorizeResponse_get_promissory_file(self.raw_msg()).as_ref() };
    view
  }
  pub fn set_promissory_file(&mut self, val: impl ::__pb::SettableValue<[u8]>) {
    self.promissory_file_mut().set(val);
  }
  const __PROMISSORY_FILE_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__authority_api_v1_AuthorizeResponse_get_promissory_file,
      __rust_proto_thunk__authority_api_v1_AuthorizeResponse_set_promissory_file,
    );
  fn promissory_file_mut(&mut self) -> ::__pb::Mut<'_, [u8]> {
    unsafe {
      <::__pb::Mut<[u8]>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          AuthorizeResponse::__PROMISSORY_FILE_VTABLE,
        )
      )
    }
  }

}  // impl AuthorizeResponse

impl ::__std::ops::Drop for AuthorizeResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_AuthorizeResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__authority_api_v1_AuthorizeResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_AuthorizeResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_AuthorizeResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__authority_api_v1_AuthorizeResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__authority_api_v1_AuthorizeResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_AuthorizeResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__authority_api_v1_AuthorizeResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_AuthorizeResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_AuthorizeResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_AuthorizeResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__authority_api_v1_AuthorizeResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__authority_api_v1_AuthorizeResponse_get_promissory_file(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__authority_api_v1_AuthorizeResponse_set_promissory_file(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for AuthorizeResponse


impl AuthorizeResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> AuthorizeResponseMut<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut ::__pb::__runtime::RawMessage) -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_raw_msg(::__pb::__internal::Private, msg)
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.raw_msg()
  }
}

impl<'a> AuthorizeResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct GetNetworkInfoRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for GetNetworkInfoRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for GetNetworkInfoRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `GetNetworkInfoRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetNetworkInfoRequestMut`.
unsafe impl Sync for GetNetworkInfoRequest {}

// SAFETY:
// - `GetNetworkInfoRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetNetworkInfoRequest {}

impl ::__pb::Proxied for GetNetworkInfoRequest {
  type View<'msg> = GetNetworkInfoRequestView<'msg>;
  type Mut<'msg> = GetNetworkInfoRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetNetworkInfoRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for GetNetworkInfoRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> GetNetworkInfoRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_serialize(self.raw_msg()) }
  }

}

// SAFETY:
// - `GetNetworkInfoRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetNetworkInfoRequestView<'_> {}

// SAFETY:
// - `GetNetworkInfoRequestView` is `Send` because while its alive a `GetNetworkInfoRequestMut` cannot.
// - `GetNetworkInfoRequestView` does not use thread-local data.
unsafe impl Send for GetNetworkInfoRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for GetNetworkInfoRequestView<'msg> {
  type Proxied = GetNetworkInfoRequest;

  fn as_view(&self) -> ::__pb::View<'msg, GetNetworkInfoRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, GetNetworkInfoRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for GetNetworkInfoRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    GetNetworkInfoRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    GetNetworkInfoRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for GetNetworkInfoRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for GetNetworkInfoRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, GetNetworkInfoRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, GetNetworkInfoRequest>;

  fn clear_present_field(present_mutator: Self::PresentMutData<'_>)
     -> Self::AbsentMutData<'_> {
     // SAFETY: The raw ptr msg_ref is valid
    unsafe {
      (present_mutator.optional_vtable().clearer)(present_mutator.msg_ref().msg());

     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       present_mutator.msg_ref(),
       present_mutator.optional_vtable())
    }
  }

  fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>)
     -> Self::PresentMutData<'_> {
   unsafe {
     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       absent_mutator.msg_ref(),
       absent_mutator.optional_vtable())
   }
  }
}

impl<'msg> ::__pb::SettableValue<GetNetworkInfoRequest> for GetNetworkInfoRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, GetNetworkInfoRequest>)
    where GetNetworkInfoRequest: 'dst {
    unsafe { __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<GetNetworkInfoRequest> for GetNetworkInfoRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, GetNetworkInfoRequest>)
    where GetNetworkInfoRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for GetNetworkInfoRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: ::__pb::View<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_copy_from(
        __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
        v.raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_copy_from(new_elem, v.raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    mut dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for GetNetworkInfoRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNetworkInfoRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> ::__pb::MapIter<'_, i32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i32, Self>) -> Option<(::__pb::View<'a, i32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetNetworkInfoRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for GetNetworkInfoRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNetworkInfoRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> ::__pb::MapIter<'_, u32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u32, Self>) -> Option<(::__pb::View<'a, u32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetNetworkInfoRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for GetNetworkInfoRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNetworkInfoRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> ::__pb::MapIter<'_, i64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i64, Self>) -> Option<(::__pb::View<'a, i64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetNetworkInfoRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for GetNetworkInfoRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNetworkInfoRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> ::__pb::MapIter<'_, u64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u64, Self>) -> Option<(::__pb::View<'a, u64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetNetworkInfoRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for GetNetworkInfoRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNetworkInfoRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> ::__pb::MapIter<'_, bool, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, bool, Self>) -> Option<(::__pb::View<'a, bool>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<bool, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetNetworkInfoRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for GetNetworkInfoRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNetworkInfoRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> ::__pb::MapIter<'_, ::__pb::ProtoStr, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, ::__pb::ProtoStr, Self>) -> Option<(::__pb::View<'a, ::__pb::ProtoStr>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<::__pb::ProtoStr, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| GetNetworkInfoRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetNetworkInfoRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for GetNetworkInfoRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> GetNetworkInfoRequestMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__runtime::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(
               ::__pb::__internal::Private, parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(_private, msg) }
  }

  #[deprecated = "This .or_default() is a no-op, usages can be safely removed"]
  pub fn or_default(self) -> Self { self }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg()
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    ::__pb::ViewProxy::as_view(self).serialize()
  }


}

// SAFETY:
// - `GetNetworkInfoRequestMut` does not perform any shared mutation.
// - `GetNetworkInfoRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetNetworkInfoRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for GetNetworkInfoRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, GetNetworkInfoRequest> {
    GetNetworkInfoRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, GetNetworkInfoRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for GetNetworkInfoRequestMut<'msg> {
  type Proxied = GetNetworkInfoRequest;
  fn as_view(&self) -> ::__pb::View<'_, GetNetworkInfoRequest> {
    GetNetworkInfoRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, GetNetworkInfoRequest> where 'msg: 'shorter {
    GetNetworkInfoRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl GetNetworkInfoRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_new() } } }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(::__pb::__internal::Private, &mut self.inner)
  }


  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    self.as_view().serialize()
  }
  #[deprecated = "Prefer Msg::parse(), or use the new name 'clear_and_parse' to parse into a pre-existing message."]
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    self.clear_and_parse(data)
  }
  pub fn clear_and_parse(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::__pb::__runtime::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> GetNetworkInfoRequestView {
    GetNetworkInfoRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetNetworkInfoRequestMut {
    GetNetworkInfoRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

}  // impl GetNetworkInfoRequest

impl ::__std::ops::Drop for GetNetworkInfoRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);


}  // extern "C" for GetNetworkInfoRequest


impl GetNetworkInfoRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> GetNetworkInfoRequestMut<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut ::__pb::__runtime::RawMessage) -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_raw_msg(::__pb::__internal::Private, msg)
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.raw_msg()
  }
}

impl<'a> GetNetworkInfoRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct GetNetworkInfoResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for GetNetworkInfoResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for GetNetworkInfoResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `GetNetworkInfoResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GetNetworkInfoResponseMut`.
unsafe impl Sync for GetNetworkInfoResponse {}

// SAFETY:
// - `GetNetworkInfoResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetNetworkInfoResponse {}

impl ::__pb::Proxied for GetNetworkInfoResponse {
  type View<'msg> = GetNetworkInfoResponseView<'msg>;
  type Mut<'msg> = GetNetworkInfoResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetNetworkInfoResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for GetNetworkInfoResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> GetNetworkInfoResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_serialize(self.raw_msg()) }
  }

  // emissary: optional string
  pub fn emissary(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_get_emissary(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // transaction_manager: optional string
  pub fn transaction_manager(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_get_transaction_manager(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

}

// SAFETY:
// - `GetNetworkInfoResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetNetworkInfoResponseView<'_> {}

// SAFETY:
// - `GetNetworkInfoResponseView` is `Send` because while its alive a `GetNetworkInfoResponseMut` cannot.
// - `GetNetworkInfoResponseView` does not use thread-local data.
unsafe impl Send for GetNetworkInfoResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for GetNetworkInfoResponseView<'msg> {
  type Proxied = GetNetworkInfoResponse;

  fn as_view(&self) -> ::__pb::View<'msg, GetNetworkInfoResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, GetNetworkInfoResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for GetNetworkInfoResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    GetNetworkInfoResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    GetNetworkInfoResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for GetNetworkInfoResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for GetNetworkInfoResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, GetNetworkInfoResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, GetNetworkInfoResponse>;

  fn clear_present_field(present_mutator: Self::PresentMutData<'_>)
     -> Self::AbsentMutData<'_> {
     // SAFETY: The raw ptr msg_ref is valid
    unsafe {
      (present_mutator.optional_vtable().clearer)(present_mutator.msg_ref().msg());

     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       present_mutator.msg_ref(),
       present_mutator.optional_vtable())
    }
  }

  fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>)
     -> Self::PresentMutData<'_> {
   unsafe {
     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       absent_mutator.msg_ref(),
       absent_mutator.optional_vtable())
   }
  }
}

impl<'msg> ::__pb::SettableValue<GetNetworkInfoResponse> for GetNetworkInfoResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, GetNetworkInfoResponse>)
    where GetNetworkInfoResponse: 'dst {
    unsafe { __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<GetNetworkInfoResponse> for GetNetworkInfoResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, GetNetworkInfoResponse>)
    where GetNetworkInfoResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for GetNetworkInfoResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: ::__pb::View<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_copy_from(
        __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
        v.raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_copy_from(new_elem, v.raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    mut dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for GetNetworkInfoResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNetworkInfoResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> ::__pb::MapIter<'_, i32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i32, Self>) -> Option<(::__pb::View<'a, i32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetNetworkInfoResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for GetNetworkInfoResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNetworkInfoResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> ::__pb::MapIter<'_, u32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u32, Self>) -> Option<(::__pb::View<'a, u32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetNetworkInfoResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for GetNetworkInfoResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNetworkInfoResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> ::__pb::MapIter<'_, i64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i64, Self>) -> Option<(::__pb::View<'a, i64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetNetworkInfoResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for GetNetworkInfoResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNetworkInfoResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> ::__pb::MapIter<'_, u64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u64, Self>) -> Option<(::__pb::View<'a, u64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetNetworkInfoResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for GetNetworkInfoResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNetworkInfoResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> ::__pb::MapIter<'_, bool, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, bool, Self>) -> Option<(::__pb::View<'a, bool>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<bool, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetNetworkInfoResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for GetNetworkInfoResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNetworkInfoResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> ::__pb::MapIter<'_, ::__pb::ProtoStr, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, ::__pb::ProtoStr, Self>) -> Option<(::__pb::View<'a, ::__pb::ProtoStr>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<::__pb::ProtoStr, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| GetNetworkInfoResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetNetworkInfoResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for GetNetworkInfoResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> GetNetworkInfoResponseMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__runtime::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(
               ::__pb::__internal::Private, parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(_private, msg) }
  }

  #[deprecated = "This .or_default() is a no-op, usages can be safely removed"]
  pub fn or_default(self) -> Self { self }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg()
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    ::__pb::ViewProxy::as_view(self).serialize()
  }


  // emissary: optional string
  pub fn emissary(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_get_emissary(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_emissary(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.emissary_mut().set(val);
  }
  fn emissary_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          GetNetworkInfoResponse::__EMISSARY_VTABLE,
        )
      )
    }
  }

  // transaction_manager: optional string
  pub fn transaction_manager(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_get_transaction_manager(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_transaction_manager(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.transaction_manager_mut().set(val);
  }
  fn transaction_manager_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          GetNetworkInfoResponse::__TRANSACTION_MANAGER_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `GetNetworkInfoResponseMut` does not perform any shared mutation.
// - `GetNetworkInfoResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetNetworkInfoResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for GetNetworkInfoResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, GetNetworkInfoResponse> {
    GetNetworkInfoResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, GetNetworkInfoResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for GetNetworkInfoResponseMut<'msg> {
  type Proxied = GetNetworkInfoResponse;
  fn as_view(&self) -> ::__pb::View<'_, GetNetworkInfoResponse> {
    GetNetworkInfoResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, GetNetworkInfoResponse> where 'msg: 'shorter {
    GetNetworkInfoResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl GetNetworkInfoResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_new() } } }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(::__pb::__internal::Private, &mut self.inner)
  }


  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    self.as_view().serialize()
  }
  #[deprecated = "Prefer Msg::parse(), or use the new name 'clear_and_parse' to parse into a pre-existing message."]
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    self.clear_and_parse(data)
  }
  pub fn clear_and_parse(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::__pb::__runtime::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> GetNetworkInfoResponseView {
    GetNetworkInfoResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetNetworkInfoResponseMut {
    GetNetworkInfoResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // emissary: optional string
  pub fn emissary(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_get_emissary(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_emissary(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.emissary_mut().set(val);
  }
  const __EMISSARY_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_get_emissary,
      __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_set_emissary,
    );
  fn emissary_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          GetNetworkInfoResponse::__EMISSARY_VTABLE,
        )
      )
    }
  }

  // transaction_manager: optional string
  pub fn transaction_manager(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_get_transaction_manager(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_transaction_manager(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.transaction_manager_mut().set(val);
  }
  const __TRANSACTION_MANAGER_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_get_transaction_manager,
      __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_set_transaction_manager,
    );
  fn transaction_manager_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          GetNetworkInfoResponse::__TRANSACTION_MANAGER_VTABLE,
        )
      )
    }
  }

}  // impl GetNetworkInfoResponse

impl ::__std::ops::Drop for GetNetworkInfoResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_get_emissary(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_set_emissary(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_get_transaction_manager(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_set_transaction_manager(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for GetNetworkInfoResponse


impl GetNetworkInfoResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> GetNetworkInfoResponseMut<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut ::__pb::__runtime::RawMessage) -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_raw_msg(::__pb::__internal::Private, msg)
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.raw_msg()
  }
}

impl<'a> GetNetworkInfoResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct GetNotaryRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for GetNotaryRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for GetNotaryRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `GetNotaryRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetNotaryRequestMut`.
unsafe impl Sync for GetNotaryRequest {}

// SAFETY:
// - `GetNotaryRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetNotaryRequest {}

impl ::__pb::Proxied for GetNotaryRequest {
  type View<'msg> = GetNotaryRequestView<'msg>;
  type Mut<'msg> = GetNotaryRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetNotaryRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for GetNotaryRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> GetNotaryRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__authority_api_v1_GetNotaryRequest_serialize(self.raw_msg()) }
  }

  // verifier: optional string
  pub fn verifier(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_GetNotaryRequest_get_verifier(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

}

// SAFETY:
// - `GetNotaryRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetNotaryRequestView<'_> {}

// SAFETY:
// - `GetNotaryRequestView` is `Send` because while its alive a `GetNotaryRequestMut` cannot.
// - `GetNotaryRequestView` does not use thread-local data.
unsafe impl Send for GetNotaryRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for GetNotaryRequestView<'msg> {
  type Proxied = GetNotaryRequest;

  fn as_view(&self) -> ::__pb::View<'msg, GetNotaryRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, GetNotaryRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for GetNotaryRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    GetNotaryRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    GetNotaryRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for GetNotaryRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for GetNotaryRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, GetNotaryRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, GetNotaryRequest>;

  fn clear_present_field(present_mutator: Self::PresentMutData<'_>)
     -> Self::AbsentMutData<'_> {
     // SAFETY: The raw ptr msg_ref is valid
    unsafe {
      (present_mutator.optional_vtable().clearer)(present_mutator.msg_ref().msg());

     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       present_mutator.msg_ref(),
       present_mutator.optional_vtable())
    }
  }

  fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>)
     -> Self::PresentMutData<'_> {
   unsafe {
     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       absent_mutator.msg_ref(),
       absent_mutator.optional_vtable())
   }
  }
}

impl<'msg> ::__pb::SettableValue<GetNotaryRequest> for GetNotaryRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, GetNotaryRequest>)
    where GetNotaryRequest: 'dst {
    unsafe { __rust_proto_thunk__authority_api_v1_GetNotaryRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<GetNotaryRequest> for GetNotaryRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, GetNotaryRequest>)
    where GetNotaryRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for GetNotaryRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_GetNotaryRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: ::__pb::View<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_GetNotaryRequest_copy_from(
        __rust_proto_thunk__authority_api_v1_GetNotaryRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
        v.raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { __rust_proto_thunk__authority_api_v1_GetNotaryRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_GetNotaryRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__authority_api_v1_GetNotaryRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__authority_api_v1_GetNotaryRequest_copy_from(new_elem, v.raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    mut dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_GetNotaryRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for GetNotaryRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNotaryRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> ::__pb::MapIter<'_, i32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i32, Self>) -> Option<(::__pb::View<'a, i32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetNotaryRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for GetNotaryRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNotaryRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> ::__pb::MapIter<'_, u32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u32, Self>) -> Option<(::__pb::View<'a, u32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetNotaryRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for GetNotaryRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNotaryRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> ::__pb::MapIter<'_, i64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i64, Self>) -> Option<(::__pb::View<'a, i64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetNotaryRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for GetNotaryRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNotaryRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> ::__pb::MapIter<'_, u64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u64, Self>) -> Option<(::__pb::View<'a, u64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetNotaryRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for GetNotaryRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNotaryRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> ::__pb::MapIter<'_, bool, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, bool, Self>) -> Option<(::__pb::View<'a, bool>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<bool, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetNotaryRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for GetNotaryRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNotaryRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> ::__pb::MapIter<'_, ::__pb::ProtoStr, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, ::__pb::ProtoStr, Self>) -> Option<(::__pb::View<'a, ::__pb::ProtoStr>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<::__pb::ProtoStr, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| GetNotaryRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetNotaryRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for GetNotaryRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> GetNotaryRequestMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__runtime::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(
               ::__pb::__internal::Private, parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(_private, msg) }
  }

  #[deprecated = "This .or_default() is a no-op, usages can be safely removed"]
  pub fn or_default(self) -> Self { self }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg()
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    ::__pb::ViewProxy::as_view(self).serialize()
  }


  // verifier: optional string
  pub fn verifier(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_GetNotaryRequest_get_verifier(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_verifier(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.verifier_mut().set(val);
  }
  fn verifier_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          GetNotaryRequest::__VERIFIER_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `GetNotaryRequestMut` does not perform any shared mutation.
// - `GetNotaryRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetNotaryRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for GetNotaryRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, GetNotaryRequest> {
    GetNotaryRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, GetNotaryRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for GetNotaryRequestMut<'msg> {
  type Proxied = GetNotaryRequest;
  fn as_view(&self) -> ::__pb::View<'_, GetNotaryRequest> {
    GetNotaryRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, GetNotaryRequest> where 'msg: 'shorter {
    GetNotaryRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl GetNotaryRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__authority_api_v1_GetNotaryRequest_new() } } }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(::__pb::__internal::Private, &mut self.inner)
  }


  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    self.as_view().serialize()
  }
  #[deprecated = "Prefer Msg::parse(), or use the new name 'clear_and_parse' to parse into a pre-existing message."]
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    self.clear_and_parse(data)
  }
  pub fn clear_and_parse(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::__pb::__runtime::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__authority_api_v1_GetNotaryRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> GetNotaryRequestView {
    GetNotaryRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetNotaryRequestMut {
    GetNotaryRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // verifier: optional string
  pub fn verifier(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_GetNotaryRequest_get_verifier(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_verifier(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.verifier_mut().set(val);
  }
  const __VERIFIER_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__authority_api_v1_GetNotaryRequest_get_verifier,
      __rust_proto_thunk__authority_api_v1_GetNotaryRequest_set_verifier,
    );
  fn verifier_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          GetNotaryRequest::__VERIFIER_VTABLE,
        )
      )
    }
  }

}  // impl GetNotaryRequest

impl ::__std::ops::Drop for GetNotaryRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_GetNotaryRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__authority_api_v1_GetNotaryRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetNotaryRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_GetNotaryRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__authority_api_v1_GetNotaryRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__authority_api_v1_GetNotaryRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_GetNotaryRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__authority_api_v1_GetNotaryRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetNotaryRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetNotaryRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetNotaryRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__authority_api_v1_GetNotaryRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__authority_api_v1_GetNotaryRequest_get_verifier(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__authority_api_v1_GetNotaryRequest_set_verifier(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for GetNotaryRequest


impl GetNotaryRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> GetNotaryRequestMut<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut ::__pb::__runtime::RawMessage) -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_raw_msg(::__pb::__internal::Private, msg)
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.raw_msg()
  }
}

impl<'a> GetNotaryRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct GetNotaryResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for GetNotaryResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for GetNotaryResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `GetNotaryResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GetNotaryResponseMut`.
unsafe impl Sync for GetNotaryResponse {}

// SAFETY:
// - `GetNotaryResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetNotaryResponse {}

impl ::__pb::Proxied for GetNotaryResponse {
  type View<'msg> = GetNotaryResponseView<'msg>;
  type Mut<'msg> = GetNotaryResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetNotaryResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for GetNotaryResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> GetNotaryResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__authority_api_v1_GetNotaryResponse_serialize(self.raw_msg()) }
  }

  // signature: optional bytes
  pub fn signature(self) -> &'msg [u8] {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_GetNotaryResponse_get_signature(self.raw_msg()).as_ref() };
    view
  }

}

// SAFETY:
// - `GetNotaryResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetNotaryResponseView<'_> {}

// SAFETY:
// - `GetNotaryResponseView` is `Send` because while its alive a `GetNotaryResponseMut` cannot.
// - `GetNotaryResponseView` does not use thread-local data.
unsafe impl Send for GetNotaryResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for GetNotaryResponseView<'msg> {
  type Proxied = GetNotaryResponse;

  fn as_view(&self) -> ::__pb::View<'msg, GetNotaryResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, GetNotaryResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for GetNotaryResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    GetNotaryResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    GetNotaryResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for GetNotaryResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for GetNotaryResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, GetNotaryResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, GetNotaryResponse>;

  fn clear_present_field(present_mutator: Self::PresentMutData<'_>)
     -> Self::AbsentMutData<'_> {
     // SAFETY: The raw ptr msg_ref is valid
    unsafe {
      (present_mutator.optional_vtable().clearer)(present_mutator.msg_ref().msg());

     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       present_mutator.msg_ref(),
       present_mutator.optional_vtable())
    }
  }

  fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>)
     -> Self::PresentMutData<'_> {
   unsafe {
     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       absent_mutator.msg_ref(),
       absent_mutator.optional_vtable())
   }
  }
}

impl<'msg> ::__pb::SettableValue<GetNotaryResponse> for GetNotaryResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, GetNotaryResponse>)
    where GetNotaryResponse: 'dst {
    unsafe { __rust_proto_thunk__authority_api_v1_GetNotaryResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<GetNotaryResponse> for GetNotaryResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, GetNotaryResponse>)
    where GetNotaryResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for GetNotaryResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_GetNotaryResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: ::__pb::View<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_GetNotaryResponse_copy_from(
        __rust_proto_thunk__authority_api_v1_GetNotaryResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
        v.raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { __rust_proto_thunk__authority_api_v1_GetNotaryResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_GetNotaryResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__authority_api_v1_GetNotaryResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__authority_api_v1_GetNotaryResponse_copy_from(new_elem, v.raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    mut dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_GetNotaryResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for GetNotaryResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNotaryResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> ::__pb::MapIter<'_, i32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i32, Self>) -> Option<(::__pb::View<'a, i32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetNotaryResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for GetNotaryResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNotaryResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> ::__pb::MapIter<'_, u32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u32, Self>) -> Option<(::__pb::View<'a, u32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetNotaryResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for GetNotaryResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNotaryResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> ::__pb::MapIter<'_, i64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i64, Self>) -> Option<(::__pb::View<'a, i64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetNotaryResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for GetNotaryResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNotaryResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> ::__pb::MapIter<'_, u64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u64, Self>) -> Option<(::__pb::View<'a, u64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetNotaryResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for GetNotaryResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNotaryResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> ::__pb::MapIter<'_, bool, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, bool, Self>) -> Option<(::__pb::View<'a, bool>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<bool, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetNotaryResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for GetNotaryResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetNotaryResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> ::__pb::MapIter<'_, ::__pb::ProtoStr, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, ::__pb::ProtoStr, Self>) -> Option<(::__pb::View<'a, ::__pb::ProtoStr>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<::__pb::ProtoStr, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| GetNotaryResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetNotaryResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for GetNotaryResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> GetNotaryResponseMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__runtime::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(
               ::__pb::__internal::Private, parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(_private, msg) }
  }

  #[deprecated = "This .or_default() is a no-op, usages can be safely removed"]
  pub fn or_default(self) -> Self { self }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg()
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    ::__pb::ViewProxy::as_view(self).serialize()
  }


  // signature: optional bytes
  pub fn signature(&self) -> &'_ [u8] {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_GetNotaryResponse_get_signature(self.raw_msg()).as_ref() };
    view
  }
  pub fn set_signature(&mut self, val: impl ::__pb::SettableValue<[u8]>) {
    self.signature_mut().set(val);
  }
  fn signature_mut(&mut self) -> ::__pb::Mut<'_, [u8]> {
    unsafe {
      <::__pb::Mut<[u8]>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          GetNotaryResponse::__SIGNATURE_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `GetNotaryResponseMut` does not perform any shared mutation.
// - `GetNotaryResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetNotaryResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for GetNotaryResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, GetNotaryResponse> {
    GetNotaryResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, GetNotaryResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for GetNotaryResponseMut<'msg> {
  type Proxied = GetNotaryResponse;
  fn as_view(&self) -> ::__pb::View<'_, GetNotaryResponse> {
    GetNotaryResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, GetNotaryResponse> where 'msg: 'shorter {
    GetNotaryResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl GetNotaryResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__authority_api_v1_GetNotaryResponse_new() } } }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(::__pb::__internal::Private, &mut self.inner)
  }


  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    self.as_view().serialize()
  }
  #[deprecated = "Prefer Msg::parse(), or use the new name 'clear_and_parse' to parse into a pre-existing message."]
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    self.clear_and_parse(data)
  }
  pub fn clear_and_parse(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::__pb::__runtime::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__authority_api_v1_GetNotaryResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> GetNotaryResponseView {
    GetNotaryResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetNotaryResponseMut {
    GetNotaryResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // signature: optional bytes
  pub fn signature(&self) -> &'_ [u8] {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_GetNotaryResponse_get_signature(self.raw_msg()).as_ref() };
    view
  }
  pub fn set_signature(&mut self, val: impl ::__pb::SettableValue<[u8]>) {
    self.signature_mut().set(val);
  }
  const __SIGNATURE_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__authority_api_v1_GetNotaryResponse_get_signature,
      __rust_proto_thunk__authority_api_v1_GetNotaryResponse_set_signature,
    );
  fn signature_mut(&mut self) -> ::__pb::Mut<'_, [u8]> {
    unsafe {
      <::__pb::Mut<[u8]>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          GetNotaryResponse::__SIGNATURE_VTABLE,
        )
      )
    }
  }

}  // impl GetNotaryResponse

impl ::__std::ops::Drop for GetNotaryResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_GetNotaryResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__authority_api_v1_GetNotaryResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetNotaryResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_GetNotaryResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__authority_api_v1_GetNotaryResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__authority_api_v1_GetNotaryResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_GetNotaryResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__authority_api_v1_GetNotaryResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetNotaryResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetNotaryResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_GetNotaryResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__authority_api_v1_GetNotaryResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__authority_api_v1_GetNotaryResponse_get_signature(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__authority_api_v1_GetNotaryResponse_set_signature(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for GetNotaryResponse


impl GetNotaryResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> GetNotaryResponseMut<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut ::__pb::__runtime::RawMessage) -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_raw_msg(::__pb::__internal::Private, msg)
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.raw_msg()
  }
}

impl<'a> GetNotaryResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct ListGovernedAssetsRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for ListGovernedAssetsRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for ListGovernedAssetsRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `ListGovernedAssetsRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `ListGovernedAssetsRequestMut`.
unsafe impl Sync for ListGovernedAssetsRequest {}

// SAFETY:
// - `ListGovernedAssetsRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ListGovernedAssetsRequest {}

impl ::__pb::Proxied for ListGovernedAssetsRequest {
  type View<'msg> = ListGovernedAssetsRequestView<'msg>;
  type Mut<'msg> = ListGovernedAssetsRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListGovernedAssetsRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for ListGovernedAssetsRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> ListGovernedAssetsRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_serialize(self.raw_msg()) }
  }

}

// SAFETY:
// - `ListGovernedAssetsRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for ListGovernedAssetsRequestView<'_> {}

// SAFETY:
// - `ListGovernedAssetsRequestView` is `Send` because while its alive a `ListGovernedAssetsRequestMut` cannot.
// - `ListGovernedAssetsRequestView` does not use thread-local data.
unsafe impl Send for ListGovernedAssetsRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for ListGovernedAssetsRequestView<'msg> {
  type Proxied = ListGovernedAssetsRequest;

  fn as_view(&self) -> ::__pb::View<'msg, ListGovernedAssetsRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, ListGovernedAssetsRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for ListGovernedAssetsRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    ListGovernedAssetsRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    ListGovernedAssetsRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for ListGovernedAssetsRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for ListGovernedAssetsRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, ListGovernedAssetsRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, ListGovernedAssetsRequest>;

  fn clear_present_field(present_mutator: Self::PresentMutData<'_>)
     -> Self::AbsentMutData<'_> {
     // SAFETY: The raw ptr msg_ref is valid
    unsafe {
      (present_mutator.optional_vtable().clearer)(present_mutator.msg_ref().msg());

     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       present_mutator.msg_ref(),
       present_mutator.optional_vtable())
    }
  }

  fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>)
     -> Self::PresentMutData<'_> {
   unsafe {
     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       absent_mutator.msg_ref(),
       absent_mutator.optional_vtable())
   }
  }
}

impl<'msg> ::__pb::SettableValue<ListGovernedAssetsRequest> for ListGovernedAssetsRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, ListGovernedAssetsRequest>)
    where ListGovernedAssetsRequest: 'dst {
    unsafe { __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<ListGovernedAssetsRequest> for ListGovernedAssetsRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, ListGovernedAssetsRequest>)
    where ListGovernedAssetsRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for ListGovernedAssetsRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: ::__pb::View<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_copy_from(
        __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
        v.raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_copy_from(new_elem, v.raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    mut dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for ListGovernedAssetsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ListGovernedAssetsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> ::__pb::MapIter<'_, i32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i32, Self>) -> Option<(::__pb::View<'a, i32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ListGovernedAssetsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for ListGovernedAssetsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ListGovernedAssetsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> ::__pb::MapIter<'_, u32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u32, Self>) -> Option<(::__pb::View<'a, u32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ListGovernedAssetsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for ListGovernedAssetsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ListGovernedAssetsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> ::__pb::MapIter<'_, i64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i64, Self>) -> Option<(::__pb::View<'a, i64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ListGovernedAssetsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for ListGovernedAssetsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ListGovernedAssetsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> ::__pb::MapIter<'_, u64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u64, Self>) -> Option<(::__pb::View<'a, u64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ListGovernedAssetsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for ListGovernedAssetsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ListGovernedAssetsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> ::__pb::MapIter<'_, bool, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, bool, Self>) -> Option<(::__pb::View<'a, bool>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<bool, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ListGovernedAssetsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for ListGovernedAssetsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ListGovernedAssetsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> ::__pb::MapIter<'_, ::__pb::ProtoStr, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, ::__pb::ProtoStr, Self>) -> Option<(::__pb::View<'a, ::__pb::ProtoStr>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<::__pb::ProtoStr, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| ListGovernedAssetsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListGovernedAssetsRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for ListGovernedAssetsRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> ListGovernedAssetsRequestMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__runtime::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(
               ::__pb::__internal::Private, parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(_private, msg) }
  }

  #[deprecated = "This .or_default() is a no-op, usages can be safely removed"]
  pub fn or_default(self) -> Self { self }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg()
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    ::__pb::ViewProxy::as_view(self).serialize()
  }


}

// SAFETY:
// - `ListGovernedAssetsRequestMut` does not perform any shared mutation.
// - `ListGovernedAssetsRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for ListGovernedAssetsRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for ListGovernedAssetsRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, ListGovernedAssetsRequest> {
    ListGovernedAssetsRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, ListGovernedAssetsRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for ListGovernedAssetsRequestMut<'msg> {
  type Proxied = ListGovernedAssetsRequest;
  fn as_view(&self) -> ::__pb::View<'_, ListGovernedAssetsRequest> {
    ListGovernedAssetsRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, ListGovernedAssetsRequest> where 'msg: 'shorter {
    ListGovernedAssetsRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl ListGovernedAssetsRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_new() } } }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(::__pb::__internal::Private, &mut self.inner)
  }


  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    self.as_view().serialize()
  }
  #[deprecated = "Prefer Msg::parse(), or use the new name 'clear_and_parse' to parse into a pre-existing message."]
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    self.clear_and_parse(data)
  }
  pub fn clear_and_parse(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::__pb::__runtime::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> ListGovernedAssetsRequestView {
    ListGovernedAssetsRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> ListGovernedAssetsRequestMut {
    ListGovernedAssetsRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

}  // impl ListGovernedAssetsRequest

impl ::__std::ops::Drop for ListGovernedAssetsRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);


}  // extern "C" for ListGovernedAssetsRequest


impl ListGovernedAssetsRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> ListGovernedAssetsRequestMut<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut ::__pb::__runtime::RawMessage) -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_raw_msg(::__pb::__internal::Private, msg)
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.raw_msg()
  }
}

impl<'a> ListGovernedAssetsRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct ListGovernedAssetsResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for ListGovernedAssetsResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for ListGovernedAssetsResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `ListGovernedAssetsResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `ListGovernedAssetsResponseMut`.
unsafe impl Sync for ListGovernedAssetsResponse {}

// SAFETY:
// - `ListGovernedAssetsResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ListGovernedAssetsResponse {}

impl ::__pb::Proxied for ListGovernedAssetsResponse {
  type View<'msg> = ListGovernedAssetsResponseView<'msg>;
  type Mut<'msg> = ListGovernedAssetsResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListGovernedAssetsResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for ListGovernedAssetsResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> ListGovernedAssetsResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_serialize(self.raw_msg()) }
  }

  // asset_data: repeated message authority_api.v1.ListGovernedAssetsResponse.AssetDataEntry
  pub fn asset_data(self)
    -> ::__pb::MapView<'msg, ::__pb::ProtoStr, crate::AssetDenomination> {
    unsafe {
      ::__pb::MapView::from_raw(::__pb::__internal::Private,
        __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_get_asset_data(self.raw_msg()))
    }
  }

}

// SAFETY:
// - `ListGovernedAssetsResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for ListGovernedAssetsResponseView<'_> {}

// SAFETY:
// - `ListGovernedAssetsResponseView` is `Send` because while its alive a `ListGovernedAssetsResponseMut` cannot.
// - `ListGovernedAssetsResponseView` does not use thread-local data.
unsafe impl Send for ListGovernedAssetsResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for ListGovernedAssetsResponseView<'msg> {
  type Proxied = ListGovernedAssetsResponse;

  fn as_view(&self) -> ::__pb::View<'msg, ListGovernedAssetsResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, ListGovernedAssetsResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for ListGovernedAssetsResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    ListGovernedAssetsResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    ListGovernedAssetsResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for ListGovernedAssetsResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for ListGovernedAssetsResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, ListGovernedAssetsResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, ListGovernedAssetsResponse>;

  fn clear_present_field(present_mutator: Self::PresentMutData<'_>)
     -> Self::AbsentMutData<'_> {
     // SAFETY: The raw ptr msg_ref is valid
    unsafe {
      (present_mutator.optional_vtable().clearer)(present_mutator.msg_ref().msg());

     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       present_mutator.msg_ref(),
       present_mutator.optional_vtable())
    }
  }

  fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>)
     -> Self::PresentMutData<'_> {
   unsafe {
     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       absent_mutator.msg_ref(),
       absent_mutator.optional_vtable())
   }
  }
}

impl<'msg> ::__pb::SettableValue<ListGovernedAssetsResponse> for ListGovernedAssetsResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, ListGovernedAssetsResponse>)
    where ListGovernedAssetsResponse: 'dst {
    unsafe { __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<ListGovernedAssetsResponse> for ListGovernedAssetsResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, ListGovernedAssetsResponse>)
    where ListGovernedAssetsResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for ListGovernedAssetsResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: ::__pb::View<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_copy_from(
        __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
        v.raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_copy_from(new_elem, v.raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    mut dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for ListGovernedAssetsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ListGovernedAssetsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> ::__pb::MapIter<'_, i32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i32, Self>) -> Option<(::__pb::View<'a, i32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ListGovernedAssetsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for ListGovernedAssetsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ListGovernedAssetsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> ::__pb::MapIter<'_, u32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u32, Self>) -> Option<(::__pb::View<'a, u32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ListGovernedAssetsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for ListGovernedAssetsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ListGovernedAssetsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> ::__pb::MapIter<'_, i64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i64, Self>) -> Option<(::__pb::View<'a, i64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ListGovernedAssetsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for ListGovernedAssetsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ListGovernedAssetsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> ::__pb::MapIter<'_, u64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u64, Self>) -> Option<(::__pb::View<'a, u64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ListGovernedAssetsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for ListGovernedAssetsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ListGovernedAssetsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> ::__pb::MapIter<'_, bool, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, bool, Self>) -> Option<(::__pb::View<'a, bool>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<bool, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ListGovernedAssetsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for ListGovernedAssetsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ListGovernedAssetsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> ::__pb::MapIter<'_, ::__pb::ProtoStr, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, ::__pb::ProtoStr, Self>) -> Option<(::__pb::View<'a, ::__pb::ProtoStr>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<::__pb::ProtoStr, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| ListGovernedAssetsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListGovernedAssetsResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for ListGovernedAssetsResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> ListGovernedAssetsResponseMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__runtime::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(
               ::__pb::__internal::Private, parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(_private, msg) }
  }

  #[deprecated = "This .or_default() is a no-op, usages can be safely removed"]
  pub fn or_default(self) -> Self { self }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg()
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    ::__pb::ViewProxy::as_view(self).serialize()
  }


  // asset_data: repeated message authority_api.v1.ListGovernedAssetsResponse.AssetDataEntry
  pub fn asset_data(&self)
    -> ::__pb::MapView<'_, ::__pb::ProtoStr, crate::AssetDenomination> {
    unsafe {
      ::__pb::MapView::from_raw(::__pb::__internal::Private,
        __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_get_asset_data(self.raw_msg()))
    }
  }
  pub fn asset_data_mut(&mut self)
    -> ::__pb::MapMut<'_, ::__pb::ProtoStr, crate::AssetDenomination> {
    let inner = ::__pb::__runtime::InnerMapMut::new(::__pb::__internal::Private,
      unsafe { __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_get_mut_asset_data(self.raw_msg()) });
    unsafe { ::__pb::MapMut::from_inner(::__pb::__internal::Private, inner) }
  }

}

// SAFETY:
// - `ListGovernedAssetsResponseMut` does not perform any shared mutation.
// - `ListGovernedAssetsResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for ListGovernedAssetsResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for ListGovernedAssetsResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, ListGovernedAssetsResponse> {
    ListGovernedAssetsResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, ListGovernedAssetsResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for ListGovernedAssetsResponseMut<'msg> {
  type Proxied = ListGovernedAssetsResponse;
  fn as_view(&self) -> ::__pb::View<'_, ListGovernedAssetsResponse> {
    ListGovernedAssetsResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, ListGovernedAssetsResponse> where 'msg: 'shorter {
    ListGovernedAssetsResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl ListGovernedAssetsResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_new() } } }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(::__pb::__internal::Private, &mut self.inner)
  }


  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    self.as_view().serialize()
  }
  #[deprecated = "Prefer Msg::parse(), or use the new name 'clear_and_parse' to parse into a pre-existing message."]
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    self.clear_and_parse(data)
  }
  pub fn clear_and_parse(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::__pb::__runtime::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> ListGovernedAssetsResponseView {
    ListGovernedAssetsResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> ListGovernedAssetsResponseMut {
    ListGovernedAssetsResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // asset_data: repeated message authority_api.v1.ListGovernedAssetsResponse.AssetDataEntry
  pub fn asset_data(&self)
    -> ::__pb::MapView<'_, ::__pb::ProtoStr, crate::AssetDenomination> {
    unsafe {
      ::__pb::MapView::from_raw(::__pb::__internal::Private,
        __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_get_asset_data(self.raw_msg()))
    }
  }
  pub fn asset_data_mut(&mut self)
    -> ::__pb::MapMut<'_, ::__pb::ProtoStr, crate::AssetDenomination> {
    let inner = ::__pb::__runtime::InnerMapMut::new(::__pb::__internal::Private,
      unsafe { __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_get_mut_asset_data(self.raw_msg()) });
    unsafe { ::__pb::MapMut::from_inner(::__pb::__internal::Private, inner) }
  }

}  // impl ListGovernedAssetsResponse

impl ::__std::ops::Drop for ListGovernedAssetsResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_get_asset_data(msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_get_mut_asset_data(msg: ::__pb::__runtime::RawMessage,) -> ::__pb::__runtime::RawMap;


}  // extern "C" for ListGovernedAssetsResponse

pub mod list_governed_assets_response {

}  // mod list_governed_assets_response

impl ListGovernedAssetsResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> ListGovernedAssetsResponseMut<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut ::__pb::__runtime::RawMessage) -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_raw_msg(::__pb::__internal::Private, msg)
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.raw_msg()
  }
}

impl<'a> ListGovernedAssetsResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct SetGovernedAssetsRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for SetGovernedAssetsRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for SetGovernedAssetsRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `SetGovernedAssetsRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `SetGovernedAssetsRequestMut`.
unsafe impl Sync for SetGovernedAssetsRequest {}

// SAFETY:
// - `SetGovernedAssetsRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SetGovernedAssetsRequest {}

impl ::__pb::Proxied for SetGovernedAssetsRequest {
  type View<'msg> = SetGovernedAssetsRequestView<'msg>;
  type Mut<'msg> = SetGovernedAssetsRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SetGovernedAssetsRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for SetGovernedAssetsRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> SetGovernedAssetsRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_serialize(self.raw_msg()) }
  }

  // asset_data: repeated message authority_api.v1.SetGovernedAssetsRequest.AssetDataEntry
  pub fn asset_data(self)
    -> ::__pb::MapView<'msg, ::__pb::ProtoStr, crate::AssetDenomination> {
    unsafe {
      ::__pb::MapView::from_raw(::__pb::__internal::Private,
        __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_get_asset_data(self.raw_msg()))
    }
  }

}

// SAFETY:
// - `SetGovernedAssetsRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for SetGovernedAssetsRequestView<'_> {}

// SAFETY:
// - `SetGovernedAssetsRequestView` is `Send` because while its alive a `SetGovernedAssetsRequestMut` cannot.
// - `SetGovernedAssetsRequestView` does not use thread-local data.
unsafe impl Send for SetGovernedAssetsRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for SetGovernedAssetsRequestView<'msg> {
  type Proxied = SetGovernedAssetsRequest;

  fn as_view(&self) -> ::__pb::View<'msg, SetGovernedAssetsRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, SetGovernedAssetsRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for SetGovernedAssetsRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    SetGovernedAssetsRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    SetGovernedAssetsRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for SetGovernedAssetsRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for SetGovernedAssetsRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, SetGovernedAssetsRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, SetGovernedAssetsRequest>;

  fn clear_present_field(present_mutator: Self::PresentMutData<'_>)
     -> Self::AbsentMutData<'_> {
     // SAFETY: The raw ptr msg_ref is valid
    unsafe {
      (present_mutator.optional_vtable().clearer)(present_mutator.msg_ref().msg());

     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       present_mutator.msg_ref(),
       present_mutator.optional_vtable())
    }
  }

  fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>)
     -> Self::PresentMutData<'_> {
   unsafe {
     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       absent_mutator.msg_ref(),
       absent_mutator.optional_vtable())
   }
  }
}

impl<'msg> ::__pb::SettableValue<SetGovernedAssetsRequest> for SetGovernedAssetsRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, SetGovernedAssetsRequest>)
    where SetGovernedAssetsRequest: 'dst {
    unsafe { __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<SetGovernedAssetsRequest> for SetGovernedAssetsRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, SetGovernedAssetsRequest>)
    where SetGovernedAssetsRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for SetGovernedAssetsRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: ::__pb::View<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_copy_from(
        __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
        v.raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_copy_from(new_elem, v.raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    mut dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for SetGovernedAssetsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetGovernedAssetsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> ::__pb::MapIter<'_, i32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i32, Self>) -> Option<(::__pb::View<'a, i32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetGovernedAssetsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for SetGovernedAssetsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetGovernedAssetsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> ::__pb::MapIter<'_, u32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u32, Self>) -> Option<(::__pb::View<'a, u32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetGovernedAssetsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for SetGovernedAssetsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetGovernedAssetsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> ::__pb::MapIter<'_, i64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i64, Self>) -> Option<(::__pb::View<'a, i64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetGovernedAssetsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for SetGovernedAssetsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetGovernedAssetsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> ::__pb::MapIter<'_, u64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u64, Self>) -> Option<(::__pb::View<'a, u64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetGovernedAssetsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for SetGovernedAssetsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetGovernedAssetsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> ::__pb::MapIter<'_, bool, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, bool, Self>) -> Option<(::__pb::View<'a, bool>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<bool, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetGovernedAssetsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for SetGovernedAssetsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetGovernedAssetsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> ::__pb::MapIter<'_, ::__pb::ProtoStr, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, ::__pb::ProtoStr, Self>) -> Option<(::__pb::View<'a, ::__pb::ProtoStr>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<::__pb::ProtoStr, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| SetGovernedAssetsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SetGovernedAssetsRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for SetGovernedAssetsRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> SetGovernedAssetsRequestMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__runtime::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(
               ::__pb::__internal::Private, parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(_private, msg) }
  }

  #[deprecated = "This .or_default() is a no-op, usages can be safely removed"]
  pub fn or_default(self) -> Self { self }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg()
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    ::__pb::ViewProxy::as_view(self).serialize()
  }


  // asset_data: repeated message authority_api.v1.SetGovernedAssetsRequest.AssetDataEntry
  pub fn asset_data(&self)
    -> ::__pb::MapView<'_, ::__pb::ProtoStr, crate::AssetDenomination> {
    unsafe {
      ::__pb::MapView::from_raw(::__pb::__internal::Private,
        __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_get_asset_data(self.raw_msg()))
    }
  }
  pub fn asset_data_mut(&mut self)
    -> ::__pb::MapMut<'_, ::__pb::ProtoStr, crate::AssetDenomination> {
    let inner = ::__pb::__runtime::InnerMapMut::new(::__pb::__internal::Private,
      unsafe { __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_get_mut_asset_data(self.raw_msg()) });
    unsafe { ::__pb::MapMut::from_inner(::__pb::__internal::Private, inner) }
  }

}

// SAFETY:
// - `SetGovernedAssetsRequestMut` does not perform any shared mutation.
// - `SetGovernedAssetsRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for SetGovernedAssetsRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for SetGovernedAssetsRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, SetGovernedAssetsRequest> {
    SetGovernedAssetsRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, SetGovernedAssetsRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for SetGovernedAssetsRequestMut<'msg> {
  type Proxied = SetGovernedAssetsRequest;
  fn as_view(&self) -> ::__pb::View<'_, SetGovernedAssetsRequest> {
    SetGovernedAssetsRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, SetGovernedAssetsRequest> where 'msg: 'shorter {
    SetGovernedAssetsRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl SetGovernedAssetsRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_new() } } }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(::__pb::__internal::Private, &mut self.inner)
  }


  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    self.as_view().serialize()
  }
  #[deprecated = "Prefer Msg::parse(), or use the new name 'clear_and_parse' to parse into a pre-existing message."]
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    self.clear_and_parse(data)
  }
  pub fn clear_and_parse(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::__pb::__runtime::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> SetGovernedAssetsRequestView {
    SetGovernedAssetsRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> SetGovernedAssetsRequestMut {
    SetGovernedAssetsRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // asset_data: repeated message authority_api.v1.SetGovernedAssetsRequest.AssetDataEntry
  pub fn asset_data(&self)
    -> ::__pb::MapView<'_, ::__pb::ProtoStr, crate::AssetDenomination> {
    unsafe {
      ::__pb::MapView::from_raw(::__pb::__internal::Private,
        __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_get_asset_data(self.raw_msg()))
    }
  }
  pub fn asset_data_mut(&mut self)
    -> ::__pb::MapMut<'_, ::__pb::ProtoStr, crate::AssetDenomination> {
    let inner = ::__pb::__runtime::InnerMapMut::new(::__pb::__internal::Private,
      unsafe { __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_get_mut_asset_data(self.raw_msg()) });
    unsafe { ::__pb::MapMut::from_inner(::__pb::__internal::Private, inner) }
  }

}  // impl SetGovernedAssetsRequest

impl ::__std::ops::Drop for SetGovernedAssetsRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_get_asset_data(msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_get_mut_asset_data(msg: ::__pb::__runtime::RawMessage,) -> ::__pb::__runtime::RawMap;


}  // extern "C" for SetGovernedAssetsRequest

pub mod set_governed_assets_request {

}  // mod set_governed_assets_request

impl SetGovernedAssetsRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> SetGovernedAssetsRequestMut<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut ::__pb::__runtime::RawMessage) -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_raw_msg(::__pb::__internal::Private, msg)
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.raw_msg()
  }
}

impl<'a> SetGovernedAssetsRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct SetGovernedAssetsResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for SetGovernedAssetsResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for SetGovernedAssetsResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `SetGovernedAssetsResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `SetGovernedAssetsResponseMut`.
unsafe impl Sync for SetGovernedAssetsResponse {}

// SAFETY:
// - `SetGovernedAssetsResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SetGovernedAssetsResponse {}

impl ::__pb::Proxied for SetGovernedAssetsResponse {
  type View<'msg> = SetGovernedAssetsResponseView<'msg>;
  type Mut<'msg> = SetGovernedAssetsResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SetGovernedAssetsResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for SetGovernedAssetsResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> SetGovernedAssetsResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_serialize(self.raw_msg()) }
  }

}

// SAFETY:
// - `SetGovernedAssetsResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for SetGovernedAssetsResponseView<'_> {}

// SAFETY:
// - `SetGovernedAssetsResponseView` is `Send` because while its alive a `SetGovernedAssetsResponseMut` cannot.
// - `SetGovernedAssetsResponseView` does not use thread-local data.
unsafe impl Send for SetGovernedAssetsResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for SetGovernedAssetsResponseView<'msg> {
  type Proxied = SetGovernedAssetsResponse;

  fn as_view(&self) -> ::__pb::View<'msg, SetGovernedAssetsResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, SetGovernedAssetsResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for SetGovernedAssetsResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    SetGovernedAssetsResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    SetGovernedAssetsResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for SetGovernedAssetsResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for SetGovernedAssetsResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, SetGovernedAssetsResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, SetGovernedAssetsResponse>;

  fn clear_present_field(present_mutator: Self::PresentMutData<'_>)
     -> Self::AbsentMutData<'_> {
     // SAFETY: The raw ptr msg_ref is valid
    unsafe {
      (present_mutator.optional_vtable().clearer)(present_mutator.msg_ref().msg());

     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       present_mutator.msg_ref(),
       present_mutator.optional_vtable())
    }
  }

  fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>)
     -> Self::PresentMutData<'_> {
   unsafe {
     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       absent_mutator.msg_ref(),
       absent_mutator.optional_vtable())
   }
  }
}

impl<'msg> ::__pb::SettableValue<SetGovernedAssetsResponse> for SetGovernedAssetsResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, SetGovernedAssetsResponse>)
    where SetGovernedAssetsResponse: 'dst {
    unsafe { __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<SetGovernedAssetsResponse> for SetGovernedAssetsResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, SetGovernedAssetsResponse>)
    where SetGovernedAssetsResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for SetGovernedAssetsResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: ::__pb::View<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_copy_from(
        __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
        v.raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_copy_from(new_elem, v.raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    mut dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for SetGovernedAssetsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetGovernedAssetsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> ::__pb::MapIter<'_, i32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i32, Self>) -> Option<(::__pb::View<'a, i32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetGovernedAssetsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for SetGovernedAssetsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetGovernedAssetsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> ::__pb::MapIter<'_, u32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u32, Self>) -> Option<(::__pb::View<'a, u32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetGovernedAssetsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for SetGovernedAssetsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetGovernedAssetsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> ::__pb::MapIter<'_, i64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i64, Self>) -> Option<(::__pb::View<'a, i64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetGovernedAssetsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for SetGovernedAssetsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetGovernedAssetsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> ::__pb::MapIter<'_, u64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u64, Self>) -> Option<(::__pb::View<'a, u64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetGovernedAssetsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for SetGovernedAssetsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetGovernedAssetsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> ::__pb::MapIter<'_, bool, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, bool, Self>) -> Option<(::__pb::View<'a, bool>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<bool, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetGovernedAssetsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for SetGovernedAssetsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetGovernedAssetsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> ::__pb::MapIter<'_, ::__pb::ProtoStr, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, ::__pb::ProtoStr, Self>) -> Option<(::__pb::View<'a, ::__pb::ProtoStr>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<::__pb::ProtoStr, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| SetGovernedAssetsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SetGovernedAssetsResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for SetGovernedAssetsResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> SetGovernedAssetsResponseMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__runtime::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(
               ::__pb::__internal::Private, parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(_private, msg) }
  }

  #[deprecated = "This .or_default() is a no-op, usages can be safely removed"]
  pub fn or_default(self) -> Self { self }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg()
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    ::__pb::ViewProxy::as_view(self).serialize()
  }


}

// SAFETY:
// - `SetGovernedAssetsResponseMut` does not perform any shared mutation.
// - `SetGovernedAssetsResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for SetGovernedAssetsResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for SetGovernedAssetsResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, SetGovernedAssetsResponse> {
    SetGovernedAssetsResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, SetGovernedAssetsResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for SetGovernedAssetsResponseMut<'msg> {
  type Proxied = SetGovernedAssetsResponse;
  fn as_view(&self) -> ::__pb::View<'_, SetGovernedAssetsResponse> {
    SetGovernedAssetsResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, SetGovernedAssetsResponse> where 'msg: 'shorter {
    SetGovernedAssetsResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl SetGovernedAssetsResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_new() } } }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(::__pb::__internal::Private, &mut self.inner)
  }


  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    self.as_view().serialize()
  }
  #[deprecated = "Prefer Msg::parse(), or use the new name 'clear_and_parse' to parse into a pre-existing message."]
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    self.clear_and_parse(data)
  }
  pub fn clear_and_parse(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::__pb::__runtime::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> SetGovernedAssetsResponseView {
    SetGovernedAssetsResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> SetGovernedAssetsResponseMut {
    SetGovernedAssetsResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

}  // impl SetGovernedAssetsResponse

impl ::__std::ops::Drop for SetGovernedAssetsResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);


}  // extern "C" for SetGovernedAssetsResponse


impl SetGovernedAssetsResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> SetGovernedAssetsResponseMut<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut ::__pb::__runtime::RawMessage) -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_raw_msg(::__pb::__internal::Private, msg)
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.raw_msg()
  }
}

impl<'a> SetGovernedAssetsResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct AddAssetRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for AddAssetRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for AddAssetRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `AddAssetRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `AddAssetRequestMut`.
unsafe impl Sync for AddAssetRequest {}

// SAFETY:
// - `AddAssetRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for AddAssetRequest {}

impl ::__pb::Proxied for AddAssetRequest {
  type View<'msg> = AddAssetRequestView<'msg>;
  type Mut<'msg> = AddAssetRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AddAssetRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for AddAssetRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> AddAssetRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__authority_api_v1_AddAssetRequest_serialize(self.raw_msg()) }
  }

  // currency_code: optional string
  pub fn currency_code(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_AddAssetRequest_get_currency_code(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // asset_denomination: optional message common.AssetDenomination
  pub fn asset_denomination(self) -> crate::AssetDenominationView<'msg> {
                  let submsg = unsafe { __rust_proto_thunk__authority_api_v1_AddAssetRequest_get_asset_denomination(self.raw_msg()) };
    crate::AssetDenominationView::new(::__pb::__internal::Private, submsg)
  }
  pub fn asset_denomination_opt(self) ->
  ::__pb::Optional<crate::AssetDenominationView<'msg>> {
    let view = self.asset_denomination();
    ::__pb::Optional::new(view, self.has_asset_denomination())
  }
  pub fn has_asset_denomination(self) -> bool {
    unsafe { __rust_proto_thunk__authority_api_v1_AddAssetRequest_has_asset_denomination(self.raw_msg()) }
  }

}

// SAFETY:
// - `AddAssetRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for AddAssetRequestView<'_> {}

// SAFETY:
// - `AddAssetRequestView` is `Send` because while its alive a `AddAssetRequestMut` cannot.
// - `AddAssetRequestView` does not use thread-local data.
unsafe impl Send for AddAssetRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for AddAssetRequestView<'msg> {
  type Proxied = AddAssetRequest;

  fn as_view(&self) -> ::__pb::View<'msg, AddAssetRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, AddAssetRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for AddAssetRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    AddAssetRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    AddAssetRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for AddAssetRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for AddAssetRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, AddAssetRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, AddAssetRequest>;

  fn clear_present_field(present_mutator: Self::PresentMutData<'_>)
     -> Self::AbsentMutData<'_> {
     // SAFETY: The raw ptr msg_ref is valid
    unsafe {
      (present_mutator.optional_vtable().clearer)(present_mutator.msg_ref().msg());

     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       present_mutator.msg_ref(),
       present_mutator.optional_vtable())
    }
  }

  fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>)
     -> Self::PresentMutData<'_> {
   unsafe {
     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       absent_mutator.msg_ref(),
       absent_mutator.optional_vtable())
   }
  }
}

impl<'msg> ::__pb::SettableValue<AddAssetRequest> for AddAssetRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, AddAssetRequest>)
    where AddAssetRequest: 'dst {
    unsafe { __rust_proto_thunk__authority_api_v1_AddAssetRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<AddAssetRequest> for AddAssetRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, AddAssetRequest>)
    where AddAssetRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for AddAssetRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_AddAssetRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: ::__pb::View<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_AddAssetRequest_copy_from(
        __rust_proto_thunk__authority_api_v1_AddAssetRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
        v.raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { __rust_proto_thunk__authority_api_v1_AddAssetRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_AddAssetRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__authority_api_v1_AddAssetRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__authority_api_v1_AddAssetRequest_copy_from(new_elem, v.raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    mut dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_AddAssetRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for AddAssetRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> ::__pb::MapIter<'_, i32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i32, Self>) -> Option<(::__pb::View<'a, i32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AddAssetRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for AddAssetRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> ::__pb::MapIter<'_, u32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u32, Self>) -> Option<(::__pb::View<'a, u32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AddAssetRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for AddAssetRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> ::__pb::MapIter<'_, i64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i64, Self>) -> Option<(::__pb::View<'a, i64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AddAssetRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for AddAssetRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> ::__pb::MapIter<'_, u64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u64, Self>) -> Option<(::__pb::View<'a, u64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AddAssetRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for AddAssetRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> ::__pb::MapIter<'_, bool, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, bool, Self>) -> Option<(::__pb::View<'a, bool>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<bool, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AddAssetRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for AddAssetRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> ::__pb::MapIter<'_, ::__pb::ProtoStr, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, ::__pb::ProtoStr, Self>) -> Option<(::__pb::View<'a, ::__pb::ProtoStr>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<::__pb::ProtoStr, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| AddAssetRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AddAssetRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for AddAssetRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> AddAssetRequestMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__runtime::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(
               ::__pb::__internal::Private, parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(_private, msg) }
  }

  #[deprecated = "This .or_default() is a no-op, usages can be safely removed"]
  pub fn or_default(self) -> Self { self }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg()
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    ::__pb::ViewProxy::as_view(self).serialize()
  }


  // currency_code: optional string
  pub fn currency_code(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_AddAssetRequest_get_currency_code(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_currency_code(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.currency_code_mut().set(val);
  }
  fn currency_code_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          AddAssetRequest::__CURRENCY_CODE_VTABLE,
        )
      )
    }
  }

  // asset_denomination: optional message common.AssetDenomination
  pub fn asset_denomination(&self) -> crate::AssetDenominationView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__authority_api_v1_AddAssetRequest_get_asset_denomination(self.raw_msg()) };
    crate::AssetDenominationView::new(::__pb::__internal::Private, submsg)
  }
  pub fn asset_denomination_mut(&mut self) -> crate::AssetDenominationMut<'_> {
    self.asset_denomination_entry().or_default()
  }
  fn asset_denomination_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::AssetDenomination> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__authority_api_v1_AddAssetRequest_get_asset_denomination,
                                __rust_proto_thunk__authority_api_v1_AddAssetRequest_get_mut_asset_denomination,
                                __rust_proto_thunk__authority_api_v1_AddAssetRequest_clear_asset_denomination);
    unsafe {
      let has = self.has_asset_denomination();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn asset_denomination_opt(&self) ->
  ::__pb::Optional<crate::AssetDenominationView<'_>> {
    let view = self.asset_denomination();
    ::__pb::Optional::new(view, self.has_asset_denomination())
  }
  pub fn set_asset_denomination(&mut self, val: impl ::__pb::SettableValue<crate::AssetDenomination>) {
    self.asset_denomination_entry().set(val);
  }
  pub fn has_asset_denomination(&self) -> bool {
    unsafe { __rust_proto_thunk__authority_api_v1_AddAssetRequest_has_asset_denomination(self.raw_msg()) }
  }
  pub fn clear_asset_denomination(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_AddAssetRequest_clear_asset_denomination(self.raw_msg()) }
  }

}

// SAFETY:
// - `AddAssetRequestMut` does not perform any shared mutation.
// - `AddAssetRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for AddAssetRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for AddAssetRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, AddAssetRequest> {
    AddAssetRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, AddAssetRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for AddAssetRequestMut<'msg> {
  type Proxied = AddAssetRequest;
  fn as_view(&self) -> ::__pb::View<'_, AddAssetRequest> {
    AddAssetRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, AddAssetRequest> where 'msg: 'shorter {
    AddAssetRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl AddAssetRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__authority_api_v1_AddAssetRequest_new() } } }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(::__pb::__internal::Private, &mut self.inner)
  }


  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    self.as_view().serialize()
  }
  #[deprecated = "Prefer Msg::parse(), or use the new name 'clear_and_parse' to parse into a pre-existing message."]
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    self.clear_and_parse(data)
  }
  pub fn clear_and_parse(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::__pb::__runtime::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__authority_api_v1_AddAssetRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> AddAssetRequestView {
    AddAssetRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> AddAssetRequestMut {
    AddAssetRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // currency_code: optional string
  pub fn currency_code(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_AddAssetRequest_get_currency_code(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_currency_code(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.currency_code_mut().set(val);
  }
  const __CURRENCY_CODE_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__authority_api_v1_AddAssetRequest_get_currency_code,
      __rust_proto_thunk__authority_api_v1_AddAssetRequest_set_currency_code,
    );
  fn currency_code_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          AddAssetRequest::__CURRENCY_CODE_VTABLE,
        )
      )
    }
  }

  // asset_denomination: optional message common.AssetDenomination
  pub fn asset_denomination(&self) -> crate::AssetDenominationView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__authority_api_v1_AddAssetRequest_get_asset_denomination(self.raw_msg()) };
    crate::AssetDenominationView::new(::__pb::__internal::Private, submsg)
  }
  pub fn asset_denomination_mut(&mut self) -> crate::AssetDenominationMut<'_> {
    self.asset_denomination_entry().or_default()
  }
  fn asset_denomination_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::AssetDenomination> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__authority_api_v1_AddAssetRequest_get_asset_denomination,
                                __rust_proto_thunk__authority_api_v1_AddAssetRequest_get_mut_asset_denomination,
                                __rust_proto_thunk__authority_api_v1_AddAssetRequest_clear_asset_denomination);
    unsafe {
      let has = self.has_asset_denomination();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn asset_denomination_opt(&self) ->
  ::__pb::Optional<crate::AssetDenominationView<'_>> {
    let view = self.asset_denomination();
    ::__pb::Optional::new(view, self.has_asset_denomination())
  }
  pub fn set_asset_denomination(&mut self, val: impl ::__pb::SettableValue<crate::AssetDenomination>) {
    self.asset_denomination_entry().set(val);
  }
  pub fn has_asset_denomination(&self) -> bool {
    unsafe { __rust_proto_thunk__authority_api_v1_AddAssetRequest_has_asset_denomination(self.raw_msg()) }
  }
  pub fn clear_asset_denomination(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_AddAssetRequest_clear_asset_denomination(self.raw_msg()) }
  }

}  // impl AddAssetRequest

impl ::__std::ops::Drop for AddAssetRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_AddAssetRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__authority_api_v1_AddAssetRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_AddAssetRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_AddAssetRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__authority_api_v1_AddAssetRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__authority_api_v1_AddAssetRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_AddAssetRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__authority_api_v1_AddAssetRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_AddAssetRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_AddAssetRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_AddAssetRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__authority_api_v1_AddAssetRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__authority_api_v1_AddAssetRequest_get_currency_code(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__authority_api_v1_AddAssetRequest_set_currency_code(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__authority_api_v1_AddAssetRequest_get_asset_denomination(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_AddAssetRequest_get_mut_asset_denomination(raw_msg: ::__pb::__runtime::RawMessage)
     -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_AddAssetRequest_clear_asset_denomination(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_AddAssetRequest_has_asset_denomination(raw_msg: ::__pb::__runtime::RawMessage) -> bool;


}  // extern "C" for AddAssetRequest


impl AddAssetRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> AddAssetRequestMut<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut ::__pb::__runtime::RawMessage) -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_raw_msg(::__pb::__internal::Private, msg)
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.raw_msg()
  }
}

impl<'a> AddAssetRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct AddAssetResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for AddAssetResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for AddAssetResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `AddAssetResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `AddAssetResponseMut`.
unsafe impl Sync for AddAssetResponse {}

// SAFETY:
// - `AddAssetResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for AddAssetResponse {}

impl ::__pb::Proxied for AddAssetResponse {
  type View<'msg> = AddAssetResponseView<'msg>;
  type Mut<'msg> = AddAssetResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AddAssetResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for AddAssetResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> AddAssetResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__authority_api_v1_AddAssetResponse_serialize(self.raw_msg()) }
  }

}

// SAFETY:
// - `AddAssetResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for AddAssetResponseView<'_> {}

// SAFETY:
// - `AddAssetResponseView` is `Send` because while its alive a `AddAssetResponseMut` cannot.
// - `AddAssetResponseView` does not use thread-local data.
unsafe impl Send for AddAssetResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for AddAssetResponseView<'msg> {
  type Proxied = AddAssetResponse;

  fn as_view(&self) -> ::__pb::View<'msg, AddAssetResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, AddAssetResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for AddAssetResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    AddAssetResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    AddAssetResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for AddAssetResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for AddAssetResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, AddAssetResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, AddAssetResponse>;

  fn clear_present_field(present_mutator: Self::PresentMutData<'_>)
     -> Self::AbsentMutData<'_> {
     // SAFETY: The raw ptr msg_ref is valid
    unsafe {
      (present_mutator.optional_vtable().clearer)(present_mutator.msg_ref().msg());

     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       present_mutator.msg_ref(),
       present_mutator.optional_vtable())
    }
  }

  fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>)
     -> Self::PresentMutData<'_> {
   unsafe {
     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       absent_mutator.msg_ref(),
       absent_mutator.optional_vtable())
   }
  }
}

impl<'msg> ::__pb::SettableValue<AddAssetResponse> for AddAssetResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, AddAssetResponse>)
    where AddAssetResponse: 'dst {
    unsafe { __rust_proto_thunk__authority_api_v1_AddAssetResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<AddAssetResponse> for AddAssetResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, AddAssetResponse>)
    where AddAssetResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for AddAssetResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_AddAssetResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: ::__pb::View<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_AddAssetResponse_copy_from(
        __rust_proto_thunk__authority_api_v1_AddAssetResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
        v.raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { __rust_proto_thunk__authority_api_v1_AddAssetResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_AddAssetResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__authority_api_v1_AddAssetResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__authority_api_v1_AddAssetResponse_copy_from(new_elem, v.raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    mut dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_AddAssetResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for AddAssetResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> ::__pb::MapIter<'_, i32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i32, Self>) -> Option<(::__pb::View<'a, i32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AddAssetResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for AddAssetResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> ::__pb::MapIter<'_, u32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u32, Self>) -> Option<(::__pb::View<'a, u32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AddAssetResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for AddAssetResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> ::__pb::MapIter<'_, i64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i64, Self>) -> Option<(::__pb::View<'a, i64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AddAssetResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for AddAssetResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> ::__pb::MapIter<'_, u64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u64, Self>) -> Option<(::__pb::View<'a, u64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AddAssetResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for AddAssetResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> ::__pb::MapIter<'_, bool, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, bool, Self>) -> Option<(::__pb::View<'a, bool>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<bool, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AddAssetResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for AddAssetResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> ::__pb::MapIter<'_, ::__pb::ProtoStr, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, ::__pb::ProtoStr, Self>) -> Option<(::__pb::View<'a, ::__pb::ProtoStr>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<::__pb::ProtoStr, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| AddAssetResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AddAssetResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for AddAssetResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> AddAssetResponseMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__runtime::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(
               ::__pb::__internal::Private, parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(_private, msg) }
  }

  #[deprecated = "This .or_default() is a no-op, usages can be safely removed"]
  pub fn or_default(self) -> Self { self }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg()
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    ::__pb::ViewProxy::as_view(self).serialize()
  }


}

// SAFETY:
// - `AddAssetResponseMut` does not perform any shared mutation.
// - `AddAssetResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for AddAssetResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for AddAssetResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, AddAssetResponse> {
    AddAssetResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, AddAssetResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for AddAssetResponseMut<'msg> {
  type Proxied = AddAssetResponse;
  fn as_view(&self) -> ::__pb::View<'_, AddAssetResponse> {
    AddAssetResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, AddAssetResponse> where 'msg: 'shorter {
    AddAssetResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl AddAssetResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__authority_api_v1_AddAssetResponse_new() } } }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(::__pb::__internal::Private, &mut self.inner)
  }


  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    self.as_view().serialize()
  }
  #[deprecated = "Prefer Msg::parse(), or use the new name 'clear_and_parse' to parse into a pre-existing message."]
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    self.clear_and_parse(data)
  }
  pub fn clear_and_parse(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::__pb::__runtime::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__authority_api_v1_AddAssetResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> AddAssetResponseView {
    AddAssetResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> AddAssetResponseMut {
    AddAssetResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

}  // impl AddAssetResponse

impl ::__std::ops::Drop for AddAssetResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_AddAssetResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__authority_api_v1_AddAssetResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_AddAssetResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_AddAssetResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__authority_api_v1_AddAssetResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__authority_api_v1_AddAssetResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_AddAssetResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__authority_api_v1_AddAssetResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_AddAssetResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_AddAssetResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_AddAssetResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__authority_api_v1_AddAssetResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);


}  // extern "C" for AddAssetResponse


impl AddAssetResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> AddAssetResponseMut<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut ::__pb::__runtime::RawMessage) -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_raw_msg(::__pb::__internal::Private, msg)
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.raw_msg()
  }
}

impl<'a> AddAssetResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct RemoveAssetRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for RemoveAssetRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for RemoveAssetRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `RemoveAssetRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `RemoveAssetRequestMut`.
unsafe impl Sync for RemoveAssetRequest {}

// SAFETY:
// - `RemoveAssetRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for RemoveAssetRequest {}

impl ::__pb::Proxied for RemoveAssetRequest {
  type View<'msg> = RemoveAssetRequestView<'msg>;
  type Mut<'msg> = RemoveAssetRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RemoveAssetRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for RemoveAssetRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> RemoveAssetRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_serialize(self.raw_msg()) }
  }

  // currency_code: optional string
  pub fn currency_code(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_get_currency_code(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

}

// SAFETY:
// - `RemoveAssetRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for RemoveAssetRequestView<'_> {}

// SAFETY:
// - `RemoveAssetRequestView` is `Send` because while its alive a `RemoveAssetRequestMut` cannot.
// - `RemoveAssetRequestView` does not use thread-local data.
unsafe impl Send for RemoveAssetRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for RemoveAssetRequestView<'msg> {
  type Proxied = RemoveAssetRequest;

  fn as_view(&self) -> ::__pb::View<'msg, RemoveAssetRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, RemoveAssetRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for RemoveAssetRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    RemoveAssetRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    RemoveAssetRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for RemoveAssetRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for RemoveAssetRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, RemoveAssetRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, RemoveAssetRequest>;

  fn clear_present_field(present_mutator: Self::PresentMutData<'_>)
     -> Self::AbsentMutData<'_> {
     // SAFETY: The raw ptr msg_ref is valid
    unsafe {
      (present_mutator.optional_vtable().clearer)(present_mutator.msg_ref().msg());

     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       present_mutator.msg_ref(),
       present_mutator.optional_vtable())
    }
  }

  fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>)
     -> Self::PresentMutData<'_> {
   unsafe {
     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       absent_mutator.msg_ref(),
       absent_mutator.optional_vtable())
   }
  }
}

impl<'msg> ::__pb::SettableValue<RemoveAssetRequest> for RemoveAssetRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, RemoveAssetRequest>)
    where RemoveAssetRequest: 'dst {
    unsafe { __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<RemoveAssetRequest> for RemoveAssetRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, RemoveAssetRequest>)
    where RemoveAssetRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for RemoveAssetRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: ::__pb::View<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_copy_from(
        __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
        v.raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_copy_from(new_elem, v.raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    mut dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for RemoveAssetRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(RemoveAssetRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> ::__pb::MapIter<'_, i32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i32, Self>) -> Option<(::__pb::View<'a, i32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| RemoveAssetRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for RemoveAssetRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(RemoveAssetRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> ::__pb::MapIter<'_, u32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u32, Self>) -> Option<(::__pb::View<'a, u32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| RemoveAssetRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for RemoveAssetRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(RemoveAssetRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> ::__pb::MapIter<'_, i64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i64, Self>) -> Option<(::__pb::View<'a, i64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| RemoveAssetRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for RemoveAssetRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(RemoveAssetRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> ::__pb::MapIter<'_, u64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u64, Self>) -> Option<(::__pb::View<'a, u64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| RemoveAssetRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for RemoveAssetRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(RemoveAssetRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> ::__pb::MapIter<'_, bool, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, bool, Self>) -> Option<(::__pb::View<'a, bool>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<bool, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| RemoveAssetRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for RemoveAssetRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(RemoveAssetRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> ::__pb::MapIter<'_, ::__pb::ProtoStr, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, ::__pb::ProtoStr, Self>) -> Option<(::__pb::View<'a, ::__pb::ProtoStr>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<::__pb::ProtoStr, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| RemoveAssetRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RemoveAssetRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for RemoveAssetRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> RemoveAssetRequestMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__runtime::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(
               ::__pb::__internal::Private, parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(_private, msg) }
  }

  #[deprecated = "This .or_default() is a no-op, usages can be safely removed"]
  pub fn or_default(self) -> Self { self }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg()
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    ::__pb::ViewProxy::as_view(self).serialize()
  }


  // currency_code: optional string
  pub fn currency_code(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_get_currency_code(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_currency_code(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.currency_code_mut().set(val);
  }
  fn currency_code_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          RemoveAssetRequest::__CURRENCY_CODE_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `RemoveAssetRequestMut` does not perform any shared mutation.
// - `RemoveAssetRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for RemoveAssetRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for RemoveAssetRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, RemoveAssetRequest> {
    RemoveAssetRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, RemoveAssetRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for RemoveAssetRequestMut<'msg> {
  type Proxied = RemoveAssetRequest;
  fn as_view(&self) -> ::__pb::View<'_, RemoveAssetRequest> {
    RemoveAssetRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, RemoveAssetRequest> where 'msg: 'shorter {
    RemoveAssetRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl RemoveAssetRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_new() } } }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(::__pb::__internal::Private, &mut self.inner)
  }


  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    self.as_view().serialize()
  }
  #[deprecated = "Prefer Msg::parse(), or use the new name 'clear_and_parse' to parse into a pre-existing message."]
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    self.clear_and_parse(data)
  }
  pub fn clear_and_parse(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::__pb::__runtime::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> RemoveAssetRequestView {
    RemoveAssetRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> RemoveAssetRequestMut {
    RemoveAssetRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // currency_code: optional string
  pub fn currency_code(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_get_currency_code(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_currency_code(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.currency_code_mut().set(val);
  }
  const __CURRENCY_CODE_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_get_currency_code,
      __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_set_currency_code,
    );
  fn currency_code_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          RemoveAssetRequest::__CURRENCY_CODE_VTABLE,
        )
      )
    }
  }

}  // impl RemoveAssetRequest

impl ::__std::ops::Drop for RemoveAssetRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_get_currency_code(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_set_currency_code(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for RemoveAssetRequest


impl RemoveAssetRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> RemoveAssetRequestMut<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut ::__pb::__runtime::RawMessage) -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_raw_msg(::__pb::__internal::Private, msg)
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.raw_msg()
  }
}

impl<'a> RemoveAssetRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct RemoveAssetResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for RemoveAssetResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for RemoveAssetResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `RemoveAssetResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `RemoveAssetResponseMut`.
unsafe impl Sync for RemoveAssetResponse {}

// SAFETY:
// - `RemoveAssetResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for RemoveAssetResponse {}

impl ::__pb::Proxied for RemoveAssetResponse {
  type View<'msg> = RemoveAssetResponseView<'msg>;
  type Mut<'msg> = RemoveAssetResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RemoveAssetResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for RemoveAssetResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> RemoveAssetResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_serialize(self.raw_msg()) }
  }

}

// SAFETY:
// - `RemoveAssetResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for RemoveAssetResponseView<'_> {}

// SAFETY:
// - `RemoveAssetResponseView` is `Send` because while its alive a `RemoveAssetResponseMut` cannot.
// - `RemoveAssetResponseView` does not use thread-local data.
unsafe impl Send for RemoveAssetResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for RemoveAssetResponseView<'msg> {
  type Proxied = RemoveAssetResponse;

  fn as_view(&self) -> ::__pb::View<'msg, RemoveAssetResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, RemoveAssetResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for RemoveAssetResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    RemoveAssetResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    RemoveAssetResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for RemoveAssetResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for RemoveAssetResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, RemoveAssetResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, RemoveAssetResponse>;

  fn clear_present_field(present_mutator: Self::PresentMutData<'_>)
     -> Self::AbsentMutData<'_> {
     // SAFETY: The raw ptr msg_ref is valid
    unsafe {
      (present_mutator.optional_vtable().clearer)(present_mutator.msg_ref().msg());

     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       present_mutator.msg_ref(),
       present_mutator.optional_vtable())
    }
  }

  fn set_absent_to_default(absent_mutator: Self::AbsentMutData<'_>)
     -> Self::PresentMutData<'_> {
   unsafe {
     ::__pb::__internal::RawVTableOptionalMutatorData::new(::__pb::__internal::Private,
       absent_mutator.msg_ref(),
       absent_mutator.optional_vtable())
   }
  }
}

impl<'msg> ::__pb::SettableValue<RemoveAssetResponse> for RemoveAssetResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, RemoveAssetResponse>)
    where RemoveAssetResponse: 'dst {
    unsafe { __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<RemoveAssetResponse> for RemoveAssetResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, RemoveAssetResponse>)
    where RemoveAssetResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for RemoveAssetResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
  }

  unsafe fn repeated_set_unchecked(
    mut f: ::__pb::Mut<::__pb::Repeated<Self>>,
    i: usize,
    v: ::__pb::View<Self>,
  ) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `i < len(f)` is promised by caller.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_copy_from(
        __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
        v.raw_msg(),
      );
    }
  }

  unsafe fn repeated_get_unchecked(
    f: ::__pb::View<::__pb::Repeated<Self>>,
    i: usize,
  ) -> ::__pb::View<Self> {
    // SAFETY:
    // - `f.as_raw()` is a valid `const RepeatedPtrField&`.
    // - `i < len(f)` is promised by caller.
    let msg = unsafe { __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_copy_from(new_elem, v.raw_msg());
    }
  }

  fn repeated_copy_from(
    src: ::__pb::View<::__pb::Repeated<Self>>,
    mut dest: ::__pb::Mut<::__pb::Repeated<Self>>,
  ) {
    // SAFETY:
    // - `dest.as_raw()` is a valid `RepeatedPtrField*`.
    // - `src.as_raw()` is a valid `const RepeatedPtrField&`.
    unsafe {
      __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for RemoveAssetResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(RemoveAssetResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> ::__pb::MapIter<'_, i32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i32, Self>) -> Option<(::__pb::View<'a, i32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| RemoveAssetResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for RemoveAssetResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(RemoveAssetResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> ::__pb::MapIter<'_, u32, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u32, Self>) -> Option<(::__pb::View<'a, u32>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u32, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| RemoveAssetResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for RemoveAssetResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(RemoveAssetResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> ::__pb::MapIter<'_, i64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, i64, Self>) -> Option<(::__pb::View<'a, i64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<i64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| RemoveAssetResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for RemoveAssetResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(RemoveAssetResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> ::__pb::MapIter<'_, u64, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, u64, Self>) -> Option<(::__pb::View<'a, u64>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<u64, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| RemoveAssetResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for RemoveAssetResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(RemoveAssetResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> ::__pb::MapIter<'_, bool, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, bool, Self>) -> Option<(::__pb::View<'a, bool>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<bool, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| RemoveAssetResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for RemoveAssetResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(RemoveAssetResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
    }

    fn map_iter(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> ::__pb::MapIter<'_, ::__pb::ProtoStr, Self> {
        // SAFETY:
        // - The backing map for `map.as_raw` is valid for at least '_.
        // - A View that is live for '_ guarantees the backing map is unmodified for '_.
        // - The `iter` function produces an iterator that is valid for the key
        //   and value types, and live for at least '_.
        unsafe {
            ::__pb::MapIter::from_raw(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_iter(map.as_raw(::__pb::__internal::Private))
            )
        }
    }

    fn map_iter_next<'a>(iter: &mut ::__pb::MapIter<'a, ::__pb::ProtoStr, Self>) -> Option<(::__pb::View<'a, ::__pb::ProtoStr>, ::__pb::View<'a, Self>)> {
        // SAFETY:
        // - The `MapIter` API forbids the backing map from being mutated for 'a,
        //   and guarantees that it's the correct key and value types.
        // - The thunk is safe to call as long as the iterator isn't at the end.
        // - The thunk always writes to key and value fields and does not read.
        // - The thunk does not increment the iterator.
        unsafe {
            iter.as_raw_mut(::__pb::__internal::Private).next_unchecked::<::__pb::ProtoStr, Self, _, _>(
                ::__pb::__internal::Private,
                __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| RemoveAssetResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RemoveAssetResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for RemoveAssetResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> RemoveAssetResponseMut<'msg> {
  #[doc(hidden)]
  pub fn from_parent(
             _private: ::__pb::__internal::Private,
             parent: ::__pb::__runtime::MutatorMessageRef<'msg>,
             msg: ::__pb::__runtime::RawMessage)
    -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_parent(
               ::__pb::__internal::Private, parent, msg)
    }
  }

  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: &'msg mut ::__pb::__runtime::MessageInner) -> Self {
    Self{ inner: ::__pb::__runtime::MutatorMessageRef::new(_private, msg) }
  }

  #[deprecated = "This .or_default() is a no-op, usages can be safely removed"]
  pub fn or_default(self) -> Self { self }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg()
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef<'msg> {
    self.inner
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    ::__pb::ViewProxy::as_view(self).serialize()
  }


}

// SAFETY:
// - `RemoveAssetResponseMut` does not perform any shared mutation.
// - `RemoveAssetResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for RemoveAssetResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for RemoveAssetResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, RemoveAssetResponse> {
    RemoveAssetResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, RemoveAssetResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for RemoveAssetResponseMut<'msg> {
  type Proxied = RemoveAssetResponse;
  fn as_view(&self) -> ::__pb::View<'_, RemoveAssetResponse> {
    RemoveAssetResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, RemoveAssetResponse> where 'msg: 'shorter {
    RemoveAssetResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl RemoveAssetResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_new() } } }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.inner.msg
  }

  fn as_mutator_message_ref(&mut self) -> ::__pb::__runtime::MutatorMessageRef {
    ::__pb::__runtime::MutatorMessageRef::new(::__pb::__internal::Private, &mut self.inner)
  }


  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    self.as_view().serialize()
  }
  #[deprecated = "Prefer Msg::parse(), or use the new name 'clear_and_parse' to parse into a pre-existing message."]
  pub fn deserialize(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    self.clear_and_parse(data)
  }
  pub fn clear_and_parse(&mut self, data: &[u8]) -> Result<(), ::__pb::ParseError> {
    let success = unsafe {
      // SAFETY: `data.as_ptr()` is valid to read for `data.len()`.
      let data = ::__pb::__runtime::SerializedData::from_raw_parts(
        ::__std::ptr::NonNull::new(data.as_ptr() as *mut _).unwrap(),
        data.len(),
      );

      __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> RemoveAssetResponseView {
    RemoveAssetResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> RemoveAssetResponseMut {
    RemoveAssetResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

}  // impl RemoveAssetResponse

impl ::__std::ops::Drop for RemoveAssetResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);


}  // extern "C" for RemoveAssetResponse


impl RemoveAssetResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> RemoveAssetResponseMut<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
      msg: &'a mut ::__pb::__runtime::RawMessage) -> Self {
    Self {
      inner: ::__pb::__runtime::MutatorMessageRef::from_raw_msg(::__pb::__internal::Private, msg)
    }
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.raw_msg()
  }
}

impl<'a> RemoveAssetResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

