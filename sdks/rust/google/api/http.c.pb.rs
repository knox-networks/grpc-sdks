extern crate protobuf_cpp as __pb;
extern crate std as __std;
#[allow(non_camel_case_types)]
pub struct Http {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for Http {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for Http {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `Http` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpMut`.
unsafe impl Sync for Http {}

// SAFETY:
// - `Http` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Http {}

impl ::__pb::Proxied for Http {
  type View<'msg> = HttpView<'msg>;
  type Mut<'msg> = HttpMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for HttpView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> HttpView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__google_api_Http_serialize(self.raw_msg()) }
  }

  // rules: repeated message google.api.HttpRule
  pub fn rules(self) -> ::__pb::RepeatedView<'msg, crate::HttpRule> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__google_api_Http_get_rules(self.raw_msg()),
      )
    }
  }

  // fully_decode_reserved_expansion: optional bool
  pub fn fully_decode_reserved_expansion(self) -> bool {
    unsafe { __rust_proto_thunk__google_api_Http_get_fully_decode_reserved_expansion(self.raw_msg()) }
  }

}

// SAFETY:
// - `HttpView` is `Sync` because it does not support mutation.
unsafe impl Sync for HttpView<'_> {}

// SAFETY:
// - `HttpView` is `Send` because while its alive a `HttpMut` cannot.
// - `HttpView` does not use thread-local data.
unsafe impl Send for HttpView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for HttpView<'msg> {
  type Proxied = Http;

  fn as_view(&self) -> ::__pb::View<'msg, Http> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, Http> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for Http {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    HttpView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    HttpMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for Http {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for Http {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, Http>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, Http>;

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

impl<'msg> ::__pb::SettableValue<Http> for HttpView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, Http>)
    where Http: 'dst {
    unsafe { __rust_proto_thunk__google_api_Http_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<Http> for Http {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, Http>)
    where Http: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for Http {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__google_api_Http_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__google_api_Http_copy_from(
        __rust_proto_thunk__google_api_Http_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__google_api_Http_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__google_api_Http_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__google_api_Http_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__google_api_Http_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__google_api_Http_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_google_api_Http_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_google_api_Http_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_google_api_Http_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_google_api_Http_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_google_api_Http_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_google_api_Http_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_google_api_Http_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_google_api_Http_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_google_api_Http_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for Http {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_google_api_Http_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_google_api_Http_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_google_api_Http_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_google_api_Http_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_google_api_Http_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_google_api_Http_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(HttpView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_google_api_Http_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_google_api_Http_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_google_api_Http_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| HttpView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_google_api_Http_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_google_api_Http_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_google_api_Http_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_google_api_Http_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_google_api_Http_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_google_api_Http_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_google_api_Http_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_google_api_Http_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_google_api_Http_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for Http {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_google_api_Http_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_google_api_Http_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_google_api_Http_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_google_api_Http_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_google_api_Http_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_google_api_Http_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(HttpView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_google_api_Http_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_google_api_Http_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_google_api_Http_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| HttpView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_google_api_Http_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_google_api_Http_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_google_api_Http_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_google_api_Http_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_google_api_Http_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_google_api_Http_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_google_api_Http_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_google_api_Http_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_google_api_Http_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for Http {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_google_api_Http_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_google_api_Http_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_google_api_Http_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_google_api_Http_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_google_api_Http_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_google_api_Http_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(HttpView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_google_api_Http_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_google_api_Http_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_google_api_Http_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| HttpView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_google_api_Http_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_google_api_Http_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_google_api_Http_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_google_api_Http_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_google_api_Http_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_google_api_Http_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_google_api_Http_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_google_api_Http_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_google_api_Http_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for Http {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_google_api_Http_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_google_api_Http_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_google_api_Http_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_google_api_Http_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_google_api_Http_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_google_api_Http_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(HttpView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_google_api_Http_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_google_api_Http_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_google_api_Http_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| HttpView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_google_api_Http_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_google_api_Http_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_google_api_Http_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_google_api_Http_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_google_api_Http_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_google_api_Http_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_google_api_Http_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_google_api_Http_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_google_api_Http_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for Http {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_google_api_Http_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_google_api_Http_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_google_api_Http_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_google_api_Http_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_google_api_Http_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_google_api_Http_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(HttpView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_google_api_Http_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_google_api_Http_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_google_api_Http_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| HttpView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_google_api_Http_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_google_api_Http_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_google_api_Http_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_google_api_Http_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_google_api_Http_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_google_api_Http_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_google_api_Http_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_google_api_Http_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_google_api_Http_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for Http {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_google_api_Http_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_api_Http_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_api_Http_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_api_Http_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_api_Http_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_google_api_Http_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(HttpView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_api_Http_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_google_api_Http_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_google_api_Http_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| HttpView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for HttpMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> HttpMut<'msg> {
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


  // rules: repeated message google.api.HttpRule
  pub fn rules(&self) -> ::__pb::RepeatedView<'_, crate::HttpRule> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__google_api_Http_get_rules(self.raw_msg()),
      )
    }
  }
  pub fn rules_mut(&mut self) -> ::__pb::RepeatedMut<'_, crate::HttpRule> {
    unsafe {
      ::__pb::RepeatedMut::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__runtime::InnerRepeatedMut::new(
          ::__pb::__internal::Private,
          __rust_proto_thunk__google_api_Http_get_mut_rules(self.raw_msg()),
        ),
      )
    }
  }

  // fully_decode_reserved_expansion: optional bool
  pub fn fully_decode_reserved_expansion(&self) -> bool {
    unsafe { __rust_proto_thunk__google_api_Http_get_fully_decode_reserved_expansion(self.raw_msg()) }
  }
  pub fn set_fully_decode_reserved_expansion(&mut self, val: bool) {
    unsafe { __rust_proto_thunk__google_api_Http_set_fully_decode_reserved_expansion(self.raw_msg(), val) }
  }

}

// SAFETY:
// - `HttpMut` does not perform any shared mutation.
// - `HttpMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for HttpMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for HttpMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, Http> {
    HttpMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, Http> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for HttpMut<'msg> {
  type Proxied = Http;
  fn as_view(&self) -> ::__pb::View<'_, Http> {
    HttpView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, Http> where 'msg: 'shorter {
    HttpView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl Http {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__google_api_Http_new() } } }
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

