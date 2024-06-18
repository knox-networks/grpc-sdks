extern crate protobuf_cpp as __pb;
extern crate std as __std;
#[allow(non_camel_case_types)]
pub struct IssueRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for IssueRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for IssueRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `IssueRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `IssueRequestMut`.
unsafe impl Sync for IssueRequest {}

// SAFETY:
// - `IssueRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for IssueRequest {}

impl ::__pb::Proxied for IssueRequest {
  type View<'msg> = IssueRequestView<'msg>;
  type Mut<'msg> = IssueRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct IssueRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for IssueRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> IssueRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_serialize(self.raw_msg()) }
  }

  // authentication_signature: optional message common.AuthenticationSignature
  pub fn authentication_signature(self) -> crate::AuthenticationSignatureView<'msg> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_get_authentication_signature(self.raw_msg()) };
    crate::AuthenticationSignatureView::new(::__pb::__internal::Private, submsg)
  }
  pub fn authentication_signature_opt(self) ->
  ::__pb::Optional<crate::AuthenticationSignatureView<'msg>> {
    let view = self.authentication_signature();
    ::__pb::Optional::new(view, self.has_authentication_signature())
  }
  pub fn has_authentication_signature(self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_has_authentication_signature(self.raw_msg()) }
  }

  // amount: optional message common.Amount
  pub fn amount(self) -> crate::AmountView<'msg> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_get_amount(self.raw_msg()) };
    crate::AmountView::new(::__pb::__internal::Private, submsg)
  }
  pub fn amount_opt(self) ->
  ::__pb::Optional<crate::AmountView<'msg>> {
    let view = self.amount();
    ::__pb::Optional::new(view, self.has_amount())
  }
  pub fn has_amount(self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_has_amount(self.raw_msg()) }
  }

  // authority: optional message common.DynamicVerifier
  pub fn authority(self) -> crate::DynamicVerifierView<'msg> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_get_authority(self.raw_msg()) };
    crate::DynamicVerifierView::new(::__pb::__internal::Private, submsg)
  }
  pub fn authority_opt(self) ->
  ::__pb::Optional<crate::DynamicVerifierView<'msg>> {
    let view = self.authority();
    ::__pb::Optional::new(view, self.has_authority())
  }
  pub fn has_authority(self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_has_authority(self.raw_msg()) }
  }

  // recipient: optional message common.DynamicVerifier
  pub fn recipient(self) -> crate::DynamicVerifierView<'msg> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_get_recipient(self.raw_msg()) };
    crate::DynamicVerifierView::new(::__pb::__internal::Private, submsg)
  }
  pub fn recipient_opt(self) ->
  ::__pb::Optional<crate::DynamicVerifierView<'msg>> {
    let view = self.recipient();
    ::__pb::Optional::new(view, self.has_recipient())
  }
  pub fn has_recipient(self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_has_recipient(self.raw_msg()) }
  }

  // distribution: optional message common.Distribution
  pub fn distribution(self) -> crate::DistributionView<'msg> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_get_distribution(self.raw_msg()) };
    crate::DistributionView::new(::__pb::__internal::Private, submsg)
  }
  pub fn distribution_opt(self) ->
  ::__pb::Optional<crate::DistributionView<'msg>> {
    let view = self.distribution();
    ::__pb::Optional::new(view, self.has_distribution())
  }
  pub fn has_distribution(self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_has_distribution(self.raw_msg()) }
  }

  // session: optional string
  pub fn session(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_get_session(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // signature_system: optional enum common.SignatureSystem
  pub fn signature_system(self) -> crate::SignatureSystem {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_get_signature_system(self.raw_msg()) }
  }

}

// SAFETY:
// - `IssueRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for IssueRequestView<'_> {}

// SAFETY:
// - `IssueRequestView` is `Send` because while its alive a `IssueRequestMut` cannot.
// - `IssueRequestView` does not use thread-local data.
unsafe impl Send for IssueRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for IssueRequestView<'msg> {
  type Proxied = IssueRequest;

  fn as_view(&self) -> ::__pb::View<'msg, IssueRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, IssueRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for IssueRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    IssueRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    IssueRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for IssueRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for IssueRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, IssueRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, IssueRequest>;

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

impl<'msg> ::__pb::SettableValue<IssueRequest> for IssueRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, IssueRequest>)
    where IssueRequest: 'dst {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<IssueRequest> for IssueRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, IssueRequest>)
    where IssueRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for IssueRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__issuer_api_v1_IssueRequest_copy_from(
        __rust_proto_thunk__issuer_api_v1_IssueRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__issuer_api_v1_IssueRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__issuer_api_v1_IssueRequest_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__issuer_api_v1_IssueRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for IssueRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_IssueRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssueRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for IssueRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_IssueRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssueRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for IssueRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_IssueRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssueRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for IssueRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_IssueRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssueRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for IssueRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_IssueRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssueRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for IssueRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| IssueRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct IssueRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for IssueRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> IssueRequestMut<'msg> {
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


  // authentication_signature: optional message common.AuthenticationSignature
  pub fn authentication_signature(&self) -> crate::AuthenticationSignatureView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_get_authentication_signature(self.raw_msg()) };
    crate::AuthenticationSignatureView::new(::__pb::__internal::Private, submsg)
  }
  pub fn authentication_signature_mut(&mut self) -> crate::AuthenticationSignatureMut<'_> {
    self.authentication_signature_entry().or_default()
  }
  fn authentication_signature_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::AuthenticationSignature> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_get_authentication_signature,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_get_mut_authentication_signature,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_authentication_signature);
    unsafe {
      let has = self.has_authentication_signature();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn authentication_signature_opt(&self) ->
  ::__pb::Optional<crate::AuthenticationSignatureView<'_>> {
    let view = self.authentication_signature();
    ::__pb::Optional::new(view, self.has_authentication_signature())
  }
  pub fn set_authentication_signature(&mut self, val: impl ::__pb::SettableValue<crate::AuthenticationSignature>) {
    self.authentication_signature_entry().set(val);
  }
  pub fn has_authentication_signature(&self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_has_authentication_signature(self.raw_msg()) }
  }
  pub fn clear_authentication_signature(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_authentication_signature(self.raw_msg()) }
  }

  // amount: optional message common.Amount
  pub fn amount(&self) -> crate::AmountView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_get_amount(self.raw_msg()) };
    crate::AmountView::new(::__pb::__internal::Private, submsg)
  }
  pub fn amount_mut(&mut self) -> crate::AmountMut<'_> {
    self.amount_entry().or_default()
  }
  fn amount_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::Amount> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_get_amount,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_get_mut_amount,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_amount);
    unsafe {
      let has = self.has_amount();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn amount_opt(&self) ->
  ::__pb::Optional<crate::AmountView<'_>> {
    let view = self.amount();
    ::__pb::Optional::new(view, self.has_amount())
  }
  pub fn set_amount(&mut self, val: impl ::__pb::SettableValue<crate::Amount>) {
    self.amount_entry().set(val);
  }
  pub fn has_amount(&self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_has_amount(self.raw_msg()) }
  }
  pub fn clear_amount(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_amount(self.raw_msg()) }
  }

  // authority: optional message common.DynamicVerifier
  pub fn authority(&self) -> crate::DynamicVerifierView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_get_authority(self.raw_msg()) };
    crate::DynamicVerifierView::new(::__pb::__internal::Private, submsg)
  }
  pub fn authority_mut(&mut self) -> crate::DynamicVerifierMut<'_> {
    self.authority_entry().or_default()
  }
  fn authority_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::DynamicVerifier> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_get_authority,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_get_mut_authority,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_authority);
    unsafe {
      let has = self.has_authority();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn authority_opt(&self) ->
  ::__pb::Optional<crate::DynamicVerifierView<'_>> {
    let view = self.authority();
    ::__pb::Optional::new(view, self.has_authority())
  }
  pub fn set_authority(&mut self, val: impl ::__pb::SettableValue<crate::DynamicVerifier>) {
    self.authority_entry().set(val);
  }
  pub fn has_authority(&self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_has_authority(self.raw_msg()) }
  }
  pub fn clear_authority(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_authority(self.raw_msg()) }
  }

  // recipient: optional message common.DynamicVerifier
  pub fn recipient(&self) -> crate::DynamicVerifierView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_get_recipient(self.raw_msg()) };
    crate::DynamicVerifierView::new(::__pb::__internal::Private, submsg)
  }
  pub fn recipient_mut(&mut self) -> crate::DynamicVerifierMut<'_> {
    self.recipient_entry().or_default()
  }
  fn recipient_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::DynamicVerifier> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_get_recipient,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_get_mut_recipient,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_recipient);
    unsafe {
      let has = self.has_recipient();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn recipient_opt(&self) ->
  ::__pb::Optional<crate::DynamicVerifierView<'_>> {
    let view = self.recipient();
    ::__pb::Optional::new(view, self.has_recipient())
  }
  pub fn set_recipient(&mut self, val: impl ::__pb::SettableValue<crate::DynamicVerifier>) {
    self.recipient_entry().set(val);
  }
  pub fn has_recipient(&self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_has_recipient(self.raw_msg()) }
  }
  pub fn clear_recipient(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_recipient(self.raw_msg()) }
  }

  // distribution: optional message common.Distribution
  pub fn distribution(&self) -> crate::DistributionView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_get_distribution(self.raw_msg()) };
    crate::DistributionView::new(::__pb::__internal::Private, submsg)
  }
  pub fn distribution_mut(&mut self) -> crate::DistributionMut<'_> {
    self.distribution_entry().or_default()
  }
  fn distribution_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::Distribution> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_get_distribution,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_get_mut_distribution,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_distribution);
    unsafe {
      let has = self.has_distribution();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn distribution_opt(&self) ->
  ::__pb::Optional<crate::DistributionView<'_>> {
    let view = self.distribution();
    ::__pb::Optional::new(view, self.has_distribution())
  }
  pub fn set_distribution(&mut self, val: impl ::__pb::SettableValue<crate::Distribution>) {
    self.distribution_entry().set(val);
  }
  pub fn has_distribution(&self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_has_distribution(self.raw_msg()) }
  }
  pub fn clear_distribution(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_distribution(self.raw_msg()) }
  }

  // session: optional string
  pub fn session(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_get_session(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_session(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.session_mut().set(val);
  }
  fn session_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          IssueRequest::__SESSION_VTABLE,
        )
      )
    }
  }

  // signature_system: optional enum common.SignatureSystem
  pub fn signature_system(&self) -> crate::SignatureSystem {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_get_signature_system(self.raw_msg()) }
  }
  pub fn set_signature_system(&mut self, val: crate::SignatureSystem) {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_set_signature_system(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `IssueRequestMut` does not perform any shared mutation.
// - `IssueRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for IssueRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for IssueRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, IssueRequest> {
    IssueRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, IssueRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for IssueRequestMut<'msg> {
  type Proxied = IssueRequest;
  fn as_view(&self) -> ::__pb::View<'_, IssueRequest> {
    IssueRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, IssueRequest> where 'msg: 'shorter {
    IssueRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl IssueRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_new() } } }
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

      __rust_proto_thunk__issuer_api_v1_IssueRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> IssueRequestView {
    IssueRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> IssueRequestMut {
    IssueRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // authentication_signature: optional message common.AuthenticationSignature
  pub fn authentication_signature(&self) -> crate::AuthenticationSignatureView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_get_authentication_signature(self.raw_msg()) };
    crate::AuthenticationSignatureView::new(::__pb::__internal::Private, submsg)
  }
  pub fn authentication_signature_mut(&mut self) -> crate::AuthenticationSignatureMut<'_> {
    self.authentication_signature_entry().or_default()
  }
  fn authentication_signature_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::AuthenticationSignature> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_get_authentication_signature,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_get_mut_authentication_signature,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_authentication_signature);
    unsafe {
      let has = self.has_authentication_signature();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn authentication_signature_opt(&self) ->
  ::__pb::Optional<crate::AuthenticationSignatureView<'_>> {
    let view = self.authentication_signature();
    ::__pb::Optional::new(view, self.has_authentication_signature())
  }
  pub fn set_authentication_signature(&mut self, val: impl ::__pb::SettableValue<crate::AuthenticationSignature>) {
    self.authentication_signature_entry().set(val);
  }
  pub fn has_authentication_signature(&self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_has_authentication_signature(self.raw_msg()) }
  }
  pub fn clear_authentication_signature(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_authentication_signature(self.raw_msg()) }
  }

  // amount: optional message common.Amount
  pub fn amount(&self) -> crate::AmountView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_get_amount(self.raw_msg()) };
    crate::AmountView::new(::__pb::__internal::Private, submsg)
  }
  pub fn amount_mut(&mut self) -> crate::AmountMut<'_> {
    self.amount_entry().or_default()
  }
  fn amount_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::Amount> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_get_amount,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_get_mut_amount,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_amount);
    unsafe {
      let has = self.has_amount();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn amount_opt(&self) ->
  ::__pb::Optional<crate::AmountView<'_>> {
    let view = self.amount();
    ::__pb::Optional::new(view, self.has_amount())
  }
  pub fn set_amount(&mut self, val: impl ::__pb::SettableValue<crate::Amount>) {
    self.amount_entry().set(val);
  }
  pub fn has_amount(&self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_has_amount(self.raw_msg()) }
  }
  pub fn clear_amount(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_amount(self.raw_msg()) }
  }

  // authority: optional message common.DynamicVerifier
  pub fn authority(&self) -> crate::DynamicVerifierView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_get_authority(self.raw_msg()) };
    crate::DynamicVerifierView::new(::__pb::__internal::Private, submsg)
  }
  pub fn authority_mut(&mut self) -> crate::DynamicVerifierMut<'_> {
    self.authority_entry().or_default()
  }
  fn authority_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::DynamicVerifier> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_get_authority,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_get_mut_authority,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_authority);
    unsafe {
      let has = self.has_authority();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn authority_opt(&self) ->
  ::__pb::Optional<crate::DynamicVerifierView<'_>> {
    let view = self.authority();
    ::__pb::Optional::new(view, self.has_authority())
  }
  pub fn set_authority(&mut self, val: impl ::__pb::SettableValue<crate::DynamicVerifier>) {
    self.authority_entry().set(val);
  }
  pub fn has_authority(&self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_has_authority(self.raw_msg()) }
  }
  pub fn clear_authority(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_authority(self.raw_msg()) }
  }

  // recipient: optional message common.DynamicVerifier
  pub fn recipient(&self) -> crate::DynamicVerifierView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_get_recipient(self.raw_msg()) };
    crate::DynamicVerifierView::new(::__pb::__internal::Private, submsg)
  }
  pub fn recipient_mut(&mut self) -> crate::DynamicVerifierMut<'_> {
    self.recipient_entry().or_default()
  }
  fn recipient_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::DynamicVerifier> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_get_recipient,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_get_mut_recipient,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_recipient);
    unsafe {
      let has = self.has_recipient();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn recipient_opt(&self) ->
  ::__pb::Optional<crate::DynamicVerifierView<'_>> {
    let view = self.recipient();
    ::__pb::Optional::new(view, self.has_recipient())
  }
  pub fn set_recipient(&mut self, val: impl ::__pb::SettableValue<crate::DynamicVerifier>) {
    self.recipient_entry().set(val);
  }
  pub fn has_recipient(&self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_has_recipient(self.raw_msg()) }
  }
  pub fn clear_recipient(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_recipient(self.raw_msg()) }
  }

  // distribution: optional message common.Distribution
  pub fn distribution(&self) -> crate::DistributionView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_get_distribution(self.raw_msg()) };
    crate::DistributionView::new(::__pb::__internal::Private, submsg)
  }
  pub fn distribution_mut(&mut self) -> crate::DistributionMut<'_> {
    self.distribution_entry().or_default()
  }
  fn distribution_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::Distribution> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_get_distribution,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_get_mut_distribution,
                                __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_distribution);
    unsafe {
      let has = self.has_distribution();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn distribution_opt(&self) ->
  ::__pb::Optional<crate::DistributionView<'_>> {
    let view = self.distribution();
    ::__pb::Optional::new(view, self.has_distribution())
  }
  pub fn set_distribution(&mut self, val: impl ::__pb::SettableValue<crate::Distribution>) {
    self.distribution_entry().set(val);
  }
  pub fn has_distribution(&self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_has_distribution(self.raw_msg()) }
  }
  pub fn clear_distribution(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_distribution(self.raw_msg()) }
  }

  // session: optional string
  pub fn session(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_get_session(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_session(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.session_mut().set(val);
  }
  const __SESSION_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__issuer_api_v1_IssueRequest_get_session,
      __rust_proto_thunk__issuer_api_v1_IssueRequest_set_session,
    );
  fn session_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          IssueRequest::__SESSION_VTABLE,
        )
      )
    }
  }

  // signature_system: optional enum common.SignatureSystem
  pub fn signature_system(&self) -> crate::SignatureSystem {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_get_signature_system(self.raw_msg()) }
  }
  pub fn set_signature_system(&mut self, val: crate::SignatureSystem) {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_set_signature_system(self.raw_msg(), val) }
  }

}  // impl IssueRequest

