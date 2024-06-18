extern crate protobuf_cpp as __pb;
extern crate std as __std;
#[allow(non_camel_case_types)]
pub struct CredentialProof {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for CredentialProof {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for CredentialProof {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `CredentialProof` is `Sync` because it does not implement interior mutability.
//    Neither does `CredentialProofMut`.
unsafe impl Sync for CredentialProof {}

// SAFETY:
// - `CredentialProof` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for CredentialProof {}

impl ::__pb::Proxied for CredentialProof {
  type View<'msg> = CredentialProofView<'msg>;
  type Mut<'msg> = CredentialProofMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CredentialProofView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for CredentialProofView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> CredentialProofView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_serialize(self.raw_msg()) }
  }

  // type: optional string
  pub fn r#type(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_get_type(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // created: optional string
  pub fn created(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_get_created(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // verification_method: optional string
  pub fn verification_method(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_get_verification_method(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // proof_purpose: optional string
  pub fn proof_purpose(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_get_proof_purpose(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // proof_value: optional string
  pub fn proof_value(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_get_proof_value(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

}

// SAFETY:
// - `CredentialProofView` is `Sync` because it does not support mutation.
unsafe impl Sync for CredentialProofView<'_> {}

// SAFETY:
// - `CredentialProofView` is `Send` because while its alive a `CredentialProofMut` cannot.
// - `CredentialProofView` does not use thread-local data.
unsafe impl Send for CredentialProofView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for CredentialProofView<'msg> {
  type Proxied = CredentialProof;

  fn as_view(&self) -> ::__pb::View<'msg, CredentialProof> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, CredentialProof> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for CredentialProof {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    CredentialProofView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    CredentialProofMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for CredentialProof {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for CredentialProof {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, CredentialProof>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, CredentialProof>;

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

impl<'msg> ::__pb::SettableValue<CredentialProof> for CredentialProofView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, CredentialProof>)
    where CredentialProof: 'dst {
    unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<CredentialProof> for CredentialProof {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, CredentialProof>)
    where CredentialProof: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for CredentialProof {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__vc_api_v1_CredentialProof_copy_from(
        __rust_proto_thunk__vc_api_v1_CredentialProof_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__vc_api_v1_CredentialProof_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__vc_api_v1_CredentialProof_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__vc_api_v1_CredentialProof_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for CredentialProof {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CredentialProofView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_vc_api_v1_CredentialProof_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CredentialProofView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for CredentialProof {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CredentialProofView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_vc_api_v1_CredentialProof_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CredentialProofView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for CredentialProof {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CredentialProofView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_vc_api_v1_CredentialProof_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CredentialProofView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for CredentialProof {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CredentialProofView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_vc_api_v1_CredentialProof_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CredentialProofView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for CredentialProof {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CredentialProofView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_vc_api_v1_CredentialProof_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CredentialProofView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for CredentialProof {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CredentialProofView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialProof_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| CredentialProofView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CredentialProofMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for CredentialProofMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> CredentialProofMut<'msg> {
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


  // type: optional string
  pub fn r#type(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_get_type(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_type(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.type_mut().set(val);
  }
  fn type_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CredentialProof::__TYPE_VTABLE,
        )
      )
    }
  }

  // created: optional string
  pub fn created(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_get_created(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_created(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.created_mut().set(val);
  }
  fn created_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CredentialProof::__CREATED_VTABLE,
        )
      )
    }
  }

  // verification_method: optional string
  pub fn verification_method(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_get_verification_method(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_verification_method(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.verification_method_mut().set(val);
  }
  fn verification_method_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CredentialProof::__VERIFICATION_METHOD_VTABLE,
        )
      )
    }
  }

  // proof_purpose: optional string
  pub fn proof_purpose(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_get_proof_purpose(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_proof_purpose(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.proof_purpose_mut().set(val);
  }
  fn proof_purpose_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CredentialProof::__PROOF_PURPOSE_VTABLE,
        )
      )
    }
  }

  // proof_value: optional string
  pub fn proof_value(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_get_proof_value(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_proof_value(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.proof_value_mut().set(val);
  }
  fn proof_value_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CredentialProof::__PROOF_VALUE_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `CredentialProofMut` does not perform any shared mutation.
// - `CredentialProofMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for CredentialProofMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for CredentialProofMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, CredentialProof> {
    CredentialProofMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, CredentialProof> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for CredentialProofMut<'msg> {
  type Proxied = CredentialProof;
  fn as_view(&self) -> ::__pb::View<'_, CredentialProof> {
    CredentialProofView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, CredentialProof> where 'msg: 'shorter {
    CredentialProofView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl CredentialProof {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_new() } } }
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

      __rust_proto_thunk__vc_api_v1_CredentialProof_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> CredentialProofView {
    CredentialProofView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> CredentialProofMut {
    CredentialProofMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // type: optional string
  pub fn r#type(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_get_type(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_type(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.type_mut().set(val);
  }
  const __TYPE_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_CredentialProof_get_type,
      __rust_proto_thunk__vc_api_v1_CredentialProof_set_type,
    );
  fn type_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CredentialProof::__TYPE_VTABLE,
        )
      )
    }
  }

  // created: optional string
  pub fn created(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_get_created(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_created(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.created_mut().set(val);
  }
  const __CREATED_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_CredentialProof_get_created,
      __rust_proto_thunk__vc_api_v1_CredentialProof_set_created,
    );
  fn created_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CredentialProof::__CREATED_VTABLE,
        )
      )
    }
  }

  // verification_method: optional string
  pub fn verification_method(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_get_verification_method(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_verification_method(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.verification_method_mut().set(val);
  }
  const __VERIFICATION_METHOD_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_CredentialProof_get_verification_method,
      __rust_proto_thunk__vc_api_v1_CredentialProof_set_verification_method,
    );
  fn verification_method_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CredentialProof::__VERIFICATION_METHOD_VTABLE,
        )
      )
    }
  }

  // proof_purpose: optional string
  pub fn proof_purpose(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_get_proof_purpose(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_proof_purpose(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.proof_purpose_mut().set(val);
  }
  const __PROOF_PURPOSE_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_CredentialProof_get_proof_purpose,
      __rust_proto_thunk__vc_api_v1_CredentialProof_set_proof_purpose,
    );
  fn proof_purpose_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CredentialProof::__PROOF_PURPOSE_VTABLE,
        )
      )
    }
  }

  // proof_value: optional string
  pub fn proof_value(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_get_proof_value(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_proof_value(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.proof_value_mut().set(val);
  }
  const __PROOF_VALUE_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_CredentialProof_get_proof_value,
      __rust_proto_thunk__vc_api_v1_CredentialProof_set_proof_value,
    );
  fn proof_value_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CredentialProof::__PROOF_VALUE_VTABLE,
        )
      )
    }
  }

}  // impl CredentialProof

