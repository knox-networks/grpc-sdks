extern crate protobuf_cpp as __pb;
extern crate std as __std;
#[allow(non_camel_case_types)]
pub struct Payload {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for Payload {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for Payload {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `Payload` is `Sync` because it does not implement interior mutability.
//    Neither does `PayloadMut`.
unsafe impl Sync for Payload {}

// SAFETY:
// - `Payload` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Payload {}

impl ::__pb::Proxied for Payload {
  type View<'msg> = PayloadView<'msg>;
  type Mut<'msg> = PayloadMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PayloadView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for PayloadView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__common_Payload_serialize(self.raw_msg()) }
  }

  // payload_type: optional enum common.PayloadType
  pub fn payload_type(self) -> crate::PayloadType {
    unsafe { __rust_proto_thunk__common_Payload_get_payload_type(self.raw_msg()) }
  }

  // data: optional bytes
  pub fn data(self) -> &'msg [u8] {
    let view = unsafe { __rust_proto_thunk__common_Payload_get_data(self.raw_msg()).as_ref() };
    view
  }

}

// SAFETY:
// - `PayloadView` is `Sync` because it does not support mutation.
unsafe impl Sync for PayloadView<'_> {}

// SAFETY:
// - `PayloadView` is `Send` because while its alive a `PayloadMut` cannot.
// - `PayloadView` does not use thread-local data.
unsafe impl Send for PayloadView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for PayloadView<'msg> {
  type Proxied = Payload;