impl ::__std::ops::Drop for IssueRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_get_authentication_signature(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_get_mut_authentication_signature(raw_msg: ::__pb::__runtime::RawMessage)
     -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_authentication_signature(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_has_authentication_signature(raw_msg: ::__pb::__runtime::RawMessage) -> bool;

  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_get_amount(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_get_mut_amount(raw_msg: ::__pb::__runtime::RawMessage)
     -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_amount(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_has_amount(raw_msg: ::__pb::__runtime::RawMessage) -> bool;

  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_get_authority(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_get_mut_authority(raw_msg: ::__pb::__runtime::RawMessage)
     -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_authority(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_has_authority(raw_msg: ::__pb::__runtime::RawMessage) -> bool;

  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_get_recipient(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_get_mut_recipient(raw_msg: ::__pb::__runtime::RawMessage)
     -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_recipient(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_has_recipient(raw_msg: ::__pb::__runtime::RawMessage) -> bool;

  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_get_distribution(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_get_mut_distribution(raw_msg: ::__pb::__runtime::RawMessage)
     -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_clear_distribution(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_has_distribution(raw_msg: ::__pb::__runtime::RawMessage) -> bool;

  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_get_session(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_set_session(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_get_signature_system(raw_msg: ::__pb::__runtime::RawMessage) -> crate::SignatureSystem;
  fn __rust_proto_thunk__issuer_api_v1_IssueRequest_set_signature_system(raw_msg: ::__pb::__runtime::RawMessage, val: crate::SignatureSystem);


}  // extern "C" for IssueRequest


impl IssueRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> IssueRequestMut<'a> {
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

impl<'a> IssueRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct IssueResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for IssueResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for IssueResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `IssueResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `IssueResponseMut`.
unsafe impl Sync for IssueResponse {}

// SAFETY:
// - `IssueResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for IssueResponse {}

impl ::__pb::Proxied for IssueResponse {
  type View<'msg> = IssueResponseView<'msg>;
  type Mut<'msg> = IssueResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct IssueResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for IssueResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> IssueResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueResponse_serialize(self.raw_msg()) }
  }

  // status: optional enum issuer_api.v1.IssueResponse.Status
  pub fn status(self) -> crate::issue_response::Status {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueResponse_get_status(self.raw_msg()) }
  }

  // uetr: optional string
  pub fn uetr(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__issuer_api_v1_IssueResponse_get_uetr(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // num_promissory_files: optional uint32
  pub fn num_promissory_files(self) -> u32 {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueResponse_get_num_promissory_files(self.raw_msg()) }
  }

}

// SAFETY:
// - `IssueResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for IssueResponseView<'_> {}

// SAFETY:
// - `IssueResponseView` is `Send` because while its alive a `IssueResponseMut` cannot.
// - `IssueResponseView` does not use thread-local data.
unsafe impl Send for IssueResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for IssueResponseView<'msg> {
  type Proxied = IssueResponse;

  fn as_view(&self) -> ::__pb::View<'msg, IssueResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, IssueResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for IssueResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    IssueResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    IssueResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for IssueResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for IssueResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, IssueResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, IssueResponse>;

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

impl<'msg> ::__pb::SettableValue<IssueResponse> for IssueResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, IssueResponse>)
    where IssueResponse: 'dst {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<IssueResponse> for IssueResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, IssueResponse>)
    where IssueResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for IssueResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__issuer_api_v1_IssueResponse_copy_from(
        __rust_proto_thunk__issuer_api_v1_IssueResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__issuer_api_v1_IssueResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__issuer_api_v1_IssueResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__issuer_api_v1_IssueResponse_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__issuer_api_v1_IssueResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for IssueResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssueResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for IssueResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssueResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for IssueResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssueResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for IssueResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssueResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for IssueResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssueResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for IssueResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| IssueResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct IssueResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for IssueResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> IssueResponseMut<'msg> {
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


  // status: optional enum issuer_api.v1.IssueResponse.Status
  pub fn status(&self) -> crate::issue_response::Status {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueResponse_get_status(self.raw_msg()) }
  }
  pub fn set_status(&mut self, val: crate::issue_response::Status) {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueResponse_set_status(self.raw_msg(), val) }
  }

  // uetr: optional string
  pub fn uetr(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__issuer_api_v1_IssueResponse_get_uetr(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_uetr(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.uetr_mut().set(val);
  }
  fn uetr_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          IssueResponse::__UETR_VTABLE,
        )
      )
    }
  }

  // num_promissory_files: optional uint32
  pub fn num_promissory_files(&self) -> u32 {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueResponse_get_num_promissory_files(self.raw_msg()) }
  }
  pub fn set_num_promissory_files(&mut self, val: u32) {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueResponse_set_num_promissory_files(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `IssueResponseMut` does not perform any shared mutation.
// - `IssueResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for IssueResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for IssueResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, IssueResponse> {
    IssueResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, IssueResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for IssueResponseMut<'msg> {
  type Proxied = IssueResponse;
  fn as_view(&self) -> ::__pb::View<'_, IssueResponse> {
    IssueResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, IssueResponse> where 'msg: 'shorter {
    IssueResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl IssueResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__issuer_api_v1_IssueResponse_new() } } }
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

      __rust_proto_thunk__issuer_api_v1_IssueResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> IssueResponseView {
    IssueResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> IssueResponseMut {
    IssueResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // status: optional enum issuer_api.v1.IssueResponse.Status
  pub fn status(&self) -> crate::issue_response::Status {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueResponse_get_status(self.raw_msg()) }
  }
  pub fn set_status(&mut self, val: crate::issue_response::Status) {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueResponse_set_status(self.raw_msg(), val) }
  }

  // uetr: optional string
  pub fn uetr(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__issuer_api_v1_IssueResponse_get_uetr(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_uetr(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.uetr_mut().set(val);
  }
  const __UETR_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__issuer_api_v1_IssueResponse_get_uetr,
      __rust_proto_thunk__issuer_api_v1_IssueResponse_set_uetr,
    );
  fn uetr_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          IssueResponse::__UETR_VTABLE,
        )
      )
    }
  }

  // num_promissory_files: optional uint32
  pub fn num_promissory_files(&self) -> u32 {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueResponse_get_num_promissory_files(self.raw_msg()) }
  }
  pub fn set_num_promissory_files(&mut self, val: u32) {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueResponse_set_num_promissory_files(self.raw_msg(), val) }
  }

}  // impl IssueResponse