impl ::__std::ops::Drop for CredentialProof {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__vc_api_v1_CredentialProof_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__vc_api_v1_CredentialProof_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_CredentialProof_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_CredentialProof_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__vc_api_v1_CredentialProof_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__vc_api_v1_CredentialProof_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_CredentialProof_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__vc_api_v1_CredentialProof_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_CredentialProof_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_CredentialProof_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_CredentialProof_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__vc_api_v1_CredentialProof_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__vc_api_v1_CredentialProof_get_type(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_CredentialProof_set_type(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__vc_api_v1_CredentialProof_get_created(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_CredentialProof_set_created(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__vc_api_v1_CredentialProof_get_verification_method(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_CredentialProof_set_verification_method(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__vc_api_v1_CredentialProof_get_proof_purpose(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_CredentialProof_set_proof_purpose(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__vc_api_v1_CredentialProof_get_proof_value(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_CredentialProof_set_proof_value(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for CredentialProof


impl CredentialProof {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> CredentialProofMut<'a> {
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

impl<'a> CredentialProofView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct CreateIssuanceChallengeRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for CreateIssuanceChallengeRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for CreateIssuanceChallengeRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `CreateIssuanceChallengeRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `CreateIssuanceChallengeRequestMut`.
unsafe impl Sync for CreateIssuanceChallengeRequest {}

// SAFETY:
// - `CreateIssuanceChallengeRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for CreateIssuanceChallengeRequest {}

impl ::__pb::Proxied for CreateIssuanceChallengeRequest {
  type View<'msg> = CreateIssuanceChallengeRequestView<'msg>;
  type Mut<'msg> = CreateIssuanceChallengeRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CreateIssuanceChallengeRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for CreateIssuanceChallengeRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> CreateIssuanceChallengeRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_serialize(self.raw_msg()) }
  }

  // credential_type: optional enum vc_api.v1.CredentialType
  pub fn credential_type(self) -> crate::CredentialType {
    unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_get_credential_type(self.raw_msg()) }
  }

  // did: optional string
  pub fn did(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_get_did(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

}

// SAFETY:
// - `CreateIssuanceChallengeRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for CreateIssuanceChallengeRequestView<'_> {}

// SAFETY:
// - `CreateIssuanceChallengeRequestView` is `Send` because while its alive a `CreateIssuanceChallengeRequestMut` cannot.
// - `CreateIssuanceChallengeRequestView` does not use thread-local data.
unsafe impl Send for CreateIssuanceChallengeRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for CreateIssuanceChallengeRequestView<'msg> {
  type Proxied = CreateIssuanceChallengeRequest;

  fn as_view(&self) -> ::__pb::View<'msg, CreateIssuanceChallengeRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, CreateIssuanceChallengeRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for CreateIssuanceChallengeRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    CreateIssuanceChallengeRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    CreateIssuanceChallengeRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for CreateIssuanceChallengeRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for CreateIssuanceChallengeRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, CreateIssuanceChallengeRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, CreateIssuanceChallengeRequest>;

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

impl<'msg> ::__pb::SettableValue<CreateIssuanceChallengeRequest> for CreateIssuanceChallengeRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, CreateIssuanceChallengeRequest>)
    where CreateIssuanceChallengeRequest: 'dst {
    unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<CreateIssuanceChallengeRequest> for CreateIssuanceChallengeRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, CreateIssuanceChallengeRequest>)
    where CreateIssuanceChallengeRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for CreateIssuanceChallengeRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_copy_from(
        __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for CreateIssuanceChallengeRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreateIssuanceChallengeRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CreateIssuanceChallengeRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for CreateIssuanceChallengeRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreateIssuanceChallengeRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CreateIssuanceChallengeRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for CreateIssuanceChallengeRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreateIssuanceChallengeRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CreateIssuanceChallengeRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for CreateIssuanceChallengeRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreateIssuanceChallengeRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CreateIssuanceChallengeRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for CreateIssuanceChallengeRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreateIssuanceChallengeRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CreateIssuanceChallengeRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for CreateIssuanceChallengeRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreateIssuanceChallengeRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| CreateIssuanceChallengeRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CreateIssuanceChallengeRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for CreateIssuanceChallengeRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> CreateIssuanceChallengeRequestMut<'msg> {
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


  // credential_type: optional enum vc_api.v1.CredentialType
  pub fn credential_type(&self) -> crate::CredentialType {
    unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_get_credential_type(self.raw_msg()) }
  }
  pub fn set_credential_type(&mut self, val: crate::CredentialType) {
    unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_set_credential_type(self.raw_msg(), val) }
  }

  // did: optional string
  pub fn did(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_get_did(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_did(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.did_mut().set(val);
  }
  fn did_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CreateIssuanceChallengeRequest::__DID_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `CreateIssuanceChallengeRequestMut` does not perform any shared mutation.
// - `CreateIssuanceChallengeRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for CreateIssuanceChallengeRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for CreateIssuanceChallengeRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, CreateIssuanceChallengeRequest> {
    CreateIssuanceChallengeRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, CreateIssuanceChallengeRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for CreateIssuanceChallengeRequestMut<'msg> {
  type Proxied = CreateIssuanceChallengeRequest;
  fn as_view(&self) -> ::__pb::View<'_, CreateIssuanceChallengeRequest> {
    CreateIssuanceChallengeRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, CreateIssuanceChallengeRequest> where 'msg: 'shorter {
    CreateIssuanceChallengeRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl CreateIssuanceChallengeRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_new() } } }
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

      __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> CreateIssuanceChallengeRequestView {
    CreateIssuanceChallengeRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> CreateIssuanceChallengeRequestMut {
    CreateIssuanceChallengeRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // credential_type: optional enum vc_api.v1.CredentialType
  pub fn credential_type(&self) -> crate::CredentialType {
    unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_get_credential_type(self.raw_msg()) }
  }
  pub fn set_credential_type(&mut self, val: crate::CredentialType) {
    unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_set_credential_type(self.raw_msg(), val) }
  }

  // did: optional string
  pub fn did(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_get_did(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_did(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.did_mut().set(val);
  }
  const __DID_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_get_did,
      __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_set_did,
    );
  fn did_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CreateIssuanceChallengeRequest::__DID_VTABLE,
        )
      )
    }
  }

}  // impl CreateIssuanceChallengeRequest

impl ::__std::ops::Drop for CreateIssuanceChallengeRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_get_credential_type(raw_msg: ::__pb::__runtime::RawMessage) -> crate::CredentialType;
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_set_credential_type(raw_msg: ::__pb::__runtime::RawMessage, val: crate::CredentialType);

  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_get_did(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeRequest_set_did(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for CreateIssuanceChallengeRequest


impl CreateIssuanceChallengeRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> CreateIssuanceChallengeRequestMut<'a> {
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

impl<'a> CreateIssuanceChallengeRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct CreateIssuanceChallengeResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for CreateIssuanceChallengeResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for CreateIssuanceChallengeResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `CreateIssuanceChallengeResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `CreateIssuanceChallengeResponseMut`.
unsafe impl Sync for CreateIssuanceChallengeResponse {}

// SAFETY:
// - `CreateIssuanceChallengeResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for CreateIssuanceChallengeResponse {}

impl ::__pb::Proxied for CreateIssuanceChallengeResponse {
  type View<'msg> = CreateIssuanceChallengeResponseView<'msg>;
  type Mut<'msg> = CreateIssuanceChallengeResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CreateIssuanceChallengeResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for CreateIssuanceChallengeResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> CreateIssuanceChallengeResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_serialize(self.raw_msg()) }
  }

  // nonce: optional string
  pub fn nonce(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_get_nonce(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // endpoint: optional string
  pub fn endpoint(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_get_endpoint(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // credential_type: optional enum vc_api.v1.CredentialType
  pub fn credential_type(self) -> crate::CredentialType {
    unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_get_credential_type(self.raw_msg()) }
  }

}

// SAFETY:
// - `CreateIssuanceChallengeResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for CreateIssuanceChallengeResponseView<'_> {}

// SAFETY:
// - `CreateIssuanceChallengeResponseView` is `Send` because while its alive a `CreateIssuanceChallengeResponseMut` cannot.
// - `CreateIssuanceChallengeResponseView` does not use thread-local data.
unsafe impl Send for CreateIssuanceChallengeResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for CreateIssuanceChallengeResponseView<'msg> {
  type Proxied = CreateIssuanceChallengeResponse;

  fn as_view(&self) -> ::__pb::View<'msg, CreateIssuanceChallengeResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, CreateIssuanceChallengeResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for CreateIssuanceChallengeResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    CreateIssuanceChallengeResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    CreateIssuanceChallengeResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for CreateIssuanceChallengeResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for CreateIssuanceChallengeResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, CreateIssuanceChallengeResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, CreateIssuanceChallengeResponse>;

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

impl<'msg> ::__pb::SettableValue<CreateIssuanceChallengeResponse> for CreateIssuanceChallengeResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, CreateIssuanceChallengeResponse>)
    where CreateIssuanceChallengeResponse: 'dst {
    unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<CreateIssuanceChallengeResponse> for CreateIssuanceChallengeResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, CreateIssuanceChallengeResponse>)
    where CreateIssuanceChallengeResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for CreateIssuanceChallengeResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_copy_from(
        __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for CreateIssuanceChallengeResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreateIssuanceChallengeResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_vc_api_v1_CreateIssuanceChallengeResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CreateIssuanceChallengeResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for CreateIssuanceChallengeResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreateIssuanceChallengeResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_vc_api_v1_CreateIssuanceChallengeResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CreateIssuanceChallengeResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for CreateIssuanceChallengeResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreateIssuanceChallengeResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_vc_api_v1_CreateIssuanceChallengeResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CreateIssuanceChallengeResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for CreateIssuanceChallengeResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreateIssuanceChallengeResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_vc_api_v1_CreateIssuanceChallengeResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CreateIssuanceChallengeResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for CreateIssuanceChallengeResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreateIssuanceChallengeResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_vc_api_v1_CreateIssuanceChallengeResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CreateIssuanceChallengeResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for CreateIssuanceChallengeResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreateIssuanceChallengeResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreateIssuanceChallengeResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| CreateIssuanceChallengeResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CreateIssuanceChallengeResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for CreateIssuanceChallengeResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> CreateIssuanceChallengeResponseMut<'msg> {
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


  // nonce: optional string
  pub fn nonce(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_get_nonce(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_nonce(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.nonce_mut().set(val);
  }
  fn nonce_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CreateIssuanceChallengeResponse::__NONCE_VTABLE,
        )
      )
    }
  }

  // endpoint: optional string
  pub fn endpoint(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_get_endpoint(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_endpoint(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.endpoint_mut().set(val);
  }
  fn endpoint_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CreateIssuanceChallengeResponse::__ENDPOINT_VTABLE,
        )
      )
    }
  }

  // credential_type: optional enum vc_api.v1.CredentialType
  pub fn credential_type(&self) -> crate::CredentialType {
    unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_get_credential_type(self.raw_msg()) }
  }
  pub fn set_credential_type(&mut self, val: crate::CredentialType) {
    unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_set_credential_type(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `CreateIssuanceChallengeResponseMut` does not perform any shared mutation.
// - `CreateIssuanceChallengeResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for CreateIssuanceChallengeResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for CreateIssuanceChallengeResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, CreateIssuanceChallengeResponse> {
    CreateIssuanceChallengeResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, CreateIssuanceChallengeResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for CreateIssuanceChallengeResponseMut<'msg> {
  type Proxied = CreateIssuanceChallengeResponse;
  fn as_view(&self) -> ::__pb::View<'_, CreateIssuanceChallengeResponse> {
    CreateIssuanceChallengeResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, CreateIssuanceChallengeResponse> where 'msg: 'shorter {
    CreateIssuanceChallengeResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl CreateIssuanceChallengeResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_new() } } }
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

      __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> CreateIssuanceChallengeResponseView {
    CreateIssuanceChallengeResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> CreateIssuanceChallengeResponseMut {
    CreateIssuanceChallengeResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // nonce: optional string
  pub fn nonce(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_get_nonce(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_nonce(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.nonce_mut().set(val);
  }
  const __NONCE_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_get_nonce,
      __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_set_nonce,
    );
  fn nonce_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CreateIssuanceChallengeResponse::__NONCE_VTABLE,
        )
      )
    }
  }

  // endpoint: optional string
  pub fn endpoint(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_get_endpoint(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_endpoint(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.endpoint_mut().set(val);
  }
  const __ENDPOINT_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_get_endpoint,
      __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_set_endpoint,
    );
  fn endpoint_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CreateIssuanceChallengeResponse::__ENDPOINT_VTABLE,
        )
      )
    }
  }

  // credential_type: optional enum vc_api.v1.CredentialType
  pub fn credential_type(&self) -> crate::CredentialType {
    unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_get_credential_type(self.raw_msg()) }
  }
  pub fn set_credential_type(&mut self, val: crate::CredentialType) {
    unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_set_credential_type(self.raw_msg(), val) }
  }

}  // impl CreateIssuanceChallengeResponse

impl ::__std::ops::Drop for CreateIssuanceChallengeResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_get_nonce(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_set_nonce(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_get_endpoint(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_set_endpoint(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_get_credential_type(raw_msg: ::__pb::__runtime::RawMessage) -> crate::CredentialType;
  fn __rust_proto_thunk__vc_api_v1_CreateIssuanceChallengeResponse_set_credential_type(raw_msg: ::__pb::__runtime::RawMessage, val: crate::CredentialType);


}  // extern "C" for CreateIssuanceChallengeResponse


impl CreateIssuanceChallengeResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> CreateIssuanceChallengeResponseMut<'a> {
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

impl<'a> CreateIssuanceChallengeResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct IssueVerifiableCredentialRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for IssueVerifiableCredentialRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for IssueVerifiableCredentialRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `IssueVerifiableCredentialRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `IssueVerifiableCredentialRequestMut`.
unsafe impl Sync for IssueVerifiableCredentialRequest {}

// SAFETY:
// - `IssueVerifiableCredentialRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for IssueVerifiableCredentialRequest {}

impl ::__pb::Proxied for IssueVerifiableCredentialRequest {
  type View<'msg> = IssueVerifiableCredentialRequestView<'msg>;
  type Mut<'msg> = IssueVerifiableCredentialRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct IssueVerifiableCredentialRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for IssueVerifiableCredentialRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> IssueVerifiableCredentialRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_serialize(self.raw_msg()) }
  }

  // credential_type: optional enum vc_api.v1.CredentialType
  pub fn credential_type(self) -> crate::CredentialType {
    unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_credential_type(self.raw_msg()) }
  }

  // nonce: optional string
  pub fn nonce(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_nonce(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // signature: optional bytes
  pub fn signature(self) -> &'msg [u8] {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_signature(self.raw_msg()).as_ref() };
    view
  }

  // did: optional string
  pub fn did(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_did(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

}

// SAFETY:
// - `IssueVerifiableCredentialRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for IssueVerifiableCredentialRequestView<'_> {}

// SAFETY:
// - `IssueVerifiableCredentialRequestView` is `Send` because while its alive a `IssueVerifiableCredentialRequestMut` cannot.
// - `IssueVerifiableCredentialRequestView` does not use thread-local data.
unsafe impl Send for IssueVerifiableCredentialRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for IssueVerifiableCredentialRequestView<'msg> {
  type Proxied = IssueVerifiableCredentialRequest;

  fn as_view(&self) -> ::__pb::View<'msg, IssueVerifiableCredentialRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, IssueVerifiableCredentialRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for IssueVerifiableCredentialRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    IssueVerifiableCredentialRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    IssueVerifiableCredentialRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for IssueVerifiableCredentialRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for IssueVerifiableCredentialRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, IssueVerifiableCredentialRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, IssueVerifiableCredentialRequest>;

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

impl<'msg> ::__pb::SettableValue<IssueVerifiableCredentialRequest> for IssueVerifiableCredentialRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, IssueVerifiableCredentialRequest>)
    where IssueVerifiableCredentialRequest: 'dst {
    unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<IssueVerifiableCredentialRequest> for IssueVerifiableCredentialRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, IssueVerifiableCredentialRequest>)
    where IssueVerifiableCredentialRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for IssueVerifiableCredentialRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_copy_from(
        __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for IssueVerifiableCredentialRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueVerifiableCredentialRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssueVerifiableCredentialRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for IssueVerifiableCredentialRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueVerifiableCredentialRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssueVerifiableCredentialRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for IssueVerifiableCredentialRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueVerifiableCredentialRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssueVerifiableCredentialRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for IssueVerifiableCredentialRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueVerifiableCredentialRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssueVerifiableCredentialRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for IssueVerifiableCredentialRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueVerifiableCredentialRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssueVerifiableCredentialRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for IssueVerifiableCredentialRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueVerifiableCredentialRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| IssueVerifiableCredentialRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct IssueVerifiableCredentialRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for IssueVerifiableCredentialRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> IssueVerifiableCredentialRequestMut<'msg> {
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


  // credential_type: optional enum vc_api.v1.CredentialType
  pub fn credential_type(&self) -> crate::CredentialType {
    unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_credential_type(self.raw_msg()) }
  }
  pub fn set_credential_type(&mut self, val: crate::CredentialType) {
    unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_set_credential_type(self.raw_msg(), val) }
  }

  // nonce: optional string
  pub fn nonce(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_nonce(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_nonce(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.nonce_mut().set(val);
  }
  fn nonce_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          IssueVerifiableCredentialRequest::__NONCE_VTABLE,
        )
      )
    }
  }

  // signature: optional bytes
  pub fn signature(&self) -> &'_ [u8] {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_signature(self.raw_msg()).as_ref() };
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
          IssueVerifiableCredentialRequest::__SIGNATURE_VTABLE,
        )
      )
    }
  }

  // did: optional string
  pub fn did(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_did(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_did(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.did_mut().set(val);
  }
  fn did_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          IssueVerifiableCredentialRequest::__DID_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `IssueVerifiableCredentialRequestMut` does not perform any shared mutation.
// - `IssueVerifiableCredentialRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for IssueVerifiableCredentialRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for IssueVerifiableCredentialRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, IssueVerifiableCredentialRequest> {
    IssueVerifiableCredentialRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, IssueVerifiableCredentialRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for IssueVerifiableCredentialRequestMut<'msg> {
  type Proxied = IssueVerifiableCredentialRequest;
  fn as_view(&self) -> ::__pb::View<'_, IssueVerifiableCredentialRequest> {
    IssueVerifiableCredentialRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, IssueVerifiableCredentialRequest> where 'msg: 'shorter {
    IssueVerifiableCredentialRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl IssueVerifiableCredentialRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_new() } } }
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

      __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> IssueVerifiableCredentialRequestView {
    IssueVerifiableCredentialRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> IssueVerifiableCredentialRequestMut {
    IssueVerifiableCredentialRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // credential_type: optional enum vc_api.v1.CredentialType
  pub fn credential_type(&self) -> crate::CredentialType {
    unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_credential_type(self.raw_msg()) }
  }
  pub fn set_credential_type(&mut self, val: crate::CredentialType) {
    unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_set_credential_type(self.raw_msg(), val) }
  }

  // nonce: optional string
  pub fn nonce(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_nonce(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_nonce(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.nonce_mut().set(val);
  }
  const __NONCE_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_nonce,
      __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_set_nonce,
    );
  fn nonce_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          IssueVerifiableCredentialRequest::__NONCE_VTABLE,
        )
      )
    }
  }

  // signature: optional bytes
  pub fn signature(&self) -> &'_ [u8] {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_signature(self.raw_msg()).as_ref() };
    view
  }
  pub fn set_signature(&mut self, val: impl ::__pb::SettableValue<[u8]>) {
    self.signature_mut().set(val);
  }
  const __SIGNATURE_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_signature,
      __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_set_signature,
    );
  fn signature_mut(&mut self) -> ::__pb::Mut<'_, [u8]> {
    unsafe {
      <::__pb::Mut<[u8]>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          IssueVerifiableCredentialRequest::__SIGNATURE_VTABLE,
        )
      )
    }
  }

  // did: optional string
  pub fn did(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_did(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_did(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.did_mut().set(val);
  }
  const __DID_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_did,
      __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_set_did,
    );
  fn did_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          IssueVerifiableCredentialRequest::__DID_VTABLE,
        )
      )
    }
  }

}  // impl IssueVerifiableCredentialRequest