  fn as_view(&self) -> ::__pb::View<'msg, Payload> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, Payload> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for Payload {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    PayloadView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    PayloadMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for Payload {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for Payload {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, Payload>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, Payload>;

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

impl<'msg> ::__pb::SettableValue<Payload> for PayloadView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, Payload>)
    where Payload: 'dst {
    unsafe { __rust_proto_thunk__common_Payload_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<Payload> for Payload {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, Payload>)
    where Payload: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for Payload {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__common_Payload_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__common_Payload_copy_from(
        __rust_proto_thunk__common_Payload_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__common_Payload_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__common_Payload_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__common_Payload_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__common_Payload_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__common_Payload_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_common_Payload_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_common_Payload_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_common_Payload_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_common_Payload_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_common_Payload_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_Payload_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_Payload_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_Payload_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_common_Payload_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for Payload {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_common_Payload_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_common_Payload_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_common_Payload_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_common_Payload_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_common_Payload_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_common_Payload_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(PayloadView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_common_Payload_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_common_Payload_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_common_Payload_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| PayloadView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_common_Payload_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_common_Payload_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_common_Payload_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_common_Payload_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_common_Payload_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_Payload_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_Payload_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_Payload_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_common_Payload_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for Payload {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_common_Payload_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_common_Payload_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_common_Payload_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_common_Payload_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_common_Payload_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_common_Payload_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(PayloadView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_common_Payload_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_common_Payload_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_common_Payload_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| PayloadView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_common_Payload_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_common_Payload_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_common_Payload_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_common_Payload_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_common_Payload_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_Payload_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_Payload_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_Payload_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_common_Payload_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for Payload {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_common_Payload_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_common_Payload_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_common_Payload_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_common_Payload_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_common_Payload_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_common_Payload_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(PayloadView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_common_Payload_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_common_Payload_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_common_Payload_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| PayloadView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_common_Payload_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_common_Payload_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_common_Payload_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_common_Payload_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_common_Payload_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_Payload_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_Payload_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_Payload_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_common_Payload_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for Payload {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_common_Payload_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_common_Payload_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_common_Payload_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_common_Payload_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_common_Payload_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_common_Payload_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(PayloadView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_common_Payload_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_common_Payload_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_common_Payload_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| PayloadView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_common_Payload_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_common_Payload_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_common_Payload_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_common_Payload_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_common_Payload_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_Payload_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_Payload_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_Payload_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_common_Payload_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for Payload {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_common_Payload_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_common_Payload_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_common_Payload_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_common_Payload_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_common_Payload_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_common_Payload_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(PayloadView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_common_Payload_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_common_Payload_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_common_Payload_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| PayloadView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_common_Payload_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_common_Payload_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_common_Payload_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_common_Payload_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_common_Payload_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_Payload_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_Payload_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_Payload_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_common_Payload_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for Payload {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_common_Payload_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_Payload_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_Payload_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_Payload_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_Payload_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_common_Payload_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(PayloadView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_Payload_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_common_Payload_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_common_Payload_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| PayloadView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PayloadMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for PayloadMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> PayloadMut<'msg> {
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


  // payload_type: optional enum common.PayloadType
  pub fn payload_type(&self) -> crate::PayloadType {
    unsafe { __rust_proto_thunk__common_Payload_get_payload_type(self.raw_msg()) }
  }
  pub fn set_payload_type(&mut self, val: crate::PayloadType) {
    unsafe { __rust_proto_thunk__common_Payload_set_payload_type(self.raw_msg(), val) }
  }

  // data: optional bytes
  pub fn data(&self) -> &'_ [u8] {
    let view = unsafe { __rust_proto_thunk__common_Payload_get_data(self.raw_msg()).as_ref() };
    view
  }
  pub fn set_data(&mut self, val: impl ::__pb::SettableValue<[u8]>) {
    self.data_mut().set(val);
  }
  fn data_mut(&mut self) -> ::__pb::Mut<'_, [u8]> {
    unsafe {
      <::__pb::Mut<[u8]>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          Payload::__DATA_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `PayloadMut` does not perform any shared mutation.
// - `PayloadMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for PayloadMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for PayloadMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, Payload> {
    PayloadMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, Payload> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for PayloadMut<'msg> {
  type Proxied = Payload;
  fn as_view(&self) -> ::__pb::View<'_, Payload> {
    PayloadView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, Payload> where 'msg: 'shorter {
    PayloadView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl Payload {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__common_Payload_new() } } }
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

      __rust_proto_thunk__common_Payload_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> PayloadView {
    PayloadView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> PayloadMut {
    PayloadMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // payload_type: optional enum common.PayloadType
  pub fn payload_type(&self) -> crate::PayloadType {
    unsafe { __rust_proto_thunk__common_Payload_get_payload_type(self.raw_msg()) }
  }
  pub fn set_payload_type(&mut self, val: crate::PayloadType) {
    unsafe { __rust_proto_thunk__common_Payload_set_payload_type(self.raw_msg(), val) }
  }

  // data: optional bytes
  pub fn data(&self) -> &'_ [u8] {
    let view = unsafe { __rust_proto_thunk__common_Payload_get_data(self.raw_msg()).as_ref() };
    view
  }
  pub fn set_data(&mut self, val: impl ::__pb::SettableValue<[u8]>) {
    self.data_mut().set(val);
  }
  const __DATA_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__common_Payload_get_data,
      __rust_proto_thunk__common_Payload_set_data,
    );
  fn data_mut(&mut self) -> ::__pb::Mut<'_, [u8]> {
    unsafe {
      <::__pb::Mut<[u8]>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          Payload::__DATA_VTABLE,
        )
      )
    }
  }

}  // impl Payload

impl ::__std::ops::Drop for Payload {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__common_Payload_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__common_Payload_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_Payload_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__common_Payload_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__common_Payload_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__common_Payload_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__common_Payload_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__common_Payload_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_Payload_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_Payload_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_Payload_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__common_Payload_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__common_Payload_get_payload_type(raw_msg: ::__pb::__runtime::RawMessage) -> crate::PayloadType;
  fn __rust_proto_thunk__common_Payload_set_payload_type(raw_msg: ::__pb::__runtime::RawMessage, val: crate::PayloadType);

  fn __rust_proto_thunk__common_Payload_get_data(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__common_Payload_set_data(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for Payload


impl Payload {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> PayloadMut<'a> {
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

impl<'a> PayloadView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct NewTransactionData {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for NewTransactionData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for NewTransactionData {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `NewTransactionData` is `Sync` because it does not implement interior mutability.
//    Neither does `NewTransactionDataMut`.
unsafe impl Sync for NewTransactionData {}

// SAFETY:
// - `NewTransactionData` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for NewTransactionData {}

impl ::__pb::Proxied for NewTransactionData {
  type View<'msg> = NewTransactionDataView<'msg>;
  type Mut<'msg> = NewTransactionDataMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct NewTransactionDataView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for NewTransactionDataView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> NewTransactionDataView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__common_NewTransactionData_serialize(self.raw_msg()) }
  }

  // participant: optional string
  pub fn participant(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_NewTransactionData_get_participant(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // contract: optional message common.Contract
  pub fn contract(self) -> crate::ContractView<'msg> {
                  let submsg = unsafe { __rust_proto_thunk__common_NewTransactionData_get_contract(self.raw_msg()) };
    crate::ContractView::new(::__pb::__internal::Private, submsg)
  }
  pub fn contract_opt(self) ->
  ::__pb::Optional<crate::ContractView<'msg>> {
    let view = self.contract();
    ::__pb::Optional::new(view, self.has_contract())
  }
  pub fn has_contract(self) -> bool {
    unsafe { __rust_proto_thunk__common_NewTransactionData_has_contract(self.raw_msg()) }
  }

}

// SAFETY:
// - `NewTransactionDataView` is `Sync` because it does not support mutation.
unsafe impl Sync for NewTransactionDataView<'_> {}

// SAFETY:
// - `NewTransactionDataView` is `Send` because while its alive a `NewTransactionDataMut` cannot.
// - `NewTransactionDataView` does not use thread-local data.
unsafe impl Send for NewTransactionDataView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for NewTransactionDataView<'msg> {
  type Proxied = NewTransactionData;

  fn as_view(&self) -> ::__pb::View<'msg, NewTransactionData> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, NewTransactionData> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for NewTransactionData {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    NewTransactionDataView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    NewTransactionDataMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for NewTransactionData {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for NewTransactionData {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, NewTransactionData>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, NewTransactionData>;

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

impl<'msg> ::__pb::SettableValue<NewTransactionData> for NewTransactionDataView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, NewTransactionData>)
    where NewTransactionData: 'dst {
    unsafe { __rust_proto_thunk__common_NewTransactionData_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<NewTransactionData> for NewTransactionData {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, NewTransactionData>)
    where NewTransactionData: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for NewTransactionData {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__common_NewTransactionData_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__common_NewTransactionData_copy_from(
        __rust_proto_thunk__common_NewTransactionData_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__common_NewTransactionData_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__common_NewTransactionData_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__common_NewTransactionData_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__common_NewTransactionData_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__common_NewTransactionData_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_common_NewTransactionData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_common_NewTransactionData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_common_NewTransactionData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_common_NewTransactionData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_common_NewTransactionData_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_NewTransactionData_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_NewTransactionData_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_NewTransactionData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_common_NewTransactionData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for NewTransactionData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_common_NewTransactionData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_common_NewTransactionData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_common_NewTransactionData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_common_NewTransactionData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_common_NewTransactionData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_common_NewTransactionData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(NewTransactionDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_common_NewTransactionData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_common_NewTransactionData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_common_NewTransactionData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| NewTransactionDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_common_NewTransactionData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_common_NewTransactionData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_common_NewTransactionData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_common_NewTransactionData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_common_NewTransactionData_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_NewTransactionData_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_NewTransactionData_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_NewTransactionData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_common_NewTransactionData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for NewTransactionData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_common_NewTransactionData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_common_NewTransactionData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_common_NewTransactionData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_common_NewTransactionData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_common_NewTransactionData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_common_NewTransactionData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(NewTransactionDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_common_NewTransactionData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_common_NewTransactionData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_common_NewTransactionData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| NewTransactionDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_common_NewTransactionData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_common_NewTransactionData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_common_NewTransactionData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_common_NewTransactionData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_common_NewTransactionData_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_NewTransactionData_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_NewTransactionData_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_NewTransactionData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_common_NewTransactionData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for NewTransactionData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_common_NewTransactionData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_common_NewTransactionData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_common_NewTransactionData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_common_NewTransactionData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_common_NewTransactionData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_common_NewTransactionData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(NewTransactionDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_common_NewTransactionData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_common_NewTransactionData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_common_NewTransactionData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| NewTransactionDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_common_NewTransactionData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_common_NewTransactionData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_common_NewTransactionData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_common_NewTransactionData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_common_NewTransactionData_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_NewTransactionData_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_NewTransactionData_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_NewTransactionData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_common_NewTransactionData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for NewTransactionData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_common_NewTransactionData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_common_NewTransactionData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_common_NewTransactionData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_common_NewTransactionData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_common_NewTransactionData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_common_NewTransactionData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(NewTransactionDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_common_NewTransactionData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_common_NewTransactionData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_common_NewTransactionData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| NewTransactionDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_common_NewTransactionData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_common_NewTransactionData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_common_NewTransactionData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_common_NewTransactionData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_common_NewTransactionData_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_NewTransactionData_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_NewTransactionData_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_NewTransactionData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_common_NewTransactionData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for NewTransactionData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_common_NewTransactionData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_common_NewTransactionData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_common_NewTransactionData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_common_NewTransactionData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_common_NewTransactionData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_common_NewTransactionData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(NewTransactionDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_common_NewTransactionData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_common_NewTransactionData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_common_NewTransactionData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| NewTransactionDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for NewTransactionData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(NewTransactionDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_common_NewTransactionData_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| NewTransactionDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct NewTransactionDataMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for NewTransactionDataMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> NewTransactionDataMut<'msg> {
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


  // participant: optional string
  pub fn participant(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_NewTransactionData_get_participant(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_participant(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.participant_mut().set(val);
  }
  fn participant_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          NewTransactionData::__PARTICIPANT_VTABLE,
        )
      )
    }
  }

  // contract: optional message common.Contract
  pub fn contract(&self) -> crate::ContractView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__common_NewTransactionData_get_contract(self.raw_msg()) };
    crate::ContractView::new(::__pb::__internal::Private, submsg)
  }
  pub fn contract_mut(&mut self) -> crate::ContractMut<'_> {
    self.contract_entry().or_default()
  }
  fn contract_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::Contract> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__common_NewTransactionData_get_contract,
                                __rust_proto_thunk__common_NewTransactionData_get_mut_contract,
                                __rust_proto_thunk__common_NewTransactionData_clear_contract);
    unsafe {
      let has = self.has_contract();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn contract_opt(&self) ->
  ::__pb::Optional<crate::ContractView<'_>> {
    let view = self.contract();
    ::__pb::Optional::new(view, self.has_contract())
  }
  pub fn set_contract(&mut self, val: impl ::__pb::SettableValue<crate::Contract>) {
    self.contract_entry().set(val);
  }
  pub fn has_contract(&self) -> bool {
    unsafe { __rust_proto_thunk__common_NewTransactionData_has_contract(self.raw_msg()) }
  }
  pub fn clear_contract(&mut self) {
    unsafe { __rust_proto_thunk__common_NewTransactionData_clear_contract(self.raw_msg()) }
  }

}

// SAFETY:
// - `NewTransactionDataMut` does not perform any shared mutation.
// - `NewTransactionDataMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for NewTransactionDataMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for NewTransactionDataMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, NewTransactionData> {
    NewTransactionDataMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, NewTransactionData> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for NewTransactionDataMut<'msg> {
  type Proxied = NewTransactionData;
  fn as_view(&self) -> ::__pb::View<'_, NewTransactionData> {
    NewTransactionDataView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, NewTransactionData> where 'msg: 'shorter {
    NewTransactionDataView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl NewTransactionData {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__common_NewTransactionData_new() } } }
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

      __rust_proto_thunk__common_NewTransactionData_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> NewTransactionDataView {
    NewTransactionDataView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> NewTransactionDataMut {
    NewTransactionDataMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // participant: optional string
  pub fn participant(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_NewTransactionData_get_participant(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_participant(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.participant_mut().set(val);
  }
  const __PARTICIPANT_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__common_NewTransactionData_get_participant,
      __rust_proto_thunk__common_NewTransactionData_set_participant,
    );
  fn participant_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          NewTransactionData::__PARTICIPANT_VTABLE,
        )
      )
    }
  }

  // contract: optional message common.Contract
  pub fn contract(&self) -> crate::ContractView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__common_NewTransactionData_get_contract(self.raw_msg()) };
    crate::ContractView::new(::__pb::__internal::Private, submsg)
  }
  pub fn contract_mut(&mut self) -> crate::ContractMut<'_> {
    self.contract_entry().or_default()
  }
  fn contract_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::Contract> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__common_NewTransactionData_get_contract,
                                __rust_proto_thunk__common_NewTransactionData_get_mut_contract,
                                __rust_proto_thunk__common_NewTransactionData_clear_contract);
    unsafe {
      let has = self.has_contract();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn contract_opt(&self) ->
  ::__pb::Optional<crate::ContractView<'_>> {
    let view = self.contract();
    ::__pb::Optional::new(view, self.has_contract())
  }
  pub fn set_contract(&mut self, val: impl ::__pb::SettableValue<crate::Contract>) {
    self.contract_entry().set(val);
  }
  pub fn has_contract(&self) -> bool {
    unsafe { __rust_proto_thunk__common_NewTransactionData_has_contract(self.raw_msg()) }
  }
  pub fn clear_contract(&mut self) {
    unsafe { __rust_proto_thunk__common_NewTransactionData_clear_contract(self.raw_msg()) }
  }

}  // impl NewTransactionData

impl ::__std::ops::Drop for NewTransactionData {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__common_NewTransactionData_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__common_NewTransactionData_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_NewTransactionData_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__common_NewTransactionData_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__common_NewTransactionData_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__common_NewTransactionData_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__common_NewTransactionData_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__common_NewTransactionData_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_NewTransactionData_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_NewTransactionData_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_NewTransactionData_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__common_NewTransactionData_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__common_NewTransactionData_get_participant(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__common_NewTransactionData_set_participant(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__common_NewTransactionData_get_contract(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_NewTransactionData_get_mut_contract(raw_msg: ::__pb::__runtime::RawMessage)
     -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_NewTransactionData_clear_contract(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__common_NewTransactionData_has_contract(raw_msg: ::__pb::__runtime::RawMessage) -> bool;


}  // extern "C" for NewTransactionData


impl NewTransactionData {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> NewTransactionDataMut<'a> {
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

impl<'a> NewTransactionDataView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct AckProposalData {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for AckProposalData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for AckProposalData {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `AckProposalData` is `Sync` because it does not implement interior mutability.
//    Neither does `AckProposalDataMut`.
unsafe impl Sync for AckProposalData {}

// SAFETY:
// - `AckProposalData` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for AckProposalData {}

impl ::__pb::Proxied for AckProposalData {
  type View<'msg> = AckProposalDataView<'msg>;
  type Mut<'msg> = AckProposalDataMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AckProposalDataView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for AckProposalDataView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> AckProposalDataView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__common_AckProposalData_serialize(self.raw_msg()) }
  }

  // participant: optional string
  pub fn participant(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_AckProposalData_get_participant(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // contract: optional message common.Contract
  pub fn contract(self) -> crate::ContractView<'msg> {
                  let submsg = unsafe { __rust_proto_thunk__common_AckProposalData_get_contract(self.raw_msg()) };
    crate::ContractView::new(::__pb::__internal::Private, submsg)
  }
  pub fn contract_opt(self) ->
  ::__pb::Optional<crate::ContractView<'msg>> {
    let view = self.contract();
    ::__pb::Optional::new(view, self.has_contract())
  }
  pub fn has_contract(self) -> bool {
    unsafe { __rust_proto_thunk__common_AckProposalData_has_contract(self.raw_msg()) }
  }

  // originator: optional string
  pub fn originator(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_AckProposalData_get_originator(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

}

// SAFETY:
// - `AckProposalDataView` is `Sync` because it does not support mutation.
unsafe impl Sync for AckProposalDataView<'_> {}

// SAFETY:
// - `AckProposalDataView` is `Send` because while its alive a `AckProposalDataMut` cannot.
// - `AckProposalDataView` does not use thread-local data.
unsafe impl Send for AckProposalDataView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for AckProposalDataView<'msg> {
  type Proxied = AckProposalData;

  fn as_view(&self) -> ::__pb::View<'msg, AckProposalData> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, AckProposalData> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for AckProposalData {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    AckProposalDataView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    AckProposalDataMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for AckProposalData {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for AckProposalData {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, AckProposalData>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, AckProposalData>;

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

impl<'msg> ::__pb::SettableValue<AckProposalData> for AckProposalDataView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, AckProposalData>)
    where AckProposalData: 'dst {
    unsafe { __rust_proto_thunk__common_AckProposalData_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<AckProposalData> for AckProposalData {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, AckProposalData>)
    where AckProposalData: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for AckProposalData {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__common_AckProposalData_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__common_AckProposalData_copy_from(
        __rust_proto_thunk__common_AckProposalData_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__common_AckProposalData_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__common_AckProposalData_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__common_AckProposalData_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__common_AckProposalData_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__common_AckProposalData_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_common_AckProposalData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_common_AckProposalData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_common_AckProposalData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_common_AckProposalData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_common_AckProposalData_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_AckProposalData_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_AckProposalData_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_AckProposalData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_common_AckProposalData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for AckProposalData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_common_AckProposalData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_common_AckProposalData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_common_AckProposalData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_common_AckProposalData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_common_AckProposalData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_common_AckProposalData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AckProposalDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_common_AckProposalData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_common_AckProposalData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_common_AckProposalData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AckProposalDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_common_AckProposalData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_common_AckProposalData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_common_AckProposalData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_common_AckProposalData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_common_AckProposalData_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_AckProposalData_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_AckProposalData_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_AckProposalData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_common_AckProposalData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for AckProposalData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_common_AckProposalData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_common_AckProposalData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_common_AckProposalData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_common_AckProposalData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_common_AckProposalData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_common_AckProposalData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AckProposalDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_common_AckProposalData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_common_AckProposalData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_common_AckProposalData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AckProposalDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_common_AckProposalData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_common_AckProposalData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_common_AckProposalData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_common_AckProposalData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_common_AckProposalData_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_AckProposalData_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_AckProposalData_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_AckProposalData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_common_AckProposalData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for AckProposalData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_common_AckProposalData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_common_AckProposalData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_common_AckProposalData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_common_AckProposalData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_common_AckProposalData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_common_AckProposalData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AckProposalDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_common_AckProposalData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_common_AckProposalData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_common_AckProposalData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AckProposalDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_common_AckProposalData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_common_AckProposalData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_common_AckProposalData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_common_AckProposalData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_common_AckProposalData_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_AckProposalData_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_AckProposalData_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_AckProposalData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_common_AckProposalData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for AckProposalData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_common_AckProposalData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_common_AckProposalData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_common_AckProposalData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_common_AckProposalData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_common_AckProposalData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_common_AckProposalData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AckProposalDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_common_AckProposalData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_common_AckProposalData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_common_AckProposalData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AckProposalDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_common_AckProposalData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_common_AckProposalData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_common_AckProposalData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_common_AckProposalData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_common_AckProposalData_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_AckProposalData_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_AckProposalData_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_AckProposalData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_common_AckProposalData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for AckProposalData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_common_AckProposalData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_common_AckProposalData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_common_AckProposalData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_common_AckProposalData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_common_AckProposalData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_common_AckProposalData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AckProposalDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_common_AckProposalData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_common_AckProposalData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_common_AckProposalData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| AckProposalDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for AckProposalData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(AckProposalDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_common_AckProposalData_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| AckProposalDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AckProposalDataMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for AckProposalDataMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> AckProposalDataMut<'msg> {
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


  // participant: optional string
  pub fn participant(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_AckProposalData_get_participant(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_participant(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.participant_mut().set(val);
  }
  fn participant_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          AckProposalData::__PARTICIPANT_VTABLE,
        )
      )
    }
  }

  // contract: optional message common.Contract
  pub fn contract(&self) -> crate::ContractView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__common_AckProposalData_get_contract(self.raw_msg()) };
    crate::ContractView::new(::__pb::__internal::Private, submsg)
  }
  pub fn contract_mut(&mut self) -> crate::ContractMut<'_> {
    self.contract_entry().or_default()
  }
  fn contract_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::Contract> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__common_AckProposalData_get_contract,
                                __rust_proto_thunk__common_AckProposalData_get_mut_contract,
                                __rust_proto_thunk__common_AckProposalData_clear_contract);
    unsafe {
      let has = self.has_contract();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn contract_opt(&self) ->
  ::__pb::Optional<crate::ContractView<'_>> {
    let view = self.contract();
    ::__pb::Optional::new(view, self.has_contract())
  }
  pub fn set_contract(&mut self, val: impl ::__pb::SettableValue<crate::Contract>) {
    self.contract_entry().set(val);
  }
  pub fn has_contract(&self) -> bool {
    unsafe { __rust_proto_thunk__common_AckProposalData_has_contract(self.raw_msg()) }
  }
  pub fn clear_contract(&mut self) {
    unsafe { __rust_proto_thunk__common_AckProposalData_clear_contract(self.raw_msg()) }
  }

  // originator: optional string
  pub fn originator(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_AckProposalData_get_originator(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_originator(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.originator_mut().set(val);
  }
  fn originator_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          AckProposalData::__ORIGINATOR_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `AckProposalDataMut` does not perform any shared mutation.
// - `AckProposalDataMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for AckProposalDataMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for AckProposalDataMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, AckProposalData> {
    AckProposalDataMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, AckProposalData> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for AckProposalDataMut<'msg> {
  type Proxied = AckProposalData;
  fn as_view(&self) -> ::__pb::View<'_, AckProposalData> {
    AckProposalDataView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, AckProposalData> where 'msg: 'shorter {
    AckProposalDataView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl AckProposalData {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__common_AckProposalData_new() } } }
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

      __rust_proto_thunk__common_AckProposalData_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> AckProposalDataView {
    AckProposalDataView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> AckProposalDataMut {
    AckProposalDataMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // participant: optional string
  pub fn participant(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_AckProposalData_get_participant(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_participant(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.participant_mut().set(val);
  }
  const __PARTICIPANT_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__common_AckProposalData_get_participant,
      __rust_proto_thunk__common_AckProposalData_set_participant,
    );
  fn participant_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          AckProposalData::__PARTICIPANT_VTABLE,
        )
      )
    }
  }

  // contract: optional message common.Contract
  pub fn contract(&self) -> crate::ContractView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__common_AckProposalData_get_contract(self.raw_msg()) };
    crate::ContractView::new(::__pb::__internal::Private, submsg)
  }
  pub fn contract_mut(&mut self) -> crate::ContractMut<'_> {
    self.contract_entry().or_default()
  }
  fn contract_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::Contract> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__common_AckProposalData_get_contract,
                                __rust_proto_thunk__common_AckProposalData_get_mut_contract,
                                __rust_proto_thunk__common_AckProposalData_clear_contract);
    unsafe {
      let has = self.has_contract();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn contract_opt(&self) ->
  ::__pb::Optional<crate::ContractView<'_>> {
    let view = self.contract();
    ::__pb::Optional::new(view, self.has_contract())
  }
  pub fn set_contract(&mut self, val: impl ::__pb::SettableValue<crate::Contract>) {
    self.contract_entry().set(val);
  }
  pub fn has_contract(&self) -> bool {
    unsafe { __rust_proto_thunk__common_AckProposalData_has_contract(self.raw_msg()) }
  }
  pub fn clear_contract(&mut self) {
    unsafe { __rust_proto_thunk__common_AckProposalData_clear_contract(self.raw_msg()) }
  }

  // originator: optional string
  pub fn originator(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_AckProposalData_get_originator(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_originator(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.originator_mut().set(val);
  }
  const __ORIGINATOR_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__common_AckProposalData_get_originator,
      __rust_proto_thunk__common_AckProposalData_set_originator,
    );
  fn originator_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          AckProposalData::__ORIGINATOR_VTABLE,
        )
      )
    }
  }

}  // impl AckProposalData

impl ::__std::ops::Drop for AckProposalData {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__common_AckProposalData_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__common_AckProposalData_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_AckProposalData_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__common_AckProposalData_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__common_AckProposalData_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__common_AckProposalData_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__common_AckProposalData_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__common_AckProposalData_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_AckProposalData_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_AckProposalData_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_AckProposalData_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__common_AckProposalData_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__common_AckProposalData_get_participant(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__common_AckProposalData_set_participant(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__common_AckProposalData_get_contract(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_AckProposalData_get_mut_contract(raw_msg: ::__pb::__runtime::RawMessage)
     -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_AckProposalData_clear_contract(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__common_AckProposalData_has_contract(raw_msg: ::__pb::__runtime::RawMessage) -> bool;

  fn __rust_proto_thunk__common_AckProposalData_get_originator(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__common_AckProposalData_set_originator(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for AckProposalData


impl AckProposalData {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> AckProposalDataMut<'a> {
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

impl<'a> AckProposalDataView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct ProposalAckNackData {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for ProposalAckNackData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for ProposalAckNackData {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `ProposalAckNackData` is `Sync` because it does not implement interior mutability.
//    Neither does `ProposalAckNackDataMut`.
unsafe impl Sync for ProposalAckNackData {}

// SAFETY:
// - `ProposalAckNackData` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ProposalAckNackData {}

impl ::__pb::Proxied for ProposalAckNackData {
  type View<'msg> = ProposalAckNackDataView<'msg>;
  type Mut<'msg> = ProposalAckNackDataMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ProposalAckNackDataView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for ProposalAckNackDataView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> ProposalAckNackDataView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__common_ProposalAckNackData_serialize(self.raw_msg()) }
  }

  // participant: optional string
  pub fn participant(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_ProposalAckNackData_get_participant(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // contract: optional message common.Contract
  pub fn contract(self) -> crate::ContractView<'msg> {
                  let submsg = unsafe { __rust_proto_thunk__common_ProposalAckNackData_get_contract(self.raw_msg()) };
    crate::ContractView::new(::__pb::__internal::Private, submsg)
  }
  pub fn contract_opt(self) ->
  ::__pb::Optional<crate::ContractView<'msg>> {
    let view = self.contract();
    ::__pb::Optional::new(view, self.has_contract())
  }
  pub fn has_contract(self) -> bool {
    unsafe { __rust_proto_thunk__common_ProposalAckNackData_has_contract(self.raw_msg()) }
  }

  // originator: optional string
  pub fn originator(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_ProposalAckNackData_get_originator(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

}

// SAFETY:
// - `ProposalAckNackDataView` is `Sync` because it does not support mutation.
unsafe impl Sync for ProposalAckNackDataView<'_> {}

// SAFETY:
// - `ProposalAckNackDataView` is `Send` because while its alive a `ProposalAckNackDataMut` cannot.
// - `ProposalAckNackDataView` does not use thread-local data.
unsafe impl Send for ProposalAckNackDataView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for ProposalAckNackDataView<'msg> {
  type Proxied = ProposalAckNackData;

  fn as_view(&self) -> ::__pb::View<'msg, ProposalAckNackData> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, ProposalAckNackData> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for ProposalAckNackData {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    ProposalAckNackDataView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    ProposalAckNackDataMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for ProposalAckNackData {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for ProposalAckNackData {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, ProposalAckNackData>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, ProposalAckNackData>;

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

impl<'msg> ::__pb::SettableValue<ProposalAckNackData> for ProposalAckNackDataView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, ProposalAckNackData>)
    where ProposalAckNackData: 'dst {
    unsafe { __rust_proto_thunk__common_ProposalAckNackData_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<ProposalAckNackData> for ProposalAckNackData {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, ProposalAckNackData>)
    where ProposalAckNackData: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for ProposalAckNackData {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__common_ProposalAckNackData_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__common_ProposalAckNackData_copy_from(
        __rust_proto_thunk__common_ProposalAckNackData_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__common_ProposalAckNackData_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__common_ProposalAckNackData_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__common_ProposalAckNackData_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__common_ProposalAckNackData_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__common_ProposalAckNackData_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_common_ProposalAckNackData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_common_ProposalAckNackData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_common_ProposalAckNackData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_common_ProposalAckNackData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_common_ProposalAckNackData_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_ProposalAckNackData_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_ProposalAckNackData_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_ProposalAckNackData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_common_ProposalAckNackData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for ProposalAckNackData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_common_ProposalAckNackData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_common_ProposalAckNackData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_common_ProposalAckNackData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_common_ProposalAckNackData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_common_ProposalAckNackData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_common_ProposalAckNackData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ProposalAckNackDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_common_ProposalAckNackData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_common_ProposalAckNackData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_common_ProposalAckNackData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ProposalAckNackDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_common_ProposalAckNackData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_common_ProposalAckNackData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_common_ProposalAckNackData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_common_ProposalAckNackData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_common_ProposalAckNackData_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_ProposalAckNackData_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_ProposalAckNackData_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_ProposalAckNackData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_common_ProposalAckNackData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for ProposalAckNackData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_common_ProposalAckNackData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_common_ProposalAckNackData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_common_ProposalAckNackData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_common_ProposalAckNackData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_common_ProposalAckNackData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_common_ProposalAckNackData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ProposalAckNackDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_common_ProposalAckNackData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_common_ProposalAckNackData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_common_ProposalAckNackData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ProposalAckNackDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_common_ProposalAckNackData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_common_ProposalAckNackData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_common_ProposalAckNackData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_common_ProposalAckNackData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_common_ProposalAckNackData_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_ProposalAckNackData_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_ProposalAckNackData_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_ProposalAckNackData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_common_ProposalAckNackData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for ProposalAckNackData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_common_ProposalAckNackData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_common_ProposalAckNackData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_common_ProposalAckNackData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_common_ProposalAckNackData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_common_ProposalAckNackData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_common_ProposalAckNackData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ProposalAckNackDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_common_ProposalAckNackData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_common_ProposalAckNackData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_common_ProposalAckNackData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ProposalAckNackDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_common_ProposalAckNackData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_common_ProposalAckNackData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_common_ProposalAckNackData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_common_ProposalAckNackData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_common_ProposalAckNackData_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_ProposalAckNackData_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_ProposalAckNackData_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_ProposalAckNackData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_common_ProposalAckNackData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for ProposalAckNackData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_common_ProposalAckNackData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_common_ProposalAckNackData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_common_ProposalAckNackData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_common_ProposalAckNackData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_common_ProposalAckNackData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_common_ProposalAckNackData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ProposalAckNackDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_common_ProposalAckNackData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_common_ProposalAckNackData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_common_ProposalAckNackData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ProposalAckNackDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_common_ProposalAckNackData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_common_ProposalAckNackData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_common_ProposalAckNackData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_common_ProposalAckNackData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_common_ProposalAckNackData_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_ProposalAckNackData_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_ProposalAckNackData_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_ProposalAckNackData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_common_ProposalAckNackData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for ProposalAckNackData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_common_ProposalAckNackData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_common_ProposalAckNackData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_common_ProposalAckNackData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_common_ProposalAckNackData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_common_ProposalAckNackData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_common_ProposalAckNackData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ProposalAckNackDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_common_ProposalAckNackData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_common_ProposalAckNackData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_common_ProposalAckNackData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ProposalAckNackDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for ProposalAckNackData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ProposalAckNackDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_common_ProposalAckNackData_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| ProposalAckNackDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ProposalAckNackDataMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for ProposalAckNackDataMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> ProposalAckNackDataMut<'msg> {
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


  // participant: optional string
  pub fn participant(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_ProposalAckNackData_get_participant(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_participant(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.participant_mut().set(val);
  }
  fn participant_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          ProposalAckNackData::__PARTICIPANT_VTABLE,
        )
      )
    }
  }

  // contract: optional message common.Contract
  pub fn contract(&self) -> crate::ContractView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__common_ProposalAckNackData_get_contract(self.raw_msg()) };
    crate::ContractView::new(::__pb::__internal::Private, submsg)
  }
  pub fn contract_mut(&mut self) -> crate::ContractMut<'_> {
    self.contract_entry().or_default()
  }
  fn contract_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::Contract> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__common_ProposalAckNackData_get_contract,
                                __rust_proto_thunk__common_ProposalAckNackData_get_mut_contract,
                                __rust_proto_thunk__common_ProposalAckNackData_clear_contract);
    unsafe {
      let has = self.has_contract();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn contract_opt(&self) ->
  ::__pb::Optional<crate::ContractView<'_>> {
    let view = self.contract();
    ::__pb::Optional::new(view, self.has_contract())
  }
  pub fn set_contract(&mut self, val: impl ::__pb::SettableValue<crate::Contract>) {
    self.contract_entry().set(val);
  }
  pub fn has_contract(&self) -> bool {
    unsafe { __rust_proto_thunk__common_ProposalAckNackData_has_contract(self.raw_msg()) }
  }
  pub fn clear_contract(&mut self) {
    unsafe { __rust_proto_thunk__common_ProposalAckNackData_clear_contract(self.raw_msg()) }
  }

  // originator: optional string
  pub fn originator(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_ProposalAckNackData_get_originator(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_originator(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.originator_mut().set(val);
  }
  fn originator_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          ProposalAckNackData::__ORIGINATOR_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `ProposalAckNackDataMut` does not perform any shared mutation.
// - `ProposalAckNackDataMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for ProposalAckNackDataMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for ProposalAckNackDataMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, ProposalAckNackData> {
    ProposalAckNackDataMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, ProposalAckNackData> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for ProposalAckNackDataMut<'msg> {
  type Proxied = ProposalAckNackData;
  fn as_view(&self) -> ::__pb::View<'_, ProposalAckNackData> {
    ProposalAckNackDataView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, ProposalAckNackData> where 'msg: 'shorter {
    ProposalAckNackDataView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl ProposalAckNackData {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__common_ProposalAckNackData_new() } } }
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

      __rust_proto_thunk__common_ProposalAckNackData_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> ProposalAckNackDataView {
    ProposalAckNackDataView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> ProposalAckNackDataMut {
    ProposalAckNackDataMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // participant: optional string
  pub fn participant(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_ProposalAckNackData_get_participant(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_participant(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.participant_mut().set(val);
  }
  const __PARTICIPANT_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__common_ProposalAckNackData_get_participant,
      __rust_proto_thunk__common_ProposalAckNackData_set_participant,
    );
  fn participant_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          ProposalAckNackData::__PARTICIPANT_VTABLE,
        )
      )
    }
  }

  // contract: optional message common.Contract
  pub fn contract(&self) -> crate::ContractView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__common_ProposalAckNackData_get_contract(self.raw_msg()) };
    crate::ContractView::new(::__pb::__internal::Private, submsg)
  }
  pub fn contract_mut(&mut self) -> crate::ContractMut<'_> {
    self.contract_entry().or_default()
  }
  fn contract_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::Contract> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__common_ProposalAckNackData_get_contract,
                                __rust_proto_thunk__common_ProposalAckNackData_get_mut_contract,
                                __rust_proto_thunk__common_ProposalAckNackData_clear_contract);
    unsafe {
      let has = self.has_contract();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn contract_opt(&self) ->
  ::__pb::Optional<crate::ContractView<'_>> {
    let view = self.contract();
    ::__pb::Optional::new(view, self.has_contract())
  }
  pub fn set_contract(&mut self, val: impl ::__pb::SettableValue<crate::Contract>) {
    self.contract_entry().set(val);
  }
  pub fn has_contract(&self) -> bool {
    unsafe { __rust_proto_thunk__common_ProposalAckNackData_has_contract(self.raw_msg()) }
  }
  pub fn clear_contract(&mut self) {
    unsafe { __rust_proto_thunk__common_ProposalAckNackData_clear_contract(self.raw_msg()) }
  }

  // originator: optional string
  pub fn originator(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_ProposalAckNackData_get_originator(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_originator(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.originator_mut().set(val);
  }
  const __ORIGINATOR_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__common_ProposalAckNackData_get_originator,
      __rust_proto_thunk__common_ProposalAckNackData_set_originator,
    );
  fn originator_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          ProposalAckNackData::__ORIGINATOR_VTABLE,
        )
      )
    }
  }

}  // impl ProposalAckNackData

impl ::__std::ops::Drop for ProposalAckNackData {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__common_ProposalAckNackData_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__common_ProposalAckNackData_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_ProposalAckNackData_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__common_ProposalAckNackData_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__common_ProposalAckNackData_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__common_ProposalAckNackData_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__common_ProposalAckNackData_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__common_ProposalAckNackData_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_ProposalAckNackData_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_ProposalAckNackData_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_ProposalAckNackData_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__common_ProposalAckNackData_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__common_ProposalAckNackData_get_participant(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__common_ProposalAckNackData_set_participant(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__common_ProposalAckNackData_get_contract(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_ProposalAckNackData_get_mut_contract(raw_msg: ::__pb::__runtime::RawMessage)
     -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_ProposalAckNackData_clear_contract(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__common_ProposalAckNackData_has_contract(raw_msg: ::__pb::__runtime::RawMessage) -> bool;

  fn __rust_proto_thunk__common_ProposalAckNackData_get_originator(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__common_ProposalAckNackData_set_originator(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for ProposalAckNackData


impl ProposalAckNackData {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> ProposalAckNackDataMut<'a> {
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

impl<'a> ProposalAckNackDataView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct RevealSecretData {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for RevealSecretData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for RevealSecretData {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `RevealSecretData` is `Sync` because it does not implement interior mutability.
//    Neither does `RevealSecretDataMut`.
unsafe impl Sync for RevealSecretData {}

// SAFETY:
// - `RevealSecretData` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for RevealSecretData {}

impl ::__pb::Proxied for RevealSecretData {
  type View<'msg> = RevealSecretDataView<'msg>;
  type Mut<'msg> = RevealSecretDataMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RevealSecretDataView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for RevealSecretDataView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> RevealSecretDataView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__common_RevealSecretData_serialize(self.raw_msg()) }
  }

  // participant: optional string
  pub fn participant(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_RevealSecretData_get_participant(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // contract_id: optional string
  pub fn contract_id(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_RevealSecretData_get_contract_id(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

}

// SAFETY:
// - `RevealSecretDataView` is `Sync` because it does not support mutation.
unsafe impl Sync for RevealSecretDataView<'_> {}

// SAFETY:
// - `RevealSecretDataView` is `Send` because while its alive a `RevealSecretDataMut` cannot.
// - `RevealSecretDataView` does not use thread-local data.
unsafe impl Send for RevealSecretDataView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for RevealSecretDataView<'msg> {
  type Proxied = RevealSecretData;

  fn as_view(&self) -> ::__pb::View<'msg, RevealSecretData> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, RevealSecretData> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for RevealSecretData {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    RevealSecretDataView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    RevealSecretDataMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for RevealSecretData {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for RevealSecretData {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, RevealSecretData>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, RevealSecretData>;

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

impl<'msg> ::__pb::SettableValue<RevealSecretData> for RevealSecretDataView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, RevealSecretData>)
    where RevealSecretData: 'dst {
    unsafe { __rust_proto_thunk__common_RevealSecretData_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<RevealSecretData> for RevealSecretData {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, RevealSecretData>)
    where RevealSecretData: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for RevealSecretData {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__common_RevealSecretData_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__common_RevealSecretData_copy_from(
        __rust_proto_thunk__common_RevealSecretData_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__common_RevealSecretData_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__common_RevealSecretData_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__common_RevealSecretData_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__common_RevealSecretData_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__common_RevealSecretData_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_common_RevealSecretData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_common_RevealSecretData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_common_RevealSecretData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_common_RevealSecretData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_common_RevealSecretData_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_RevealSecretData_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_RevealSecretData_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_RevealSecretData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_common_RevealSecretData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for RevealSecretData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_common_RevealSecretData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_common_RevealSecretData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_common_RevealSecretData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_common_RevealSecretData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_common_RevealSecretData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_common_RevealSecretData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(RevealSecretDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_common_RevealSecretData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_common_RevealSecretData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_common_RevealSecretData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| RevealSecretDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_common_RevealSecretData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_common_RevealSecretData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_common_RevealSecretData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_common_RevealSecretData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_common_RevealSecretData_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_RevealSecretData_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_RevealSecretData_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_RevealSecretData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_common_RevealSecretData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for RevealSecretData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_common_RevealSecretData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_common_RevealSecretData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_common_RevealSecretData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_common_RevealSecretData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_common_RevealSecretData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_common_RevealSecretData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(RevealSecretDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_common_RevealSecretData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_common_RevealSecretData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_common_RevealSecretData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| RevealSecretDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_common_RevealSecretData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_common_RevealSecretData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_common_RevealSecretData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_common_RevealSecretData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_common_RevealSecretData_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_RevealSecretData_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_RevealSecretData_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_RevealSecretData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_common_RevealSecretData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for RevealSecretData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_common_RevealSecretData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_common_RevealSecretData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_common_RevealSecretData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_common_RevealSecretData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_common_RevealSecretData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_common_RevealSecretData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(RevealSecretDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_common_RevealSecretData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_common_RevealSecretData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_common_RevealSecretData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| RevealSecretDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_common_RevealSecretData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_common_RevealSecretData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_common_RevealSecretData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_common_RevealSecretData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_common_RevealSecretData_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_RevealSecretData_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_RevealSecretData_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_RevealSecretData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_common_RevealSecretData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for RevealSecretData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_common_RevealSecretData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_common_RevealSecretData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_common_RevealSecretData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_common_RevealSecretData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_common_RevealSecretData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_common_RevealSecretData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(RevealSecretDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_common_RevealSecretData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_common_RevealSecretData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_common_RevealSecretData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| RevealSecretDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_common_RevealSecretData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_common_RevealSecretData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_common_RevealSecretData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_common_RevealSecretData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_common_RevealSecretData_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_RevealSecretData_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_RevealSecretData_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_RevealSecretData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_common_RevealSecretData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for RevealSecretData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_common_RevealSecretData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_common_RevealSecretData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_common_RevealSecretData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_common_RevealSecretData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_common_RevealSecretData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_common_RevealSecretData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(RevealSecretDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_common_RevealSecretData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_common_RevealSecretData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_common_RevealSecretData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| RevealSecretDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for RevealSecretData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(RevealSecretDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_common_RevealSecretData_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| RevealSecretDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RevealSecretDataMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for RevealSecretDataMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> RevealSecretDataMut<'msg> {
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


  // participant: optional string
  pub fn participant(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_RevealSecretData_get_participant(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_participant(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.participant_mut().set(val);
  }
  fn participant_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          RevealSecretData::__PARTICIPANT_VTABLE,
        )
      )
    }
  }

  // contract_id: optional string
  pub fn contract_id(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_RevealSecretData_get_contract_id(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_contract_id(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.contract_id_mut().set(val);
  }
  fn contract_id_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          RevealSecretData::__CONTRACT_ID_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `RevealSecretDataMut` does not perform any shared mutation.
// - `RevealSecretDataMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for RevealSecretDataMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for RevealSecretDataMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, RevealSecretData> {
    RevealSecretDataMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, RevealSecretData> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for RevealSecretDataMut<'msg> {
  type Proxied = RevealSecretData;
  fn as_view(&self) -> ::__pb::View<'_, RevealSecretData> {
    RevealSecretDataView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, RevealSecretData> where 'msg: 'shorter {
    RevealSecretDataView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl RevealSecretData {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__common_RevealSecretData_new() } } }
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

      __rust_proto_thunk__common_RevealSecretData_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> RevealSecretDataView {
    RevealSecretDataView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> RevealSecretDataMut {
    RevealSecretDataMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // participant: optional string
  pub fn participant(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_RevealSecretData_get_participant(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_participant(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.participant_mut().set(val);
  }
  const __PARTICIPANT_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__common_RevealSecretData_get_participant,
      __rust_proto_thunk__common_RevealSecretData_set_participant,
    );
  fn participant_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          RevealSecretData::__PARTICIPANT_VTABLE,
        )
      )
    }
  }

  // contract_id: optional string
  pub fn contract_id(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_RevealSecretData_get_contract_id(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_contract_id(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.contract_id_mut().set(val);
  }
  const __CONTRACT_ID_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__common_RevealSecretData_get_contract_id,
      __rust_proto_thunk__common_RevealSecretData_set_contract_id,
    );
  fn contract_id_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          RevealSecretData::__CONTRACT_ID_VTABLE,
        )
      )
    }
  }

}  // impl RevealSecretData

impl ::__std::ops::Drop for RevealSecretData {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__common_RevealSecretData_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__common_RevealSecretData_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_RevealSecretData_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__common_RevealSecretData_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__common_RevealSecretData_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__common_RevealSecretData_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__common_RevealSecretData_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__common_RevealSecretData_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_RevealSecretData_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_RevealSecretData_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_RevealSecretData_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__common_RevealSecretData_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__common_RevealSecretData_get_participant(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__common_RevealSecretData_set_participant(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__common_RevealSecretData_get_contract_id(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__common_RevealSecretData_set_contract_id(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for RevealSecretData


impl RevealSecretData {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> RevealSecretDataMut<'a> {
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

impl<'a> RevealSecretDataView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct SecretRevealedData {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for SecretRevealedData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for SecretRevealedData {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `SecretRevealedData` is `Sync` because it does not implement interior mutability.
//    Neither does `SecretRevealedDataMut`.
unsafe impl Sync for SecretRevealedData {}

// SAFETY:
// - `SecretRevealedData` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for SecretRevealedData {}

impl ::__pb::Proxied for SecretRevealedData {
  type View<'msg> = SecretRevealedDataView<'msg>;
  type Mut<'msg> = SecretRevealedDataMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SecretRevealedDataView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for SecretRevealedDataView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> SecretRevealedDataView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__common_SecretRevealedData_serialize(self.raw_msg()) }
  }

  // participant: optional string
  pub fn participant(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_SecretRevealedData_get_participant(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // contract_id: optional string
  pub fn contract_id(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_SecretRevealedData_get_contract_id(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // secret: optional string
  pub fn secret(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_SecretRevealedData_get_secret(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

}

// SAFETY:
// - `SecretRevealedDataView` is `Sync` because it does not support mutation.
unsafe impl Sync for SecretRevealedDataView<'_> {}

// SAFETY:
// - `SecretRevealedDataView` is `Send` because while its alive a `SecretRevealedDataMut` cannot.
// - `SecretRevealedDataView` does not use thread-local data.
unsafe impl Send for SecretRevealedDataView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for SecretRevealedDataView<'msg> {
  type Proxied = SecretRevealedData;

  fn as_view(&self) -> ::__pb::View<'msg, SecretRevealedData> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, SecretRevealedData> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for SecretRevealedData {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    SecretRevealedDataView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    SecretRevealedDataMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for SecretRevealedData {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for SecretRevealedData {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, SecretRevealedData>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, SecretRevealedData>;

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

impl<'msg> ::__pb::SettableValue<SecretRevealedData> for SecretRevealedDataView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, SecretRevealedData>)
    where SecretRevealedData: 'dst {
    unsafe { __rust_proto_thunk__common_SecretRevealedData_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<SecretRevealedData> for SecretRevealedData {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, SecretRevealedData>)
    where SecretRevealedData: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for SecretRevealedData {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__common_SecretRevealedData_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__common_SecretRevealedData_copy_from(
        __rust_proto_thunk__common_SecretRevealedData_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__common_SecretRevealedData_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__common_SecretRevealedData_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__common_SecretRevealedData_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__common_SecretRevealedData_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__common_SecretRevealedData_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_common_SecretRevealedData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_common_SecretRevealedData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_common_SecretRevealedData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_common_SecretRevealedData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_common_SecretRevealedData_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_SecretRevealedData_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_SecretRevealedData_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_SecretRevealedData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_common_SecretRevealedData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for SecretRevealedData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_common_SecretRevealedData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_common_SecretRevealedData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_common_SecretRevealedData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_common_SecretRevealedData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_common_SecretRevealedData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_common_SecretRevealedData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SecretRevealedDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_common_SecretRevealedData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_common_SecretRevealedData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_common_SecretRevealedData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SecretRevealedDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_common_SecretRevealedData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_common_SecretRevealedData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_common_SecretRevealedData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_common_SecretRevealedData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_common_SecretRevealedData_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_SecretRevealedData_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_SecretRevealedData_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_SecretRevealedData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_common_SecretRevealedData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for SecretRevealedData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_common_SecretRevealedData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_common_SecretRevealedData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_common_SecretRevealedData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_common_SecretRevealedData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_common_SecretRevealedData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_common_SecretRevealedData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SecretRevealedDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_common_SecretRevealedData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_common_SecretRevealedData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_common_SecretRevealedData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SecretRevealedDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_common_SecretRevealedData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_common_SecretRevealedData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_common_SecretRevealedData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_common_SecretRevealedData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_common_SecretRevealedData_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_SecretRevealedData_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_SecretRevealedData_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_SecretRevealedData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_common_SecretRevealedData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for SecretRevealedData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_common_SecretRevealedData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_common_SecretRevealedData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_common_SecretRevealedData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_common_SecretRevealedData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_common_SecretRevealedData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_common_SecretRevealedData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SecretRevealedDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_common_SecretRevealedData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_common_SecretRevealedData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_common_SecretRevealedData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SecretRevealedDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_common_SecretRevealedData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_common_SecretRevealedData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_common_SecretRevealedData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_common_SecretRevealedData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_common_SecretRevealedData_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_SecretRevealedData_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_SecretRevealedData_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_SecretRevealedData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_common_SecretRevealedData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for SecretRevealedData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_common_SecretRevealedData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_common_SecretRevealedData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_common_SecretRevealedData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_common_SecretRevealedData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_common_SecretRevealedData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_common_SecretRevealedData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SecretRevealedDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_common_SecretRevealedData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_common_SecretRevealedData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_common_SecretRevealedData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SecretRevealedDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_common_SecretRevealedData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_common_SecretRevealedData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_common_SecretRevealedData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_common_SecretRevealedData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_common_SecretRevealedData_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_SecretRevealedData_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_SecretRevealedData_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_SecretRevealedData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_common_SecretRevealedData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for SecretRevealedData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_common_SecretRevealedData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_common_SecretRevealedData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_common_SecretRevealedData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_common_SecretRevealedData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_common_SecretRevealedData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_common_SecretRevealedData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SecretRevealedDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_common_SecretRevealedData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_common_SecretRevealedData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_common_SecretRevealedData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| SecretRevealedDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for SecretRevealedData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(SecretRevealedDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_common_SecretRevealedData_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| SecretRevealedDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SecretRevealedDataMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for SecretRevealedDataMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> SecretRevealedDataMut<'msg> {
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


  // participant: optional string
  pub fn participant(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_SecretRevealedData_get_participant(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_participant(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.participant_mut().set(val);
  }
  fn participant_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          SecretRevealedData::__PARTICIPANT_VTABLE,
        )
      )
    }
  }

  // contract_id: optional string
  pub fn contract_id(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_SecretRevealedData_get_contract_id(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_contract_id(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.contract_id_mut().set(val);
  }
  fn contract_id_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          SecretRevealedData::__CONTRACT_ID_VTABLE,
        )
      )
    }
  }

  // secret: optional string
  pub fn secret(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_SecretRevealedData_get_secret(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_secret(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.secret_mut().set(val);
  }
  fn secret_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          SecretRevealedData::__SECRET_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `SecretRevealedDataMut` does not perform any shared mutation.
// - `SecretRevealedDataMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for SecretRevealedDataMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for SecretRevealedDataMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, SecretRevealedData> {
    SecretRevealedDataMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, SecretRevealedData> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for SecretRevealedDataMut<'msg> {
  type Proxied = SecretRevealedData;
  fn as_view(&self) -> ::__pb::View<'_, SecretRevealedData> {
    SecretRevealedDataView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, SecretRevealedData> where 'msg: 'shorter {
    SecretRevealedDataView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl SecretRevealedData {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__common_SecretRevealedData_new() } } }
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

      __rust_proto_thunk__common_SecretRevealedData_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> SecretRevealedDataView {
    SecretRevealedDataView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> SecretRevealedDataMut {
    SecretRevealedDataMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // participant: optional string
  pub fn participant(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_SecretRevealedData_get_participant(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_participant(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.participant_mut().set(val);
  }
  const __PARTICIPANT_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__common_SecretRevealedData_get_participant,
      __rust_proto_thunk__common_SecretRevealedData_set_participant,
    );
  fn participant_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          SecretRevealedData::__PARTICIPANT_VTABLE,
        )
      )
    }
  }

  // contract_id: optional string
  pub fn contract_id(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_SecretRevealedData_get_contract_id(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_contract_id(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.contract_id_mut().set(val);
  }
  const __CONTRACT_ID_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__common_SecretRevealedData_get_contract_id,
      __rust_proto_thunk__common_SecretRevealedData_set_contract_id,
    );
  fn contract_id_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          SecretRevealedData::__CONTRACT_ID_VTABLE,
        )
      )
    }
  }

  // secret: optional string
  pub fn secret(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_SecretRevealedData_get_secret(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_secret(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.secret_mut().set(val);
  }
  const __SECRET_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__common_SecretRevealedData_get_secret,
      __rust_proto_thunk__common_SecretRevealedData_set_secret,
    );
  fn secret_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          SecretRevealedData::__SECRET_VTABLE,
        )
      )
    }
  }

}  // impl SecretRevealedData

impl ::__std::ops::Drop for SecretRevealedData {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__common_SecretRevealedData_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__common_SecretRevealedData_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_SecretRevealedData_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__common_SecretRevealedData_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__common_SecretRevealedData_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__common_SecretRevealedData_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__common_SecretRevealedData_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__common_SecretRevealedData_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_SecretRevealedData_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_SecretRevealedData_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_SecretRevealedData_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__common_SecretRevealedData_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__common_SecretRevealedData_get_participant(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__common_SecretRevealedData_set_participant(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__common_SecretRevealedData_get_contract_id(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__common_SecretRevealedData_set_contract_id(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__common_SecretRevealedData_get_secret(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__common_SecretRevealedData_set_secret(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for SecretRevealedData


impl SecretRevealedData {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> SecretRevealedDataMut<'a> {
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

impl<'a> SecretRevealedDataView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct ReceiveSecretData {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for ReceiveSecretData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for ReceiveSecretData {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `ReceiveSecretData` is `Sync` because it does not implement interior mutability.
//    Neither does `ReceiveSecretDataMut`.
unsafe impl Sync for ReceiveSecretData {}

// SAFETY:
// - `ReceiveSecretData` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ReceiveSecretData {}

impl ::__pb::Proxied for ReceiveSecretData {
  type View<'msg> = ReceiveSecretDataView<'msg>;
  type Mut<'msg> = ReceiveSecretDataMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ReceiveSecretDataView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for ReceiveSecretDataView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> ReceiveSecretDataView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__common_ReceiveSecretData_serialize(self.raw_msg()) }
  }

  // participant: optional string
  pub fn participant(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_ReceiveSecretData_get_participant(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // contract_id: optional string
  pub fn contract_id(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_ReceiveSecretData_get_contract_id(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // secret: optional string
  pub fn secret(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_ReceiveSecretData_get_secret(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

}

// SAFETY:
// - `ReceiveSecretDataView` is `Sync` because it does not support mutation.
unsafe impl Sync for ReceiveSecretDataView<'_> {}

// SAFETY:
// - `ReceiveSecretDataView` is `Send` because while its alive a `ReceiveSecretDataMut` cannot.
// - `ReceiveSecretDataView` does not use thread-local data.
unsafe impl Send for ReceiveSecretDataView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for ReceiveSecretDataView<'msg> {
  type Proxied = ReceiveSecretData;

  fn as_view(&self) -> ::__pb::View<'msg, ReceiveSecretData> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, ReceiveSecretData> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for ReceiveSecretData {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    ReceiveSecretDataView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    ReceiveSecretDataMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for ReceiveSecretData {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for ReceiveSecretData {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, ReceiveSecretData>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, ReceiveSecretData>;

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

impl<'msg> ::__pb::SettableValue<ReceiveSecretData> for ReceiveSecretDataView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, ReceiveSecretData>)
    where ReceiveSecretData: 'dst {
    unsafe { __rust_proto_thunk__common_ReceiveSecretData_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<ReceiveSecretData> for ReceiveSecretData {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, ReceiveSecretData>)
    where ReceiveSecretData: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for ReceiveSecretData {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__common_ReceiveSecretData_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__common_ReceiveSecretData_copy_from(
        __rust_proto_thunk__common_ReceiveSecretData_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__common_ReceiveSecretData_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__common_ReceiveSecretData_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__common_ReceiveSecretData_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__common_ReceiveSecretData_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__common_ReceiveSecretData_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_common_ReceiveSecretData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_common_ReceiveSecretData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_common_ReceiveSecretData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_common_ReceiveSecretData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_common_ReceiveSecretData_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_ReceiveSecretData_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_ReceiveSecretData_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_ReceiveSecretData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_common_ReceiveSecretData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for ReceiveSecretData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_common_ReceiveSecretData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_common_ReceiveSecretData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_common_ReceiveSecretData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_common_ReceiveSecretData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_common_ReceiveSecretData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_common_ReceiveSecretData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ReceiveSecretDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_common_ReceiveSecretData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_common_ReceiveSecretData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_common_ReceiveSecretData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ReceiveSecretDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_common_ReceiveSecretData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_common_ReceiveSecretData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_common_ReceiveSecretData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_common_ReceiveSecretData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_common_ReceiveSecretData_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_ReceiveSecretData_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_ReceiveSecretData_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_ReceiveSecretData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_common_ReceiveSecretData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for ReceiveSecretData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_common_ReceiveSecretData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_common_ReceiveSecretData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_common_ReceiveSecretData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_common_ReceiveSecretData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_common_ReceiveSecretData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_common_ReceiveSecretData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ReceiveSecretDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_common_ReceiveSecretData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_common_ReceiveSecretData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_common_ReceiveSecretData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ReceiveSecretDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_common_ReceiveSecretData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_common_ReceiveSecretData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_common_ReceiveSecretData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_common_ReceiveSecretData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_common_ReceiveSecretData_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_ReceiveSecretData_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_ReceiveSecretData_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_ReceiveSecretData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_common_ReceiveSecretData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for ReceiveSecretData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_common_ReceiveSecretData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_common_ReceiveSecretData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_common_ReceiveSecretData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_common_ReceiveSecretData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_common_ReceiveSecretData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_common_ReceiveSecretData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ReceiveSecretDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_common_ReceiveSecretData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_common_ReceiveSecretData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_common_ReceiveSecretData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ReceiveSecretDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_common_ReceiveSecretData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_common_ReceiveSecretData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_common_ReceiveSecretData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_common_ReceiveSecretData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_common_ReceiveSecretData_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_ReceiveSecretData_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_ReceiveSecretData_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_ReceiveSecretData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_common_ReceiveSecretData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for ReceiveSecretData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_common_ReceiveSecretData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_common_ReceiveSecretData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_common_ReceiveSecretData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_common_ReceiveSecretData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_common_ReceiveSecretData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_common_ReceiveSecretData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ReceiveSecretDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_common_ReceiveSecretData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_common_ReceiveSecretData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_common_ReceiveSecretData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ReceiveSecretDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_common_ReceiveSecretData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_common_ReceiveSecretData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_common_ReceiveSecretData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_common_ReceiveSecretData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_common_ReceiveSecretData_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_ReceiveSecretData_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_ReceiveSecretData_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_ReceiveSecretData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_common_ReceiveSecretData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for ReceiveSecretData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_common_ReceiveSecretData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_common_ReceiveSecretData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_common_ReceiveSecretData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_common_ReceiveSecretData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_common_ReceiveSecretData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_common_ReceiveSecretData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ReceiveSecretDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_common_ReceiveSecretData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_common_ReceiveSecretData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_common_ReceiveSecretData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ReceiveSecretDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for ReceiveSecretData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ReceiveSecretDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_common_ReceiveSecretData_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| ReceiveSecretDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ReceiveSecretDataMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for ReceiveSecretDataMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> ReceiveSecretDataMut<'msg> {
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


  // participant: optional string
  pub fn participant(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_ReceiveSecretData_get_participant(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_participant(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.participant_mut().set(val);
  }
  fn participant_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          ReceiveSecretData::__PARTICIPANT_VTABLE,
        )
      )
    }
  }

  // contract_id: optional string
  pub fn contract_id(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_ReceiveSecretData_get_contract_id(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_contract_id(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.contract_id_mut().set(val);
  }
  fn contract_id_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          ReceiveSecretData::__CONTRACT_ID_VTABLE,
        )
      )
    }
  }

  // secret: optional string
  pub fn secret(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_ReceiveSecretData_get_secret(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_secret(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.secret_mut().set(val);
  }
  fn secret_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          ReceiveSecretData::__SECRET_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `ReceiveSecretDataMut` does not perform any shared mutation.
// - `ReceiveSecretDataMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for ReceiveSecretDataMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for ReceiveSecretDataMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, ReceiveSecretData> {
    ReceiveSecretDataMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, ReceiveSecretData> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for ReceiveSecretDataMut<'msg> {
  type Proxied = ReceiveSecretData;
  fn as_view(&self) -> ::__pb::View<'_, ReceiveSecretData> {
    ReceiveSecretDataView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, ReceiveSecretData> where 'msg: 'shorter {
    ReceiveSecretDataView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl ReceiveSecretData {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__common_ReceiveSecretData_new() } } }
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

      __rust_proto_thunk__common_ReceiveSecretData_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> ReceiveSecretDataView {
    ReceiveSecretDataView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> ReceiveSecretDataMut {
    ReceiveSecretDataMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // participant: optional string
  pub fn participant(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_ReceiveSecretData_get_participant(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_participant(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.participant_mut().set(val);
  }
  const __PARTICIPANT_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__common_ReceiveSecretData_get_participant,
      __rust_proto_thunk__common_ReceiveSecretData_set_participant,
    );
  fn participant_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          ReceiveSecretData::__PARTICIPANT_VTABLE,
        )
      )
    }
  }

  // contract_id: optional string
  pub fn contract_id(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_ReceiveSecretData_get_contract_id(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_contract_id(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.contract_id_mut().set(val);
  }
  const __CONTRACT_ID_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__common_ReceiveSecretData_get_contract_id,
      __rust_proto_thunk__common_ReceiveSecretData_set_contract_id,
    );
  fn contract_id_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          ReceiveSecretData::__CONTRACT_ID_VTABLE,
        )
      )
    }
  }

  // secret: optional string
  pub fn secret(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_ReceiveSecretData_get_secret(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_secret(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.secret_mut().set(val);
  }
  const __SECRET_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__common_ReceiveSecretData_get_secret,
      __rust_proto_thunk__common_ReceiveSecretData_set_secret,
    );
  fn secret_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          ReceiveSecretData::__SECRET_VTABLE,
        )
      )
    }
  }

}  // impl ReceiveSecretData

impl ::__std::ops::Drop for ReceiveSecretData {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__common_ReceiveSecretData_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__common_ReceiveSecretData_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_ReceiveSecretData_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__common_ReceiveSecretData_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__common_ReceiveSecretData_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__common_ReceiveSecretData_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__common_ReceiveSecretData_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__common_ReceiveSecretData_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_ReceiveSecretData_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_ReceiveSecretData_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_ReceiveSecretData_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__common_ReceiveSecretData_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__common_ReceiveSecretData_get_participant(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__common_ReceiveSecretData_set_participant(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__common_ReceiveSecretData_get_contract_id(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__common_ReceiveSecretData_set_contract_id(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__common_ReceiveSecretData_get_secret(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__common_ReceiveSecretData_set_secret(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for ReceiveSecretData


impl ReceiveSecretData {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> ReceiveSecretDataMut<'a> {
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

impl<'a> ReceiveSecretDataView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct ReceiveTerminationData {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for ReceiveTerminationData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for ReceiveTerminationData {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `ReceiveTerminationData` is `Sync` because it does not implement interior mutability.
//    Neither does `ReceiveTerminationDataMut`.
unsafe impl Sync for ReceiveTerminationData {}

// SAFETY:
// - `ReceiveTerminationData` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ReceiveTerminationData {}

impl ::__pb::Proxied for ReceiveTerminationData {
  type View<'msg> = ReceiveTerminationDataView<'msg>;
  type Mut<'msg> = ReceiveTerminationDataMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ReceiveTerminationDataView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for ReceiveTerminationDataView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> ReceiveTerminationDataView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__common_ReceiveTerminationData_serialize(self.raw_msg()) }
  }

  // participant: optional string
  pub fn participant(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_ReceiveTerminationData_get_participant(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // contract_id: optional string
  pub fn contract_id(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_ReceiveTerminationData_get_contract_id(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // result: optional enum common.TerminationResult
  pub fn result(self) -> crate::TerminationResult {
    unsafe { __rust_proto_thunk__common_ReceiveTerminationData_get_result(self.raw_msg()) }
  }

}

// SAFETY:
// - `ReceiveTerminationDataView` is `Sync` because it does not support mutation.
unsafe impl Sync for ReceiveTerminationDataView<'_> {}

// SAFETY:
// - `ReceiveTerminationDataView` is `Send` because while its alive a `ReceiveTerminationDataMut` cannot.
// - `ReceiveTerminationDataView` does not use thread-local data.
unsafe impl Send for ReceiveTerminationDataView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for ReceiveTerminationDataView<'msg> {
  type Proxied = ReceiveTerminationData;

  fn as_view(&self) -> ::__pb::View<'msg, ReceiveTerminationData> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, ReceiveTerminationData> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for ReceiveTerminationData {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    ReceiveTerminationDataView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    ReceiveTerminationDataMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for ReceiveTerminationData {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for ReceiveTerminationData {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, ReceiveTerminationData>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, ReceiveTerminationData>;

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

impl<'msg> ::__pb::SettableValue<ReceiveTerminationData> for ReceiveTerminationDataView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, ReceiveTerminationData>)
    where ReceiveTerminationData: 'dst {
    unsafe { __rust_proto_thunk__common_ReceiveTerminationData_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<ReceiveTerminationData> for ReceiveTerminationData {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, ReceiveTerminationData>)
    where ReceiveTerminationData: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for ReceiveTerminationData {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__common_ReceiveTerminationData_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__common_ReceiveTerminationData_copy_from(
        __rust_proto_thunk__common_ReceiveTerminationData_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__common_ReceiveTerminationData_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__common_ReceiveTerminationData_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__common_ReceiveTerminationData_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__common_ReceiveTerminationData_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__common_ReceiveTerminationData_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for ReceiveTerminationData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ReceiveTerminationDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_common_ReceiveTerminationData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ReceiveTerminationDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for ReceiveTerminationData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ReceiveTerminationDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_common_ReceiveTerminationData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ReceiveTerminationDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for ReceiveTerminationData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ReceiveTerminationDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_common_ReceiveTerminationData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ReceiveTerminationDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for ReceiveTerminationData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ReceiveTerminationDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_common_ReceiveTerminationData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ReceiveTerminationDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for ReceiveTerminationData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ReceiveTerminationDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_common_ReceiveTerminationData_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ReceiveTerminationDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for ReceiveTerminationData {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ReceiveTerminationDataView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_common_ReceiveTerminationData_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| ReceiveTerminationDataView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ReceiveTerminationDataMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for ReceiveTerminationDataMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> ReceiveTerminationDataMut<'msg> {
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


  // participant: optional string
  pub fn participant(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_ReceiveTerminationData_get_participant(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_participant(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.participant_mut().set(val);
  }
  fn participant_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          ReceiveTerminationData::__PARTICIPANT_VTABLE,
        )
      )
    }
  }

  // contract_id: optional string
  pub fn contract_id(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_ReceiveTerminationData_get_contract_id(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_contract_id(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.contract_id_mut().set(val);
  }
  fn contract_id_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          ReceiveTerminationData::__CONTRACT_ID_VTABLE,
        )
      )
    }
  }

  // result: optional enum common.TerminationResult
  pub fn result(&self) -> crate::TerminationResult {
    unsafe { __rust_proto_thunk__common_ReceiveTerminationData_get_result(self.raw_msg()) }
  }
  pub fn set_result(&mut self, val: crate::TerminationResult) {
    unsafe { __rust_proto_thunk__common_ReceiveTerminationData_set_result(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `ReceiveTerminationDataMut` does not perform any shared mutation.
// - `ReceiveTerminationDataMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for ReceiveTerminationDataMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for ReceiveTerminationDataMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, ReceiveTerminationData> {
    ReceiveTerminationDataMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, ReceiveTerminationData> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for ReceiveTerminationDataMut<'msg> {
  type Proxied = ReceiveTerminationData;
  fn as_view(&self) -> ::__pb::View<'_, ReceiveTerminationData> {
    ReceiveTerminationDataView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, ReceiveTerminationData> where 'msg: 'shorter {
    ReceiveTerminationDataView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl ReceiveTerminationData {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__common_ReceiveTerminationData_new() } } }
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

      __rust_proto_thunk__common_ReceiveTerminationData_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> ReceiveTerminationDataView {
    ReceiveTerminationDataView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> ReceiveTerminationDataMut {
    ReceiveTerminationDataMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // participant: optional string
  pub fn participant(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_ReceiveTerminationData_get_participant(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_participant(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.participant_mut().set(val);
  }
  const __PARTICIPANT_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__common_ReceiveTerminationData_get_participant,
      __rust_proto_thunk__common_ReceiveTerminationData_set_participant,
    );
  fn participant_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          ReceiveTerminationData::__PARTICIPANT_VTABLE,
        )
      )
    }
  }

  // contract_id: optional string
  pub fn contract_id(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__common_ReceiveTerminationData_get_contract_id(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_contract_id(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.contract_id_mut().set(val);
  }
  const __CONTRACT_ID_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__common_ReceiveTerminationData_get_contract_id,
      __rust_proto_thunk__common_ReceiveTerminationData_set_contract_id,
    );
  fn contract_id_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          ReceiveTerminationData::__CONTRACT_ID_VTABLE,
        )
      )
    }
  }

  // result: optional enum common.TerminationResult
  pub fn result(&self) -> crate::TerminationResult {
    unsafe { __rust_proto_thunk__common_ReceiveTerminationData_get_result(self.raw_msg()) }
  }
  pub fn set_result(&mut self, val: crate::TerminationResult) {
    unsafe { __rust_proto_thunk__common_ReceiveTerminationData_set_result(self.raw_msg(), val) }
  }

}  // impl ReceiveTerminationData

impl ::__std::ops::Drop for ReceiveTerminationData {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__common_ReceiveTerminationData_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__common_ReceiveTerminationData_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_ReceiveTerminationData_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__common_ReceiveTerminationData_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__common_ReceiveTerminationData_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__common_ReceiveTerminationData_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__common_ReceiveTerminationData_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__common_ReceiveTerminationData_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_ReceiveTerminationData_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_ReceiveTerminationData_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__common_ReceiveTerminationData_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__common_ReceiveTerminationData_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__common_ReceiveTerminationData_get_participant(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__common_ReceiveTerminationData_set_participant(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__common_ReceiveTerminationData_get_contract_id(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__common_ReceiveTerminationData_set_contract_id(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__common_ReceiveTerminationData_get_result(raw_msg: ::__pb::__runtime::RawMessage) -> crate::TerminationResult;
  fn __rust_proto_thunk__common_ReceiveTerminationData_set_result(raw_msg: ::__pb::__runtime::RawMessage, val: crate::TerminationResult);


}  // extern "C" for ReceiveTerminationData


impl ReceiveTerminationData {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> ReceiveTerminationDataMut<'a> {
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

impl<'a> ReceiveTerminationDataView<'a> {
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
pub struct PayloadType(i32);

#[allow(non_upper_case_globals)]
impl PayloadType {
  pub const Newtransaction: PayloadType = PayloadType(0);
  pub const Ackproposal: PayloadType = PayloadType(1);
  pub const Proposalacked: PayloadType = PayloadType(2);
  pub const Proposalnacked: PayloadType = PayloadType(3);
  pub const Revealsecret: PayloadType = PayloadType(4);
  pub const Secretrevealed: PayloadType = PayloadType(5);
  pub const Receivesecret: PayloadType = PayloadType(6);
  pub const Receivetermination: PayloadType = PayloadType(7);
}

impl ::__std::convert::From<PayloadType> for i32 {
  fn from(val: PayloadType) -> i32 {
    val.0
  }
}

impl ::__std::convert::From<i32> for PayloadType {
  fn from(val: i32) -> PayloadType {
    Self(val)
  }
}

impl ::__std::default::Default for PayloadType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::__std::fmt::Debug for PayloadType {
  fn fmt(&self, f: &mut ::__std::fmt::Formatter<'_>) -> ::__std::fmt::Result {
    f.debug_tuple(stringify!(PayloadType)).field(&self.0).finish()
  }
}

impl ::__pb::Proxied for PayloadType {
  type View<'a> = PayloadType;
  type Mut<'a> = ::__pb::PrimitiveMut<'a, PayloadType>;
}

impl ::__pb::ViewProxy<'_> for PayloadType {
  type Proxied = PayloadType;

  fn as_view(&self) -> PayloadType {
    *self
  }

  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, PayloadType> {
    self
  }
}

impl ::__pb::SettableValue<PayloadType> for PayloadType {
  fn set_on<'msg>(
      self,
      private: ::__pb::__internal::Private,
      mut mutator: ::__pb::Mut<'msg, PayloadType>
  ) where PayloadType: 'msg {
    mutator.set_primitive(private, self)
  }
}

impl ::__pb::ProxiedWithPresence for PayloadType {
  type PresentMutData<'a> = ::__pb::__internal::RawVTableOptionalMutatorData<'a, PayloadType>;
  type AbsentMutData<'a> = ::__pb::__internal::RawVTableOptionalMutatorData<'a, PayloadType>;

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

unsafe impl ::__pb::ProxiedInRepeated for PayloadType {
  fn repeated_len(r: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    ::__pb::__runtime::cast_enum_repeated_view(::__pb::__internal::Private, r).len()
  }

  fn repeated_push(r: ::__pb::Mut<::__pb::Repeated<Self>>, val: PayloadType) {
    ::__pb::__runtime::cast_enum_repeated_mut(::__pb::__internal::Private, r).push(val.into())
  }

  fn repeated_clear(r: ::__pb::Mut<::__pb::Repeated<Self>>) {
    ::__pb::__runtime::cast_enum_repeated_mut(::__pb::__internal::Private, r).clear()
  }

  unsafe fn repeated_get_unchecked(
      r: ::__pb::View<::__pb::Repeated<Self>>,
      index: usize,
  ) -> ::__pb::View<PayloadType> {
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
      val: PayloadType,
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

impl ::__pb::__internal::PrimitiveWithRawVTable for PayloadType {}

// SAFETY: this is an enum type
unsafe impl ::__pb::__internal::Enum for PayloadType {
  const NAME: &'static str = "PayloadType";
}

extern "C" {
  fn __rust_proto_thunk__Map_i32_common_PayloadType_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_i32_common_PayloadType_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_i32_common_PayloadType_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_i32_common_PayloadType_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_i32_common_PayloadType_insert(m: ::__pb::__runtime::RawMap, key: i32, value: PayloadType) -> bool;
  fn __rust_proto_thunk__Map_i32_common_PayloadType_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut PayloadType) -> bool;
  fn __rust_proto_thunk__Map_i32_common_PayloadType_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut PayloadType) -> bool;
  fn __rust_proto_thunk__Map_i32_common_PayloadType_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_i32_common_PayloadType_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut PayloadType);
}
impl ::__pb::ProxiedInMapValue<i32> for PayloadType {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_common_PayloadType_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
      unsafe { __rust_proto_thunk__Map_i32_common_PayloadType_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
      unsafe { __rust_proto_thunk__Map_i32_common_PayloadType_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_i32_common_PayloadType_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_i32_common_PayloadType_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_i32_common_PayloadType_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_i32_common_PayloadType_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_i32_common_PayloadType_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_i32_common_PayloadType_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_u32_common_PayloadType_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_u32_common_PayloadType_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_u32_common_PayloadType_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_u32_common_PayloadType_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_u32_common_PayloadType_insert(m: ::__pb::__runtime::RawMap, key: u32, value: PayloadType) -> bool;
  fn __rust_proto_thunk__Map_u32_common_PayloadType_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut PayloadType) -> bool;
  fn __rust_proto_thunk__Map_u32_common_PayloadType_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut PayloadType) -> bool;
  fn __rust_proto_thunk__Map_u32_common_PayloadType_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_u32_common_PayloadType_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut PayloadType);
}
impl ::__pb::ProxiedInMapValue<u32> for PayloadType {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_common_PayloadType_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
      unsafe { __rust_proto_thunk__Map_u32_common_PayloadType_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
      unsafe { __rust_proto_thunk__Map_u32_common_PayloadType_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_u32_common_PayloadType_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_u32_common_PayloadType_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_u32_common_PayloadType_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_u32_common_PayloadType_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_u32_common_PayloadType_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_u32_common_PayloadType_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_i64_common_PayloadType_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_i64_common_PayloadType_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_i64_common_PayloadType_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_i64_common_PayloadType_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_i64_common_PayloadType_insert(m: ::__pb::__runtime::RawMap, key: i64, value: PayloadType) -> bool;
  fn __rust_proto_thunk__Map_i64_common_PayloadType_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut PayloadType) -> bool;
  fn __rust_proto_thunk__Map_i64_common_PayloadType_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut PayloadType) -> bool;
  fn __rust_proto_thunk__Map_i64_common_PayloadType_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_i64_common_PayloadType_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut PayloadType);
}
impl ::__pb::ProxiedInMapValue<i64> for PayloadType {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_common_PayloadType_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
      unsafe { __rust_proto_thunk__Map_i64_common_PayloadType_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
      unsafe { __rust_proto_thunk__Map_i64_common_PayloadType_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_i64_common_PayloadType_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_i64_common_PayloadType_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_i64_common_PayloadType_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_i64_common_PayloadType_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_i64_common_PayloadType_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_i64_common_PayloadType_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_u64_common_PayloadType_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_u64_common_PayloadType_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_u64_common_PayloadType_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_u64_common_PayloadType_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_u64_common_PayloadType_insert(m: ::__pb::__runtime::RawMap, key: u64, value: PayloadType) -> bool;
  fn __rust_proto_thunk__Map_u64_common_PayloadType_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut PayloadType) -> bool;
  fn __rust_proto_thunk__Map_u64_common_PayloadType_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut PayloadType) -> bool;
  fn __rust_proto_thunk__Map_u64_common_PayloadType_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_u64_common_PayloadType_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut PayloadType);
}
impl ::__pb::ProxiedInMapValue<u64> for PayloadType {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_common_PayloadType_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
      unsafe { __rust_proto_thunk__Map_u64_common_PayloadType_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
      unsafe { __rust_proto_thunk__Map_u64_common_PayloadType_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_u64_common_PayloadType_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_u64_common_PayloadType_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_u64_common_PayloadType_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_u64_common_PayloadType_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_u64_common_PayloadType_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_u64_common_PayloadType_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_bool_common_PayloadType_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_bool_common_PayloadType_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_bool_common_PayloadType_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_bool_common_PayloadType_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_bool_common_PayloadType_insert(m: ::__pb::__runtime::RawMap, key: bool, value: PayloadType) -> bool;
  fn __rust_proto_thunk__Map_bool_common_PayloadType_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut PayloadType) -> bool;
  fn __rust_proto_thunk__Map_bool_common_PayloadType_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut PayloadType) -> bool;
  fn __rust_proto_thunk__Map_bool_common_PayloadType_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_bool_common_PayloadType_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut PayloadType);
}
impl ::__pb::ProxiedInMapValue<bool> for PayloadType {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_common_PayloadType_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
      unsafe { __rust_proto_thunk__Map_bool_common_PayloadType_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
      unsafe { __rust_proto_thunk__Map_bool_common_PayloadType_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_bool_common_PayloadType_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_bool_common_PayloadType_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_bool_common_PayloadType_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_bool_common_PayloadType_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_bool_common_PayloadType_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_bool_common_PayloadType_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_ProtoStr_common_PayloadType_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_ProtoStr_common_PayloadType_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_ProtoStr_common_PayloadType_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_ProtoStr_common_PayloadType_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_ProtoStr_common_PayloadType_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: PayloadType) -> bool;
  fn __rust_proto_thunk__Map_ProtoStr_common_PayloadType_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut PayloadType) -> bool;
  fn __rust_proto_thunk__Map_ProtoStr_common_PayloadType_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut PayloadType) -> bool;
  fn __rust_proto_thunk__Map_ProtoStr_common_PayloadType_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_ProtoStr_common_PayloadType_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut PayloadType);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for PayloadType {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_common_PayloadType_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
      unsafe { __rust_proto_thunk__Map_ProtoStr_common_PayloadType_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
      unsafe { __rust_proto_thunk__Map_ProtoStr_common_PayloadType_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_ProtoStr_common_PayloadType_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_ProtoStr_common_PayloadType_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
      let key = key.as_bytes().into();
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_ProtoStr_common_PayloadType_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_ProtoStr_common_PayloadType_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_ProtoStr_common_PayloadType_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_ProtoStr_common_PayloadType_iter_get,
              |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
              ::__std::convert::identity,
          )
      }
  }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TerminationResult(i32);

#[allow(non_upper_case_globals)]
impl TerminationResult {
  pub const Contractfailed: TerminationResult = TerminationResult(0);
  pub const Txnfailed: TerminationResult = TerminationResult(1);
  pub const Txncompleted: TerminationResult = TerminationResult(2);
}

impl ::__std::convert::From<TerminationResult> for i32 {
  fn from(val: TerminationResult) -> i32 {
    val.0
  }
}

impl ::__std::convert::From<i32> for TerminationResult {
  fn from(val: i32) -> TerminationResult {
    Self(val)
  }
}

impl ::__std::default::Default for TerminationResult {
  fn default() -> Self {
    Self(0)
  }
}

impl ::__std::fmt::Debug for TerminationResult {
  fn fmt(&self, f: &mut ::__std::fmt::Formatter<'_>) -> ::__std::fmt::Result {
    f.debug_tuple(stringify!(TerminationResult)).field(&self.0).finish()
  }
}

impl ::__pb::Proxied for TerminationResult {
  type View<'a> = TerminationResult;
  type Mut<'a> = ::__pb::PrimitiveMut<'a, TerminationResult>;
}

impl ::__pb::ViewProxy<'_> for TerminationResult {
  type Proxied = TerminationResult;

  fn as_view(&self) -> TerminationResult {
    *self
  }

  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, TerminationResult> {
    self
  }
}

impl ::__pb::SettableValue<TerminationResult> for TerminationResult {
  fn set_on<'msg>(
      self,
      private: ::__pb::__internal::Private,
      mut mutator: ::__pb::Mut<'msg, TerminationResult>
  ) where TerminationResult: 'msg {
    mutator.set_primitive(private, self)
  }
}

impl ::__pb::ProxiedWithPresence for TerminationResult {
  type PresentMutData<'a> = ::__pb::__internal::RawVTableOptionalMutatorData<'a, TerminationResult>;
  type AbsentMutData<'a> = ::__pb::__internal::RawVTableOptionalMutatorData<'a, TerminationResult>;

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

unsafe impl ::__pb::ProxiedInRepeated for TerminationResult {
  fn repeated_len(r: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    ::__pb::__runtime::cast_enum_repeated_view(::__pb::__internal::Private, r).len()
  }

  fn repeated_push(r: ::__pb::Mut<::__pb::Repeated<Self>>, val: TerminationResult) {
    ::__pb::__runtime::cast_enum_repeated_mut(::__pb::__internal::Private, r).push(val.into())
  }

  fn repeated_clear(r: ::__pb::Mut<::__pb::Repeated<Self>>) {
    ::__pb::__runtime::cast_enum_repeated_mut(::__pb::__internal::Private, r).clear()
  }

  unsafe fn repeated_get_unchecked(
      r: ::__pb::View<::__pb::Repeated<Self>>,
      index: usize,
  ) -> ::__pb::View<TerminationResult> {
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
      val: TerminationResult,
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

impl ::__pb::__internal::PrimitiveWithRawVTable for TerminationResult {}

// SAFETY: this is an enum type
unsafe impl ::__pb::__internal::Enum for TerminationResult {
  const NAME: &'static str = "TerminationResult";
}

extern "C" {
  fn __rust_proto_thunk__Map_i32_common_TerminationResult_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_i32_common_TerminationResult_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_i32_common_TerminationResult_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_i32_common_TerminationResult_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_i32_common_TerminationResult_insert(m: ::__pb::__runtime::RawMap, key: i32, value: TerminationResult) -> bool;
  fn __rust_proto_thunk__Map_i32_common_TerminationResult_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut TerminationResult) -> bool;
  fn __rust_proto_thunk__Map_i32_common_TerminationResult_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut TerminationResult) -> bool;
  fn __rust_proto_thunk__Map_i32_common_TerminationResult_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_i32_common_TerminationResult_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut TerminationResult);
}
impl ::__pb::ProxiedInMapValue<i32> for TerminationResult {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_common_TerminationResult_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
      unsafe { __rust_proto_thunk__Map_i32_common_TerminationResult_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
      unsafe { __rust_proto_thunk__Map_i32_common_TerminationResult_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_i32_common_TerminationResult_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_i32_common_TerminationResult_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_i32_common_TerminationResult_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_i32_common_TerminationResult_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_i32_common_TerminationResult_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_i32_common_TerminationResult_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_u32_common_TerminationResult_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_u32_common_TerminationResult_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_u32_common_TerminationResult_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_u32_common_TerminationResult_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_u32_common_TerminationResult_insert(m: ::__pb::__runtime::RawMap, key: u32, value: TerminationResult) -> bool;
  fn __rust_proto_thunk__Map_u32_common_TerminationResult_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut TerminationResult) -> bool;
  fn __rust_proto_thunk__Map_u32_common_TerminationResult_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut TerminationResult) -> bool;
  fn __rust_proto_thunk__Map_u32_common_TerminationResult_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_u32_common_TerminationResult_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut TerminationResult);
}
impl ::__pb::ProxiedInMapValue<u32> for TerminationResult {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_common_TerminationResult_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
      unsafe { __rust_proto_thunk__Map_u32_common_TerminationResult_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
      unsafe { __rust_proto_thunk__Map_u32_common_TerminationResult_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_u32_common_TerminationResult_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_u32_common_TerminationResult_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_u32_common_TerminationResult_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_u32_common_TerminationResult_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_u32_common_TerminationResult_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_u32_common_TerminationResult_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_i64_common_TerminationResult_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_i64_common_TerminationResult_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_i64_common_TerminationResult_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_i64_common_TerminationResult_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_i64_common_TerminationResult_insert(m: ::__pb::__runtime::RawMap, key: i64, value: TerminationResult) -> bool;
  fn __rust_proto_thunk__Map_i64_common_TerminationResult_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut TerminationResult) -> bool;
  fn __rust_proto_thunk__Map_i64_common_TerminationResult_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut TerminationResult) -> bool;
  fn __rust_proto_thunk__Map_i64_common_TerminationResult_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_i64_common_TerminationResult_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut TerminationResult);
}
impl ::__pb::ProxiedInMapValue<i64> for TerminationResult {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_common_TerminationResult_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
      unsafe { __rust_proto_thunk__Map_i64_common_TerminationResult_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
      unsafe { __rust_proto_thunk__Map_i64_common_TerminationResult_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_i64_common_TerminationResult_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_i64_common_TerminationResult_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_i64_common_TerminationResult_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_i64_common_TerminationResult_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_i64_common_TerminationResult_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_i64_common_TerminationResult_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_u64_common_TerminationResult_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_u64_common_TerminationResult_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_u64_common_TerminationResult_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_u64_common_TerminationResult_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_u64_common_TerminationResult_insert(m: ::__pb::__runtime::RawMap, key: u64, value: TerminationResult) -> bool;
  fn __rust_proto_thunk__Map_u64_common_TerminationResult_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut TerminationResult) -> bool;
  fn __rust_proto_thunk__Map_u64_common_TerminationResult_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut TerminationResult) -> bool;
  fn __rust_proto_thunk__Map_u64_common_TerminationResult_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_u64_common_TerminationResult_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut TerminationResult);
}
impl ::__pb::ProxiedInMapValue<u64> for TerminationResult {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_common_TerminationResult_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
      unsafe { __rust_proto_thunk__Map_u64_common_TerminationResult_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
      unsafe { __rust_proto_thunk__Map_u64_common_TerminationResult_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_u64_common_TerminationResult_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_u64_common_TerminationResult_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_u64_common_TerminationResult_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_u64_common_TerminationResult_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_u64_common_TerminationResult_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_u64_common_TerminationResult_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_bool_common_TerminationResult_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_bool_common_TerminationResult_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_bool_common_TerminationResult_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_bool_common_TerminationResult_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_bool_common_TerminationResult_insert(m: ::__pb::__runtime::RawMap, key: bool, value: TerminationResult) -> bool;
  fn __rust_proto_thunk__Map_bool_common_TerminationResult_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut TerminationResult) -> bool;
  fn __rust_proto_thunk__Map_bool_common_TerminationResult_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut TerminationResult) -> bool;
  fn __rust_proto_thunk__Map_bool_common_TerminationResult_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_bool_common_TerminationResult_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut TerminationResult);
}
impl ::__pb::ProxiedInMapValue<bool> for TerminationResult {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_common_TerminationResult_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
      unsafe { __rust_proto_thunk__Map_bool_common_TerminationResult_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
      unsafe { __rust_proto_thunk__Map_bool_common_TerminationResult_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_bool_common_TerminationResult_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_bool_common_TerminationResult_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_bool_common_TerminationResult_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_bool_common_TerminationResult_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_bool_common_TerminationResult_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_bool_common_TerminationResult_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_ProtoStr_common_TerminationResult_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_ProtoStr_common_TerminationResult_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_ProtoStr_common_TerminationResult_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_ProtoStr_common_TerminationResult_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_ProtoStr_common_TerminationResult_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: TerminationResult) -> bool;
  fn __rust_proto_thunk__Map_ProtoStr_common_TerminationResult_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut TerminationResult) -> bool;
  fn __rust_proto_thunk__Map_ProtoStr_common_TerminationResult_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut TerminationResult) -> bool;
  fn __rust_proto_thunk__Map_ProtoStr_common_TerminationResult_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_ProtoStr_common_TerminationResult_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut TerminationResult);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for TerminationResult {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_common_TerminationResult_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
      unsafe { __rust_proto_thunk__Map_ProtoStr_common_TerminationResult_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
      unsafe { __rust_proto_thunk__Map_ProtoStr_common_TerminationResult_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_ProtoStr_common_TerminationResult_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_ProtoStr_common_TerminationResult_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
      let key = key.as_bytes().into();
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_ProtoStr_common_TerminationResult_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_ProtoStr_common_TerminationResult_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_ProtoStr_common_TerminationResult_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_ProtoStr_common_TerminationResult_iter_get,
              |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
              ::__std::convert::identity,
          )
      }
  }
}