impl ::__std::ops::Drop for IssueResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssueResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__issuer_api_v1_IssueResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssueResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_IssueResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__issuer_api_v1_IssueResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__issuer_api_v1_IssueResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_IssueResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__issuer_api_v1_IssueResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssueResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssueResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssueResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__issuer_api_v1_IssueResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__issuer_api_v1_IssueResponse_get_status(raw_msg: ::__pb::__runtime::RawMessage) -> crate::issue_response::Status;
  fn __rust_proto_thunk__issuer_api_v1_IssueResponse_set_status(raw_msg: ::__pb::__runtime::RawMessage, val: crate::issue_response::Status);

  fn __rust_proto_thunk__issuer_api_v1_IssueResponse_get_uetr(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__issuer_api_v1_IssueResponse_set_uetr(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__issuer_api_v1_IssueResponse_get_num_promissory_files(raw_msg: ::__pb::__runtime::RawMessage) -> u32;
  fn __rust_proto_thunk__issuer_api_v1_IssueResponse_set_num_promissory_files(raw_msg: ::__pb::__runtime::RawMessage, val: u32);


}  // extern "C" for IssueResponse

pub mod issue_response {
  #[repr(transparent)]
  #[derive(Clone, Copy, PartialEq, Eq)]
  pub struct Status(i32);

  #[allow(non_upper_case_globals)]
  impl Status {
    pub const Unspecified: Status = Status(0);
    pub const Success: Status = Status(1);
    pub const Failure: Status = Status(2);
  }

  impl ::__std::convert::From<Status> for i32 {
    fn from(val: Status) -> i32 {
      val.0
    }
  }

  impl ::__std::convert::From<i32> for Status {
    fn from(val: i32) -> Status {
      Self(val)
    }
  }

  impl ::__std::default::Default for Status {
    fn default() -> Self {
      Self(0)
    }
  }

  impl ::__std::fmt::Debug for Status {
    fn fmt(&self, f: &mut ::__std::fmt::Formatter<'_>) -> ::__std::fmt::Result {
      f.debug_tuple(stringify!(Status)).field(&self.0).finish()
    }
  }

  impl ::__pb::Proxied for Status {
    type View<'a> = Status;
    type Mut<'a> = ::__pb::PrimitiveMut<'a, Status>;
  }

  impl ::__pb::ViewProxy<'_> for Status {
    type Proxied = Status;

    fn as_view(&self) -> Status {
      *self
    }

    fn into_view<'shorter>(self) -> ::__pb::View<'shorter, Status> {
      self
    }
  }

  impl ::__pb::SettableValue<Status> for Status {
    fn set_on<'msg>(
        self,
        private: ::__pb::__internal::Private,
        mut mutator: ::__pb::Mut<'msg, Status>
    ) where Status: 'msg {
      mutator.set_primitive(private, self)
    }
  }

  impl ::__pb::ProxiedWithPresence for Status {
    type PresentMutData<'a> = ::__pb::__internal::RawVTableOptionalMutatorData<'a, Status>;
    type AbsentMutData<'a> = ::__pb::__internal::RawVTableOptionalMutatorData<'a, Status>;

    fn clear_present_field(
      present_mutator: Self::PresentMutData<'_>,
    ) -> Self::AbsentMutData<'_> {
      present_mutator.clear(::__pb::__internal::Private)
    }

    fn set_absent_to_default(
      absent_mutator: Self::AbsentMutData<'_>,
    ) -> Self::PresentMutData<'_> {
      absent_mutator.set_absent_to_default(::__pb::__internal::Private)
    }
  }

  unsafe impl ::__pb::ProxiedInRepeated for Status {
    fn repeated_len(r: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
      ::__pb::__runtime::cast_enum_repeated_view(::__pb::__internal::Private, r).len()
    }

    fn repeated_push(r: ::__pb::Mut<::__pb::Repeated<Self>>, val: Status) {
      ::__pb::__runtime::cast_enum_repeated_mut(::__pb::__internal::Private, r).push(val.into())
    }

    fn repeated_clear(r: ::__pb::Mut<::__pb::Repeated<Self>>) {
      ::__pb::__runtime::cast_enum_repeated_mut(::__pb::__internal::Private, r).clear()
    }

    unsafe fn repeated_get_unchecked(
        r: ::__pb::View<::__pb::Repeated<Self>>,
        index: usize,
    ) -> ::__pb::View<Status> {
      // SAFETY: In-bounds as promised by the caller.
      unsafe {
        ::__pb::__runtime::cast_enum_repeated_view(::__pb::__internal::Private, r)
          .get_unchecked(index)
          .try_into()
          .unwrap_unchecked()
      }
    }

    unsafe fn repeated_set_unchecked(
        r: ::__pb::Mut<::__pb::Repeated<Self>>,
        index: usize,
        val: Status,
    ) {
      // SAFETY: In-bounds as promised by the caller.
      unsafe {
        ::__pb::__runtime::cast_enum_repeated_mut(::__pb::__internal::Private, r)
          .set_unchecked(index, val.into())
      }
    }

    fn repeated_copy_from(
        src: ::__pb::View<::__pb::Repeated<Self>>,
        dest: ::__pb::Mut<::__pb::Repeated<Self>>,
    ) {
      ::__pb::__runtime::cast_enum_repeated_mut(::__pb::__internal::Private, dest)
        .copy_from(::__pb::__runtime::cast_enum_repeated_view(::__pb::__internal::Private, src))
    }
  }

  impl ::__pb::__internal::PrimitiveWithRawVTable for Status {}

  // SAFETY: this is an enum type
  unsafe impl ::__pb::__internal::Enum for Status {
    const NAME: &'static str = "Status";
  }

  extern "C" {
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_Status_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_Status_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_Status_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_Status_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_Status_insert(m: ::__pb::__runtime::RawMap, key: i32, value: Status) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_Status_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut Status) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_Status_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut Status) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_Status_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_Status_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut Status);
  }
  impl ::__pb::ProxiedInMapValue<i32> for Status {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_Status_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_Status_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_Status_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_Status_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_Status_insert(map.as_raw(::__pb::__internal::Private), key, value) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_Status_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(unsafe { value.assume_init() })
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_Status_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_Status_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_IssueResponse_Status_iter_get,
                |ffi_key| ffi_key,
                ::__std::convert::identity,
            )
        }
    }
  }
  extern "C" {
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_Status_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_Status_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_Status_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_Status_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_Status_insert(m: ::__pb::__runtime::RawMap, key: u32, value: Status) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_Status_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut Status) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_Status_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut Status) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_Status_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_Status_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut Status);
  }
  impl ::__pb::ProxiedInMapValue<u32> for Status {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_Status_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_Status_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_Status_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_Status_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_Status_insert(map.as_raw(::__pb::__internal::Private), key, value) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_Status_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(unsafe { value.assume_init() })
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_Status_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_Status_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_IssueResponse_Status_iter_get,
                |ffi_key| ffi_key,
                ::__std::convert::identity,
            )
        }
    }
  }
  extern "C" {
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_Status_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_Status_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_Status_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_Status_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_Status_insert(m: ::__pb::__runtime::RawMap, key: i64, value: Status) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_Status_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut Status) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_Status_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut Status) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_Status_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_Status_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut Status);
  }
  impl ::__pb::ProxiedInMapValue<i64> for Status {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_Status_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_Status_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_Status_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_Status_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_Status_insert(map.as_raw(::__pb::__internal::Private), key, value) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_Status_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(unsafe { value.assume_init() })
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_Status_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_Status_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_IssueResponse_Status_iter_get,
                |ffi_key| ffi_key,
                ::__std::convert::identity,
            )
        }
    }
  }
  extern "C" {
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_Status_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_Status_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_Status_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_Status_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_Status_insert(m: ::__pb::__runtime::RawMap, key: u64, value: Status) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_Status_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut Status) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_Status_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut Status) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_Status_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_Status_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut Status);
  }
  impl ::__pb::ProxiedInMapValue<u64> for Status {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_Status_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_Status_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_Status_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_Status_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_Status_insert(map.as_raw(::__pb::__internal::Private), key, value) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_Status_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(unsafe { value.assume_init() })
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_Status_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_Status_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_IssueResponse_Status_iter_get,
                |ffi_key| ffi_key,
                ::__std::convert::identity,
            )
        }
    }
  }
  extern "C" {
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_Status_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_Status_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_Status_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_Status_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_Status_insert(m: ::__pb::__runtime::RawMap, key: bool, value: Status) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_Status_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut Status) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_Status_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut Status) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_Status_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_Status_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut Status);
  }
  impl ::__pb::ProxiedInMapValue<bool> for Status {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_Status_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_Status_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_Status_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_Status_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_Status_insert(map.as_raw(::__pb::__internal::Private), key, value) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_Status_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(unsafe { value.assume_init() })
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_Status_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_Status_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_IssueResponse_Status_iter_get,
                |ffi_key| ffi_key,
                ::__std::convert::identity,
            )
        }
    }
  }
  extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_Status_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_Status_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_Status_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_Status_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_Status_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: Status) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_Status_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut Status) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_Status_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut Status) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_Status_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_Status_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut Status);
  }
  impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for Status {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_Status_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_Status_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_Status_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_Status_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_Status_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_Status_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(unsafe { value.assume_init() })
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_Status_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_Status_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssueResponse_Status_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                ::__std::convert::identity,
            )
        }
    }
  }

}  // mod issue_response

impl IssueResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> IssueResponseMut<'a> {
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

impl<'a> IssueResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct AuthenticateRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for AuthenticateRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for AuthenticateRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `AuthenticateRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `AuthenticateRequestMut`.
unsafe impl Sync for AuthenticateRequest {}

// SAFETY:
// - `AuthenticateRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for AuthenticateRequest {}

impl ::__pb::Proxied for AuthenticateRequest {
  type View<'msg> = AuthenticateRequestView<'msg>;
  type Mut<'msg> = AuthenticateRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AuthenticateRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for AuthenticateRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> AuthenticateRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_serialize(self.raw_msg()) }
  }

  // verifier: optional message common.DynamicVerifier
  pub fn verifier(self) -> crate::DynamicVerifierView<'msg> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_get_verifier(self.raw_msg()) };
    crate::DynamicVerifierView::new(::__pb::__internal::Private, submsg)
  }
  pub fn verifier_opt(self) ->
  ::__pb::Optional<crate::DynamicVerifierView<'msg>> {
    let view = self.verifier();
    ::__pb::Optional::new(view, self.has_verifier())
  }
  pub fn has_verifier(self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_has_verifier(self.raw_msg()) }
  }

}

// SAFETY:
// - `AuthenticateRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for AuthenticateRequestView<'_> {}

// SAFETY:
// - `AuthenticateRequestView` is `Send` because while its alive a `AuthenticateRequestMut` cannot.
// - `AuthenticateRequestView` does not use thread-local data.
unsafe impl Send for AuthenticateRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for AuthenticateRequestView<'msg> {
  type Proxied = AuthenticateRequest;

  fn as_view(&self) -> ::__pb::View<'msg, AuthenticateRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, AuthenticateRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for AuthenticateRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    AuthenticateRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    AuthenticateRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for AuthenticateRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for AuthenticateRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, AuthenticateRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, AuthenticateRequest>;

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

impl<'msg> ::__pb::SettableValue<AuthenticateRequest> for AuthenticateRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, AuthenticateRequest>)
    where AuthenticateRequest: 'dst {
    unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<AuthenticateRequest> for AuthenticateRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, AuthenticateRequest>)
    where AuthenticateRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for AuthenticateRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_copy_from(
        __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for AuthenticateRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthenticateRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AuthenticateRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for AuthenticateRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthenticateRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AuthenticateRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for AuthenticateRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthenticateRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AuthenticateRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for AuthenticateRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthenticateRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AuthenticateRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for AuthenticateRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthenticateRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AuthenticateRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for AuthenticateRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthenticateRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| AuthenticateRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AuthenticateRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for AuthenticateRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> AuthenticateRequestMut<'msg> {
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


  // verifier: optional message common.DynamicVerifier
  pub fn verifier(&self) -> crate::DynamicVerifierView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_get_verifier(self.raw_msg()) };
    crate::DynamicVerifierView::new(::__pb::__internal::Private, submsg)
  }
  pub fn verifier_mut(&mut self) -> crate::DynamicVerifierMut<'_> {
    self.verifier_entry().or_default()
  }
  fn verifier_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::DynamicVerifier> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_get_verifier,
                                __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_get_mut_verifier,
                                __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_clear_verifier);
    unsafe {
      let has = self.has_verifier();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn verifier_opt(&self) ->
  ::__pb::Optional<crate::DynamicVerifierView<'_>> {
    let view = self.verifier();
    ::__pb::Optional::new(view, self.has_verifier())
  }
  pub fn set_verifier(&mut self, val: impl ::__pb::SettableValue<crate::DynamicVerifier>) {
    self.verifier_entry().set(val);
  }
  pub fn has_verifier(&self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_has_verifier(self.raw_msg()) }
  }
  pub fn clear_verifier(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_clear_verifier(self.raw_msg()) }
  }

}