      __rust_proto_thunk__google_api_Http_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> HttpView {
    HttpView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> HttpMut {
    HttpMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // rules: repeated message google.api.HttpRule
  pub fn rules(&self) -> ::__pb::RepeatedView<'_, crate::HttpRule> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__google_api_Http_get_rules(self.raw_msg()),
      )
    }
  }
  pub fn rules_mut(&mut self) -> ::__pb::RepeatedMut<'_, crate::HttpRule> {
    unsafe {
      ::__pb::RepeatedMut::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__runtime::InnerRepeatedMut::new(
          ::__pb::__internal::Private,
          __rust_proto_thunk__google_api_Http_get_mut_rules(self.raw_msg()),
        ),
      )
    }
  }

  // fully_decode_reserved_expansion: optional bool
  pub fn fully_decode_reserved_expansion(&self) -> bool {
    unsafe { __rust_proto_thunk__google_api_Http_get_fully_decode_reserved_expansion(self.raw_msg()) }
  }
  pub fn set_fully_decode_reserved_expansion(&mut self, val: bool) {
    unsafe { __rust_proto_thunk__google_api_Http_set_fully_decode_reserved_expansion(self.raw_msg(), val) }
  }

}  // impl Http

impl ::__std::ops::Drop for Http {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__google_api_Http_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__google_api_Http_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_api_Http_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_api_Http_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__google_api_Http_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__google_api_Http_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_api_Http_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__google_api_Http_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_api_Http_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_api_Http_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_api_Http_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__google_api_Http_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__google_api_Http_clear_rules(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_api_Http_get_mut_rules(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawRepeatedField;
  fn __rust_proto_thunk__google_api_Http_get_rules(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawRepeatedField;

  fn __rust_proto_thunk__google_api_Http_get_fully_decode_reserved_expansion(raw_msg: ::__pb::__runtime::RawMessage) -> bool;
  fn __rust_proto_thunk__google_api_Http_set_fully_decode_reserved_expansion(raw_msg: ::__pb::__runtime::RawMessage, val: bool);


}  // extern "C" for Http


impl Http {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> HttpMut<'a> {
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

impl<'a> HttpView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct HttpRule {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for HttpRule {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for HttpRule {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `HttpRule` is `Sync` because it does not implement interior mutability.
//    Neither does `HttpRuleMut`.
unsafe impl Sync for HttpRule {}

// SAFETY:
// - `HttpRule` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for HttpRule {}

impl ::__pb::Proxied for HttpRule {
  type View<'msg> = HttpRuleView<'msg>;
  type Mut<'msg> = HttpRuleMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct HttpRuleView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for HttpRuleView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> HttpRuleView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__google_api_HttpRule_serialize(self.raw_msg()) }
  }

  // selector: optional string
  pub fn selector(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_selector(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // get: optional string
  pub fn get(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_get(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn get_opt(self) -> ::__pb::Optional<&'msg ::__pb::ProtoStr> {
      ::__pb::Optional::new(
        self.get(),
        self.has_get()
      )
    }
  pub fn has_get(self) -> bool {
    unsafe { __rust_proto_thunk__google_api_HttpRule_has_get(self.raw_msg()) }
  }

  // put: optional string
  pub fn put(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_put(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn put_opt(self) -> ::__pb::Optional<&'msg ::__pb::ProtoStr> {
      ::__pb::Optional::new(
        self.put(),
        self.has_put()
      )
    }
  pub fn has_put(self) -> bool {
    unsafe { __rust_proto_thunk__google_api_HttpRule_has_put(self.raw_msg()) }
  }

  // post: optional string
  pub fn post(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_post(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn post_opt(self) -> ::__pb::Optional<&'msg ::__pb::ProtoStr> {
      ::__pb::Optional::new(
        self.post(),
        self.has_post()
      )
    }
  pub fn has_post(self) -> bool {
    unsafe { __rust_proto_thunk__google_api_HttpRule_has_post(self.raw_msg()) }
  }

  // delete: optional string
  pub fn delete(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_delete(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn delete_opt(self) -> ::__pb::Optional<&'msg ::__pb::ProtoStr> {
      ::__pb::Optional::new(
        self.delete(),
        self.has_delete()
      )
    }
  pub fn has_delete(self) -> bool {
    unsafe { __rust_proto_thunk__google_api_HttpRule_has_delete(self.raw_msg()) }
  }

  // patch: optional string
  pub fn patch(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_patch(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn patch_opt(self) -> ::__pb::Optional<&'msg ::__pb::ProtoStr> {
      ::__pb::Optional::new(
        self.patch(),
        self.has_patch()
      )
    }
  pub fn has_patch(self) -> bool {
    unsafe { __rust_proto_thunk__google_api_HttpRule_has_patch(self.raw_msg()) }
  }

  // custom: optional message google.api.CustomHttpPattern
  pub fn custom(self) -> crate::CustomHttpPatternView<'msg> {
                  let submsg = unsafe { __rust_proto_thunk__google_api_HttpRule_get_custom(self.raw_msg()) };
    crate::CustomHttpPatternView::new(::__pb::__internal::Private, submsg)
  }
  pub fn custom_opt(self) ->
  ::__pb::Optional<crate::CustomHttpPatternView<'msg>> {
    let view = self.custom();
    ::__pb::Optional::new(view, self.has_custom())
  }
  pub fn has_custom(self) -> bool {
    unsafe { __rust_proto_thunk__google_api_HttpRule_has_custom(self.raw_msg()) }
  }

  // body: optional string
  pub fn body(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_body(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // response_body: optional string
  pub fn response_body(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_response_body(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // additional_bindings: repeated message google.api.HttpRule
  pub fn additional_bindings(self) -> ::__pb::RepeatedView<'msg, crate::HttpRule> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__google_api_HttpRule_get_additional_bindings(self.raw_msg()),
      )
    }
  }

  pub fn pattern(self) -> crate::http_rule::Pattern<'msg> {
    match self.pattern_case() {
      crate::http_rule::PatternCase::Get =>
          crate::http_rule::Pattern::Get(self.get()),
      crate::http_rule::PatternCase::Put =>
          crate::http_rule::Pattern::Put(self.put()),
      crate::http_rule::PatternCase::Post =>
          crate::http_rule::Pattern::Post(self.post()),
      crate::http_rule::PatternCase::Delete =>
          crate::http_rule::Pattern::Delete(self.delete()),
      crate::http_rule::PatternCase::Patch =>
          crate::http_rule::Pattern::Patch(self.patch()),
      crate::http_rule::PatternCase::Custom =>
          crate::http_rule::Pattern::Custom(self.custom()),
      _ => crate::http_rule::Pattern::not_set(std::marker::PhantomData)
    }
  }

  pub fn pattern_case(self) -> crate::http_rule::PatternCase {
    unsafe { __rust_proto_thunk__google_api_HttpRule_case_pattern(self.raw_msg()) }
  }
}

// SAFETY:
// - `HttpRuleView` is `Sync` because it does not support mutation.
unsafe impl Sync for HttpRuleView<'_> {}

// SAFETY:
// - `HttpRuleView` is `Send` because while its alive a `HttpRuleMut` cannot.
// - `HttpRuleView` does not use thread-local data.
unsafe impl Send for HttpRuleView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for HttpRuleView<'msg> {
  type Proxied = HttpRule;

  fn as_view(&self) -> ::__pb::View<'msg, HttpRule> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, HttpRule> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for HttpRule {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    HttpRuleView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    HttpRuleMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for HttpRule {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for HttpRule {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, HttpRule>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, HttpRule>;

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

impl<'msg> ::__pb::SettableValue<HttpRule> for HttpRuleView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, HttpRule>)
    where HttpRule: 'dst {
    unsafe { __rust_proto_thunk__google_api_HttpRule_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<HttpRule> for HttpRule {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, HttpRule>)
    where HttpRule: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for HttpRule {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__google_api_HttpRule_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__google_api_HttpRule_copy_from(
        __rust_proto_thunk__google_api_HttpRule_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__google_api_HttpRule_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__google_api_HttpRule_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__google_api_HttpRule_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__google_api_HttpRule_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__google_api_HttpRule_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_google_api_HttpRule_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_google_api_HttpRule_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_google_api_HttpRule_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_google_api_HttpRule_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_google_api_HttpRule_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_google_api_HttpRule_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_google_api_HttpRule_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_google_api_HttpRule_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_google_api_HttpRule_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for HttpRule {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_google_api_HttpRule_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_google_api_HttpRule_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_google_api_HttpRule_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_google_api_HttpRule_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_google_api_HttpRule_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_google_api_HttpRule_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(HttpRuleView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_google_api_HttpRule_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_google_api_HttpRule_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_google_api_HttpRule_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| HttpRuleView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_google_api_HttpRule_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_google_api_HttpRule_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_google_api_HttpRule_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_google_api_HttpRule_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_google_api_HttpRule_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_google_api_HttpRule_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_google_api_HttpRule_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_google_api_HttpRule_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_google_api_HttpRule_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for HttpRule {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_google_api_HttpRule_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_google_api_HttpRule_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_google_api_HttpRule_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_google_api_HttpRule_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_google_api_HttpRule_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_google_api_HttpRule_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(HttpRuleView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_google_api_HttpRule_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_google_api_HttpRule_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_google_api_HttpRule_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| HttpRuleView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_google_api_HttpRule_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_google_api_HttpRule_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_google_api_HttpRule_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_google_api_HttpRule_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_google_api_HttpRule_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_google_api_HttpRule_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_google_api_HttpRule_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_google_api_HttpRule_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_google_api_HttpRule_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for HttpRule {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_google_api_HttpRule_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_google_api_HttpRule_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_google_api_HttpRule_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_google_api_HttpRule_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_google_api_HttpRule_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_google_api_HttpRule_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(HttpRuleView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_google_api_HttpRule_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_google_api_HttpRule_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_google_api_HttpRule_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| HttpRuleView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_google_api_HttpRule_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_google_api_HttpRule_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_google_api_HttpRule_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_google_api_HttpRule_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_google_api_HttpRule_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_google_api_HttpRule_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_google_api_HttpRule_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_google_api_HttpRule_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_google_api_HttpRule_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for HttpRule {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_google_api_HttpRule_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_google_api_HttpRule_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_google_api_HttpRule_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_google_api_HttpRule_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_google_api_HttpRule_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_google_api_HttpRule_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(HttpRuleView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_google_api_HttpRule_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_google_api_HttpRule_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_google_api_HttpRule_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| HttpRuleView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_google_api_HttpRule_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_google_api_HttpRule_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_google_api_HttpRule_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_google_api_HttpRule_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_google_api_HttpRule_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_google_api_HttpRule_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_google_api_HttpRule_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_google_api_HttpRule_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_google_api_HttpRule_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for HttpRule {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_google_api_HttpRule_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_google_api_HttpRule_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_google_api_HttpRule_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_google_api_HttpRule_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_google_api_HttpRule_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_google_api_HttpRule_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(HttpRuleView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_google_api_HttpRule_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_google_api_HttpRule_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_google_api_HttpRule_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| HttpRuleView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for HttpRule {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(HttpRuleView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| HttpRuleView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct HttpRuleMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for HttpRuleMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> HttpRuleMut<'msg> {
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


  // selector: optional string
  pub fn selector(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_selector(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_selector(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.selector_mut().set(val);
  }
  fn selector_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          HttpRule::__SELECTOR_VTABLE,
        )
      )
    }
  }

  // get: optional string
  pub fn get(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_get(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn get_opt(&self) -> ::__pb::Optional<&'_ ::__pb::ProtoStr> {
      ::__pb::Optional::new(
        self.get(),
        self.has_get()
      )
    }
  pub fn set_get(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.get_mut().set(val);
  }
  pub fn has_get(&self) -> bool {
    unsafe { __rust_proto_thunk__google_api_HttpRule_has_get(self.raw_msg()) }
  }
  pub fn clear_get(&mut self) {
    unsafe { __rust_proto_thunk__google_api_HttpRule_clear_get(self.raw_msg()) }
  }
  fn get_mut(&mut self) -> ::__pb::FieldEntry<'_, ::__pb::ProtoStr> {
    let out = unsafe {
      let has = __rust_proto_thunk__google_api_HttpRule_has_get(self.raw_msg());
      ::__pb::__internal::new_vtable_field_entry(
        ::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        HttpRule::__GET_VTABLE,
        has,
      )
    };
    ::__pb::ProtoStrMut::field_entry_from_bytes(
      ::__pb::__internal::Private, out
    )
  }

  // put: optional string
  pub fn put(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_put(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn put_opt(&self) -> ::__pb::Optional<&'_ ::__pb::ProtoStr> {
      ::__pb::Optional::new(
        self.put(),
        self.has_put()
      )
    }
  pub fn set_put(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.put_mut().set(val);
  }
  pub fn has_put(&self) -> bool {
    unsafe { __rust_proto_thunk__google_api_HttpRule_has_put(self.raw_msg()) }
  }
  pub fn clear_put(&mut self) {
    unsafe { __rust_proto_thunk__google_api_HttpRule_clear_put(self.raw_msg()) }
  }
  fn put_mut(&mut self) -> ::__pb::FieldEntry<'_, ::__pb::ProtoStr> {
    let out = unsafe {
      let has = __rust_proto_thunk__google_api_HttpRule_has_put(self.raw_msg());
      ::__pb::__internal::new_vtable_field_entry(
        ::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        HttpRule::__PUT_VTABLE,
        has,
      )
    };
    ::__pb::ProtoStrMut::field_entry_from_bytes(
      ::__pb::__internal::Private, out
    )
  }

  // post: optional string
  pub fn post(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_post(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn post_opt(&self) -> ::__pb::Optional<&'_ ::__pb::ProtoStr> {
      ::__pb::Optional::new(
        self.post(),
        self.has_post()
      )
    }
  pub fn set_post(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.post_mut().set(val);
  }
  pub fn has_post(&self) -> bool {
    unsafe { __rust_proto_thunk__google_api_HttpRule_has_post(self.raw_msg()) }
  }
  pub fn clear_post(&mut self) {
    unsafe { __rust_proto_thunk__google_api_HttpRule_clear_post(self.raw_msg()) }
  }
  fn post_mut(&mut self) -> ::__pb::FieldEntry<'_, ::__pb::ProtoStr> {
    let out = unsafe {
      let has = __rust_proto_thunk__google_api_HttpRule_has_post(self.raw_msg());
      ::__pb::__internal::new_vtable_field_entry(
        ::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        HttpRule::__POST_VTABLE,
        has,
      )
    };
    ::__pb::ProtoStrMut::field_entry_from_bytes(
      ::__pb::__internal::Private, out
    )
  }

  // delete: optional string
  pub fn delete(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_delete(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn delete_opt(&self) -> ::__pb::Optional<&'_ ::__pb::ProtoStr> {
      ::__pb::Optional::new(
        self.delete(),
        self.has_delete()
      )
    }
  pub fn set_delete(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.delete_mut().set(val);
  }
  pub fn has_delete(&self) -> bool {
    unsafe { __rust_proto_thunk__google_api_HttpRule_has_delete(self.raw_msg()) }
  }
  pub fn clear_delete(&mut self) {
    unsafe { __rust_proto_thunk__google_api_HttpRule_clear_delete(self.raw_msg()) }
  }
  fn delete_mut(&mut self) -> ::__pb::FieldEntry<'_, ::__pb::ProtoStr> {
    let out = unsafe {
      let has = __rust_proto_thunk__google_api_HttpRule_has_delete(self.raw_msg());
      ::__pb::__internal::new_vtable_field_entry(
        ::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        HttpRule::__DELETE_VTABLE,
        has,
      )
    };
    ::__pb::ProtoStrMut::field_entry_from_bytes(
      ::__pb::__internal::Private, out
    )
  }

  // patch: optional string
  pub fn patch(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_patch(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn patch_opt(&self) -> ::__pb::Optional<&'_ ::__pb::ProtoStr> {
      ::__pb::Optional::new(
        self.patch(),
        self.has_patch()
      )
    }
  pub fn set_patch(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.patch_mut().set(val);
  }
  pub fn has_patch(&self) -> bool {
    unsafe { __rust_proto_thunk__google_api_HttpRule_has_patch(self.raw_msg()) }
  }
  pub fn clear_patch(&mut self) {
    unsafe { __rust_proto_thunk__google_api_HttpRule_clear_patch(self.raw_msg()) }
  }
  fn patch_mut(&mut self) -> ::__pb::FieldEntry<'_, ::__pb::ProtoStr> {
    let out = unsafe {
      let has = __rust_proto_thunk__google_api_HttpRule_has_patch(self.raw_msg());
      ::__pb::__internal::new_vtable_field_entry(
        ::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        HttpRule::__PATCH_VTABLE,
        has,
      )
    };
    ::__pb::ProtoStrMut::field_entry_from_bytes(
      ::__pb::__internal::Private, out
    )
  }

  // custom: optional message google.api.CustomHttpPattern
  pub fn custom(&self) -> crate::CustomHttpPatternView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__google_api_HttpRule_get_custom(self.raw_msg()) };
    crate::CustomHttpPatternView::new(::__pb::__internal::Private, submsg)
  }
  pub fn custom_mut(&mut self) -> crate::CustomHttpPatternMut<'_> {
    self.custom_entry().or_default()
  }
  fn custom_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::CustomHttpPattern> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__google_api_HttpRule_get_custom,
                                __rust_proto_thunk__google_api_HttpRule_get_mut_custom,
                                __rust_proto_thunk__google_api_HttpRule_clear_custom);
    unsafe {
      let has = self.has_custom();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn custom_opt(&self) ->
  ::__pb::Optional<crate::CustomHttpPatternView<'_>> {
    let view = self.custom();
    ::__pb::Optional::new(view, self.has_custom())
  }
  pub fn set_custom(&mut self, val: impl ::__pb::SettableValue<crate::CustomHttpPattern>) {
    self.custom_entry().set(val);
  }
  pub fn has_custom(&self) -> bool {
    unsafe { __rust_proto_thunk__google_api_HttpRule_has_custom(self.raw_msg()) }
  }
  pub fn clear_custom(&mut self) {
    unsafe { __rust_proto_thunk__google_api_HttpRule_clear_custom(self.raw_msg()) }
  }

  // body: optional string
  pub fn body(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_body(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_body(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.body_mut().set(val);
  }
  fn body_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          HttpRule::__BODY_VTABLE,
        )
      )
    }
  }

  // response_body: optional string
  pub fn response_body(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_response_body(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_response_body(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.response_body_mut().set(val);
  }
  fn response_body_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          HttpRule::__RESPONSE_BODY_VTABLE,
        )
      )
    }
  }

  // additional_bindings: repeated message google.api.HttpRule
  pub fn additional_bindings(&self) -> ::__pb::RepeatedView<'_, crate::HttpRule> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__google_api_HttpRule_get_additional_bindings(self.raw_msg()),
      )
    }
  }
  pub fn additional_bindings_mut(&mut self) -> ::__pb::RepeatedMut<'_, crate::HttpRule> {
    unsafe {
      ::__pb::RepeatedMut::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__runtime::InnerRepeatedMut::new(
          ::__pb::__internal::Private,
          __rust_proto_thunk__google_api_HttpRule_get_mut_additional_bindings(self.raw_msg()),
        ),
      )
    }
  }

  pub fn pattern(&self) -> crate::http_rule::Pattern<'_> {
    match &self.pattern_case() {
      crate::http_rule::PatternCase::Get =>
          crate::http_rule::Pattern::Get(self.get()),
      crate::http_rule::PatternCase::Put =>
          crate::http_rule::Pattern::Put(self.put()),
      crate::http_rule::PatternCase::Post =>
          crate::http_rule::Pattern::Post(self.post()),
      crate::http_rule::PatternCase::Delete =>
          crate::http_rule::Pattern::Delete(self.delete()),
      crate::http_rule::PatternCase::Patch =>
          crate::http_rule::Pattern::Patch(self.patch()),
      crate::http_rule::PatternCase::Custom =>
          crate::http_rule::Pattern::Custom(self.custom()),
      _ => crate::http_rule::Pattern::not_set(std::marker::PhantomData)
    }
  }

  pub fn pattern_case(&self) -> crate::http_rule::PatternCase {
    unsafe { __rust_proto_thunk__google_api_HttpRule_case_pattern(self.raw_msg()) }
  }
}

// SAFETY:
// - `HttpRuleMut` does not perform any shared mutation.
// - `HttpRuleMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for HttpRuleMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for HttpRuleMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, HttpRule> {
    HttpRuleMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, HttpRule> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for HttpRuleMut<'msg> {
  type Proxied = HttpRule;
  fn as_view(&self) -> ::__pb::View<'_, HttpRule> {
    HttpRuleView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, HttpRule> where 'msg: 'shorter {
    HttpRuleView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl HttpRule {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__google_api_HttpRule_new() } } }
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

      __rust_proto_thunk__google_api_HttpRule_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> HttpRuleView {
    HttpRuleView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> HttpRuleMut {
    HttpRuleMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // selector: optional string
  pub fn selector(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_selector(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_selector(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.selector_mut().set(val);
  }
  const __SELECTOR_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__google_api_HttpRule_get_selector,
      __rust_proto_thunk__google_api_HttpRule_set_selector,
    );
  fn selector_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          HttpRule::__SELECTOR_VTABLE,
        )
      )
    }
  }

  // get: optional string
  pub fn get(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_get(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn get_opt(&self) -> ::__pb::Optional<&'_ ::__pb::ProtoStr> {
      ::__pb::Optional::new(
        self.get(),
        self.has_get()
      )
    }
  pub fn set_get(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.get_mut().set(val);
  }
  pub fn has_get(&self) -> bool {
    unsafe { __rust_proto_thunk__google_api_HttpRule_has_get(self.raw_msg()) }
  }
  pub fn clear_get(&mut self) {
    unsafe { __rust_proto_thunk__google_api_HttpRule_clear_get(self.raw_msg()) }
  }
  // SAFETY: for `string` fields, the default value is verified as valid UTF-8
  const __GET_VTABLE: &'static ::__pb::__internal::BytesOptionalMutVTable = &unsafe {
      ::__pb::__internal::BytesOptionalMutVTable::new(
        ::__pb::__internal::Private,
        __rust_proto_thunk__google_api_HttpRule_get_get,
        __rust_proto_thunk__google_api_HttpRule_set_get,
        __rust_proto_thunk__google_api_HttpRule_clear_get,
        b"",
      )
    };
  fn get_mut(&mut self) -> ::__pb::FieldEntry<'_, ::__pb::ProtoStr> {
    let out = unsafe {
      let has = __rust_proto_thunk__google_api_HttpRule_has_get(self.raw_msg());
      ::__pb::__internal::new_vtable_field_entry(
        ::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        HttpRule::__GET_VTABLE,
        has,
      )
    };
    ::__pb::ProtoStrMut::field_entry_from_bytes(
      ::__pb::__internal::Private, out
    )
  }

  // put: optional string
  pub fn put(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_put(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn put_opt(&self) -> ::__pb::Optional<&'_ ::__pb::ProtoStr> {
      ::__pb::Optional::new(
        self.put(),
        self.has_put()
      )
    }
  pub fn set_put(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.put_mut().set(val);
  }
  pub fn has_put(&self) -> bool {
    unsafe { __rust_proto_thunk__google_api_HttpRule_has_put(self.raw_msg()) }
  }
  pub fn clear_put(&mut self) {
    unsafe { __rust_proto_thunk__google_api_HttpRule_clear_put(self.raw_msg()) }
  }
  // SAFETY: for `string` fields, the default value is verified as valid UTF-8
  const __PUT_VTABLE: &'static ::__pb::__internal::BytesOptionalMutVTable = &unsafe {
      ::__pb::__internal::BytesOptionalMutVTable::new(
        ::__pb::__internal::Private,
        __rust_proto_thunk__google_api_HttpRule_get_put,
        __rust_proto_thunk__google_api_HttpRule_set_put,
        __rust_proto_thunk__google_api_HttpRule_clear_put,
        b"",
      )
    };
  fn put_mut(&mut self) -> ::__pb::FieldEntry<'_, ::__pb::ProtoStr> {
    let out = unsafe {
      let has = __rust_proto_thunk__google_api_HttpRule_has_put(self.raw_msg());
      ::__pb::__internal::new_vtable_field_entry(
        ::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        HttpRule::__PUT_VTABLE,
        has,
      )
    };
    ::__pb::ProtoStrMut::field_entry_from_bytes(
      ::__pb::__internal::Private, out
    )
  }

  // post: optional string
  pub fn post(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_post(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn post_opt(&self) -> ::__pb::Optional<&'_ ::__pb::ProtoStr> {
      ::__pb::Optional::new(
        self.post(),
        self.has_post()
      )
    }
  pub fn set_post(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.post_mut().set(val);
  }
  pub fn has_post(&self) -> bool {
    unsafe { __rust_proto_thunk__google_api_HttpRule_has_post(self.raw_msg()) }
  }
  pub fn clear_post(&mut self) {
    unsafe { __rust_proto_thunk__google_api_HttpRule_clear_post(self.raw_msg()) }
  }
  // SAFETY: for `string` fields, the default value is verified as valid UTF-8
  const __POST_VTABLE: &'static ::__pb::__internal::BytesOptionalMutVTable = &unsafe {
      ::__pb::__internal::BytesOptionalMutVTable::new(
        ::__pb::__internal::Private,
        __rust_proto_thunk__google_api_HttpRule_get_post,
        __rust_proto_thunk__google_api_HttpRule_set_post,
        __rust_proto_thunk__google_api_HttpRule_clear_post,
        b"",
      )
    };
  fn post_mut(&mut self) -> ::__pb::FieldEntry<'_, ::__pb::ProtoStr> {
    let out = unsafe {
      let has = __rust_proto_thunk__google_api_HttpRule_has_post(self.raw_msg());
      ::__pb::__internal::new_vtable_field_entry(
        ::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        HttpRule::__POST_VTABLE,
        has,
      )
    };
    ::__pb::ProtoStrMut::field_entry_from_bytes(
      ::__pb::__internal::Private, out
    )
  }

