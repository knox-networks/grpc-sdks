extern crate protobuf_cpp as __pb;
extern crate std as __std;
#[allow(non_camel_case_types)]
pub struct Timestamp {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for Timestamp {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for Timestamp {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `Timestamp` is `Sync` because it does not implement interior mutability.
//    Neither does `TimestampMut`.
unsafe impl Sync for Timestamp {}

// SAFETY:
// - `Timestamp` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Timestamp {}

impl ::__pb::Proxied for Timestamp {
  type View<'msg> = TimestampView<'msg>;
  type Mut<'msg> = TimestampMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TimestampView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for TimestampView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> TimestampView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__google_protobuf_Timestamp_serialize(self.raw_msg()) }
  }

  // seconds: optional int64
  pub fn seconds(self) -> i64 {
    unsafe { __rust_proto_thunk__google_protobuf_Timestamp_get_seconds(self.raw_msg()) }
  }

  // nanos: optional int32
  pub fn nanos(self) -> i32 {
    unsafe { __rust_proto_thunk__google_protobuf_Timestamp_get_nanos(self.raw_msg()) }
  }

}

// SAFETY:
// - `TimestampView` is `Sync` because it does not support mutation.
unsafe impl Sync for TimestampView<'_> {}

// SAFETY:
// - `TimestampView` is `Send` because while its alive a `TimestampMut` cannot.
// - `TimestampView` does not use thread-local data.
unsafe impl Send for TimestampView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for TimestampView<'msg> {
  type Proxied = Timestamp;

  fn as_view(&self) -> ::__pb::View<'msg, Timestamp> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, Timestamp> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for Timestamp {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    TimestampView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    TimestampMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for Timestamp {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for Timestamp {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, Timestamp>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, Timestamp>;

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

impl<'msg> ::__pb::SettableValue<Timestamp> for TimestampView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, Timestamp>)
    where Timestamp: 'dst {
    unsafe { __rust_proto_thunk__google_protobuf_Timestamp_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<Timestamp> for Timestamp {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, Timestamp>)
    where Timestamp: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for Timestamp {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__google_protobuf_Timestamp_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__google_protobuf_Timestamp_copy_from(
        __rust_proto_thunk__google_protobuf_Timestamp_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__google_protobuf_Timestamp_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__google_protobuf_Timestamp_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__google_protobuf_Timestamp_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__google_protobuf_Timestamp_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__google_protobuf_Timestamp_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for Timestamp {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(TimestampView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| TimestampView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for Timestamp {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(TimestampView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| TimestampView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for Timestamp {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(TimestampView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| TimestampView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for Timestamp {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(TimestampView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| TimestampView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for Timestamp {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(TimestampView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| TimestampView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for Timestamp {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(TimestampView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| TimestampView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TimestampMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for TimestampMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> TimestampMut<'msg> {
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


  // seconds: optional int64
  pub fn seconds(&self) -> i64 {
    unsafe { __rust_proto_thunk__google_protobuf_Timestamp_get_seconds(self.raw_msg()) }
  }
  pub fn set_seconds(&mut self, val: i64) {
    unsafe { __rust_proto_thunk__google_protobuf_Timestamp_set_seconds(self.raw_msg(), val) }
  }

  // nanos: optional int32
  pub fn nanos(&self) -> i32 {
    unsafe { __rust_proto_thunk__google_protobuf_Timestamp_get_nanos(self.raw_msg()) }
  }
  pub fn set_nanos(&mut self, val: i32) {
    unsafe { __rust_proto_thunk__google_protobuf_Timestamp_set_nanos(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `TimestampMut` does not perform any shared mutation.
// - `TimestampMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for TimestampMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for TimestampMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, Timestamp> {
    TimestampMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, Timestamp> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for TimestampMut<'msg> {
  type Proxied = Timestamp;
  fn as_view(&self) -> ::__pb::View<'_, Timestamp> {
    TimestampView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, Timestamp> where 'msg: 'shorter {
    TimestampView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl Timestamp {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__google_protobuf_Timestamp_new() } } }
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

      __rust_proto_thunk__google_protobuf_Timestamp_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> TimestampView {
    TimestampView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> TimestampMut {
    TimestampMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // seconds: optional int64
  pub fn seconds(&self) -> i64 {
    unsafe { __rust_proto_thunk__google_protobuf_Timestamp_get_seconds(self.raw_msg()) }
  }
  pub fn set_seconds(&mut self, val: i64) {
    unsafe { __rust_proto_thunk__google_protobuf_Timestamp_set_seconds(self.raw_msg(), val) }
  }

  // nanos: optional int32
  pub fn nanos(&self) -> i32 {
    unsafe { __rust_proto_thunk__google_protobuf_Timestamp_get_nanos(self.raw_msg()) }
  }
  pub fn set_nanos(&mut self, val: i32) {
    unsafe { __rust_proto_thunk__google_protobuf_Timestamp_set_nanos(self.raw_msg(), val) }
  }

}  // impl Timestamp

impl ::__std::ops::Drop for Timestamp {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__google_protobuf_Timestamp_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__google_protobuf_Timestamp_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_protobuf_Timestamp_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_protobuf_Timestamp_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__google_protobuf_Timestamp_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__google_protobuf_Timestamp_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_protobuf_Timestamp_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__google_protobuf_Timestamp_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_protobuf_Timestamp_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_protobuf_Timestamp_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_protobuf_Timestamp_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__google_protobuf_Timestamp_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__google_protobuf_Timestamp_get_seconds(raw_msg: ::__pb::__runtime::RawMessage) -> i64;
  fn __rust_proto_thunk__google_protobuf_Timestamp_set_seconds(raw_msg: ::__pb::__runtime::RawMessage, val: i64);

  fn __rust_proto_thunk__google_protobuf_Timestamp_get_nanos(raw_msg: ::__pb::__runtime::RawMessage) -> i32;
  fn __rust_proto_thunk__google_protobuf_Timestamp_set_nanos(raw_msg: ::__pb::__runtime::RawMessage, val: i32);


}  // extern "C" for Timestamp


impl Timestamp {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> TimestampMut<'a> {
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

impl<'a> TimestampView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