// SAFETY:
// - `AuthenticateRequestMut` does not perform any shared mutation.
// - `AuthenticateRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for AuthenticateRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for AuthenticateRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, AuthenticateRequest> {
    AuthenticateRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, AuthenticateRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for AuthenticateRequestMut<'msg> {
  type Proxied = AuthenticateRequest;
  fn as_view(&self) -> ::__pb::View<'_, AuthenticateRequest> {
    AuthenticateRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, AuthenticateRequest> where 'msg: 'shorter {
    AuthenticateRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl AuthenticateRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_new() } } }
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

      __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> AuthenticateRequestView {
    AuthenticateRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> AuthenticateRequestMut {
    AuthenticateRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // verifier: optional message common.DynamicVerifier
  pub fn verifier(&self) -> crate::DynamicVerifierView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_get_verifier(self.raw_msg()) };
    crate::DynamicVerifierView::new(::__pb::__internal::Private, submsg)
  }
  pub fn verifier_mut(&mut self) -> crate::DynamicVerifierMut<'_> {
    self.verifier_entry().or_default()
  }
  fn verifier_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::DynamicVerifier> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_get_verifier,
                                __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_get_mut_verifier,
                                __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_clear_verifier);
    unsafe {
      let has = self.has_verifier();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn verifier_opt(&self) ->
  ::__pb::Optional<crate::DynamicVerifierView<'_>> {
    let view = self.verifier();
    ::__pb::Optional::new(view, self.has_verifier())
  }
  pub fn set_verifier(&mut self, val: impl ::__pb::SettableValue<crate::DynamicVerifier>) {
    self.verifier_entry().set(val);
  }
  pub fn has_verifier(&self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_has_verifier(self.raw_msg()) }
  }
  pub fn clear_verifier(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_clear_verifier(self.raw_msg()) }
  }

}  // impl AuthenticateRequest

impl ::__std::ops::Drop for AuthenticateRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_get_verifier(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_get_mut_verifier(raw_msg: ::__pb::__runtime::RawMessage)
     -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_clear_verifier(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateRequest_has_verifier(raw_msg: ::__pb::__runtime::RawMessage) -> bool;


}  // extern "C" for AuthenticateRequest


impl AuthenticateRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> AuthenticateRequestMut<'a> {
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

impl<'a> AuthenticateRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct AuthenticateResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for AuthenticateResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for AuthenticateResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `AuthenticateResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `AuthenticateResponseMut`.
unsafe impl Sync for AuthenticateResponse {}

// SAFETY:
// - `AuthenticateResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for AuthenticateResponse {}

impl ::__pb::Proxied for AuthenticateResponse {
  type View<'msg> = AuthenticateResponseView<'msg>;
  type Mut<'msg> = AuthenticateResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AuthenticateResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for AuthenticateResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> AuthenticateResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_serialize(self.raw_msg()) }
  }

  // challenge: optional bytes
  pub fn challenge(self) -> &'msg [u8] {
    let view = unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_get_challenge(self.raw_msg()).as_ref() };
    view
  }

  // session: optional string
  pub fn session(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_get_session(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

}

// SAFETY:
// - `AuthenticateResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for AuthenticateResponseView<'_> {}

// SAFETY:
// - `AuthenticateResponseView` is `Send` because while its alive a `AuthenticateResponseMut` cannot.
// - `AuthenticateResponseView` does not use thread-local data.
unsafe impl Send for AuthenticateResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for AuthenticateResponseView<'msg> {
  type Proxied = AuthenticateResponse;

  fn as_view(&self) -> ::__pb::View<'msg, AuthenticateResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, AuthenticateResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for AuthenticateResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    AuthenticateResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    AuthenticateResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for AuthenticateResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for AuthenticateResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, AuthenticateResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, AuthenticateResponse>;

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

impl<'msg> ::__pb::SettableValue<AuthenticateResponse> for AuthenticateResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, AuthenticateResponse>)
    where AuthenticateResponse: 'dst {
    unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<AuthenticateResponse> for AuthenticateResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, AuthenticateResponse>)
    where AuthenticateResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for AuthenticateResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_copy_from(
        __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for AuthenticateResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthenticateResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_AuthenticateResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AuthenticateResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for AuthenticateResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthenticateResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_AuthenticateResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AuthenticateResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for AuthenticateResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthenticateResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_AuthenticateResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AuthenticateResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for AuthenticateResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthenticateResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_AuthenticateResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AuthenticateResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for AuthenticateResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthenticateResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_AuthenticateResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AuthenticateResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for AuthenticateResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AuthenticateResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AuthenticateResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| AuthenticateResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AuthenticateResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for AuthenticateResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> AuthenticateResponseMut<'msg> {
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


  // challenge: optional bytes
  pub fn challenge(&self) -> &'_ [u8] {
    let view = unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_get_challenge(self.raw_msg()).as_ref() };
    view
  }
  pub fn set_challenge(&mut self, val: impl ::__pb::SettableValue<[u8]>) {
    self.challenge_mut().set(val);
  }
  fn challenge_mut(&mut self) -> ::__pb::Mut<'_, [u8]> {
    unsafe {
      <::__pb::Mut<[u8]>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          AuthenticateResponse::__CHALLENGE_VTABLE,
        )
      )
    }
  }

  // session: optional string
  pub fn session(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_get_session(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_session(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.session_mut().set(val);
  }
  fn session_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          AuthenticateResponse::__SESSION_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `AuthenticateResponseMut` does not perform any shared mutation.
// - `AuthenticateResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for AuthenticateResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for AuthenticateResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, AuthenticateResponse> {
    AuthenticateResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, AuthenticateResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for AuthenticateResponseMut<'msg> {
  type Proxied = AuthenticateResponse;
  fn as_view(&self) -> ::__pb::View<'_, AuthenticateResponse> {
    AuthenticateResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, AuthenticateResponse> where 'msg: 'shorter {
    AuthenticateResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl AuthenticateResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_new() } } }
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

      __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> AuthenticateResponseView {
    AuthenticateResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> AuthenticateResponseMut {
    AuthenticateResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // challenge: optional bytes
  pub fn challenge(&self) -> &'_ [u8] {
    let view = unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_get_challenge(self.raw_msg()).as_ref() };
    view
  }
  pub fn set_challenge(&mut self, val: impl ::__pb::SettableValue<[u8]>) {
    self.challenge_mut().set(val);
  }
  const __CHALLENGE_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_get_challenge,
      __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_set_challenge,
    );
  fn challenge_mut(&mut self) -> ::__pb::Mut<'_, [u8]> {
    unsafe {
      <::__pb::Mut<[u8]>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          AuthenticateResponse::__CHALLENGE_VTABLE,
        )
      )
    }
  }

  // session: optional string
  pub fn session(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_get_session(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_session(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.session_mut().set(val);
  }
  const __SESSION_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_get_session,
      __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_set_session,
    );
  fn session_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          AuthenticateResponse::__SESSION_VTABLE,
        )
      )
    }
  }

}  // impl AuthenticateResponse

impl ::__std::ops::Drop for AuthenticateResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_get_challenge(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_set_challenge(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_get_session(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__issuer_api_v1_AuthenticateResponse_set_session(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for AuthenticateResponse


impl AuthenticateResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> AuthenticateResponseMut<'a> {
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

impl<'a> AuthenticateResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct GetLimitsRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for GetLimitsRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for GetLimitsRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `GetLimitsRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetLimitsRequestMut`.
unsafe impl Sync for GetLimitsRequest {}

// SAFETY:
// - `GetLimitsRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetLimitsRequest {}

impl ::__pb::Proxied for GetLimitsRequest {
  type View<'msg> = GetLimitsRequestView<'msg>;
  type Mut<'msg> = GetLimitsRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetLimitsRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for GetLimitsRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> GetLimitsRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_serialize(self.raw_msg()) }
  }

}

// SAFETY:
// - `GetLimitsRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetLimitsRequestView<'_> {}

// SAFETY:
// - `GetLimitsRequestView` is `Send` because while its alive a `GetLimitsRequestMut` cannot.
// - `GetLimitsRequestView` does not use thread-local data.
unsafe impl Send for GetLimitsRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for GetLimitsRequestView<'msg> {
  type Proxied = GetLimitsRequest;

  fn as_view(&self) -> ::__pb::View<'msg, GetLimitsRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, GetLimitsRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for GetLimitsRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    GetLimitsRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    GetLimitsRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for GetLimitsRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for GetLimitsRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, GetLimitsRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, GetLimitsRequest>;

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

impl<'msg> ::__pb::SettableValue<GetLimitsRequest> for GetLimitsRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, GetLimitsRequest>)
    where GetLimitsRequest: 'dst {
    unsafe { __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<GetLimitsRequest> for GetLimitsRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, GetLimitsRequest>)
    where GetLimitsRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for GetLimitsRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_copy_from(
        __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for GetLimitsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetLimitsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetLimitsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for GetLimitsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetLimitsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetLimitsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for GetLimitsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetLimitsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetLimitsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for GetLimitsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetLimitsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetLimitsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for GetLimitsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetLimitsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetLimitsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for GetLimitsRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetLimitsRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| GetLimitsRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetLimitsRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for GetLimitsRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> GetLimitsRequestMut<'msg> {
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
// - `GetLimitsRequestMut` does not perform any shared mutation.
// - `GetLimitsRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetLimitsRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for GetLimitsRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, GetLimitsRequest> {
    GetLimitsRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, GetLimitsRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for GetLimitsRequestMut<'msg> {
  type Proxied = GetLimitsRequest;
  fn as_view(&self) -> ::__pb::View<'_, GetLimitsRequest> {
    GetLimitsRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, GetLimitsRequest> where 'msg: 'shorter {
    GetLimitsRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl GetLimitsRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_new() } } }
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

      __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> GetLimitsRequestView {
    GetLimitsRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetLimitsRequestMut {
    GetLimitsRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

}  // impl GetLimitsRequest

impl ::__std::ops::Drop for GetLimitsRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);


}  // extern "C" for GetLimitsRequest


impl GetLimitsRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> GetLimitsRequestMut<'a> {
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

impl<'a> GetLimitsRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct IssuanceLimits {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for IssuanceLimits {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for IssuanceLimits {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `IssuanceLimits` is `Sync` because it does not implement interior mutability.
//    Neither does `IssuanceLimitsMut`.
unsafe impl Sync for IssuanceLimits {}

// SAFETY:
// - `IssuanceLimits` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for IssuanceLimits {}

impl ::__pb::Proxied for IssuanceLimits {
  type View<'msg> = IssuanceLimitsView<'msg>;
  type Mut<'msg> = IssuanceLimitsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct IssuanceLimitsView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for IssuanceLimitsView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> IssuanceLimitsView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssuanceLimits_serialize(self.raw_msg()) }
  }

  // inner: repeated message common.Amount
  pub fn inner(self) -> ::__pb::RepeatedView<'msg, crate::Amount> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__issuer_api_v1_IssuanceLimits_get_inner(self.raw_msg()),
      )
    }
  }

}

// SAFETY:
// - `IssuanceLimitsView` is `Sync` because it does not support mutation.
unsafe impl Sync for IssuanceLimitsView<'_> {}

// SAFETY:
// - `IssuanceLimitsView` is `Send` because while its alive a `IssuanceLimitsMut` cannot.
// - `IssuanceLimitsView` does not use thread-local data.
unsafe impl Send for IssuanceLimitsView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for IssuanceLimitsView<'msg> {
  type Proxied = IssuanceLimits;

  fn as_view(&self) -> ::__pb::View<'msg, IssuanceLimits> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, IssuanceLimits> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for IssuanceLimits {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    IssuanceLimitsView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    IssuanceLimitsMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for IssuanceLimits {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for IssuanceLimits {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, IssuanceLimits>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, IssuanceLimits>;

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