  // delete: optional string
  pub fn delete(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_delete(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn delete_opt(&self) -> ::__pb::Optional<&'_ ::__pb::ProtoStr> {
      ::__pb::Optional::new(
        self.delete(),
        self.has_delete()
      )
    }
  pub fn set_delete(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.delete_mut().set(val);
  }
  pub fn has_delete(&self) -> bool {
    unsafe { __rust_proto_thunk__google_api_HttpRule_has_delete(self.raw_msg()) }
  }
  pub fn clear_delete(&mut self) {
    unsafe { __rust_proto_thunk__google_api_HttpRule_clear_delete(self.raw_msg()) }
  }
  // SAFETY: for `string` fields, the default value is verified as valid UTF-8
  const __DELETE_VTABLE: &'static ::__pb::__internal::BytesOptionalMutVTable = &unsafe {
      ::__pb::__internal::BytesOptionalMutVTable::new(
        ::__pb::__internal::Private,
        __rust_proto_thunk__google_api_HttpRule_get_delete,
        __rust_proto_thunk__google_api_HttpRule_set_delete,
        __rust_proto_thunk__google_api_HttpRule_clear_delete,
        b"",
      )
    };
  fn delete_mut(&mut self) -> ::__pb::FieldEntry<'_, ::__pb::ProtoStr> {
    let out = unsafe {
      let has = __rust_proto_thunk__google_api_HttpRule_has_delete(self.raw_msg());
      ::__pb::__internal::new_vtable_field_entry(
        ::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        HttpRule::__DELETE_VTABLE,
        has,
      )
    };
    ::__pb::ProtoStrMut::field_entry_from_bytes(
      ::__pb::__internal::Private, out
    )
  }

  // patch: optional string
  pub fn patch(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_patch(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn patch_opt(&self) -> ::__pb::Optional<&'_ ::__pb::ProtoStr> {
      ::__pb::Optional::new(
        self.patch(),
        self.has_patch()
      )
    }
  pub fn set_patch(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.patch_mut().set(val);
  }
  pub fn has_patch(&self) -> bool {
    unsafe { __rust_proto_thunk__google_api_HttpRule_has_patch(self.raw_msg()) }
  }
  pub fn clear_patch(&mut self) {
    unsafe { __rust_proto_thunk__google_api_HttpRule_clear_patch(self.raw_msg()) }
  }
  // SAFETY: for `string` fields, the default value is verified as valid UTF-8
  const __PATCH_VTABLE: &'static ::__pb::__internal::BytesOptionalMutVTable = &unsafe {
      ::__pb::__internal::BytesOptionalMutVTable::new(
        ::__pb::__internal::Private,
        __rust_proto_thunk__google_api_HttpRule_get_patch,
        __rust_proto_thunk__google_api_HttpRule_set_patch,
        __rust_proto_thunk__google_api_HttpRule_clear_patch,
        b"",
      )
    };
  fn patch_mut(&mut self) -> ::__pb::FieldEntry<'_, ::__pb::ProtoStr> {
    let out = unsafe {
      let has = __rust_proto_thunk__google_api_HttpRule_has_patch(self.raw_msg());
      ::__pb::__internal::new_vtable_field_entry(
        ::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        HttpRule::__PATCH_VTABLE,
        has,
      )
    };
    ::__pb::ProtoStrMut::field_entry_from_bytes(
      ::__pb::__internal::Private, out
    )
  }

  // custom: optional message google.api.CustomHttpPattern
  pub fn custom(&self) -> crate::CustomHttpPatternView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__google_api_HttpRule_get_custom(self.raw_msg()) };
    crate::CustomHttpPatternView::new(::__pb::__internal::Private, submsg)
  }
  pub fn custom_mut(&mut self) -> crate::CustomHttpPatternMut<'_> {
    self.custom_entry().or_default()
  }
  fn custom_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::CustomHttpPattern> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__google_api_HttpRule_get_custom,
                                __rust_proto_thunk__google_api_HttpRule_get_mut_custom,
                                __rust_proto_thunk__google_api_HttpRule_clear_custom);
    unsafe {
      let has = self.has_custom();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn custom_opt(&self) ->
  ::__pb::Optional<crate::CustomHttpPatternView<'_>> {
    let view = self.custom();
    ::__pb::Optional::new(view, self.has_custom())
  }
  pub fn set_custom(&mut self, val: impl ::__pb::SettableValue<crate::CustomHttpPattern>) {
    self.custom_entry().set(val);
  }
  pub fn has_custom(&self) -> bool {
    unsafe { __rust_proto_thunk__google_api_HttpRule_has_custom(self.raw_msg()) }
  }
  pub fn clear_custom(&mut self) {
    unsafe { __rust_proto_thunk__google_api_HttpRule_clear_custom(self.raw_msg()) }
  }

  // body: optional string
  pub fn body(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_body(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_body(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.body_mut().set(val);
  }
  const __BODY_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__google_api_HttpRule_get_body,
      __rust_proto_thunk__google_api_HttpRule_set_body,
    );
  fn body_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          HttpRule::__BODY_VTABLE,
        )
      )
    }
  }

  // response_body: optional string
  pub fn response_body(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_HttpRule_get_response_body(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_response_body(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.response_body_mut().set(val);
  }
  const __RESPONSE_BODY_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__google_api_HttpRule_get_response_body,
      __rust_proto_thunk__google_api_HttpRule_set_response_body,
    );
  fn response_body_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          HttpRule::__RESPONSE_BODY_VTABLE,
        )
      )
    }
  }

  // additional_bindings: repeated message google.api.HttpRule
  pub fn additional_bindings(&self) -> ::__pb::RepeatedView<'_, crate::HttpRule> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__google_api_HttpRule_get_additional_bindings(self.raw_msg()),
      )
    }
  }
  pub fn additional_bindings_mut(&mut self) -> ::__pb::RepeatedMut<'_, crate::HttpRule> {
    unsafe {
      ::__pb::RepeatedMut::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__runtime::InnerRepeatedMut::new(
          ::__pb::__internal::Private,
          __rust_proto_thunk__google_api_HttpRule_get_mut_additional_bindings(self.raw_msg()),
        ),
      )
    }
  }

  pub fn pattern(&self) -> crate::http_rule::Pattern<'_> {
    match &self.pattern_case() {
      crate::http_rule::PatternCase::Get =>
          crate::http_rule::Pattern::Get(self.get()),
      crate::http_rule::PatternCase::Put =>
          crate::http_rule::Pattern::Put(self.put()),
      crate::http_rule::PatternCase::Post =>
          crate::http_rule::Pattern::Post(self.post()),
      crate::http_rule::PatternCase::Delete =>
          crate::http_rule::Pattern::Delete(self.delete()),
      crate::http_rule::PatternCase::Patch =>
          crate::http_rule::Pattern::Patch(self.patch()),
      crate::http_rule::PatternCase::Custom =>
          crate::http_rule::Pattern::Custom(self.custom()),
      _ => crate::http_rule::Pattern::not_set(std::marker::PhantomData)
    }
  }

  pub fn pattern_case(&self) -> crate::http_rule::PatternCase {
    unsafe { __rust_proto_thunk__google_api_HttpRule_case_pattern(self.raw_msg()) }
  }
}  // impl HttpRule