impl ::__std::ops::Drop for IssueVerifiableCredentialRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_credential_type(raw_msg: ::__pb::__runtime::RawMessage) -> crate::CredentialType;
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_set_credential_type(raw_msg: ::__pb::__runtime::RawMessage, val: crate::CredentialType);

  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_nonce(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_set_nonce(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_signature(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_set_signature(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_get_did(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialRequest_set_did(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for IssueVerifiableCredentialRequest


impl IssueVerifiableCredentialRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> IssueVerifiableCredentialRequestMut<'a> {
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

impl<'a> IssueVerifiableCredentialRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct IssueVerifiableCredentialResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for IssueVerifiableCredentialResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for IssueVerifiableCredentialResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `IssueVerifiableCredentialResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `IssueVerifiableCredentialResponseMut`.
unsafe impl Sync for IssueVerifiableCredentialResponse {}

// SAFETY:
// - `IssueVerifiableCredentialResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for IssueVerifiableCredentialResponse {}

impl ::__pb::Proxied for IssueVerifiableCredentialResponse {
  type View<'msg> = IssueVerifiableCredentialResponseView<'msg>;
  type Mut<'msg> = IssueVerifiableCredentialResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct IssueVerifiableCredentialResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for IssueVerifiableCredentialResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> IssueVerifiableCredentialResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_serialize(self.raw_msg()) }
  }

  // credential: optional string
  pub fn credential(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_get_credential(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

}

// SAFETY:
// - `IssueVerifiableCredentialResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for IssueVerifiableCredentialResponseView<'_> {}

// SAFETY:
// - `IssueVerifiableCredentialResponseView` is `Send` because while its alive a `IssueVerifiableCredentialResponseMut` cannot.
// - `IssueVerifiableCredentialResponseView` does not use thread-local data.
unsafe impl Send for IssueVerifiableCredentialResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for IssueVerifiableCredentialResponseView<'msg> {
  type Proxied = IssueVerifiableCredentialResponse;

  fn as_view(&self) -> ::__pb::View<'msg, IssueVerifiableCredentialResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, IssueVerifiableCredentialResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for IssueVerifiableCredentialResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    IssueVerifiableCredentialResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    IssueVerifiableCredentialResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for IssueVerifiableCredentialResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for IssueVerifiableCredentialResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, IssueVerifiableCredentialResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, IssueVerifiableCredentialResponse>;

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

impl<'msg> ::__pb::SettableValue<IssueVerifiableCredentialResponse> for IssueVerifiableCredentialResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, IssueVerifiableCredentialResponse>)
    where IssueVerifiableCredentialResponse: 'dst {
    unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<IssueVerifiableCredentialResponse> for IssueVerifiableCredentialResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, IssueVerifiableCredentialResponse>)
    where IssueVerifiableCredentialResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for IssueVerifiableCredentialResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_copy_from(
        __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for IssueVerifiableCredentialResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueVerifiableCredentialResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_vc_api_v1_IssueVerifiableCredentialResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssueVerifiableCredentialResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for IssueVerifiableCredentialResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueVerifiableCredentialResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_vc_api_v1_IssueVerifiableCredentialResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssueVerifiableCredentialResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for IssueVerifiableCredentialResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueVerifiableCredentialResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_vc_api_v1_IssueVerifiableCredentialResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssueVerifiableCredentialResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for IssueVerifiableCredentialResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueVerifiableCredentialResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_vc_api_v1_IssueVerifiableCredentialResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssueVerifiableCredentialResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for IssueVerifiableCredentialResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueVerifiableCredentialResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_vc_api_v1_IssueVerifiableCredentialResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| IssueVerifiableCredentialResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for IssueVerifiableCredentialResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(IssueVerifiableCredentialResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_IssueVerifiableCredentialResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| IssueVerifiableCredentialResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct IssueVerifiableCredentialResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for IssueVerifiableCredentialResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> IssueVerifiableCredentialResponseMut<'msg> {
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


  // credential: optional string
  pub fn credential(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_get_credential(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_credential(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.credential_mut().set(val);
  }
  fn credential_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          IssueVerifiableCredentialResponse::__CREDENTIAL_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `IssueVerifiableCredentialResponseMut` does not perform any shared mutation.
// - `IssueVerifiableCredentialResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for IssueVerifiableCredentialResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for IssueVerifiableCredentialResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, IssueVerifiableCredentialResponse> {
    IssueVerifiableCredentialResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, IssueVerifiableCredentialResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for IssueVerifiableCredentialResponseMut<'msg> {
  type Proxied = IssueVerifiableCredentialResponse;
  fn as_view(&self) -> ::__pb::View<'_, IssueVerifiableCredentialResponse> {
    IssueVerifiableCredentialResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, IssueVerifiableCredentialResponse> where 'msg: 'shorter {
    IssueVerifiableCredentialResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl IssueVerifiableCredentialResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_new() } } }
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

      __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> IssueVerifiableCredentialResponseView {
    IssueVerifiableCredentialResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> IssueVerifiableCredentialResponseMut {
    IssueVerifiableCredentialResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // credential: optional string
  pub fn credential(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_get_credential(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_credential(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.credential_mut().set(val);
  }
  const __CREDENTIAL_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_get_credential,
      __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_set_credential,
    );
  fn credential_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          IssueVerifiableCredentialResponse::__CREDENTIAL_VTABLE,
        )
      )
    }
  }

}  // impl IssueVerifiableCredentialResponse

impl ::__std::ops::Drop for IssueVerifiableCredentialResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_get_credential(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_IssueVerifiableCredentialResponse_set_credential(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for IssueVerifiableCredentialResponse


impl IssueVerifiableCredentialResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> IssueVerifiableCredentialResponseMut<'a> {
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

impl<'a> IssueVerifiableCredentialResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct CreatePresentationChallengeRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for CreatePresentationChallengeRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for CreatePresentationChallengeRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `CreatePresentationChallengeRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `CreatePresentationChallengeRequestMut`.
unsafe impl Sync for CreatePresentationChallengeRequest {}

// SAFETY:
// - `CreatePresentationChallengeRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for CreatePresentationChallengeRequest {}

impl ::__pb::Proxied for CreatePresentationChallengeRequest {
  type View<'msg> = CreatePresentationChallengeRequestView<'msg>;
  type Mut<'msg> = CreatePresentationChallengeRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CreatePresentationChallengeRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for CreatePresentationChallengeRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> CreatePresentationChallengeRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_serialize(self.raw_msg()) }
  }

  // credential_types: repeated enum vc_api.v1.CredentialType
  pub fn credential_types(self) -> ::__pb::RepeatedView<'msg, crate::CredentialType> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_get_credential_types(self.raw_msg()),
      )
    }
  }

}

// SAFETY:
// - `CreatePresentationChallengeRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for CreatePresentationChallengeRequestView<'_> {}

// SAFETY:
// - `CreatePresentationChallengeRequestView` is `Send` because while its alive a `CreatePresentationChallengeRequestMut` cannot.
// - `CreatePresentationChallengeRequestView` does not use thread-local data.
unsafe impl Send for CreatePresentationChallengeRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for CreatePresentationChallengeRequestView<'msg> {
  type Proxied = CreatePresentationChallengeRequest;

  fn as_view(&self) -> ::__pb::View<'msg, CreatePresentationChallengeRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, CreatePresentationChallengeRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for CreatePresentationChallengeRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    CreatePresentationChallengeRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    CreatePresentationChallengeRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for CreatePresentationChallengeRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for CreatePresentationChallengeRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, CreatePresentationChallengeRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, CreatePresentationChallengeRequest>;

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

impl<'msg> ::__pb::SettableValue<CreatePresentationChallengeRequest> for CreatePresentationChallengeRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, CreatePresentationChallengeRequest>)
    where CreatePresentationChallengeRequest: 'dst {
    unsafe { __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<CreatePresentationChallengeRequest> for CreatePresentationChallengeRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, CreatePresentationChallengeRequest>)
    where CreatePresentationChallengeRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for CreatePresentationChallengeRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_copy_from(
        __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for CreatePresentationChallengeRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreatePresentationChallengeRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CreatePresentationChallengeRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for CreatePresentationChallengeRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreatePresentationChallengeRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CreatePresentationChallengeRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for CreatePresentationChallengeRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreatePresentationChallengeRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CreatePresentationChallengeRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for CreatePresentationChallengeRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreatePresentationChallengeRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CreatePresentationChallengeRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for CreatePresentationChallengeRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreatePresentationChallengeRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CreatePresentationChallengeRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for CreatePresentationChallengeRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreatePresentationChallengeRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| CreatePresentationChallengeRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CreatePresentationChallengeRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for CreatePresentationChallengeRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> CreatePresentationChallengeRequestMut<'msg> {
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


  // credential_types: repeated enum vc_api.v1.CredentialType
  pub fn credential_types(&self) -> ::__pb::RepeatedView<'_, crate::CredentialType> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_get_credential_types(self.raw_msg()),
      )
    }
  }
  pub fn credential_types_mut(&mut self) -> ::__pb::RepeatedMut<'_, crate::CredentialType> {
    unsafe {
      ::__pb::RepeatedMut::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__runtime::InnerRepeatedMut::new(
          ::__pb::__internal::Private,
          __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_get_mut_credential_types(self.raw_msg()),
        ),
      )
    }
  }

}