impl<'msg> ::__pb::SettableValue<IssuanceLimits> for IssuanceLimitsView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, IssuanceLimits>)
    where IssuanceLimits: 'dst {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssuanceLimits_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<IssuanceLimits> for IssuanceLimits {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, IssuanceLimits>)
    where IssuanceLimits: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for IssuanceLimits {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_IssuanceLimits_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__issuer_api_v1_IssuanceLimits_copy_from(
        __rust_proto_thunk__issuer_api_v1_IssuanceLimits_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__issuer_api_v1_IssuanceLimits_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_IssuanceLimits_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__issuer_api_v1_IssuanceLimits_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__issuer_api_v1_IssuanceLimits_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__issuer_api_v1_IssuanceLimits_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for IssuanceLimits {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssuanceLimitsView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_IssuanceLimits_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssuanceLimitsView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for IssuanceLimits {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssuanceLimitsView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_IssuanceLimits_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssuanceLimitsView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for IssuanceLimits {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssuanceLimitsView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_IssuanceLimits_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssuanceLimitsView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for IssuanceLimits {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssuanceLimitsView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_IssuanceLimits_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssuanceLimitsView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for IssuanceLimits {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssuanceLimitsView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_IssuanceLimits_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssuanceLimitsView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for IssuanceLimits {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssuanceLimitsView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_IssuanceLimits_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| IssuanceLimitsView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct IssuanceLimitsMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for IssuanceLimitsMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> IssuanceLimitsMut<'msg> {
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


  // inner: repeated message common.Amount
  pub fn inner(&self) -> ::__pb::RepeatedView<'_, crate::Amount> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__issuer_api_v1_IssuanceLimits_get_inner(self.raw_msg()),
      )
    }
  }
  pub fn inner_mut(&mut self) -> ::__pb::RepeatedMut<'_, crate::Amount> {
    unsafe {
      ::__pb::RepeatedMut::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__runtime::InnerRepeatedMut::new(
          ::__pb::__internal::Private,
          __rust_proto_thunk__issuer_api_v1_IssuanceLimits_get_mut_inner(self.raw_msg()),
        ),
      )
    }
  }

}

// SAFETY:
// - `IssuanceLimitsMut` does not perform any shared mutation.
// - `IssuanceLimitsMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for IssuanceLimitsMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for IssuanceLimitsMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, IssuanceLimits> {
    IssuanceLimitsMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, IssuanceLimits> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for IssuanceLimitsMut<'msg> {
  type Proxied = IssuanceLimits;
  fn as_view(&self) -> ::__pb::View<'_, IssuanceLimits> {
    IssuanceLimitsView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, IssuanceLimits> where 'msg: 'shorter {
    IssuanceLimitsView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl IssuanceLimits {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__issuer_api_v1_IssuanceLimits_new() } } }
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

      __rust_proto_thunk__issuer_api_v1_IssuanceLimits_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> IssuanceLimitsView {
    IssuanceLimitsView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> IssuanceLimitsMut {
    IssuanceLimitsMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // inner: repeated message common.Amount
  pub fn inner(&self) -> ::__pb::RepeatedView<'_, crate::Amount> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__issuer_api_v1_IssuanceLimits_get_inner(self.raw_msg()),
      )
    }
  }
  pub fn inner_mut(&mut self) -> ::__pb::RepeatedMut<'_, crate::Amount> {
    unsafe {
      ::__pb::RepeatedMut::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__runtime::InnerRepeatedMut::new(
          ::__pb::__internal::Private,
          __rust_proto_thunk__issuer_api_v1_IssuanceLimits_get_mut_inner(self.raw_msg()),
        ),
      )
    }
  }

}  // impl IssuanceLimits

impl ::__std::ops::Drop for IssuanceLimits {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_IssuanceLimits_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__issuer_api_v1_IssuanceLimits_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssuanceLimits_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_IssuanceLimits_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__issuer_api_v1_IssuanceLimits_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__issuer_api_v1_IssuanceLimits_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_IssuanceLimits_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__issuer_api_v1_IssuanceLimits_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssuanceLimits_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssuanceLimits_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_IssuanceLimits_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__issuer_api_v1_IssuanceLimits_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__issuer_api_v1_IssuanceLimits_clear_inner(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_IssuanceLimits_get_mut_inner(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawRepeatedField;
  fn __rust_proto_thunk__issuer_api_v1_IssuanceLimits_get_inner(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawRepeatedField;


}  // extern "C" for IssuanceLimits


impl IssuanceLimits {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> IssuanceLimitsMut<'a> {
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

impl<'a> IssuanceLimitsView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct GetLimitsResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for GetLimitsResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for GetLimitsResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `GetLimitsResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GetLimitsResponseMut`.
unsafe impl Sync for GetLimitsResponse {}

// SAFETY:
// - `GetLimitsResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetLimitsResponse {}

impl ::__pb::Proxied for GetLimitsResponse {
  type View<'msg> = GetLimitsResponseView<'msg>;
  type Mut<'msg> = GetLimitsResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetLimitsResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for GetLimitsResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> GetLimitsResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_serialize(self.raw_msg()) }
  }

  // limits: repeated message issuer_api.v1.GetLimitsResponse.LimitsEntry
  pub fn limits(self)
    -> ::__pb::MapView<'msg, ::__pb::ProtoStr, crate::IssuanceLimits> {
    unsafe {
      ::__pb::MapView::from_raw(::__pb::__internal::Private,
        __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_get_limits(self.raw_msg()))
    }
  }

}

// SAFETY:
// - `GetLimitsResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetLimitsResponseView<'_> {}

// SAFETY:
// - `GetLimitsResponseView` is `Send` because while its alive a `GetLimitsResponseMut` cannot.
// - `GetLimitsResponseView` does not use thread-local data.
unsafe impl Send for GetLimitsResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for GetLimitsResponseView<'msg> {
  type Proxied = GetLimitsResponse;

  fn as_view(&self) -> ::__pb::View<'msg, GetLimitsResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, GetLimitsResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for GetLimitsResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    GetLimitsResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    GetLimitsResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for GetLimitsResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for GetLimitsResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, GetLimitsResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, GetLimitsResponse>;

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

impl<'msg> ::__pb::SettableValue<GetLimitsResponse> for GetLimitsResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, GetLimitsResponse>)
    where GetLimitsResponse: 'dst {
    unsafe { __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<GetLimitsResponse> for GetLimitsResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, GetLimitsResponse>)
    where GetLimitsResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for GetLimitsResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_copy_from(
        __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for GetLimitsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetLimitsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_GetLimitsResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetLimitsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for GetLimitsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetLimitsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_GetLimitsResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetLimitsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for GetLimitsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetLimitsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_GetLimitsResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetLimitsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for GetLimitsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetLimitsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_GetLimitsResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetLimitsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for GetLimitsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetLimitsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_GetLimitsResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetLimitsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for GetLimitsResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetLimitsResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetLimitsResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| GetLimitsResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetLimitsResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for GetLimitsResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> GetLimitsResponseMut<'msg> {
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


  // limits: repeated message issuer_api.v1.GetLimitsResponse.LimitsEntry
  pub fn limits(&self)
    -> ::__pb::MapView<'_, ::__pb::ProtoStr, crate::IssuanceLimits> {
    unsafe {
      ::__pb::MapView::from_raw(::__pb::__internal::Private,
        __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_get_limits(self.raw_msg()))
    }
  }
  pub fn limits_mut(&mut self)
    -> ::__pb::MapMut<'_, ::__pb::ProtoStr, crate::IssuanceLimits> {
    let inner = ::__pb::__runtime::InnerMapMut::new(::__pb::__internal::Private,
      unsafe { __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_get_mut_limits(self.raw_msg()) });
    unsafe { ::__pb::MapMut::from_inner(::__pb::__internal::Private, inner) }
  }

}

// SAFETY:
// - `GetLimitsResponseMut` does not perform any shared mutation.
// - `GetLimitsResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetLimitsResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for GetLimitsResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, GetLimitsResponse> {
    GetLimitsResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, GetLimitsResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for GetLimitsResponseMut<'msg> {
  type Proxied = GetLimitsResponse;
  fn as_view(&self) -> ::__pb::View<'_, GetLimitsResponse> {
    GetLimitsResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, GetLimitsResponse> where 'msg: 'shorter {
    GetLimitsResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl GetLimitsResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_new() } } }
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

      __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> GetLimitsResponseView {
    GetLimitsResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetLimitsResponseMut {
    GetLimitsResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // limits: repeated message issuer_api.v1.GetLimitsResponse.LimitsEntry
  pub fn limits(&self)
    -> ::__pb::MapView<'_, ::__pb::ProtoStr, crate::IssuanceLimits> {
    unsafe {
      ::__pb::MapView::from_raw(::__pb::__internal::Private,
        __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_get_limits(self.raw_msg()))
    }
  }
  pub fn limits_mut(&mut self)
    -> ::__pb::MapMut<'_, ::__pb::ProtoStr, crate::IssuanceLimits> {
    let inner = ::__pb::__runtime::InnerMapMut::new(::__pb::__internal::Private,
      unsafe { __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_get_mut_limits(self.raw_msg()) });
    unsafe { ::__pb::MapMut::from_inner(::__pb::__internal::Private, inner) }
  }

}  // impl GetLimitsResponse

impl ::__std::ops::Drop for GetLimitsResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_get_limits(msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__issuer_api_v1_GetLimitsResponse_get_mut_limits(msg: ::__pb::__runtime::RawMessage,) -> ::__pb::__runtime::RawMap;


}  // extern "C" for GetLimitsResponse

pub mod get_limits_response {

}  // mod get_limits_response

impl GetLimitsResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> GetLimitsResponseMut<'a> {
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

impl<'a> GetLimitsResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct SetRoleRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for SetRoleRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for SetRoleRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `SetRoleRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `SetRoleRequestMut`.
unsafe impl Sync for SetRoleRequest {}

// SAFETY:
// - `SetRoleRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SetRoleRequest {}

impl ::__pb::Proxied for SetRoleRequest {
  type View<'msg> = SetRoleRequestView<'msg>;
  type Mut<'msg> = SetRoleRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SetRoleRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for SetRoleRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> SetRoleRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleRequest_serialize(self.raw_msg()) }
  }

  // role: optional message common.Role
  pub fn role(self) -> crate::RoleView<'msg> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleRequest_get_role(self.raw_msg()) };
    crate::RoleView::new(::__pb::__internal::Private, submsg)
  }
  pub fn role_opt(self) ->
  ::__pb::Optional<crate::RoleView<'msg>> {
    let view = self.role();
    ::__pb::Optional::new(view, self.has_role())
  }
  pub fn has_role(self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleRequest_has_role(self.raw_msg()) }
  }

}

// SAFETY:
// - `SetRoleRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for SetRoleRequestView<'_> {}

// SAFETY:
// - `SetRoleRequestView` is `Send` because while its alive a `SetRoleRequestMut` cannot.
// - `SetRoleRequestView` does not use thread-local data.
unsafe impl Send for SetRoleRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for SetRoleRequestView<'msg> {
  type Proxied = SetRoleRequest;

  fn as_view(&self) -> ::__pb::View<'msg, SetRoleRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, SetRoleRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for SetRoleRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    SetRoleRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    SetRoleRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for SetRoleRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for SetRoleRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, SetRoleRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, SetRoleRequest>;

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

impl<'msg> ::__pb::SettableValue<SetRoleRequest> for SetRoleRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, SetRoleRequest>)
    where SetRoleRequest: 'dst {
    unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<SetRoleRequest> for SetRoleRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, SetRoleRequest>)
    where SetRoleRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for SetRoleRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__issuer_api_v1_SetRoleRequest_copy_from(
        __rust_proto_thunk__issuer_api_v1_SetRoleRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__issuer_api_v1_SetRoleRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__issuer_api_v1_SetRoleRequest_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__issuer_api_v1_SetRoleRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for SetRoleRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetRoleRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetRoleRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for SetRoleRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetRoleRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetRoleRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for SetRoleRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetRoleRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetRoleRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for SetRoleRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetRoleRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetRoleRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for SetRoleRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetRoleRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetRoleRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for SetRoleRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetRoleRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| SetRoleRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SetRoleRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for SetRoleRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> SetRoleRequestMut<'msg> {
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


  // role: optional message common.Role
  pub fn role(&self) -> crate::RoleView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleRequest_get_role(self.raw_msg()) };
    crate::RoleView::new(::__pb::__internal::Private, submsg)
  }
  pub fn role_mut(&mut self) -> crate::RoleMut<'_> {
    self.role_entry().or_default()
  }
  fn role_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::Role> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__issuer_api_v1_SetRoleRequest_get_role,
                                __rust_proto_thunk__issuer_api_v1_SetRoleRequest_get_mut_role,
                                __rust_proto_thunk__issuer_api_v1_SetRoleRequest_clear_role);
    unsafe {
      let has = self.has_role();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn role_opt(&self) ->
  ::__pb::Optional<crate::RoleView<'_>> {
    let view = self.role();
    ::__pb::Optional::new(view, self.has_role())
  }
  pub fn set_role(&mut self, val: impl ::__pb::SettableValue<crate::Role>) {
    self.role_entry().set(val);
  }
  pub fn has_role(&self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleRequest_has_role(self.raw_msg()) }
  }
  pub fn clear_role(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleRequest_clear_role(self.raw_msg()) }
  }

}