impl ::__std::ops::Drop for HttpRule {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__google_api_HttpRule_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__google_api_HttpRule_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_api_HttpRule_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_api_HttpRule_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__google_api_HttpRule_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__google_api_HttpRule_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_api_HttpRule_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__google_api_HttpRule_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_api_HttpRule_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_api_HttpRule_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_api_HttpRule_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__google_api_HttpRule_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__google_api_HttpRule_get_selector(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__google_api_HttpRule_set_selector(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__google_api_HttpRule_has_get(raw_msg: ::__pb::__runtime::RawMessage) -> bool;
  fn __rust_proto_thunk__google_api_HttpRule_clear_get(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_api_HttpRule_get_get(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__google_api_HttpRule_set_get(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__google_api_HttpRule_has_put(raw_msg: ::__pb::__runtime::RawMessage) -> bool;
  fn __rust_proto_thunk__google_api_HttpRule_clear_put(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_api_HttpRule_get_put(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__google_api_HttpRule_set_put(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__google_api_HttpRule_has_post(raw_msg: ::__pb::__runtime::RawMessage) -> bool;
  fn __rust_proto_thunk__google_api_HttpRule_clear_post(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_api_HttpRule_get_post(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__google_api_HttpRule_set_post(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__google_api_HttpRule_has_delete(raw_msg: ::__pb::__runtime::RawMessage) -> bool;
  fn __rust_proto_thunk__google_api_HttpRule_clear_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_api_HttpRule_get_delete(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__google_api_HttpRule_set_delete(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__google_api_HttpRule_has_patch(raw_msg: ::__pb::__runtime::RawMessage) -> bool;
  fn __rust_proto_thunk__google_api_HttpRule_clear_patch(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_api_HttpRule_get_patch(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__google_api_HttpRule_set_patch(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__google_api_HttpRule_get_custom(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_api_HttpRule_get_mut_custom(raw_msg: ::__pb::__runtime::RawMessage)
     -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_api_HttpRule_clear_custom(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_api_HttpRule_has_custom(raw_msg: ::__pb::__runtime::RawMessage) -> bool;

  fn __rust_proto_thunk__google_api_HttpRule_get_body(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__google_api_HttpRule_set_body(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__google_api_HttpRule_get_response_body(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__google_api_HttpRule_set_response_body(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__google_api_HttpRule_clear_additional_bindings(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_api_HttpRule_get_mut_additional_bindings(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawRepeatedField;
  fn __rust_proto_thunk__google_api_HttpRule_get_additional_bindings(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawRepeatedField;


  fn __rust_proto_thunk__google_api_HttpRule_case_pattern(raw_msg: ::__pb::__runtime::RawMessage) -> crate::http_rule::PatternCase;
}  // extern "C" for HttpRule

pub mod http_rule {

  #[non_exhaustive]
  #[derive(Debug, Clone, Copy)]
  #[allow(dead_code)]
  #[repr(isize)]
  pub enum Pattern<'msg> {
    Get(&'msg ::__pb::ProtoStr) = 2,
    Put(&'msg ::__pb::ProtoStr) = 3,
    Post(&'msg ::__pb::ProtoStr) = 4,
    Delete(&'msg ::__pb::ProtoStr) = 5,
    Patch(&'msg ::__pb::ProtoStr) = 6,
    Custom(::__pb::View<'msg, crate::CustomHttpPattern>) = 8,

    #[allow(non_camel_case_types)]
    not_set(std::marker::PhantomData<&'msg ()>) = 0
  }
  #[repr(C)]
  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  #[allow(dead_code)]
  pub enum PatternCase {
    Get = 2,
    Put = 3,
    Post = 4,
    Delete = 5,
    Patch = 6,
    Custom = 8,

    #[allow(non_camel_case_types)]
    not_set = 0
  }
}  // mod http_rule

impl HttpRule {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> HttpRuleMut<'a> {
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

impl<'a> HttpRuleView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct CustomHttpPattern {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for CustomHttpPattern {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for CustomHttpPattern {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `CustomHttpPattern` is `Sync` because it does not implement interior mutability.
//    Neither does `CustomHttpPatternMut`.
unsafe impl Sync for CustomHttpPattern {}

// SAFETY:
// - `CustomHttpPattern` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for CustomHttpPattern {}

impl ::__pb::Proxied for CustomHttpPattern {
  type View<'msg> = CustomHttpPatternView<'msg>;
  type Mut<'msg> = CustomHttpPatternMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CustomHttpPatternView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for CustomHttpPatternView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> CustomHttpPatternView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__google_api_CustomHttpPattern_serialize(self.raw_msg()) }
  }

  // kind: optional string
  pub fn kind(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_CustomHttpPattern_get_kind(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

  // path: optional string
  pub fn path(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_CustomHttpPattern_get_path(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }

}

// SAFETY:
// - `CustomHttpPatternView` is `Sync` because it does not support mutation.
unsafe impl Sync for CustomHttpPatternView<'_> {}

// SAFETY:
// - `CustomHttpPatternView` is `Send` because while its alive a `CustomHttpPatternMut` cannot.
// - `CustomHttpPatternView` does not use thread-local data.
unsafe impl Send for CustomHttpPatternView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for CustomHttpPatternView<'msg> {
  type Proxied = CustomHttpPattern;

  fn as_view(&self) -> ::__pb::View<'msg, CustomHttpPattern> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, CustomHttpPattern> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for CustomHttpPattern {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    CustomHttpPatternView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    CustomHttpPatternMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for CustomHttpPattern {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for CustomHttpPattern {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, CustomHttpPattern>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, CustomHttpPattern>;

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

impl<'msg> ::__pb::SettableValue<CustomHttpPattern> for CustomHttpPatternView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, CustomHttpPattern>)
    where CustomHttpPattern: 'dst {
    unsafe { __rust_proto_thunk__google_api_CustomHttpPattern_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<CustomHttpPattern> for CustomHttpPattern {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, CustomHttpPattern>)
    where CustomHttpPattern: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for CustomHttpPattern {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__google_api_CustomHttpPattern_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__google_api_CustomHttpPattern_copy_from(
        __rust_proto_thunk__google_api_CustomHttpPattern_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__google_api_CustomHttpPattern_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__google_api_CustomHttpPattern_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__google_api_CustomHttpPattern_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__google_api_CustomHttpPattern_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__google_api_CustomHttpPattern_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for CustomHttpPattern {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CustomHttpPatternView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CustomHttpPatternView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for CustomHttpPattern {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CustomHttpPatternView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CustomHttpPatternView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for CustomHttpPattern {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CustomHttpPatternView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CustomHttpPatternView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for CustomHttpPattern {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CustomHttpPatternView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CustomHttpPatternView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for CustomHttpPattern {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CustomHttpPatternView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| CustomHttpPatternView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for CustomHttpPattern {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(CustomHttpPatternView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| CustomHttpPatternView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CustomHttpPatternMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for CustomHttpPatternMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> CustomHttpPatternMut<'msg> {
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


  // kind: optional string
  pub fn kind(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_CustomHttpPattern_get_kind(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_kind(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.kind_mut().set(val);
  }
  fn kind_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CustomHttpPattern::__KIND_VTABLE,
        )
      )
    }
  }

  // path: optional string
  pub fn path(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_CustomHttpPattern_get_path(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_path(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.path_mut().set(val);
  }
  fn path_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CustomHttpPattern::__PATH_VTABLE,
        )
      )
    }
  }

}

// SAFETY:
// - `CustomHttpPatternMut` does not perform any shared mutation.
// - `CustomHttpPatternMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for CustomHttpPatternMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for CustomHttpPatternMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, CustomHttpPattern> {
    CustomHttpPatternMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, CustomHttpPattern> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for CustomHttpPatternMut<'msg> {
  type Proxied = CustomHttpPattern;
  fn as_view(&self) -> ::__pb::View<'_, CustomHttpPattern> {
    CustomHttpPatternView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, CustomHttpPattern> where 'msg: 'shorter {
    CustomHttpPatternView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl CustomHttpPattern {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__google_api_CustomHttpPattern_new() } } }
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

      __rust_proto_thunk__google_api_CustomHttpPattern_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> CustomHttpPatternView {
    CustomHttpPatternView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> CustomHttpPatternMut {
    CustomHttpPatternMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // kind: optional string
  pub fn kind(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_CustomHttpPattern_get_kind(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_kind(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.kind_mut().set(val);
  }
  const __KIND_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__google_api_CustomHttpPattern_get_kind,
      __rust_proto_thunk__google_api_CustomHttpPattern_set_kind,
    );
  fn kind_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CustomHttpPattern::__KIND_VTABLE,
        )
      )
    }
  }

  // path: optional string
  pub fn path(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_api_CustomHttpPattern_get_path(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn set_path(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.path_mut().set(val);
  }
  const __PATH_VTABLE: &'static ::__pb::__internal::BytesMutVTable =
    &::__pb::__internal::BytesMutVTable::new(
      ::__pb::__internal::Private,
      __rust_proto_thunk__google_api_CustomHttpPattern_get_path,
      __rust_proto_thunk__google_api_CustomHttpPattern_set_path,
    );
  fn path_mut(&mut self) -> ::__pb::Mut<'_, ::__pb::ProtoStr> {
    unsafe {
      <::__pb::Mut<::__pb::ProtoStr>>::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__internal::RawVTableMutator::new(
          ::__pb::__internal::Private,
          self.as_mutator_message_ref(),
          CustomHttpPattern::__PATH_VTABLE,
        )
      )
    }
  }

}  // impl CustomHttpPattern

impl ::__std::ops::Drop for CustomHttpPattern {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__google_api_CustomHttpPattern_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__google_api_CustomHttpPattern_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_api_CustomHttpPattern_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_api_CustomHttpPattern_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__google_api_CustomHttpPattern_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__google_api_CustomHttpPattern_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_api_CustomHttpPattern_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__google_api_CustomHttpPattern_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_api_CustomHttpPattern_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_api_CustomHttpPattern_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_api_CustomHttpPattern_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__google_api_CustomHttpPattern_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__google_api_CustomHttpPattern_get_kind(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__google_api_CustomHttpPattern_set_kind(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__google_api_CustomHttpPattern_get_path(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__google_api_CustomHttpPattern_set_path(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);


}  // extern "C" for CustomHttpPattern


impl CustomHttpPattern {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> CustomHttpPatternMut<'a> {
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

impl<'a> CustomHttpPatternView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