// SAFETY:
// - `CreatePresentationChallengeRequestMut` does not perform any shared mutation.
// - `CreatePresentationChallengeRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for CreatePresentationChallengeRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for CreatePresentationChallengeRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, CreatePresentationChallengeRequest> {
    CreatePresentationChallengeRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, CreatePresentationChallengeRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for CreatePresentationChallengeRequestMut<'msg> {
  type Proxied = CreatePresentationChallengeRequest;
  fn as_view(&self) -> ::__pb::View<'_, CreatePresentationChallengeRequest> {
    CreatePresentationChallengeRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, CreatePresentationChallengeRequest> where 'msg: 'shorter {
    CreatePresentationChallengeRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl CreatePresentationChallengeRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_new() } } }
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

      __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> CreatePresentationChallengeRequestView {
    CreatePresentationChallengeRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> CreatePresentationChallengeRequestMut {
    CreatePresentationChallengeRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // credential_types: repeated enum vc_api.v1.CredentialType
  pub fn credential_types(&self) -> ::__pb::RepeatedView<'_, crate::CredentialType> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_get_credential_types(self.raw_msg()),
      )
    }
  }
  pub fn credential_types_mut(&mut self) -> ::__pb::RepeatedMut<'_, crate::CredentialType> {
    unsafe {
      ::__pb::RepeatedMut::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__runtime::InnerRepeatedMut::new(
          ::__pb::__internal::Private,
          __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_get_mut_credential_types(self.raw_msg()),
        ),
      )
    }
  }

}  // impl CreatePresentationChallengeRequest

impl ::__std::ops::Drop for CreatePresentationChallengeRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_clear_credential_types(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_get_mut_credential_types(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawRepeatedField;
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeRequest_get_credential_types(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawRepeatedField;


}  // extern "C" for CreatePresentationChallengeRequest


impl CreatePresentationChallengeRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> CreatePresentationChallengeRequestMut<'a> {
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

impl<'a> CreatePresentationChallengeRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct CreatePresentationChallengeResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for CreatePresentationChallengeResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for CreatePresentationChallengeResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `CreatePresentationChallengeResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `CreatePresentationChallengeResponseMut`.
unsafe impl Sync for CreatePresentationChallengeResponse {}

// SAFETY:
// - `CreatePresentationChallengeResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for CreatePresentationChallengeResponse {}

impl ::__pb::Proxied for CreatePresentationChallengeResponse {
  type View<'msg> = CreatePresentationChallengeResponseView<'msg>;
  type Mut<'msg> = CreatePresentationChallengeResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CreatePresentationChallengeResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for CreatePresentationChallengeResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> CreatePresentationChallengeResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_serialize(self.raw_msg()) }
  }

  // nonce: optional string
  pub fn nonce(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_get_nonce(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // endpoint: optional string
  pub fn endpoint(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_get_endpoint(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // credential_types: repeated enum vc_api.v1.CredentialType
  pub fn credential_types(self) -> ::__pb::RepeatedView<'msg, crate::CredentialType> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_get_credential_types(self.raw_msg()),
      )
    }
  }

}

// SAFETY:
// - `CreatePresentationChallengeResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for CreatePresentationChallengeResponseView<'_> {}

// SAFETY:
// - `CreatePresentationChallengeResponseView` is `Send` because while its alive a `CreatePresentationChallengeResponseMut` cannot.
// - `CreatePresentationChallengeResponseView` does not use thread-local data.
unsafe impl Send for CreatePresentationChallengeResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for CreatePresentationChallengeResponseView<'msg> {
  type Proxied = CreatePresentationChallengeResponse;

  fn as_view(&self) -> ::__pb::View<'msg, CreatePresentationChallengeResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, CreatePresentationChallengeResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for CreatePresentationChallengeResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    CreatePresentationChallengeResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    CreatePresentationChallengeResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for CreatePresentationChallengeResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for CreatePresentationChallengeResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, CreatePresentationChallengeResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, CreatePresentationChallengeResponse>;

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

impl<'msg> ::__pb::SettableValue<CreatePresentationChallengeResponse> for CreatePresentationChallengeResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, CreatePresentationChallengeResponse>)
    where CreatePresentationChallengeResponse: 'dst {
    unsafe { __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<CreatePresentationChallengeResponse> for CreatePresentationChallengeResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, CreatePresentationChallengeResponse>)
    where CreatePresentationChallengeResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for CreatePresentationChallengeResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_copy_from(
        __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for CreatePresentationChallengeResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreatePresentationChallengeResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_vc_api_v1_CreatePresentationChallengeResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CreatePresentationChallengeResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for CreatePresentationChallengeResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreatePresentationChallengeResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_vc_api_v1_CreatePresentationChallengeResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CreatePresentationChallengeResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for CreatePresentationChallengeResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreatePresentationChallengeResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_vc_api_v1_CreatePresentationChallengeResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CreatePresentationChallengeResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for CreatePresentationChallengeResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreatePresentationChallengeResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_vc_api_v1_CreatePresentationChallengeResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CreatePresentationChallengeResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for CreatePresentationChallengeResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreatePresentationChallengeResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_vc_api_v1_CreatePresentationChallengeResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CreatePresentationChallengeResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for CreatePresentationChallengeResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CreatePresentationChallengeResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CreatePresentationChallengeResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| CreatePresentationChallengeResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CreatePresentationChallengeResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for CreatePresentationChallengeResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> CreatePresentationChallengeResponseMut<'msg> {
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


  // nonce: optional string
  pub fn nonce(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_get_nonce(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_nonce(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.nonce_mut().set(val);
  }
  fn nonce_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CreatePresentationChallengeResponse::__NONCE_VTABLE,
        )
      )
    }
  }

  // endpoint: optional string
  pub fn endpoint(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_get_endpoint(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_endpoint(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.endpoint_mut().set(val);
  }
  fn endpoint_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CreatePresentationChallengeResponse::__ENDPOINT_VTABLE,
        )
      )
    }
  }

  // credential_types: repeated enum vc_api.v1.CredentialType
  pub fn credential_types(&self) -> ::__pb::RepeatedView<'_, crate::CredentialType> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_get_credential_types(self.raw_msg()),
      )
    }
  }
  pub fn credential_types_mut(&mut self) -> ::__pb::RepeatedMut<'_, crate::CredentialType> {
    unsafe {
      ::__pb::RepeatedMut::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__runtime::InnerRepeatedMut::new(
          ::__pb::__internal::Private,
          __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_get_mut_credential_types(self.raw_msg()),
        ),
      )
    }
  }

}

// SAFETY:
// - `CreatePresentationChallengeResponseMut` does not perform any shared mutation.
// - `CreatePresentationChallengeResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for CreatePresentationChallengeResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for CreatePresentationChallengeResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, CreatePresentationChallengeResponse> {
    CreatePresentationChallengeResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, CreatePresentationChallengeResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for CreatePresentationChallengeResponseMut<'msg> {
  type Proxied = CreatePresentationChallengeResponse;
  fn as_view(&self) -> ::__pb::View<'_, CreatePresentationChallengeResponse> {
    CreatePresentationChallengeResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, CreatePresentationChallengeResponse> where 'msg: 'shorter {
    CreatePresentationChallengeResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl CreatePresentationChallengeResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_new() } } }
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

      __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> CreatePresentationChallengeResponseView {
    CreatePresentationChallengeResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> CreatePresentationChallengeResponseMut {
    CreatePresentationChallengeResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // nonce: optional string
  pub fn nonce(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_get_nonce(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_nonce(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.nonce_mut().set(val);
  }
  const __NONCE_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_get_nonce,
      __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_set_nonce,
    );
  fn nonce_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CreatePresentationChallengeResponse::__NONCE_VTABLE,
        )
      )
    }
  }

  // endpoint: optional string
  pub fn endpoint(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_get_endpoint(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_endpoint(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.endpoint_mut().set(val);
  }
  const __ENDPOINT_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_get_endpoint,
      __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_set_endpoint,
    );
  fn endpoint_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CreatePresentationChallengeResponse::__ENDPOINT_VTABLE,
        )
      )
    }
  }

  // credential_types: repeated enum vc_api.v1.CredentialType
  pub fn credential_types(&self) -> ::__pb::RepeatedView<'_, crate::CredentialType> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_get_credential_types(self.raw_msg()),
      )
    }
  }
  pub fn credential_types_mut(&mut self) -> ::__pb::RepeatedMut<'_, crate::CredentialType> {
    unsafe {
      ::__pb::RepeatedMut::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__runtime::InnerRepeatedMut::new(
          ::__pb::__internal::Private,
          __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_get_mut_credential_types(self.raw_msg()),
        ),
      )
    }
  }

}  // impl CreatePresentationChallengeResponse

impl ::__std::ops::Drop for CreatePresentationChallengeResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_get_nonce(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_set_nonce(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_get_endpoint(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_set_endpoint(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_clear_credential_types(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_get_mut_credential_types(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawRepeatedField;
  fn __rust_proto_thunk__vc_api_v1_CreatePresentationChallengeResponse_get_credential_types(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawRepeatedField;


}  // extern "C" for CreatePresentationChallengeResponse


impl CreatePresentationChallengeResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> CreatePresentationChallengeResponseMut<'a> {
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

impl<'a> CreatePresentationChallengeResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct PresentVerifiableCredentialRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for PresentVerifiableCredentialRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for PresentVerifiableCredentialRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `PresentVerifiableCredentialRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `PresentVerifiableCredentialRequestMut`.
unsafe impl Sync for PresentVerifiableCredentialRequest {}

// SAFETY:
// - `PresentVerifiableCredentialRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PresentVerifiableCredentialRequest {}

impl ::__pb::Proxied for PresentVerifiableCredentialRequest {
  type View<'msg> = PresentVerifiableCredentialRequestView<'msg>;
  type Mut<'msg> = PresentVerifiableCredentialRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PresentVerifiableCredentialRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for PresentVerifiableCredentialRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> PresentVerifiableCredentialRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_serialize(self.raw_msg()) }
  }

  // presentation: optional string
  pub fn presentation(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_presentation(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // nonce: optional string
  pub fn nonce(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_nonce(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // signature: optional bytes
  pub fn signature(self) -> &'msg [u8] {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_signature(self.raw_msg()).as_ref() };
    view
  }

  // did: optional string
  pub fn did(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_did(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // credential_type: repeated enum vc_api.v1.CredentialType
  pub fn credential_type(self) -> ::__pb::RepeatedView<'msg, crate::CredentialType> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_credential_type(self.raw_msg()),
      )
    }
  }

}

// SAFETY:
// - `PresentVerifiableCredentialRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for PresentVerifiableCredentialRequestView<'_> {}

// SAFETY:
// - `PresentVerifiableCredentialRequestView` is `Send` because while its alive a `PresentVerifiableCredentialRequestMut` cannot.
// - `PresentVerifiableCredentialRequestView` does not use thread-local data.
unsafe impl Send for PresentVerifiableCredentialRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for PresentVerifiableCredentialRequestView<'msg> {
  type Proxied = PresentVerifiableCredentialRequest;

  fn as_view(&self) -> ::__pb::View<'msg, PresentVerifiableCredentialRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, PresentVerifiableCredentialRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for PresentVerifiableCredentialRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    PresentVerifiableCredentialRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    PresentVerifiableCredentialRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for PresentVerifiableCredentialRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for PresentVerifiableCredentialRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, PresentVerifiableCredentialRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, PresentVerifiableCredentialRequest>;

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

impl<'msg> ::__pb::SettableValue<PresentVerifiableCredentialRequest> for PresentVerifiableCredentialRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, PresentVerifiableCredentialRequest>)
    where PresentVerifiableCredentialRequest: 'dst {
    unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<PresentVerifiableCredentialRequest> for PresentVerifiableCredentialRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, PresentVerifiableCredentialRequest>)
    where PresentVerifiableCredentialRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for PresentVerifiableCredentialRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_copy_from(
        __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for PresentVerifiableCredentialRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(PresentVerifiableCredentialRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| PresentVerifiableCredentialRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for PresentVerifiableCredentialRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(PresentVerifiableCredentialRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| PresentVerifiableCredentialRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for PresentVerifiableCredentialRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(PresentVerifiableCredentialRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| PresentVerifiableCredentialRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for PresentVerifiableCredentialRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(PresentVerifiableCredentialRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| PresentVerifiableCredentialRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for PresentVerifiableCredentialRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(PresentVerifiableCredentialRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| PresentVerifiableCredentialRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for PresentVerifiableCredentialRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(PresentVerifiableCredentialRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| PresentVerifiableCredentialRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PresentVerifiableCredentialRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for PresentVerifiableCredentialRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> PresentVerifiableCredentialRequestMut<'msg> {
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


  // presentation: optional string
  pub fn presentation(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_presentation(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_presentation(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.presentation_mut().set(val);
  }
  fn presentation_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          PresentVerifiableCredentialRequest::__PRESENTATION_VTABLE,
        )
      )
    }
  }

  // nonce: optional string
  pub fn nonce(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_nonce(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_nonce(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.nonce_mut().set(val);
  }
  fn nonce_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          PresentVerifiableCredentialRequest::__NONCE_VTABLE,
        )
      )
    }
  }

  // signature: optional bytes
  pub fn signature(&self) -> &'_ [u8] {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_signature(self.raw_msg()).as_ref() };
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
          PresentVerifiableCredentialRequest::__SIGNATURE_VTABLE,
        )
      )
    }
  }

  // did: optional string
  pub fn did(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_did(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_did(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.did_mut().set(val);
  }
  fn did_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          PresentVerifiableCredentialRequest::__DID_VTABLE,
        )
      )
    }
  }

  // credential_type: repeated enum vc_api.v1.CredentialType
  pub fn credential_type(&self) -> ::__pb::RepeatedView<'_, crate::CredentialType> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_credential_type(self.raw_msg()),
      )
    }
  }
  pub fn credential_type_mut(&mut self) -> ::__pb::RepeatedMut<'_, crate::CredentialType> {
    unsafe {
      ::__pb::RepeatedMut::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__runtime::InnerRepeatedMut::new(
          ::__pb::__internal::Private,
          __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_mut_credential_type(self.raw_msg()),
        ),
      )
    }
  }

}