// SAFETY:
// - `SetRoleRequestMut` does not perform any shared mutation.
// - `SetRoleRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for SetRoleRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for SetRoleRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, SetRoleRequest> {
    SetRoleRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, SetRoleRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for SetRoleRequestMut<'msg> {
  type Proxied = SetRoleRequest;
  fn as_view(&self) -> ::__pb::View<'_, SetRoleRequest> {
    SetRoleRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, SetRoleRequest> where 'msg: 'shorter {
    SetRoleRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl SetRoleRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleRequest_new() } } }
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

      __rust_proto_thunk__issuer_api_v1_SetRoleRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> SetRoleRequestView {
    SetRoleRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> SetRoleRequestMut {
    SetRoleRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // role: optional message common.Role
  pub fn role(&self) -> crate::RoleView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleRequest_get_role(self.raw_msg()) };
    crate::RoleView::new(::__pb::__internal::Private, submsg)
  }
  pub fn role_mut(&mut self) -> crate::RoleMut<'_> {
    self.role_entry().or_default()
  }
  fn role_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::Role> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__issuer_api_v1_SetRoleRequest_get_role,
                                __rust_proto_thunk__issuer_api_v1_SetRoleRequest_get_mut_role,
                                __rust_proto_thunk__issuer_api_v1_SetRoleRequest_clear_role);
    unsafe {
      let has = self.has_role();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn role_opt(&self) ->
  ::__pb::Optional<crate::RoleView<'_>> {
    let view = self.role();
    ::__pb::Optional::new(view, self.has_role())
  }
  pub fn set_role(&mut self, val: impl ::__pb::SettableValue<crate::Role>) {
    self.role_entry().set(val);
  }
  pub fn has_role(&self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleRequest_has_role(self.raw_msg()) }
  }
  pub fn clear_role(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleRequest_clear_role(self.raw_msg()) }
  }

}  // impl SetRoleRequest

impl ::__std::ops::Drop for SetRoleRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__issuer_api_v1_SetRoleRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_SetRoleRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_SetRoleRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__issuer_api_v1_SetRoleRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__issuer_api_v1_SetRoleRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_SetRoleRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__issuer_api_v1_SetRoleRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_SetRoleRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_SetRoleRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_SetRoleRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__issuer_api_v1_SetRoleRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__issuer_api_v1_SetRoleRequest_get_role(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_SetRoleRequest_get_mut_role(raw_msg: ::__pb::__runtime::RawMessage)
     -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_SetRoleRequest_clear_role(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_SetRoleRequest_has_role(raw_msg: ::__pb::__runtime::RawMessage) -> bool;


}  // extern "C" for SetRoleRequest


impl SetRoleRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> SetRoleRequestMut<'a> {
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

impl<'a> SetRoleRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct SetRoleResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for SetRoleResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for SetRoleResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `SetRoleResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `SetRoleResponseMut`.
unsafe impl Sync for SetRoleResponse {}

// SAFETY:
// - `SetRoleResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SetRoleResponse {}

impl ::__pb::Proxied for SetRoleResponse {
  type View<'msg> = SetRoleResponseView<'msg>;
  type Mut<'msg> = SetRoleResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SetRoleResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for SetRoleResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> SetRoleResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleResponse_serialize(self.raw_msg()) }
  }

}

// SAFETY:
// - `SetRoleResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for SetRoleResponseView<'_> {}

// SAFETY:
// - `SetRoleResponseView` is `Send` because while its alive a `SetRoleResponseMut` cannot.
// - `SetRoleResponseView` does not use thread-local data.
unsafe impl Send for SetRoleResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for SetRoleResponseView<'msg> {
  type Proxied = SetRoleResponse;

  fn as_view(&self) -> ::__pb::View<'msg, SetRoleResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, SetRoleResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for SetRoleResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    SetRoleResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    SetRoleResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for SetRoleResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for SetRoleResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, SetRoleResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, SetRoleResponse>;

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

impl<'msg> ::__pb::SettableValue<SetRoleResponse> for SetRoleResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, SetRoleResponse>)
    where SetRoleResponse: 'dst {
    unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<SetRoleResponse> for SetRoleResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, SetRoleResponse>)
    where SetRoleResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for SetRoleResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__issuer_api_v1_SetRoleResponse_copy_from(
        __rust_proto_thunk__issuer_api_v1_SetRoleResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__issuer_api_v1_SetRoleResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__issuer_api_v1_SetRoleResponse_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__issuer_api_v1_SetRoleResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for SetRoleResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetRoleResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_SetRoleResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetRoleResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for SetRoleResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetRoleResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_SetRoleResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetRoleResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for SetRoleResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetRoleResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_SetRoleResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetRoleResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for SetRoleResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetRoleResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_SetRoleResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetRoleResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for SetRoleResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetRoleResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_SetRoleResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetRoleResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for SetRoleResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetRoleResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetRoleResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| SetRoleResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SetRoleResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for SetRoleResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> SetRoleResponseMut<'msg> {
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
// - `SetRoleResponseMut` does not perform any shared mutation.
// - `SetRoleResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for SetRoleResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for SetRoleResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, SetRoleResponse> {
    SetRoleResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, SetRoleResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for SetRoleResponseMut<'msg> {
  type Proxied = SetRoleResponse;
  fn as_view(&self) -> ::__pb::View<'_, SetRoleResponse> {
    SetRoleResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, SetRoleResponse> where 'msg: 'shorter {
    SetRoleResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl SetRoleResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleResponse_new() } } }
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

      __rust_proto_thunk__issuer_api_v1_SetRoleResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> SetRoleResponseView {
    SetRoleResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> SetRoleResponseMut {
    SetRoleResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

}  // impl SetRoleResponse

impl ::__std::ops::Drop for SetRoleResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_SetRoleResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__issuer_api_v1_SetRoleResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_SetRoleResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_SetRoleResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__issuer_api_v1_SetRoleResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__issuer_api_v1_SetRoleResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_SetRoleResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__issuer_api_v1_SetRoleResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_SetRoleResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_SetRoleResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_SetRoleResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__issuer_api_v1_SetRoleResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);


}  // extern "C" for SetRoleResponse


impl SetRoleResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> SetRoleResponseMut<'a> {
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

impl<'a> SetRoleResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct SetAssetAuthoritiesRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for SetAssetAuthoritiesRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for SetAssetAuthoritiesRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `SetAssetAuthoritiesRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `SetAssetAuthoritiesRequestMut`.
unsafe impl Sync for SetAssetAuthoritiesRequest {}

// SAFETY:
// - `SetAssetAuthoritiesRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SetAssetAuthoritiesRequest {}

impl ::__pb::Proxied for SetAssetAuthoritiesRequest {
  type View<'msg> = SetAssetAuthoritiesRequestView<'msg>;
  type Mut<'msg> = SetAssetAuthoritiesRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SetAssetAuthoritiesRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for SetAssetAuthoritiesRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> SetAssetAuthoritiesRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_serialize(self.raw_msg()) }
  }

  // authorities: repeated message issuer_api.v1.SetAssetAuthoritiesRequest.AuthoritiesEntry
  pub fn authorities(self)
    -> ::__pb::MapView<'msg, ::__pb::ProtoStr, crate::AssetAuthority> {
    unsafe {
      ::__pb::MapView::from_raw(::__pb::__internal::Private,
        __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_get_authorities(self.raw_msg()))
    }
  }

}

// SAFETY:
// - `SetAssetAuthoritiesRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for SetAssetAuthoritiesRequestView<'_> {}

// SAFETY:
// - `SetAssetAuthoritiesRequestView` is `Send` because while its alive a `SetAssetAuthoritiesRequestMut` cannot.
// - `SetAssetAuthoritiesRequestView` does not use thread-local data.
unsafe impl Send for SetAssetAuthoritiesRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for SetAssetAuthoritiesRequestView<'msg> {
  type Proxied = SetAssetAuthoritiesRequest;

  fn as_view(&self) -> ::__pb::View<'msg, SetAssetAuthoritiesRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, SetAssetAuthoritiesRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for SetAssetAuthoritiesRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    SetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    SetAssetAuthoritiesRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for SetAssetAuthoritiesRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for SetAssetAuthoritiesRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, SetAssetAuthoritiesRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, SetAssetAuthoritiesRequest>;

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

impl<'msg> ::__pb::SettableValue<SetAssetAuthoritiesRequest> for SetAssetAuthoritiesRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, SetAssetAuthoritiesRequest>)
    where SetAssetAuthoritiesRequest: 'dst {
    unsafe { __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<SetAssetAuthoritiesRequest> for SetAssetAuthoritiesRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, SetAssetAuthoritiesRequest>)
    where SetAssetAuthoritiesRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for SetAssetAuthoritiesRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_copy_from(
        __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for SetAssetAuthoritiesRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for SetAssetAuthoritiesRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for SetAssetAuthoritiesRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for SetAssetAuthoritiesRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for SetAssetAuthoritiesRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for SetAssetAuthoritiesRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| SetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SetAssetAuthoritiesRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for SetAssetAuthoritiesRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> SetAssetAuthoritiesRequestMut<'msg> {
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


  // authorities: repeated message issuer_api.v1.SetAssetAuthoritiesRequest.AuthoritiesEntry
  pub fn authorities(&self)
    -> ::__pb::MapView<'_, ::__pb::ProtoStr, crate::AssetAuthority> {
    unsafe {
      ::__pb::MapView::from_raw(::__pb::__internal::Private,
        __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_get_authorities(self.raw_msg()))
    }
  }
  pub fn authorities_mut(&mut self)
    -> ::__pb::MapMut<'_, ::__pb::ProtoStr, crate::AssetAuthority> {
    let inner = ::__pb::__runtime::InnerMapMut::new(::__pb::__internal::Private,
      unsafe { __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_get_mut_authorities(self.raw_msg()) });
    unsafe { ::__pb::MapMut::from_inner(::__pb::__internal::Private, inner) }
  }

}

// SAFETY:
// - `SetAssetAuthoritiesRequestMut` does not perform any shared mutation.
// - `SetAssetAuthoritiesRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for SetAssetAuthoritiesRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for SetAssetAuthoritiesRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, SetAssetAuthoritiesRequest> {
    SetAssetAuthoritiesRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, SetAssetAuthoritiesRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for SetAssetAuthoritiesRequestMut<'msg> {
  type Proxied = SetAssetAuthoritiesRequest;
  fn as_view(&self) -> ::__pb::View<'_, SetAssetAuthoritiesRequest> {
    SetAssetAuthoritiesRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, SetAssetAuthoritiesRequest> where 'msg: 'shorter {
    SetAssetAuthoritiesRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl SetAssetAuthoritiesRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_new() } } }
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

      __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> SetAssetAuthoritiesRequestView {
    SetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> SetAssetAuthoritiesRequestMut {
    SetAssetAuthoritiesRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // authorities: repeated message issuer_api.v1.SetAssetAuthoritiesRequest.AuthoritiesEntry
  pub fn authorities(&self)
    -> ::__pb::MapView<'_, ::__pb::ProtoStr, crate::AssetAuthority> {
    unsafe {
      ::__pb::MapView::from_raw(::__pb::__internal::Private,
        __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_get_authorities(self.raw_msg()))
    }
  }
  pub fn authorities_mut(&mut self)
    -> ::__pb::MapMut<'_, ::__pb::ProtoStr, crate::AssetAuthority> {
    let inner = ::__pb::__runtime::InnerMapMut::new(::__pb::__internal::Private,
      unsafe { __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_get_mut_authorities(self.raw_msg()) });
    unsafe { ::__pb::MapMut::from_inner(::__pb::__internal::Private, inner) }
  }

}  // impl SetAssetAuthoritiesRequest

impl ::__std::ops::Drop for SetAssetAuthoritiesRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_get_authorities(msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesRequest_get_mut_authorities(msg: ::__pb::__runtime::RawMessage,) -> ::__pb::__runtime::RawMap;


}  // extern "C" for SetAssetAuthoritiesRequest

pub mod set_asset_authorities_request {

}  // mod set_asset_authorities_request

impl SetAssetAuthoritiesRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> SetAssetAuthoritiesRequestMut<'a> {
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

impl<'a> SetAssetAuthoritiesRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct SetAssetAuthoritiesResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for SetAssetAuthoritiesResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for SetAssetAuthoritiesResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `SetAssetAuthoritiesResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `SetAssetAuthoritiesResponseMut`.
unsafe impl Sync for SetAssetAuthoritiesResponse {}

// SAFETY:
// - `SetAssetAuthoritiesResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SetAssetAuthoritiesResponse {}

impl ::__pb::Proxied for SetAssetAuthoritiesResponse {
  type View<'msg> = SetAssetAuthoritiesResponseView<'msg>;
  type Mut<'msg> = SetAssetAuthoritiesResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SetAssetAuthoritiesResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for SetAssetAuthoritiesResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> SetAssetAuthoritiesResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_serialize(self.raw_msg()) }
  }

}