// SAFETY:
// - `PresentVerifiableCredentialRequestMut` does not perform any shared mutation.
// - `PresentVerifiableCredentialRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PresentVerifiableCredentialRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for PresentVerifiableCredentialRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, PresentVerifiableCredentialRequest> {
    PresentVerifiableCredentialRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, PresentVerifiableCredentialRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for PresentVerifiableCredentialRequestMut<'msg> {
  type Proxied = PresentVerifiableCredentialRequest;
  fn as_view(&self) -> ::__pb::View<'_, PresentVerifiableCredentialRequest> {
    PresentVerifiableCredentialRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, PresentVerifiableCredentialRequest> where 'msg: 'shorter {
    PresentVerifiableCredentialRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl PresentVerifiableCredentialRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_new() } } }
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

      __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> PresentVerifiableCredentialRequestView {
    PresentVerifiableCredentialRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> PresentVerifiableCredentialRequestMut {
    PresentVerifiableCredentialRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // presentation: optional string
  pub fn presentation(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_presentation(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_presentation(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.presentation_mut().set(val);
  }
  const __PRESENTATION_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_presentation,
      __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_set_presentation,
    );
  fn presentation_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          PresentVerifiableCredentialRequest::__PRESENTATION_VTABLE,
        )
      )
    }
  }

  // nonce: optional string
  pub fn nonce(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_nonce(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_nonce(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.nonce_mut().set(val);
  }
  const __NONCE_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_nonce,
      __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_set_nonce,
    );
  fn nonce_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          PresentVerifiableCredentialRequest::__NONCE_VTABLE,
        )
      )
    }
  }

  // signature: optional bytes
  pub fn signature(&self) -> &'_ [u8] {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_signature(self.raw_msg()).as_ref() };
    view
  }
  pub fn set_signature(&mut self, val: impl ::__pb::SettableValue<[u8]>) {
    self.signature_mut().set(val);
  }
  const __SIGNATURE_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_signature,
      __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_set_signature,
    );
  fn signature_mut(&mut self) -> ::__pb::Mut<'_, [u8]> {
    unsafe {
      <::__pb::Mut<[u8]>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          PresentVerifiableCredentialRequest::__SIGNATURE_VTABLE,
        )
      )
    }
  }

  // did: optional string
  pub fn did(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_did(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_did(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.did_mut().set(val);
  }
  const __DID_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_did,
      __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_set_did,
    );
  fn did_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          PresentVerifiableCredentialRequest::__DID_VTABLE,
        )
      )
    }
  }

  // credential_type: repeated enum vc_api.v1.CredentialType
  pub fn credential_type(&self) -> ::__pb::RepeatedView<'_, crate::CredentialType> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_credential_type(self.raw_msg()),
      )
    }
  }
  pub fn credential_type_mut(&mut self) -> ::__pb::RepeatedMut<'_, crate::CredentialType> {
    unsafe {
      ::__pb::RepeatedMut::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__runtime::InnerRepeatedMut::new(
          ::__pb::__internal::Private,
          __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_mut_credential_type(self.raw_msg()),
        ),
      )
    }
  }

}  // impl PresentVerifiableCredentialRequest