// SAFETY:
// - `SetAssetAuthoritiesResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for SetAssetAuthoritiesResponseView<'_> {}

// SAFETY:
// - `SetAssetAuthoritiesResponseView` is `Send` because while its alive a `SetAssetAuthoritiesResponseMut` cannot.
// - `SetAssetAuthoritiesResponseView` does not use thread-local data.
unsafe impl Send for SetAssetAuthoritiesResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for SetAssetAuthoritiesResponseView<'msg> {
  type Proxied = SetAssetAuthoritiesResponse;

  fn as_view(&self) -> ::__pb::View<'msg, SetAssetAuthoritiesResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, SetAssetAuthoritiesResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for SetAssetAuthoritiesResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    SetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    SetAssetAuthoritiesResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for SetAssetAuthoritiesResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for SetAssetAuthoritiesResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, SetAssetAuthoritiesResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, SetAssetAuthoritiesResponse>;

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

impl<'msg> ::__pb::SettableValue<SetAssetAuthoritiesResponse> for SetAssetAuthoritiesResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, SetAssetAuthoritiesResponse>)
    where SetAssetAuthoritiesResponse: 'dst {
    unsafe { __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<SetAssetAuthoritiesResponse> for SetAssetAuthoritiesResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, SetAssetAuthoritiesResponse>)
    where SetAssetAuthoritiesResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for SetAssetAuthoritiesResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_copy_from(
        __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for SetAssetAuthoritiesResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_SetAssetAuthoritiesResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for SetAssetAuthoritiesResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_SetAssetAuthoritiesResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for SetAssetAuthoritiesResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_SetAssetAuthoritiesResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for SetAssetAuthoritiesResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_SetAssetAuthoritiesResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for SetAssetAuthoritiesResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_SetAssetAuthoritiesResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for SetAssetAuthoritiesResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_SetAssetAuthoritiesResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| SetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SetAssetAuthoritiesResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for SetAssetAuthoritiesResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> SetAssetAuthoritiesResponseMut<'msg> {
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
// - `SetAssetAuthoritiesResponseMut` does not perform any shared mutation.
// - `SetAssetAuthoritiesResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for SetAssetAuthoritiesResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for SetAssetAuthoritiesResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, SetAssetAuthoritiesResponse> {
    SetAssetAuthoritiesResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, SetAssetAuthoritiesResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for SetAssetAuthoritiesResponseMut<'msg> {
  type Proxied = SetAssetAuthoritiesResponse;
  fn as_view(&self) -> ::__pb::View<'_, SetAssetAuthoritiesResponse> {
    SetAssetAuthoritiesResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, SetAssetAuthoritiesResponse> where 'msg: 'shorter {
    SetAssetAuthoritiesResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl SetAssetAuthoritiesResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_new() } } }
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

      __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> SetAssetAuthoritiesResponseView {
    SetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> SetAssetAuthoritiesResponseMut {
    SetAssetAuthoritiesResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

}  // impl SetAssetAuthoritiesResponse

impl ::__std::ops::Drop for SetAssetAuthoritiesResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__issuer_api_v1_SetAssetAuthoritiesResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);


}  // extern "C" for SetAssetAuthoritiesResponse


impl SetAssetAuthoritiesResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> SetAssetAuthoritiesResponseMut<'a> {
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

impl<'a> SetAssetAuthoritiesResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct AddAssetAuthorityRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for AddAssetAuthorityRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for AddAssetAuthorityRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `AddAssetAuthorityRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `AddAssetAuthorityRequestMut`.
unsafe impl Sync for AddAssetAuthorityRequest {}

// SAFETY:
// - `AddAssetAuthorityRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for AddAssetAuthorityRequest {}

impl ::__pb::Proxied for AddAssetAuthorityRequest {
  type View<'msg> = AddAssetAuthorityRequestView<'msg>;
  type Mut<'msg> = AddAssetAuthorityRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AddAssetAuthorityRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for AddAssetAuthorityRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> AddAssetAuthorityRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_serialize(self.raw_msg()) }
  }

  // currency_code: optional string
  pub fn currency_code(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_get_currency_code(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // authority: optional message common.AssetAuthority
  pub fn authority(self) -> crate::AssetAuthorityView<'msg> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_get_authority(self.raw_msg()) };
    crate::AssetAuthorityView::new(::__pb::__internal::Private, submsg)
  }
  pub fn authority_opt(self) ->
  ::__pb::Optional<crate::AssetAuthorityView<'msg>> {
    let view = self.authority();
    ::__pb::Optional::new(view, self.has_authority())
  }
  pub fn has_authority(self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_has_authority(self.raw_msg()) }
  }

}

// SAFETY:
// - `AddAssetAuthorityRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for AddAssetAuthorityRequestView<'_> {}

// SAFETY:
// - `AddAssetAuthorityRequestView` is `Send` because while its alive a `AddAssetAuthorityRequestMut` cannot.
// - `AddAssetAuthorityRequestView` does not use thread-local data.
unsafe impl Send for AddAssetAuthorityRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for AddAssetAuthorityRequestView<'msg> {
  type Proxied = AddAssetAuthorityRequest;

  fn as_view(&self) -> ::__pb::View<'msg, AddAssetAuthorityRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, AddAssetAuthorityRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for AddAssetAuthorityRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    AddAssetAuthorityRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    AddAssetAuthorityRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for AddAssetAuthorityRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for AddAssetAuthorityRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, AddAssetAuthorityRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, AddAssetAuthorityRequest>;

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

impl<'msg> ::__pb::SettableValue<AddAssetAuthorityRequest> for AddAssetAuthorityRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, AddAssetAuthorityRequest>)
    where AddAssetAuthorityRequest: 'dst {
    unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<AddAssetAuthorityRequest> for AddAssetAuthorityRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, AddAssetAuthorityRequest>)
    where AddAssetAuthorityRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for AddAssetAuthorityRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_copy_from(
        __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for AddAssetAuthorityRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetAuthorityRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AddAssetAuthorityRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for AddAssetAuthorityRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetAuthorityRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AddAssetAuthorityRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for AddAssetAuthorityRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetAuthorityRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AddAssetAuthorityRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for AddAssetAuthorityRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetAuthorityRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AddAssetAuthorityRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for AddAssetAuthorityRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetAuthorityRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AddAssetAuthorityRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for AddAssetAuthorityRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetAuthorityRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| AddAssetAuthorityRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AddAssetAuthorityRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for AddAssetAuthorityRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> AddAssetAuthorityRequestMut<'msg> {
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
    let view = unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_get_currency_code(self.raw_msg()).as_ref() };
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
          AddAssetAuthorityRequest::__CURRENCY_CODE_VTABLE,
        )
      )
    }
  }

  // authority: optional message common.AssetAuthority
  pub fn authority(&self) -> crate::AssetAuthorityView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_get_authority(self.raw_msg()) };
    crate::AssetAuthorityView::new(::__pb::__internal::Private, submsg)
  }
  pub fn authority_mut(&mut self) -> crate::AssetAuthorityMut<'_> {
    self.authority_entry().or_default()
  }
  fn authority_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::AssetAuthority> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_get_authority,
                                __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_get_mut_authority,
                                __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_clear_authority);
    unsafe {
      let has = self.has_authority();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn authority_opt(&self) ->
  ::__pb::Optional<crate::AssetAuthorityView<'_>> {
    let view = self.authority();
    ::__pb::Optional::new(view, self.has_authority())
  }
  pub fn set_authority(&mut self, val: impl ::__pb::SettableValue<crate::AssetAuthority>) {
    self.authority_entry().set(val);
  }
  pub fn has_authority(&self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_has_authority(self.raw_msg()) }
  }
  pub fn clear_authority(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_clear_authority(self.raw_msg()) }
  }

}

// SAFETY:
// - `AddAssetAuthorityRequestMut` does not perform any shared mutation.
// - `AddAssetAuthorityRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for AddAssetAuthorityRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for AddAssetAuthorityRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, AddAssetAuthorityRequest> {
    AddAssetAuthorityRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, AddAssetAuthorityRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for AddAssetAuthorityRequestMut<'msg> {
  type Proxied = AddAssetAuthorityRequest;
  fn as_view(&self) -> ::__pb::View<'_, AddAssetAuthorityRequest> {
    AddAssetAuthorityRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, AddAssetAuthorityRequest> where 'msg: 'shorter {
    AddAssetAuthorityRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl AddAssetAuthorityRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_new() } } }
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

      __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> AddAssetAuthorityRequestView {
    AddAssetAuthorityRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> AddAssetAuthorityRequestMut {
    AddAssetAuthorityRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // currency_code: optional string
  pub fn currency_code(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_get_currency_code(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_currency_code(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.currency_code_mut().set(val);
  }
  const __CURRENCY_CODE_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_get_currency_code,
      __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_set_currency_code,
    );
  fn currency_code_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          AddAssetAuthorityRequest::__CURRENCY_CODE_VTABLE,
        )
      )
    }
  }

  // authority: optional message common.AssetAuthority
  pub fn authority(&self) -> crate::AssetAuthorityView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_get_authority(self.raw_msg()) };
    crate::AssetAuthorityView::new(::__pb::__internal::Private, submsg)
  }
  pub fn authority_mut(&mut self) -> crate::AssetAuthorityMut<'_> {
    self.authority_entry().or_default()
  }
  fn authority_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::AssetAuthority> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_get_authority,
                                __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_get_mut_authority,
                                __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_clear_authority);
    unsafe {
      let has = self.has_authority();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn authority_opt(&self) ->
  ::__pb::Optional<crate::AssetAuthorityView<'_>> {
    let view = self.authority();
    ::__pb::Optional::new(view, self.has_authority())
  }
  pub fn set_authority(&mut self, val: impl ::__pb::SettableValue<crate::AssetAuthority>) {
    self.authority_entry().set(val);
  }
  pub fn has_authority(&self) -> bool {
    unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_has_authority(self.raw_msg()) }
  }
  pub fn clear_authority(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_clear_authority(self.raw_msg()) }
  }

}  // impl AddAssetAuthorityRequest

impl ::__std::ops::Drop for AddAssetAuthorityRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_get_currency_code(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_set_currency_code(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_get_authority(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_get_mut_authority(raw_msg: ::__pb::__runtime::RawMessage)
     -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_clear_authority(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityRequest_has_authority(raw_msg: ::__pb::__runtime::RawMessage) -> bool;


}  // extern "C" for AddAssetAuthorityRequest


impl AddAssetAuthorityRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> AddAssetAuthorityRequestMut<'a> {
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

impl<'a> AddAssetAuthorityRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct AddAssetAuthorityResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for AddAssetAuthorityResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for AddAssetAuthorityResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `AddAssetAuthorityResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `AddAssetAuthorityResponseMut`.
unsafe impl Sync for AddAssetAuthorityResponse {}

// SAFETY:
// - `AddAssetAuthorityResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for AddAssetAuthorityResponse {}

impl ::__pb::Proxied for AddAssetAuthorityResponse {
  type View<'msg> = AddAssetAuthorityResponseView<'msg>;
  type Mut<'msg> = AddAssetAuthorityResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AddAssetAuthorityResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for AddAssetAuthorityResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> AddAssetAuthorityResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_serialize(self.raw_msg()) }
  }

}

// SAFETY:
// - `AddAssetAuthorityResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for AddAssetAuthorityResponseView<'_> {}

// SAFETY:
// - `AddAssetAuthorityResponseView` is `Send` because while its alive a `AddAssetAuthorityResponseMut` cannot.
// - `AddAssetAuthorityResponseView` does not use thread-local data.
unsafe impl Send for AddAssetAuthorityResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for AddAssetAuthorityResponseView<'msg> {
  type Proxied = AddAssetAuthorityResponse;

  fn as_view(&self) -> ::__pb::View<'msg, AddAssetAuthorityResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, AddAssetAuthorityResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for AddAssetAuthorityResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    AddAssetAuthorityResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    AddAssetAuthorityResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for AddAssetAuthorityResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for AddAssetAuthorityResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, AddAssetAuthorityResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, AddAssetAuthorityResponse>;

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

impl<'msg> ::__pb::SettableValue<AddAssetAuthorityResponse> for AddAssetAuthorityResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, AddAssetAuthorityResponse>)
    where AddAssetAuthorityResponse: 'dst {
    unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<AddAssetAuthorityResponse> for AddAssetAuthorityResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, AddAssetAuthorityResponse>)
    where AddAssetAuthorityResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for AddAssetAuthorityResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_copy_from(
        __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for AddAssetAuthorityResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetAuthorityResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_AddAssetAuthorityResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AddAssetAuthorityResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for AddAssetAuthorityResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetAuthorityResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_AddAssetAuthorityResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AddAssetAuthorityResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for AddAssetAuthorityResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetAuthorityResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_AddAssetAuthorityResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AddAssetAuthorityResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for AddAssetAuthorityResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetAuthorityResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_AddAssetAuthorityResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AddAssetAuthorityResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for AddAssetAuthorityResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetAuthorityResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_AddAssetAuthorityResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AddAssetAuthorityResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for AddAssetAuthorityResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AddAssetAuthorityResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_AddAssetAuthorityResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| AddAssetAuthorityResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AddAssetAuthorityResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for AddAssetAuthorityResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> AddAssetAuthorityResponseMut<'msg> {
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
// - `AddAssetAuthorityResponseMut` does not perform any shared mutation.
// - `AddAssetAuthorityResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for AddAssetAuthorityResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for AddAssetAuthorityResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, AddAssetAuthorityResponse> {
    AddAssetAuthorityResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, AddAssetAuthorityResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for AddAssetAuthorityResponseMut<'msg> {
  type Proxied = AddAssetAuthorityResponse;
  fn as_view(&self) -> ::__pb::View<'_, AddAssetAuthorityResponse> {
    AddAssetAuthorityResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, AddAssetAuthorityResponse> where 'msg: 'shorter {
    AddAssetAuthorityResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl AddAssetAuthorityResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_new() } } }
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

      __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> AddAssetAuthorityResponseView {
    AddAssetAuthorityResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> AddAssetAuthorityResponseMut {
    AddAssetAuthorityResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

}  // impl AddAssetAuthorityResponse

impl ::__std::ops::Drop for AddAssetAuthorityResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__issuer_api_v1_AddAssetAuthorityResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);


}  // extern "C" for AddAssetAuthorityResponse


impl AddAssetAuthorityResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> AddAssetAuthorityResponseMut<'a> {
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

impl<'a> AddAssetAuthorityResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct GetAssetAuthoritiesRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for GetAssetAuthoritiesRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for GetAssetAuthoritiesRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `GetAssetAuthoritiesRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `GetAssetAuthoritiesRequestMut`.
unsafe impl Sync for GetAssetAuthoritiesRequest {}

// SAFETY:
// - `GetAssetAuthoritiesRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetAssetAuthoritiesRequest {}

impl ::__pb::Proxied for GetAssetAuthoritiesRequest {
  type View<'msg> = GetAssetAuthoritiesRequestView<'msg>;
  type Mut<'msg> = GetAssetAuthoritiesRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetAssetAuthoritiesRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for GetAssetAuthoritiesRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> GetAssetAuthoritiesRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_serialize(self.raw_msg()) }
  }

}

// SAFETY:
// - `GetAssetAuthoritiesRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetAssetAuthoritiesRequestView<'_> {}

// SAFETY:
// - `GetAssetAuthoritiesRequestView` is `Send` because while its alive a `GetAssetAuthoritiesRequestMut` cannot.
// - `GetAssetAuthoritiesRequestView` does not use thread-local data.
unsafe impl Send for GetAssetAuthoritiesRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for GetAssetAuthoritiesRequestView<'msg> {
  type Proxied = GetAssetAuthoritiesRequest;

  fn as_view(&self) -> ::__pb::View<'msg, GetAssetAuthoritiesRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, GetAssetAuthoritiesRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for GetAssetAuthoritiesRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    GetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    GetAssetAuthoritiesRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for GetAssetAuthoritiesRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for GetAssetAuthoritiesRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, GetAssetAuthoritiesRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, GetAssetAuthoritiesRequest>;

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

impl<'msg> ::__pb::SettableValue<GetAssetAuthoritiesRequest> for GetAssetAuthoritiesRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, GetAssetAuthoritiesRequest>)
    where GetAssetAuthoritiesRequest: 'dst {
    unsafe { __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<GetAssetAuthoritiesRequest> for GetAssetAuthoritiesRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, GetAssetAuthoritiesRequest>)
    where GetAssetAuthoritiesRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for GetAssetAuthoritiesRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_copy_from(
        __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for GetAssetAuthoritiesRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for GetAssetAuthoritiesRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for GetAssetAuthoritiesRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for GetAssetAuthoritiesRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for GetAssetAuthoritiesRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for GetAssetAuthoritiesRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| GetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetAssetAuthoritiesRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for GetAssetAuthoritiesRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> GetAssetAuthoritiesRequestMut<'msg> {
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
// - `GetAssetAuthoritiesRequestMut` does not perform any shared mutation.
// - `GetAssetAuthoritiesRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetAssetAuthoritiesRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for GetAssetAuthoritiesRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, GetAssetAuthoritiesRequest> {
    GetAssetAuthoritiesRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, GetAssetAuthoritiesRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for GetAssetAuthoritiesRequestMut<'msg> {
  type Proxied = GetAssetAuthoritiesRequest;
  fn as_view(&self) -> ::__pb::View<'_, GetAssetAuthoritiesRequest> {
    GetAssetAuthoritiesRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, GetAssetAuthoritiesRequest> where 'msg: 'shorter {
    GetAssetAuthoritiesRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl GetAssetAuthoritiesRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_new() } } }
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

      __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> GetAssetAuthoritiesRequestView {
    GetAssetAuthoritiesRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetAssetAuthoritiesRequestMut {
    GetAssetAuthoritiesRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

}  // impl GetAssetAuthoritiesRequest

impl ::__std::ops::Drop for GetAssetAuthoritiesRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);


}  // extern "C" for GetAssetAuthoritiesRequest


impl GetAssetAuthoritiesRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> GetAssetAuthoritiesRequestMut<'a> {
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

impl<'a> GetAssetAuthoritiesRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct GetAssetAuthoritiesResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for GetAssetAuthoritiesResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for GetAssetAuthoritiesResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `GetAssetAuthoritiesResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `GetAssetAuthoritiesResponseMut`.
unsafe impl Sync for GetAssetAuthoritiesResponse {}

// SAFETY:
// - `GetAssetAuthoritiesResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for GetAssetAuthoritiesResponse {}

impl ::__pb::Proxied for GetAssetAuthoritiesResponse {
  type View<'msg> = GetAssetAuthoritiesResponseView<'msg>;
  type Mut<'msg> = GetAssetAuthoritiesResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GetAssetAuthoritiesResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for GetAssetAuthoritiesResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> GetAssetAuthoritiesResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_serialize(self.raw_msg()) }
  }

  // authorities: repeated message issuer_api.v1.GetAssetAuthoritiesResponse.AuthoritiesEntry
  pub fn authorities(self)
    -> ::__pb::MapView<'msg, ::__pb::ProtoStr, crate::AssetAuthority> {
    unsafe {
      ::__pb::MapView::from_raw(::__pb::__internal::Private,
        __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_get_authorities(self.raw_msg()))
    }
  }

}

// SAFETY:
// - `GetAssetAuthoritiesResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for GetAssetAuthoritiesResponseView<'_> {}

// SAFETY:
// - `GetAssetAuthoritiesResponseView` is `Send` because while its alive a `GetAssetAuthoritiesResponseMut` cannot.
// - `GetAssetAuthoritiesResponseView` does not use thread-local data.
unsafe impl Send for GetAssetAuthoritiesResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for GetAssetAuthoritiesResponseView<'msg> {
  type Proxied = GetAssetAuthoritiesResponse;

  fn as_view(&self) -> ::__pb::View<'msg, GetAssetAuthoritiesResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, GetAssetAuthoritiesResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for GetAssetAuthoritiesResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    GetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    GetAssetAuthoritiesResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for GetAssetAuthoritiesResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for GetAssetAuthoritiesResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, GetAssetAuthoritiesResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, GetAssetAuthoritiesResponse>;

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

impl<'msg> ::__pb::SettableValue<GetAssetAuthoritiesResponse> for GetAssetAuthoritiesResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, GetAssetAuthoritiesResponse>)
    where GetAssetAuthoritiesResponse: 'dst {
    unsafe { __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<GetAssetAuthoritiesResponse> for GetAssetAuthoritiesResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, GetAssetAuthoritiesResponse>)
    where GetAssetAuthoritiesResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for GetAssetAuthoritiesResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_copy_from(
        __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for GetAssetAuthoritiesResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_issuer_api_v1_GetAssetAuthoritiesResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for GetAssetAuthoritiesResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_issuer_api_v1_GetAssetAuthoritiesResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for GetAssetAuthoritiesResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_issuer_api_v1_GetAssetAuthoritiesResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for GetAssetAuthoritiesResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_issuer_api_v1_GetAssetAuthoritiesResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for GetAssetAuthoritiesResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_issuer_api_v1_GetAssetAuthoritiesResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| GetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for GetAssetAuthoritiesResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(GetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_issuer_api_v1_GetAssetAuthoritiesResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| GetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GetAssetAuthoritiesResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for GetAssetAuthoritiesResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> GetAssetAuthoritiesResponseMut<'msg> {
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


  // authorities: repeated message issuer_api.v1.GetAssetAuthoritiesResponse.AuthoritiesEntry
  pub fn authorities(&self)
    -> ::__pb::MapView<'_, ::__pb::ProtoStr, crate::AssetAuthority> {
    unsafe {
      ::__pb::MapView::from_raw(::__pb::__internal::Private,
        __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_get_authorities(self.raw_msg()))
    }
  }
  pub fn authorities_mut(&mut self)
    -> ::__pb::MapMut<'_, ::__pb::ProtoStr, crate::AssetAuthority> {
    let inner = ::__pb::__runtime::InnerMapMut::new(::__pb::__internal::Private,
      unsafe { __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_get_mut_authorities(self.raw_msg()) });
    unsafe { ::__pb::MapMut::from_inner(::__pb::__internal::Private, inner) }
  }

}

// SAFETY:
// - `GetAssetAuthoritiesResponseMut` does not perform any shared mutation.
// - `GetAssetAuthoritiesResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for GetAssetAuthoritiesResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for GetAssetAuthoritiesResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, GetAssetAuthoritiesResponse> {
    GetAssetAuthoritiesResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, GetAssetAuthoritiesResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for GetAssetAuthoritiesResponseMut<'msg> {
  type Proxied = GetAssetAuthoritiesResponse;
  fn as_view(&self) -> ::__pb::View<'_, GetAssetAuthoritiesResponse> {
    GetAssetAuthoritiesResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, GetAssetAuthoritiesResponse> where 'msg: 'shorter {
    GetAssetAuthoritiesResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl GetAssetAuthoritiesResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_new() } } }
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

      __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> GetAssetAuthoritiesResponseView {
    GetAssetAuthoritiesResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> GetAssetAuthoritiesResponseMut {
    GetAssetAuthoritiesResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // authorities: repeated message issuer_api.v1.GetAssetAuthoritiesResponse.AuthoritiesEntry
  pub fn authorities(&self)
    -> ::__pb::MapView<'_, ::__pb::ProtoStr, crate::AssetAuthority> {
    unsafe {
      ::__pb::MapView::from_raw(::__pb::__internal::Private,
        __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_get_authorities(self.raw_msg()))
    }
  }
  pub fn authorities_mut(&mut self)
    -> ::__pb::MapMut<'_, ::__pb::ProtoStr, crate::AssetAuthority> {
    let inner = ::__pb::__runtime::InnerMapMut::new(::__pb::__internal::Private,
      unsafe { __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_get_mut_authorities(self.raw_msg()) });
    unsafe { ::__pb::MapMut::from_inner(::__pb::__internal::Private, inner) }
  }

}  // impl GetAssetAuthoritiesResponse

impl ::__std::ops::Drop for GetAssetAuthoritiesResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_get_authorities(msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__issuer_api_v1_GetAssetAuthoritiesResponse_get_mut_authorities(msg: ::__pb::__runtime::RawMessage,) -> ::__pb::__runtime::RawMap;


}  // extern "C" for GetAssetAuthoritiesResponse

pub mod get_asset_authorities_response {

}  // mod get_asset_authorities_response

impl GetAssetAuthoritiesResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> GetAssetAuthoritiesResponseMut<'a> {
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

impl<'a> GetAssetAuthoritiesResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