impl ::__std::ops::Drop for PresentVerifiableCredentialRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_presentation(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_set_presentation(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_nonce(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_set_nonce(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_signature(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_set_signature(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_did(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_set_did(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_clear_credential_type(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_mut_credential_type(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawRepeatedField;
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialRequest_get_credential_type(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawRepeatedField;


}  // extern "C" for PresentVerifiableCredentialRequest


impl PresentVerifiableCredentialRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> PresentVerifiableCredentialRequestMut<'a> {
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

impl<'a> PresentVerifiableCredentialRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct PresentVerifiableCredentialResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for PresentVerifiableCredentialResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for PresentVerifiableCredentialResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `PresentVerifiableCredentialResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `PresentVerifiableCredentialResponseMut`.
unsafe impl Sync for PresentVerifiableCredentialResponse {}

// SAFETY:
// - `PresentVerifiableCredentialResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for PresentVerifiableCredentialResponse {}

impl ::__pb::Proxied for PresentVerifiableCredentialResponse {
  type View<'msg> = PresentVerifiableCredentialResponseView<'msg>;
  type Mut<'msg> = PresentVerifiableCredentialResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PresentVerifiableCredentialResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for PresentVerifiableCredentialResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> PresentVerifiableCredentialResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_serialize(self.raw_msg()) }
  }

}

// SAFETY:
// - `PresentVerifiableCredentialResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for PresentVerifiableCredentialResponseView<'_> {}

// SAFETY:
// - `PresentVerifiableCredentialResponseView` is `Send` because while its alive a `PresentVerifiableCredentialResponseMut` cannot.
// - `PresentVerifiableCredentialResponseView` does not use thread-local data.
unsafe impl Send for PresentVerifiableCredentialResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for PresentVerifiableCredentialResponseView<'msg> {
  type Proxied = PresentVerifiableCredentialResponse;

  fn as_view(&self) -> ::__pb::View<'msg, PresentVerifiableCredentialResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, PresentVerifiableCredentialResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for PresentVerifiableCredentialResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    PresentVerifiableCredentialResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    PresentVerifiableCredentialResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for PresentVerifiableCredentialResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for PresentVerifiableCredentialResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, PresentVerifiableCredentialResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, PresentVerifiableCredentialResponse>;

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

impl<'msg> ::__pb::SettableValue<PresentVerifiableCredentialResponse> for PresentVerifiableCredentialResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, PresentVerifiableCredentialResponse>)
    where PresentVerifiableCredentialResponse: 'dst {
    unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<PresentVerifiableCredentialResponse> for PresentVerifiableCredentialResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, PresentVerifiableCredentialResponse>)
    where PresentVerifiableCredentialResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for PresentVerifiableCredentialResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_copy_from(
        __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for PresentVerifiableCredentialResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(PresentVerifiableCredentialResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_vc_api_v1_PresentVerifiableCredentialResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| PresentVerifiableCredentialResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for PresentVerifiableCredentialResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(PresentVerifiableCredentialResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_vc_api_v1_PresentVerifiableCredentialResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| PresentVerifiableCredentialResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for PresentVerifiableCredentialResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(PresentVerifiableCredentialResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_vc_api_v1_PresentVerifiableCredentialResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| PresentVerifiableCredentialResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for PresentVerifiableCredentialResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(PresentVerifiableCredentialResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_vc_api_v1_PresentVerifiableCredentialResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| PresentVerifiableCredentialResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for PresentVerifiableCredentialResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(PresentVerifiableCredentialResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_vc_api_v1_PresentVerifiableCredentialResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| PresentVerifiableCredentialResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for PresentVerifiableCredentialResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(PresentVerifiableCredentialResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_PresentVerifiableCredentialResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| PresentVerifiableCredentialResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PresentVerifiableCredentialResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for PresentVerifiableCredentialResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> PresentVerifiableCredentialResponseMut<'msg> {
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
// - `PresentVerifiableCredentialResponseMut` does not perform any shared mutation.
// - `PresentVerifiableCredentialResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PresentVerifiableCredentialResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for PresentVerifiableCredentialResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, PresentVerifiableCredentialResponse> {
    PresentVerifiableCredentialResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, PresentVerifiableCredentialResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for PresentVerifiableCredentialResponseMut<'msg> {
  type Proxied = PresentVerifiableCredentialResponse;
  fn as_view(&self) -> ::__pb::View<'_, PresentVerifiableCredentialResponse> {
    PresentVerifiableCredentialResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, PresentVerifiableCredentialResponse> where 'msg: 'shorter {
    PresentVerifiableCredentialResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl PresentVerifiableCredentialResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_new() } } }
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

      __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> PresentVerifiableCredentialResponseView {
    PresentVerifiableCredentialResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> PresentVerifiableCredentialResponseMut {
    PresentVerifiableCredentialResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

}  // impl PresentVerifiableCredentialResponse

impl ::__std::ops::Drop for PresentVerifiableCredentialResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__vc_api_v1_PresentVerifiableCredentialResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);


}  // extern "C" for PresentVerifiableCredentialResponse


impl PresentVerifiableCredentialResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> PresentVerifiableCredentialResponseMut<'a> {
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

impl<'a> PresentVerifiableCredentialResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct WaitForCompletionRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for WaitForCompletionRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for WaitForCompletionRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `WaitForCompletionRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `WaitForCompletionRequestMut`.
unsafe impl Sync for WaitForCompletionRequest {}

// SAFETY:
// - `WaitForCompletionRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for WaitForCompletionRequest {}

impl ::__pb::Proxied for WaitForCompletionRequest {
  type View<'msg> = WaitForCompletionRequestView<'msg>;
  type Mut<'msg> = WaitForCompletionRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct WaitForCompletionRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for WaitForCompletionRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> WaitForCompletionRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_serialize(self.raw_msg()) }
  }

  // nonce: optional string
  pub fn nonce(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_get_nonce(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

}

// SAFETY:
// - `WaitForCompletionRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for WaitForCompletionRequestView<'_> {}

// SAFETY:
// - `WaitForCompletionRequestView` is `Send` because while its alive a `WaitForCompletionRequestMut` cannot.
// - `WaitForCompletionRequestView` does not use thread-local data.
unsafe impl Send for WaitForCompletionRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for WaitForCompletionRequestView<'msg> {
  type Proxied = WaitForCompletionRequest;

  fn as_view(&self) -> ::__pb::View<'msg, WaitForCompletionRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, WaitForCompletionRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for WaitForCompletionRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    WaitForCompletionRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    WaitForCompletionRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for WaitForCompletionRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for WaitForCompletionRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, WaitForCompletionRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, WaitForCompletionRequest>;

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

impl<'msg> ::__pb::SettableValue<WaitForCompletionRequest> for WaitForCompletionRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, WaitForCompletionRequest>)
    where WaitForCompletionRequest: 'dst {
    unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<WaitForCompletionRequest> for WaitForCompletionRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, WaitForCompletionRequest>)
    where WaitForCompletionRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for WaitForCompletionRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_copy_from(
        __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for WaitForCompletionRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(WaitForCompletionRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| WaitForCompletionRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for WaitForCompletionRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(WaitForCompletionRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| WaitForCompletionRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for WaitForCompletionRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(WaitForCompletionRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| WaitForCompletionRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for WaitForCompletionRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(WaitForCompletionRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| WaitForCompletionRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for WaitForCompletionRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(WaitForCompletionRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| WaitForCompletionRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for WaitForCompletionRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(WaitForCompletionRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| WaitForCompletionRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct WaitForCompletionRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for WaitForCompletionRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> WaitForCompletionRequestMut<'msg> {
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


  // nonce: optional string
  pub fn nonce(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_get_nonce(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_nonce(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.nonce_mut().set(val);
  }
  fn nonce_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          WaitForCompletionRequest::__NONCE_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `WaitForCompletionRequestMut` does not perform any shared mutation.
// - `WaitForCompletionRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for WaitForCompletionRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for WaitForCompletionRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, WaitForCompletionRequest> {
    WaitForCompletionRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, WaitForCompletionRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for WaitForCompletionRequestMut<'msg> {
  type Proxied = WaitForCompletionRequest;
  fn as_view(&self) -> ::__pb::View<'_, WaitForCompletionRequest> {
    WaitForCompletionRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, WaitForCompletionRequest> where 'msg: 'shorter {
    WaitForCompletionRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl WaitForCompletionRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_new() } } }
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

      __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> WaitForCompletionRequestView {
    WaitForCompletionRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> WaitForCompletionRequestMut {
    WaitForCompletionRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // nonce: optional string
  pub fn nonce(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_get_nonce(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_nonce(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.nonce_mut().set(val);
  }
  const __NONCE_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_get_nonce,
      __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_set_nonce,
    );
  fn nonce_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          WaitForCompletionRequest::__NONCE_VTABLE,
        )
      )
    }
  }

}  // impl WaitForCompletionRequest

impl ::__std::ops::Drop for WaitForCompletionRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_get_nonce(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionRequest_set_nonce(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for WaitForCompletionRequest


impl WaitForCompletionRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> WaitForCompletionRequestMut<'a> {
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

impl<'a> WaitForCompletionRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct WaitForCompletionResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for WaitForCompletionResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for WaitForCompletionResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `WaitForCompletionResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `WaitForCompletionResponseMut`.
unsafe impl Sync for WaitForCompletionResponse {}

// SAFETY:
// - `WaitForCompletionResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for WaitForCompletionResponse {}

impl ::__pb::Proxied for WaitForCompletionResponse {
  type View<'msg> = WaitForCompletionResponseView<'msg>;
  type Mut<'msg> = WaitForCompletionResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct WaitForCompletionResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for WaitForCompletionResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> WaitForCompletionResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_serialize(self.raw_msg()) }
  }

  // did: optional string
  pub fn did(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_get_did(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // data: optional string
  pub fn data(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_get_data(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

}

// SAFETY:
// - `WaitForCompletionResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for WaitForCompletionResponseView<'_> {}

// SAFETY:
// - `WaitForCompletionResponseView` is `Send` because while its alive a `WaitForCompletionResponseMut` cannot.
// - `WaitForCompletionResponseView` does not use thread-local data.
unsafe impl Send for WaitForCompletionResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for WaitForCompletionResponseView<'msg> {
  type Proxied = WaitForCompletionResponse;

  fn as_view(&self) -> ::__pb::View<'msg, WaitForCompletionResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, WaitForCompletionResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for WaitForCompletionResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    WaitForCompletionResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    WaitForCompletionResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for WaitForCompletionResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for WaitForCompletionResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, WaitForCompletionResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, WaitForCompletionResponse>;

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

impl<'msg> ::__pb::SettableValue<WaitForCompletionResponse> for WaitForCompletionResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, WaitForCompletionResponse>)
    where WaitForCompletionResponse: 'dst {
    unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<WaitForCompletionResponse> for WaitForCompletionResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, WaitForCompletionResponse>)
    where WaitForCompletionResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for WaitForCompletionResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_copy_from(
        __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for WaitForCompletionResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(WaitForCompletionResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_vc_api_v1_WaitForCompletionResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| WaitForCompletionResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for WaitForCompletionResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(WaitForCompletionResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_vc_api_v1_WaitForCompletionResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| WaitForCompletionResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for WaitForCompletionResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(WaitForCompletionResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_vc_api_v1_WaitForCompletionResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| WaitForCompletionResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for WaitForCompletionResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(WaitForCompletionResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_vc_api_v1_WaitForCompletionResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| WaitForCompletionResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for WaitForCompletionResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(WaitForCompletionResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_vc_api_v1_WaitForCompletionResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| WaitForCompletionResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for WaitForCompletionResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(WaitForCompletionResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_WaitForCompletionResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| WaitForCompletionResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct WaitForCompletionResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for WaitForCompletionResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> WaitForCompletionResponseMut<'msg> {
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


  // did: optional string
  pub fn did(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_get_did(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_did(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.did_mut().set(val);
  }
  fn did_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          WaitForCompletionResponse::__DID_VTABLE,
        )
      )
    }
  }

  // data: optional string
  pub fn data(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_get_data(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_data(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.data_mut().set(val);
  }
  fn data_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          WaitForCompletionResponse::__DATA_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `WaitForCompletionResponseMut` does not perform any shared mutation.
// - `WaitForCompletionResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for WaitForCompletionResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for WaitForCompletionResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, WaitForCompletionResponse> {
    WaitForCompletionResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, WaitForCompletionResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for WaitForCompletionResponseMut<'msg> {
  type Proxied = WaitForCompletionResponse;
  fn as_view(&self) -> ::__pb::View<'_, WaitForCompletionResponse> {
    WaitForCompletionResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, WaitForCompletionResponse> where 'msg: 'shorter {
    WaitForCompletionResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl WaitForCompletionResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_new() } } }
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

      __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> WaitForCompletionResponseView {
    WaitForCompletionResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> WaitForCompletionResponseMut {
    WaitForCompletionResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // did: optional string
  pub fn did(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_get_did(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_did(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.did_mut().set(val);
  }
  const __DID_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_get_did,
      __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_set_did,
    );
  fn did_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          WaitForCompletionResponse::__DID_VTABLE,
        )
      )
    }
  }

  // data: optional string
  pub fn data(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_get_data(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_data(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.data_mut().set(val);
  }
  const __DATA_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_get_data,
      __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_set_data,
    );
  fn data_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          WaitForCompletionResponse::__DATA_VTABLE,
        )
      )
    }
  }

}  // impl WaitForCompletionResponse

impl ::__std::ops::Drop for WaitForCompletionResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_get_did(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_set_did(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_get_data(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_WaitForCompletionResponse_set_data(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for WaitForCompletionResponse


impl WaitForCompletionResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> WaitForCompletionResponseMut<'a> {
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

impl<'a> WaitForCompletionResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct UpdateVerifiableCredentialStatusRequest {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for UpdateVerifiableCredentialStatusRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for UpdateVerifiableCredentialStatusRequest {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `UpdateVerifiableCredentialStatusRequest` is `Sync` because it does not implement interior mutability.
//    Neither does `UpdateVerifiableCredentialStatusRequestMut`.
unsafe impl Sync for UpdateVerifiableCredentialStatusRequest {}

// SAFETY:
// - `UpdateVerifiableCredentialStatusRequest` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for UpdateVerifiableCredentialStatusRequest {}

impl ::__pb::Proxied for UpdateVerifiableCredentialStatusRequest {
  type View<'msg> = UpdateVerifiableCredentialStatusRequestView<'msg>;
  type Mut<'msg> = UpdateVerifiableCredentialStatusRequestMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UpdateVerifiableCredentialStatusRequestView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for UpdateVerifiableCredentialStatusRequestView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> UpdateVerifiableCredentialStatusRequestView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_serialize(self.raw_msg()) }
  }

  // credential_type: optional enum vc_api.v1.CredentialType
  pub fn credential_type(self) -> crate::CredentialType {
    unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_get_credential_type(self.raw_msg()) }
  }

  // did: optional string
  pub fn did(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_get_did(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // update_type: optional enum vc_api.v1.UpdateVerifiableCredentialStatusType
  pub fn update_type(self) -> crate::UpdateVerifiableCredentialStatusType {
    unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_get_update_type(self.raw_msg()) }
  }

}

// SAFETY:
// - `UpdateVerifiableCredentialStatusRequestView` is `Sync` because it does not support mutation.
unsafe impl Sync for UpdateVerifiableCredentialStatusRequestView<'_> {}

// SAFETY:
// - `UpdateVerifiableCredentialStatusRequestView` is `Send` because while its alive a `UpdateVerifiableCredentialStatusRequestMut` cannot.
// - `UpdateVerifiableCredentialStatusRequestView` does not use thread-local data.
unsafe impl Send for UpdateVerifiableCredentialStatusRequestView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for UpdateVerifiableCredentialStatusRequestView<'msg> {
  type Proxied = UpdateVerifiableCredentialStatusRequest;

  fn as_view(&self) -> ::__pb::View<'msg, UpdateVerifiableCredentialStatusRequest> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, UpdateVerifiableCredentialStatusRequest> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for UpdateVerifiableCredentialStatusRequest {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    UpdateVerifiableCredentialStatusRequestView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    UpdateVerifiableCredentialStatusRequestMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for UpdateVerifiableCredentialStatusRequest {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for UpdateVerifiableCredentialStatusRequest {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, UpdateVerifiableCredentialStatusRequest>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, UpdateVerifiableCredentialStatusRequest>;

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

impl<'msg> ::__pb::SettableValue<UpdateVerifiableCredentialStatusRequest> for UpdateVerifiableCredentialStatusRequestView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, UpdateVerifiableCredentialStatusRequest>)
    where UpdateVerifiableCredentialStatusRequest: 'dst {
    unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<UpdateVerifiableCredentialStatusRequest> for UpdateVerifiableCredentialStatusRequest {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, UpdateVerifiableCredentialStatusRequest>)
    where UpdateVerifiableCredentialStatusRequest: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for UpdateVerifiableCredentialStatusRequest {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_copy_from(
        __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for UpdateVerifiableCredentialStatusRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(UpdateVerifiableCredentialStatusRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| UpdateVerifiableCredentialStatusRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for UpdateVerifiableCredentialStatusRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(UpdateVerifiableCredentialStatusRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| UpdateVerifiableCredentialStatusRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for UpdateVerifiableCredentialStatusRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(UpdateVerifiableCredentialStatusRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| UpdateVerifiableCredentialStatusRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for UpdateVerifiableCredentialStatusRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(UpdateVerifiableCredentialStatusRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| UpdateVerifiableCredentialStatusRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for UpdateVerifiableCredentialStatusRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(UpdateVerifiableCredentialStatusRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| UpdateVerifiableCredentialStatusRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for UpdateVerifiableCredentialStatusRequest {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(UpdateVerifiableCredentialStatusRequestView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusRequest_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| UpdateVerifiableCredentialStatusRequestView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UpdateVerifiableCredentialStatusRequestMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for UpdateVerifiableCredentialStatusRequestMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> UpdateVerifiableCredentialStatusRequestMut<'msg> {
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


  // credential_type: optional enum vc_api.v1.CredentialType
  pub fn credential_type(&self) -> crate::CredentialType {
    unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_get_credential_type(self.raw_msg()) }
  }
  pub fn set_credential_type(&mut self, val: crate::CredentialType) {
    unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_set_credential_type(self.raw_msg(), val) }
  }

  // did: optional string
  pub fn did(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_get_did(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_did(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.did_mut().set(val);
  }
  fn did_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          UpdateVerifiableCredentialStatusRequest::__DID_VTABLE,
        )
      )
    }
  }

  // update_type: optional enum vc_api.v1.UpdateVerifiableCredentialStatusType
  pub fn update_type(&self) -> crate::UpdateVerifiableCredentialStatusType {
    unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_get_update_type(self.raw_msg()) }
  }
  pub fn set_update_type(&mut self, val: crate::UpdateVerifiableCredentialStatusType) {
    unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_set_update_type(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `UpdateVerifiableCredentialStatusRequestMut` does not perform any shared mutation.
// - `UpdateVerifiableCredentialStatusRequestMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for UpdateVerifiableCredentialStatusRequestMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for UpdateVerifiableCredentialStatusRequestMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, UpdateVerifiableCredentialStatusRequest> {
    UpdateVerifiableCredentialStatusRequestMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, UpdateVerifiableCredentialStatusRequest> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for UpdateVerifiableCredentialStatusRequestMut<'msg> {
  type Proxied = UpdateVerifiableCredentialStatusRequest;
  fn as_view(&self) -> ::__pb::View<'_, UpdateVerifiableCredentialStatusRequest> {
    UpdateVerifiableCredentialStatusRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, UpdateVerifiableCredentialStatusRequest> where 'msg: 'shorter {
    UpdateVerifiableCredentialStatusRequestView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl UpdateVerifiableCredentialStatusRequest {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_new() } } }
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

      __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> UpdateVerifiableCredentialStatusRequestView {
    UpdateVerifiableCredentialStatusRequestView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> UpdateVerifiableCredentialStatusRequestMut {
    UpdateVerifiableCredentialStatusRequestMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // credential_type: optional enum vc_api.v1.CredentialType
  pub fn credential_type(&self) -> crate::CredentialType {
    unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_get_credential_type(self.raw_msg()) }
  }
  pub fn set_credential_type(&mut self, val: crate::CredentialType) {
    unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_set_credential_type(self.raw_msg(), val) }
  }

  // did: optional string
  pub fn did(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_get_did(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_did(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.did_mut().set(val);
  }
  const __DID_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_get_did,
      __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_set_did,
    );
  fn did_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          UpdateVerifiableCredentialStatusRequest::__DID_VTABLE,
        )
      )
    }
  }

  // update_type: optional enum vc_api.v1.UpdateVerifiableCredentialStatusType
  pub fn update_type(&self) -> crate::UpdateVerifiableCredentialStatusType {
    unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_get_update_type(self.raw_msg()) }
  }
  pub fn set_update_type(&mut self, val: crate::UpdateVerifiableCredentialStatusType) {
    unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_set_update_type(self.raw_msg(), val) }
  }

}  // impl UpdateVerifiableCredentialStatusRequest

impl ::__std::ops::Drop for UpdateVerifiableCredentialStatusRequest {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_get_credential_type(raw_msg: ::__pb::__runtime::RawMessage) -> crate::CredentialType;
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_set_credential_type(raw_msg: ::__pb::__runtime::RawMessage, val: crate::CredentialType);

  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_get_did(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_set_did(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_get_update_type(raw_msg: ::__pb::__runtime::RawMessage) -> crate::UpdateVerifiableCredentialStatusType;
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusRequest_set_update_type(raw_msg: ::__pb::__runtime::RawMessage, val: crate::UpdateVerifiableCredentialStatusType);


}  // extern "C" for UpdateVerifiableCredentialStatusRequest


impl UpdateVerifiableCredentialStatusRequest {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> UpdateVerifiableCredentialStatusRequestMut<'a> {
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

impl<'a> UpdateVerifiableCredentialStatusRequestView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct UpdateVerifiableCredentialStatusResponse {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for UpdateVerifiableCredentialStatusResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for UpdateVerifiableCredentialStatusResponse {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `UpdateVerifiableCredentialStatusResponse` is `Sync` because it does not implement interior mutability.
//    Neither does `UpdateVerifiableCredentialStatusResponseMut`.
unsafe impl Sync for UpdateVerifiableCredentialStatusResponse {}

// SAFETY:
// - `UpdateVerifiableCredentialStatusResponse` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for UpdateVerifiableCredentialStatusResponse {}

impl ::__pb::Proxied for UpdateVerifiableCredentialStatusResponse {
  type View<'msg> = UpdateVerifiableCredentialStatusResponseView<'msg>;
  type Mut<'msg> = UpdateVerifiableCredentialStatusResponseMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct UpdateVerifiableCredentialStatusResponseView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for UpdateVerifiableCredentialStatusResponseView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> UpdateVerifiableCredentialStatusResponseView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_serialize(self.raw_msg()) }
  }

}

// SAFETY:
// - `UpdateVerifiableCredentialStatusResponseView` is `Sync` because it does not support mutation.
unsafe impl Sync for UpdateVerifiableCredentialStatusResponseView<'_> {}

// SAFETY:
// - `UpdateVerifiableCredentialStatusResponseView` is `Send` because while its alive a `UpdateVerifiableCredentialStatusResponseMut` cannot.
// - `UpdateVerifiableCredentialStatusResponseView` does not use thread-local data.
unsafe impl Send for UpdateVerifiableCredentialStatusResponseView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for UpdateVerifiableCredentialStatusResponseView<'msg> {
  type Proxied = UpdateVerifiableCredentialStatusResponse;

  fn as_view(&self) -> ::__pb::View<'msg, UpdateVerifiableCredentialStatusResponse> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, UpdateVerifiableCredentialStatusResponse> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for UpdateVerifiableCredentialStatusResponse {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    UpdateVerifiableCredentialStatusResponseView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    UpdateVerifiableCredentialStatusResponseMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for UpdateVerifiableCredentialStatusResponse {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for UpdateVerifiableCredentialStatusResponse {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, UpdateVerifiableCredentialStatusResponse>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, UpdateVerifiableCredentialStatusResponse>;

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

impl<'msg> ::__pb::SettableValue<UpdateVerifiableCredentialStatusResponse> for UpdateVerifiableCredentialStatusResponseView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, UpdateVerifiableCredentialStatusResponse>)
    where UpdateVerifiableCredentialStatusResponse: 'dst {
    unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<UpdateVerifiableCredentialStatusResponse> for UpdateVerifiableCredentialStatusResponse {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, UpdateVerifiableCredentialStatusResponse>)
    where UpdateVerifiableCredentialStatusResponse: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for UpdateVerifiableCredentialStatusResponse {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_copy_from(
        __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for UpdateVerifiableCredentialStatusResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(UpdateVerifiableCredentialStatusResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| UpdateVerifiableCredentialStatusResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for UpdateVerifiableCredentialStatusResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(UpdateVerifiableCredentialStatusResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| UpdateVerifiableCredentialStatusResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for UpdateVerifiableCredentialStatusResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(UpdateVerifiableCredentialStatusResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| UpdateVerifiableCredentialStatusResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for UpdateVerifiableCredentialStatusResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(UpdateVerifiableCredentialStatusResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| UpdateVerifiableCredentialStatusResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for UpdateVerifiableCredentialStatusResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(UpdateVerifiableCredentialStatusResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| UpdateVerifiableCredentialStatusResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for UpdateVerifiableCredentialStatusResponse {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(UpdateVerifiableCredentialStatusResponseView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusResponse_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| UpdateVerifiableCredentialStatusResponseView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct UpdateVerifiableCredentialStatusResponseMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for UpdateVerifiableCredentialStatusResponseMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> UpdateVerifiableCredentialStatusResponseMut<'msg> {
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
// - `UpdateVerifiableCredentialStatusResponseMut` does not perform any shared mutation.
// - `UpdateVerifiableCredentialStatusResponseMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for UpdateVerifiableCredentialStatusResponseMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for UpdateVerifiableCredentialStatusResponseMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, UpdateVerifiableCredentialStatusResponse> {
    UpdateVerifiableCredentialStatusResponseMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, UpdateVerifiableCredentialStatusResponse> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for UpdateVerifiableCredentialStatusResponseMut<'msg> {
  type Proxied = UpdateVerifiableCredentialStatusResponse;
  fn as_view(&self) -> ::__pb::View<'_, UpdateVerifiableCredentialStatusResponse> {
    UpdateVerifiableCredentialStatusResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, UpdateVerifiableCredentialStatusResponse> where 'msg: 'shorter {
    UpdateVerifiableCredentialStatusResponseView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl UpdateVerifiableCredentialStatusResponse {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_new() } } }
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

      __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> UpdateVerifiableCredentialStatusResponseView {
    UpdateVerifiableCredentialStatusResponseView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> UpdateVerifiableCredentialStatusResponseMut {
    UpdateVerifiableCredentialStatusResponseMut::new(::__pb::__internal::Private, &mut self.inner)
  }

}  // impl UpdateVerifiableCredentialStatusResponse

impl ::__std::ops::Drop for UpdateVerifiableCredentialStatusResponse {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__vc_api_v1_UpdateVerifiableCredentialStatusResponse_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);


}  // extern "C" for UpdateVerifiableCredentialStatusResponse


impl UpdateVerifiableCredentialStatusResponse {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> UpdateVerifiableCredentialStatusResponseMut<'a> {
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

impl<'a> UpdateVerifiableCredentialStatusResponseView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CredentialType(i32);

#[allow(non_upper_case_globals)]
impl CredentialType {
  pub const Unspecified: CredentialType = CredentialType(0);
  pub const PermanentResidentCard: CredentialType = CredentialType(1);
  pub const BankCard: CredentialType = CredentialType(2);
  pub const BankAccount: CredentialType = CredentialType(3);
}

impl ::__std::convert::From<CredentialType> for i32 {
  fn from(val: CredentialType) -> i32 {
    val.0
  }
}

impl ::__std::convert::From<i32> for CredentialType {
  fn from(val: i32) -> CredentialType {
    Self(val)
  }
}

impl ::__std::default::Default for CredentialType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::__std::fmt::Debug for CredentialType {
  fn fmt(&self, f: &mut ::__std::fmt::Formatter<'_>) -> ::__std::fmt::Result {
    f.debug_tuple(stringify!(CredentialType)).field(&self.0).finish()
  }
}

impl ::__pb::Proxied for CredentialType {
  type View<'a> = CredentialType;
  type Mut<'a> = ::__pb::PrimitiveMut<'a, CredentialType>;
}

impl ::__pb::ViewProxy<'_> for CredentialType {
  type Proxied = CredentialType;

  fn as_view(&self) -> CredentialType {
    *self
  }

  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, CredentialType> {
    self
  }
}

impl ::__pb::SettableValue<CredentialType> for CredentialType {
  fn set_on<'msg>(
      self,
      private: ::__pb::__internal::Private,
      mut mutator: ::__pb::Mut<'msg, CredentialType>
  ) where CredentialType: 'msg {
    mutator.set_primitive(private, self)
  }
}

impl ::__pb::ProxiedWithPresence for CredentialType {
  type PresentMutData<'a> = ::__pb::__internal::RawVTableOptionalMutatorData<'a, CredentialType>;
  type AbsentMutData<'a> = ::__pb::__internal::RawVTableOptionalMutatorData<'a, CredentialType>;

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

unsafe impl ::__pb::ProxiedInRepeated for CredentialType {
  fn repeated_len(r: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    ::__pb::__runtime::cast_enum_repeated_view(::__pb::__internal::Private, r).len()
  }

  fn repeated_push(r: ::__pb::Mut<::__pb::Repeated<Self>>, val: CredentialType) {
    ::__pb::__runtime::cast_enum_repeated_mut(::__pb::__internal::Private, r).push(val.into())
  }

  fn repeated_clear(r: ::__pb::Mut<::__pb::Repeated<Self>>) {
    ::__pb::__runtime::cast_enum_repeated_mut(::__pb::__internal::Private, r).clear()
  }

  unsafe fn repeated_get_unchecked(
      r: ::__pb::View<::__pb::Repeated<Self>>,
      index: usize,
  ) -> ::__pb::View<CredentialType> {
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
      val: CredentialType,
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

impl ::__pb::__internal::PrimitiveWithRawVTable for CredentialType {}

// SAFETY: this is an enum type
unsafe impl ::__pb::__internal::Enum for CredentialType {
  const NAME: &'static str = "CredentialType";
}

extern "C" {
  fn __rust_proto_thunk__Map_i32_vc_api_v1_CredentialType_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_i32_vc_api_v1_CredentialType_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_i32_vc_api_v1_CredentialType_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_i32_vc_api_v1_CredentialType_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_i32_vc_api_v1_CredentialType_insert(m: ::__pb::__runtime::RawMap, key: i32, value: CredentialType) -> bool;
  fn __rust_proto_thunk__Map_i32_vc_api_v1_CredentialType_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut CredentialType) -> bool;
  fn __rust_proto_thunk__Map_i32_vc_api_v1_CredentialType_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut CredentialType) -> bool;
  fn __rust_proto_thunk__Map_i32_vc_api_v1_CredentialType_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_i32_vc_api_v1_CredentialType_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut CredentialType);
}
impl ::__pb::ProxiedInMapValue<i32> for CredentialType {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_vc_api_v1_CredentialType_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
      unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CredentialType_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
      unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CredentialType_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CredentialType_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CredentialType_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CredentialType_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_CredentialType_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_i32_vc_api_v1_CredentialType_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_i32_vc_api_v1_CredentialType_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_u32_vc_api_v1_CredentialType_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_u32_vc_api_v1_CredentialType_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_u32_vc_api_v1_CredentialType_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_u32_vc_api_v1_CredentialType_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_u32_vc_api_v1_CredentialType_insert(m: ::__pb::__runtime::RawMap, key: u32, value: CredentialType) -> bool;
  fn __rust_proto_thunk__Map_u32_vc_api_v1_CredentialType_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut CredentialType) -> bool;
  fn __rust_proto_thunk__Map_u32_vc_api_v1_CredentialType_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut CredentialType) -> bool;
  fn __rust_proto_thunk__Map_u32_vc_api_v1_CredentialType_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_u32_vc_api_v1_CredentialType_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut CredentialType);
}
impl ::__pb::ProxiedInMapValue<u32> for CredentialType {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_vc_api_v1_CredentialType_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
      unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CredentialType_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
      unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CredentialType_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CredentialType_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CredentialType_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CredentialType_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_CredentialType_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_u32_vc_api_v1_CredentialType_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_u32_vc_api_v1_CredentialType_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_i64_vc_api_v1_CredentialType_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_i64_vc_api_v1_CredentialType_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_i64_vc_api_v1_CredentialType_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_i64_vc_api_v1_CredentialType_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_i64_vc_api_v1_CredentialType_insert(m: ::__pb::__runtime::RawMap, key: i64, value: CredentialType) -> bool;
  fn __rust_proto_thunk__Map_i64_vc_api_v1_CredentialType_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut CredentialType) -> bool;
  fn __rust_proto_thunk__Map_i64_vc_api_v1_CredentialType_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut CredentialType) -> bool;
  fn __rust_proto_thunk__Map_i64_vc_api_v1_CredentialType_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_i64_vc_api_v1_CredentialType_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut CredentialType);
}
impl ::__pb::ProxiedInMapValue<i64> for CredentialType {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_vc_api_v1_CredentialType_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
      unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CredentialType_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
      unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CredentialType_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CredentialType_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CredentialType_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CredentialType_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_CredentialType_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_i64_vc_api_v1_CredentialType_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_i64_vc_api_v1_CredentialType_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_u64_vc_api_v1_CredentialType_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_u64_vc_api_v1_CredentialType_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_u64_vc_api_v1_CredentialType_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_u64_vc_api_v1_CredentialType_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_u64_vc_api_v1_CredentialType_insert(m: ::__pb::__runtime::RawMap, key: u64, value: CredentialType) -> bool;
  fn __rust_proto_thunk__Map_u64_vc_api_v1_CredentialType_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut CredentialType) -> bool;
  fn __rust_proto_thunk__Map_u64_vc_api_v1_CredentialType_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut CredentialType) -> bool;
  fn __rust_proto_thunk__Map_u64_vc_api_v1_CredentialType_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_u64_vc_api_v1_CredentialType_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut CredentialType);
}
impl ::__pb::ProxiedInMapValue<u64> for CredentialType {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_vc_api_v1_CredentialType_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
      unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CredentialType_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
      unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CredentialType_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CredentialType_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CredentialType_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CredentialType_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_CredentialType_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_u64_vc_api_v1_CredentialType_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_u64_vc_api_v1_CredentialType_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_bool_vc_api_v1_CredentialType_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_bool_vc_api_v1_CredentialType_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_bool_vc_api_v1_CredentialType_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_bool_vc_api_v1_CredentialType_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_bool_vc_api_v1_CredentialType_insert(m: ::__pb::__runtime::RawMap, key: bool, value: CredentialType) -> bool;
  fn __rust_proto_thunk__Map_bool_vc_api_v1_CredentialType_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut CredentialType) -> bool;
  fn __rust_proto_thunk__Map_bool_vc_api_v1_CredentialType_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut CredentialType) -> bool;
  fn __rust_proto_thunk__Map_bool_vc_api_v1_CredentialType_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_bool_vc_api_v1_CredentialType_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut CredentialType);
}
impl ::__pb::ProxiedInMapValue<bool> for CredentialType {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_vc_api_v1_CredentialType_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
      unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CredentialType_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
      unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CredentialType_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CredentialType_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CredentialType_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CredentialType_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_CredentialType_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_bool_vc_api_v1_CredentialType_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_bool_vc_api_v1_CredentialType_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialType_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialType_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialType_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialType_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialType_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: CredentialType) -> bool;
  fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialType_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut CredentialType) -> bool;
  fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialType_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut CredentialType) -> bool;
  fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialType_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialType_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut CredentialType);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for CredentialType {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialType_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
      unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialType_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
      unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialType_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialType_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialType_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
      let key = key.as_bytes().into();
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialType_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialType_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialType_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_ProtoStr_vc_api_v1_CredentialType_iter_get,
              |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
              ::__std::convert::identity,
          )
      }
  }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UpdateVerifiableCredentialStatusType(i32);

#[allow(non_upper_case_globals)]
impl UpdateVerifiableCredentialStatusType {
  pub const Unspecified: UpdateVerifiableCredentialStatusType = UpdateVerifiableCredentialStatusType(0);
  pub const Revocation: UpdateVerifiableCredentialStatusType = UpdateVerifiableCredentialStatusType(1);
  pub const Suspension: UpdateVerifiableCredentialStatusType = UpdateVerifiableCredentialStatusType(2);
}

impl ::__std::convert::From<UpdateVerifiableCredentialStatusType> for i32 {
  fn from(val: UpdateVerifiableCredentialStatusType) -> i32 {
    val.0
  }
}

impl ::__std::convert::From<i32> for UpdateVerifiableCredentialStatusType {
  fn from(val: i32) -> UpdateVerifiableCredentialStatusType {
    Self(val)
  }
}

impl ::__std::default::Default for UpdateVerifiableCredentialStatusType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::__std::fmt::Debug for UpdateVerifiableCredentialStatusType {
  fn fmt(&self, f: &mut ::__std::fmt::Formatter<'_>) -> ::__std::fmt::Result {
    f.debug_tuple(stringify!(UpdateVerifiableCredentialStatusType)).field(&self.0).finish()
  }
}

impl ::__pb::Proxied for UpdateVerifiableCredentialStatusType {
  type View<'a> = UpdateVerifiableCredentialStatusType;
  type Mut<'a> = ::__pb::PrimitiveMut<'a, UpdateVerifiableCredentialStatusType>;
}

impl ::__pb::ViewProxy<'_> for UpdateVerifiableCredentialStatusType {
  type Proxied = UpdateVerifiableCredentialStatusType;

  fn as_view(&self) -> UpdateVerifiableCredentialStatusType {
    *self
  }

  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, UpdateVerifiableCredentialStatusType> {
    self
  }
}

impl ::__pb::SettableValue<UpdateVerifiableCredentialStatusType> for UpdateVerifiableCredentialStatusType {
  fn set_on<'msg>(
      self,
      private: ::__pb::__internal::Private,
      mut mutator: ::__pb::Mut<'msg, UpdateVerifiableCredentialStatusType>
  ) where UpdateVerifiableCredentialStatusType: 'msg {
    mutator.set_primitive(private, self)
  }
}

impl ::__pb::ProxiedWithPresence for UpdateVerifiableCredentialStatusType {
  type PresentMutData<'a> = ::__pb::__internal::RawVTableOptionalMutatorData<'a, UpdateVerifiableCredentialStatusType>;
  type AbsentMutData<'a> = ::__pb::__internal::RawVTableOptionalMutatorData<'a, UpdateVerifiableCredentialStatusType>;

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

unsafe impl ::__pb::ProxiedInRepeated for UpdateVerifiableCredentialStatusType {
  fn repeated_len(r: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    ::__pb::__runtime::cast_enum_repeated_view(::__pb::__internal::Private, r).len()
  }

  fn repeated_push(r: ::__pb::Mut<::__pb::Repeated<Self>>, val: UpdateVerifiableCredentialStatusType) {
    ::__pb::__runtime::cast_enum_repeated_mut(::__pb::__internal::Private, r).push(val.into())
  }

  fn repeated_clear(r: ::__pb::Mut<::__pb::Repeated<Self>>) {
    ::__pb::__runtime::cast_enum_repeated_mut(::__pb::__internal::Private, r).clear()
  }

  unsafe fn repeated_get_unchecked(
      r: ::__pb::View<::__pb::Repeated<Self>>,
      index: usize,
  ) -> ::__pb::View<UpdateVerifiableCredentialStatusType> {
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
      val: UpdateVerifiableCredentialStatusType,
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

impl ::__pb::__internal::PrimitiveWithRawVTable for UpdateVerifiableCredentialStatusType {}

// SAFETY: this is an enum type
unsafe impl ::__pb::__internal::Enum for UpdateVerifiableCredentialStatusType {
  const NAME: &'static str = "UpdateVerifiableCredentialStatusType";
}

extern "C" {
  fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusType_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusType_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusType_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusType_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusType_insert(m: ::__pb::__runtime::RawMap, key: i32, value: UpdateVerifiableCredentialStatusType) -> bool;
  fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusType_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut UpdateVerifiableCredentialStatusType) -> bool;
  fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusType_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut UpdateVerifiableCredentialStatusType) -> bool;
  fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusType_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusType_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut UpdateVerifiableCredentialStatusType);
}
impl ::__pb::ProxiedInMapValue<i32> for UpdateVerifiableCredentialStatusType {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusType_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
      unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusType_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
      unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusType_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusType_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusType_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusType_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusType_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusType_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_i32_vc_api_v1_UpdateVerifiableCredentialStatusType_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusType_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusType_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusType_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusType_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusType_insert(m: ::__pb::__runtime::RawMap, key: u32, value: UpdateVerifiableCredentialStatusType) -> bool;
  fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusType_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut UpdateVerifiableCredentialStatusType) -> bool;
  fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusType_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut UpdateVerifiableCredentialStatusType) -> bool;
  fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusType_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusType_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut UpdateVerifiableCredentialStatusType);
}
impl ::__pb::ProxiedInMapValue<u32> for UpdateVerifiableCredentialStatusType {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusType_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
      unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusType_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
      unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusType_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusType_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusType_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusType_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusType_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusType_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_u32_vc_api_v1_UpdateVerifiableCredentialStatusType_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusType_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusType_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusType_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusType_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusType_insert(m: ::__pb::__runtime::RawMap, key: i64, value: UpdateVerifiableCredentialStatusType) -> bool;
  fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusType_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut UpdateVerifiableCredentialStatusType) -> bool;
  fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusType_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut UpdateVerifiableCredentialStatusType) -> bool;
  fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusType_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusType_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut UpdateVerifiableCredentialStatusType);
}
impl ::__pb::ProxiedInMapValue<i64> for UpdateVerifiableCredentialStatusType {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusType_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
      unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusType_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
      unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusType_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusType_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusType_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusType_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusType_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusType_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_i64_vc_api_v1_UpdateVerifiableCredentialStatusType_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusType_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusType_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusType_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusType_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusType_insert(m: ::__pb::__runtime::RawMap, key: u64, value: UpdateVerifiableCredentialStatusType) -> bool;
  fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusType_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut UpdateVerifiableCredentialStatusType) -> bool;
  fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusType_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut UpdateVerifiableCredentialStatusType) -> bool;
  fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusType_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusType_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut UpdateVerifiableCredentialStatusType);
}
impl ::__pb::ProxiedInMapValue<u64> for UpdateVerifiableCredentialStatusType {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusType_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
      unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusType_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
      unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusType_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusType_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusType_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusType_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusType_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusType_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_u64_vc_api_v1_UpdateVerifiableCredentialStatusType_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusType_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusType_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusType_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusType_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusType_insert(m: ::__pb::__runtime::RawMap, key: bool, value: UpdateVerifiableCredentialStatusType) -> bool;
  fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusType_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut UpdateVerifiableCredentialStatusType) -> bool;
  fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusType_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut UpdateVerifiableCredentialStatusType) -> bool;
  fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusType_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusType_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut UpdateVerifiableCredentialStatusType);
}
impl ::__pb::ProxiedInMapValue<bool> for UpdateVerifiableCredentialStatusType {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusType_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
      unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusType_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
      unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusType_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusType_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusType_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusType_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusType_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusType_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_bool_vc_api_v1_UpdateVerifiableCredentialStatusType_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusType_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusType_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusType_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusType_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusType_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: UpdateVerifiableCredentialStatusType) -> bool;
  fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusType_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut UpdateVerifiableCredentialStatusType) -> bool;
  fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusType_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut UpdateVerifiableCredentialStatusType) -> bool;
  fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusType_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusType_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut UpdateVerifiableCredentialStatusType);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for UpdateVerifiableCredentialStatusType {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusType_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
      unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusType_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
      unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusType_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusType_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusType_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
      let key = key.as_bytes().into();
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusType_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusType_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusType_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_ProtoStr_vc_api_v1_UpdateVerifiableCredentialStatusType_iter_get,
              |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
              ::__std::convert::identity,
          )
      }
  }
}

