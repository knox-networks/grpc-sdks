extern crate protobuf_cpp as __pb;
extern crate std as __std;
#[allow(non_camel_case_types)]
pub struct Struct {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for Struct {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for Struct {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `Struct` is `Sync` because it does not implement interior mutability.
//    Neither does `StructMut`.
unsafe impl Sync for Struct {}

// SAFETY:
// - `Struct` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Struct {}

impl ::__pb::Proxied for Struct {
  type View<'msg> = StructView<'msg>;
  type Mut<'msg> = StructMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StructView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for StructView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> StructView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__google_protobuf_Struct_serialize(self.raw_msg()) }
  }

  // fields: repeated message google.protobuf.Struct.FieldsEntry
  pub fn fields(self)
    -> ::__pb::MapView<'msg, ::__pb::ProtoStr, crate::Value> {
    unsafe {
      ::__pb::MapView::from_raw(::__pb::__internal::Private,
        __rust_proto_thunk__google_protobuf_Struct_get_fields(self.raw_msg()))
    }
  }

}

// SAFETY:
// - `StructView` is `Sync` because it does not support mutation.
unsafe impl Sync for StructView<'_> {}

// SAFETY:
// - `StructView` is `Send` because while its alive a `StructMut` cannot.
// - `StructView` does not use thread-local data.
unsafe impl Send for StructView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for StructView<'msg> {
  type Proxied = Struct;

  fn as_view(&self) -> ::__pb::View<'msg, Struct> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, Struct> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for Struct {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    StructView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    StructMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for Struct {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for Struct {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, Struct>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, Struct>;

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

impl<'msg> ::__pb::SettableValue<Struct> for StructView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, Struct>)
    where Struct: 'dst {
    unsafe { __rust_proto_thunk__google_protobuf_Struct_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<Struct> for Struct {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, Struct>)
    where Struct: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for Struct {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__google_protobuf_Struct_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__google_protobuf_Struct_copy_from(
        __rust_proto_thunk__google_protobuf_Struct_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__google_protobuf_Struct_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__google_protobuf_Struct_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__google_protobuf_Struct_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__google_protobuf_Struct_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__google_protobuf_Struct_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_google_protobuf_Struct_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_google_protobuf_Struct_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_google_protobuf_Struct_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_google_protobuf_Struct_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_google_protobuf_Struct_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_google_protobuf_Struct_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_google_protobuf_Struct_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_google_protobuf_Struct_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_google_protobuf_Struct_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for Struct {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_google_protobuf_Struct_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_google_protobuf_Struct_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_google_protobuf_Struct_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_google_protobuf_Struct_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_google_protobuf_Struct_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_google_protobuf_Struct_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(StructView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_google_protobuf_Struct_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_google_protobuf_Struct_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_google_protobuf_Struct_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| StructView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_google_protobuf_Struct_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_google_protobuf_Struct_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_google_protobuf_Struct_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_google_protobuf_Struct_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_google_protobuf_Struct_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_google_protobuf_Struct_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_google_protobuf_Struct_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_google_protobuf_Struct_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_google_protobuf_Struct_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for Struct {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_google_protobuf_Struct_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_google_protobuf_Struct_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_google_protobuf_Struct_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_google_protobuf_Struct_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_google_protobuf_Struct_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_google_protobuf_Struct_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(StructView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_google_protobuf_Struct_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_google_protobuf_Struct_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_google_protobuf_Struct_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| StructView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_google_protobuf_Struct_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_google_protobuf_Struct_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_google_protobuf_Struct_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_google_protobuf_Struct_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_google_protobuf_Struct_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_google_protobuf_Struct_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_google_protobuf_Struct_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_google_protobuf_Struct_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_google_protobuf_Struct_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for Struct {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_google_protobuf_Struct_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_google_protobuf_Struct_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_google_protobuf_Struct_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_google_protobuf_Struct_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_google_protobuf_Struct_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_google_protobuf_Struct_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(StructView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_google_protobuf_Struct_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_google_protobuf_Struct_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_google_protobuf_Struct_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| StructView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_google_protobuf_Struct_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_google_protobuf_Struct_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_google_protobuf_Struct_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_google_protobuf_Struct_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_google_protobuf_Struct_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_google_protobuf_Struct_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_google_protobuf_Struct_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_google_protobuf_Struct_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_google_protobuf_Struct_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for Struct {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_google_protobuf_Struct_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_google_protobuf_Struct_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_google_protobuf_Struct_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_google_protobuf_Struct_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_google_protobuf_Struct_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_google_protobuf_Struct_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(StructView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_google_protobuf_Struct_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_google_protobuf_Struct_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_google_protobuf_Struct_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| StructView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_google_protobuf_Struct_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_google_protobuf_Struct_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_google_protobuf_Struct_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_google_protobuf_Struct_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_google_protobuf_Struct_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_google_protobuf_Struct_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_google_protobuf_Struct_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_google_protobuf_Struct_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_google_protobuf_Struct_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for Struct {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_google_protobuf_Struct_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_google_protobuf_Struct_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_google_protobuf_Struct_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_google_protobuf_Struct_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_google_protobuf_Struct_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_google_protobuf_Struct_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(StructView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_google_protobuf_Struct_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_google_protobuf_Struct_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_google_protobuf_Struct_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| StructView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for Struct {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(StructView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_google_protobuf_Struct_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| StructView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StructMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for StructMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> StructMut<'msg> {
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


  // fields: repeated message google.protobuf.Struct.FieldsEntry
  pub fn fields(&self)
    -> ::__pb::MapView<'_, ::__pb::ProtoStr, crate::Value> {
    unsafe {
      ::__pb::MapView::from_raw(::__pb::__internal::Private,
        __rust_proto_thunk__google_protobuf_Struct_get_fields(self.raw_msg()))
    }
  }
  pub fn fields_mut(&mut self)
    -> ::__pb::MapMut<'_, ::__pb::ProtoStr, crate::Value> {
    let inner = ::__pb::__runtime::InnerMapMut::new(::__pb::__internal::Private,
      unsafe { __rust_proto_thunk__google_protobuf_Struct_get_mut_fields(self.raw_msg()) });
    unsafe { ::__pb::MapMut::from_inner(::__pb::__internal::Private, inner) }
  }

}

// SAFETY:
// - `StructMut` does not perform any shared mutation.
// - `StructMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for StructMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for StructMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, Struct> {
    StructMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, Struct> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for StructMut<'msg> {
  type Proxied = Struct;
  fn as_view(&self) -> ::__pb::View<'_, Struct> {
    StructView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, Struct> where 'msg: 'shorter {
    StructView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl Struct {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__google_protobuf_Struct_new() } } }
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

      __rust_proto_thunk__google_protobuf_Struct_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> StructView {
    StructView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> StructMut {
    StructMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // fields: repeated message google.protobuf.Struct.FieldsEntry
  pub fn fields(&self)
    -> ::__pb::MapView<'_, ::__pb::ProtoStr, crate::Value> {
    unsafe {
      ::__pb::MapView::from_raw(::__pb::__internal::Private,
        __rust_proto_thunk__google_protobuf_Struct_get_fields(self.raw_msg()))
    }
  }
  pub fn fields_mut(&mut self)
    -> ::__pb::MapMut<'_, ::__pb::ProtoStr, crate::Value> {
    let inner = ::__pb::__runtime::InnerMapMut::new(::__pb::__internal::Private,
      unsafe { __rust_proto_thunk__google_protobuf_Struct_get_mut_fields(self.raw_msg()) });
    unsafe { ::__pb::MapMut::from_inner(::__pb::__internal::Private, inner) }
  }

}  // impl Struct

impl ::__std::ops::Drop for Struct {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__google_protobuf_Struct_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__google_protobuf_Struct_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_protobuf_Struct_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_protobuf_Struct_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__google_protobuf_Struct_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__google_protobuf_Struct_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_protobuf_Struct_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__google_protobuf_Struct_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_protobuf_Struct_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_protobuf_Struct_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_protobuf_Struct_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__google_protobuf_Struct_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__google_protobuf_Struct_get_fields(msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__google_protobuf_Struct_get_mut_fields(msg: ::__pb::__runtime::RawMessage,) -> ::__pb::__runtime::RawMap;


}  // extern "C" for Struct

pub mod r#struct {

}  // mod r#struct

impl Struct {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> StructMut<'a> {
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

impl<'a> StructView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct Value {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for Value {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `Value` is `Sync` because it does not implement interior mutability.
//    Neither does `ValueMut`.
unsafe impl Sync for Value {}

// SAFETY:
// - `Value` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Value {}

impl ::__pb::Proxied for Value {
  type View<'msg> = ValueView<'msg>;
  type Mut<'msg> = ValueMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ValueView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for ValueView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> ValueView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__google_protobuf_Value_serialize(self.raw_msg()) }
  }

  // null_value: optional enum google.protobuf.NullValue
  pub fn null_value(self) -> crate::NullValue {
    unsafe { __rust_proto_thunk__google_protobuf_Value_get_null_value(self.raw_msg()) }
  }
  pub fn null_value_opt(self) -> ::__pb::Optional<crate::NullValue> {
    if self.has_null_value() {
      ::__pb::Optional::Set(self.null_value())
    } else {
      ::__pb::Optional::Unset(crate::NullValue::NullValue)
    }
  }
  pub fn has_null_value(self) -> bool {
    unsafe { __rust_proto_thunk__google_protobuf_Value_has_null_value(self.raw_msg()) }
  }

  // number_value: optional double
  pub fn number_value(self) -> f64 {
    unsafe { __rust_proto_thunk__google_protobuf_Value_get_number_value(self.raw_msg()) }
  }
  pub fn number_value_opt(self) -> ::__pb::Optional<f64> {
    if self.has_number_value() {
      ::__pb::Optional::Set(self.number_value())
    } else {
      ::__pb::Optional::Unset(0f64)
    }
  }
  pub fn has_number_value(self) -> bool {
    unsafe { __rust_proto_thunk__google_protobuf_Value_has_number_value(self.raw_msg()) }
  }

  // string_value: optional string
  pub fn string_value(self) -> &'msg ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_protobuf_Value_get_string_value(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn string_value_opt(self) -> ::__pb::Optional<&'msg ::__pb::ProtoStr> {
      ::__pb::Optional::new(
        self.string_value(),
        self.has_string_value()
      )
    }
  pub fn has_string_value(self) -> bool {
    unsafe { __rust_proto_thunk__google_protobuf_Value_has_string_value(self.raw_msg()) }
  }

  // bool_value: optional bool
  pub fn bool_value(self) -> bool {
    unsafe { __rust_proto_thunk__google_protobuf_Value_get_bool_value(self.raw_msg()) }
  }
  pub fn bool_value_opt(self) -> ::__pb::Optional<bool> {
    if self.has_bool_value() {
      ::__pb::Optional::Set(self.bool_value())
    } else {
      ::__pb::Optional::Unset(false)
    }
  }
  pub fn has_bool_value(self) -> bool {
    unsafe { __rust_proto_thunk__google_protobuf_Value_has_bool_value(self.raw_msg()) }
  }

  // struct_value: optional message google.protobuf.Struct
  pub fn struct_value(self) -> crate::StructView<'msg> {
                  let submsg = unsafe { __rust_proto_thunk__google_protobuf_Value_get_struct_value(self.raw_msg()) };
    crate::StructView::new(::__pb::__internal::Private, submsg)
  }
  pub fn struct_value_opt(self) ->
  ::__pb::Optional<crate::StructView<'msg>> {
    let view = self.struct_value();
    ::__pb::Optional::new(view, self.has_struct_value())
  }
  pub fn has_struct_value(self) -> bool {
    unsafe { __rust_proto_thunk__google_protobuf_Value_has_struct_value(self.raw_msg()) }
  }

  // list_value: optional message google.protobuf.ListValue
  pub fn list_value(self) -> crate::ListValueView<'msg> {
                  let submsg = unsafe { __rust_proto_thunk__google_protobuf_Value_get_list_value(self.raw_msg()) };
    crate::ListValueView::new(::__pb::__internal::Private, submsg)
  }
  pub fn list_value_opt(self) ->
  ::__pb::Optional<crate::ListValueView<'msg>> {
    let view = self.list_value();
    ::__pb::Optional::new(view, self.has_list_value())
  }
  pub fn has_list_value(self) -> bool {
    unsafe { __rust_proto_thunk__google_protobuf_Value_has_list_value(self.raw_msg()) }
  }

  pub fn kind(self) -> crate::value::Kind<'msg> {
    match self.kind_case() {
      crate::value::KindCase::NullValue =>
          crate::value::Kind::NullValue(self.null_value()),
      crate::value::KindCase::NumberValue =>
          crate::value::Kind::NumberValue(self.number_value()),
      crate::value::KindCase::StringValue =>
          crate::value::Kind::StringValue(self.string_value()),
      crate::value::KindCase::BoolValue =>
          crate::value::Kind::BoolValue(self.bool_value()),
      crate::value::KindCase::StructValue =>
          crate::value::Kind::StructValue(self.struct_value()),
      crate::value::KindCase::ListValue =>
          crate::value::Kind::ListValue(self.list_value()),
      _ => crate::value::Kind::not_set(std::marker::PhantomData)
    }
  }

  pub fn kind_case(self) -> crate::value::KindCase {
    unsafe { __rust_proto_thunk__google_protobuf_Value_case_kind(self.raw_msg()) }
  }
}

// SAFETY:
// - `ValueView` is `Sync` because it does not support mutation.
unsafe impl Sync for ValueView<'_> {}

// SAFETY:
// - `ValueView` is `Send` because while its alive a `ValueMut` cannot.
// - `ValueView` does not use thread-local data.
unsafe impl Send for ValueView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for ValueView<'msg> {
  type Proxied = Value;

  fn as_view(&self) -> ::__pb::View<'msg, Value> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, Value> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for Value {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    ValueView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    ValueMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for Value {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for Value {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, Value>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, Value>;

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

impl<'msg> ::__pb::SettableValue<Value> for ValueView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, Value>)
    where Value: 'dst {
    unsafe { __rust_proto_thunk__google_protobuf_Value_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<Value> for Value {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, Value>)
    where Value: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for Value {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__google_protobuf_Value_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__google_protobuf_Value_copy_from(
        __rust_proto_thunk__google_protobuf_Value_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__google_protobuf_Value_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__google_protobuf_Value_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__google_protobuf_Value_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__google_protobuf_Value_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__google_protobuf_Value_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_google_protobuf_Value_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_google_protobuf_Value_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_google_protobuf_Value_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_google_protobuf_Value_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_google_protobuf_Value_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_google_protobuf_Value_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_google_protobuf_Value_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_google_protobuf_Value_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_google_protobuf_Value_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for Value {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_google_protobuf_Value_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_google_protobuf_Value_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_google_protobuf_Value_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_google_protobuf_Value_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_google_protobuf_Value_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_google_protobuf_Value_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ValueView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_google_protobuf_Value_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_google_protobuf_Value_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_google_protobuf_Value_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ValueView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_google_protobuf_Value_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_google_protobuf_Value_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_google_protobuf_Value_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_google_protobuf_Value_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_google_protobuf_Value_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_google_protobuf_Value_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_google_protobuf_Value_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_google_protobuf_Value_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_google_protobuf_Value_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for Value {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_google_protobuf_Value_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_google_protobuf_Value_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_google_protobuf_Value_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_google_protobuf_Value_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_google_protobuf_Value_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_google_protobuf_Value_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ValueView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_google_protobuf_Value_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_google_protobuf_Value_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_google_protobuf_Value_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ValueView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_google_protobuf_Value_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_google_protobuf_Value_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_google_protobuf_Value_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_google_protobuf_Value_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_google_protobuf_Value_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_google_protobuf_Value_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_google_protobuf_Value_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_google_protobuf_Value_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_google_protobuf_Value_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for Value {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_google_protobuf_Value_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_google_protobuf_Value_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_google_protobuf_Value_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_google_protobuf_Value_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_google_protobuf_Value_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_google_protobuf_Value_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ValueView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_google_protobuf_Value_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_google_protobuf_Value_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_google_protobuf_Value_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ValueView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_google_protobuf_Value_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_google_protobuf_Value_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_google_protobuf_Value_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_google_protobuf_Value_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_google_protobuf_Value_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_google_protobuf_Value_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_google_protobuf_Value_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_google_protobuf_Value_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_google_protobuf_Value_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for Value {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_google_protobuf_Value_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_google_protobuf_Value_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_google_protobuf_Value_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_google_protobuf_Value_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_google_protobuf_Value_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_google_protobuf_Value_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ValueView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_google_protobuf_Value_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_google_protobuf_Value_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_google_protobuf_Value_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ValueView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_google_protobuf_Value_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_google_protobuf_Value_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_google_protobuf_Value_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_google_protobuf_Value_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_google_protobuf_Value_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_google_protobuf_Value_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_google_protobuf_Value_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_google_protobuf_Value_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_google_protobuf_Value_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for Value {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_google_protobuf_Value_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_google_protobuf_Value_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_google_protobuf_Value_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_google_protobuf_Value_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_google_protobuf_Value_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_google_protobuf_Value_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ValueView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_google_protobuf_Value_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_google_protobuf_Value_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_google_protobuf_Value_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ValueView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for Value {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ValueView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_google_protobuf_Value_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| ValueView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ValueMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for ValueMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> ValueMut<'msg> {
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


  // null_value: optional enum google.protobuf.NullValue
  pub fn null_value(&self) -> crate::NullValue {
    unsafe { __rust_proto_thunk__google_protobuf_Value_get_null_value(self.raw_msg()) }
  }
  pub fn null_value_opt(&self) -> ::__pb::Optional<crate::NullValue> {
    if self.has_null_value() {
      ::__pb::Optional::Set(self.null_value())
    } else {
      ::__pb::Optional::Unset(crate::NullValue::NullValue)
    }
  }
  pub fn set_null_value(&mut self, val: crate::NullValue) {
    unsafe { __rust_proto_thunk__google_protobuf_Value_set_null_value(self.raw_msg(), val) }
  }
  pub fn has_null_value(&self) -> bool {
    unsafe { __rust_proto_thunk__google_protobuf_Value_has_null_value(self.raw_msg()) }
  }
  pub fn clear_null_value(&mut self) {
    unsafe { __rust_proto_thunk__google_protobuf_Value_clear_null_value(self.raw_msg()) }
  }

  // number_value: optional double
  pub fn number_value(&self) -> f64 {
    unsafe { __rust_proto_thunk__google_protobuf_Value_get_number_value(self.raw_msg()) }
  }
  pub fn number_value_opt(&self) -> ::__pb::Optional<f64> {
    if self.has_number_value() {
      ::__pb::Optional::Set(self.number_value())
    } else {
      ::__pb::Optional::Unset(0f64)
    }
  }
  pub fn set_number_value(&mut self, val: f64) {
    unsafe { __rust_proto_thunk__google_protobuf_Value_set_number_value(self.raw_msg(), val) }
  }
  pub fn has_number_value(&self) -> bool {
    unsafe { __rust_proto_thunk__google_protobuf_Value_has_number_value(self.raw_msg()) }
  }
  pub fn clear_number_value(&mut self) {
    unsafe { __rust_proto_thunk__google_protobuf_Value_clear_number_value(self.raw_msg()) }
  }

  // string_value: optional string
  pub fn string_value(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_protobuf_Value_get_string_value(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn string_value_opt(&self) -> ::__pb::Optional<&'_ ::__pb::ProtoStr> {
      ::__pb::Optional::new(
        self.string_value(),
        self.has_string_value()
      )
    }
  pub fn set_string_value(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.string_value_mut().set(val);
  }
  pub fn has_string_value(&self) -> bool {
    unsafe { __rust_proto_thunk__google_protobuf_Value_has_string_value(self.raw_msg()) }
  }
  pub fn clear_string_value(&mut self) {
    unsafe { __rust_proto_thunk__google_protobuf_Value_clear_string_value(self.raw_msg()) }
  }
  fn string_value_mut(&mut self) -> ::__pb::FieldEntry<'_, ::__pb::ProtoStr> {
    let out = unsafe {
      let has = __rust_proto_thunk__google_protobuf_Value_has_string_value(self.raw_msg());
      ::__pb::__internal::new_vtable_field_entry(
        ::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        Value::__STRING_VALUE_VTABLE,
        has,
      )
    };
    ::__pb::ProtoStrMut::field_entry_from_bytes(
      ::__pb::__internal::Private, out
    )
  }

  // bool_value: optional bool
  pub fn bool_value(&self) -> bool {
    unsafe { __rust_proto_thunk__google_protobuf_Value_get_bool_value(self.raw_msg()) }
  }
  pub fn bool_value_opt(&self) -> ::__pb::Optional<bool> {
    if self.has_bool_value() {
      ::__pb::Optional::Set(self.bool_value())
    } else {
      ::__pb::Optional::Unset(false)
    }
  }
  pub fn set_bool_value(&mut self, val: bool) {
    unsafe { __rust_proto_thunk__google_protobuf_Value_set_bool_value(self.raw_msg(), val) }
  }
  pub fn has_bool_value(&self) -> bool {
    unsafe { __rust_proto_thunk__google_protobuf_Value_has_bool_value(self.raw_msg()) }
  }
  pub fn clear_bool_value(&mut self) {
    unsafe { __rust_proto_thunk__google_protobuf_Value_clear_bool_value(self.raw_msg()) }
  }

  // struct_value: optional message google.protobuf.Struct
  pub fn struct_value(&self) -> crate::StructView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__google_protobuf_Value_get_struct_value(self.raw_msg()) };
    crate::StructView::new(::__pb::__internal::Private, submsg)
  }
  pub fn struct_value_mut(&mut self) -> crate::StructMut<'_> {
    self.struct_value_entry().or_default()
  }
  fn struct_value_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::Struct> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__google_protobuf_Value_get_struct_value,
                                __rust_proto_thunk__google_protobuf_Value_get_mut_struct_value,
                                __rust_proto_thunk__google_protobuf_Value_clear_struct_value);
    unsafe {
      let has = self.has_struct_value();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn struct_value_opt(&self) ->
  ::__pb::Optional<crate::StructView<'_>> {
    let view = self.struct_value();
    ::__pb::Optional::new(view, self.has_struct_value())
  }
  pub fn set_struct_value(&mut self, val: impl ::__pb::SettableValue<crate::Struct>) {
    self.struct_value_entry().set(val);
  }
  pub fn has_struct_value(&self) -> bool {
    unsafe { __rust_proto_thunk__google_protobuf_Value_has_struct_value(self.raw_msg()) }
  }
  pub fn clear_struct_value(&mut self) {
    unsafe { __rust_proto_thunk__google_protobuf_Value_clear_struct_value(self.raw_msg()) }
  }

  // list_value: optional message google.protobuf.ListValue
  pub fn list_value(&self) -> crate::ListValueView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__google_protobuf_Value_get_list_value(self.raw_msg()) };
    crate::ListValueView::new(::__pb::__internal::Private, submsg)
  }
  pub fn list_value_mut(&mut self) -> crate::ListValueMut<'_> {
    self.list_value_entry().or_default()
  }
  fn list_value_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::ListValue> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__google_protobuf_Value_get_list_value,
                                __rust_proto_thunk__google_protobuf_Value_get_mut_list_value,
                                __rust_proto_thunk__google_protobuf_Value_clear_list_value);
    unsafe {
      let has = self.has_list_value();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn list_value_opt(&self) ->
  ::__pb::Optional<crate::ListValueView<'_>> {
    let view = self.list_value();
    ::__pb::Optional::new(view, self.has_list_value())
  }
  pub fn set_list_value(&mut self, val: impl ::__pb::SettableValue<crate::ListValue>) {
    self.list_value_entry().set(val);
  }
  pub fn has_list_value(&self) -> bool {
    unsafe { __rust_proto_thunk__google_protobuf_Value_has_list_value(self.raw_msg()) }
  }
  pub fn clear_list_value(&mut self) {
    unsafe { __rust_proto_thunk__google_protobuf_Value_clear_list_value(self.raw_msg()) }
  }

  pub fn kind(&self) -> crate::value::Kind<'_> {
    match &self.kind_case() {
      crate::value::KindCase::NullValue =>
          crate::value::Kind::NullValue(self.null_value()),
      crate::value::KindCase::NumberValue =>
          crate::value::Kind::NumberValue(self.number_value()),
      crate::value::KindCase::StringValue =>
          crate::value::Kind::StringValue(self.string_value()),
      crate::value::KindCase::BoolValue =>
          crate::value::Kind::BoolValue(self.bool_value()),
      crate::value::KindCase::StructValue =>
          crate::value::Kind::StructValue(self.struct_value()),
      crate::value::KindCase::ListValue =>
          crate::value::Kind::ListValue(self.list_value()),
      _ => crate::value::Kind::not_set(std::marker::PhantomData)
    }
  }

  pub fn kind_case(&self) -> crate::value::KindCase {
    unsafe { __rust_proto_thunk__google_protobuf_Value_case_kind(self.raw_msg()) }
  }
}

// SAFETY:
// - `ValueMut` does not perform any shared mutation.
// - `ValueMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for ValueMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for ValueMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, Value> {
    ValueMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, Value> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for ValueMut<'msg> {
  type Proxied = Value;
  fn as_view(&self) -> ::__pb::View<'_, Value> {
    ValueView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, Value> where 'msg: 'shorter {
    ValueView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl Value {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__google_protobuf_Value_new() } } }
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

      __rust_proto_thunk__google_protobuf_Value_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> ValueView {
    ValueView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> ValueMut {
    ValueMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // null_value: optional enum google.protobuf.NullValue
  pub fn null_value(&self) -> crate::NullValue {
    unsafe { __rust_proto_thunk__google_protobuf_Value_get_null_value(self.raw_msg()) }
  }
  pub fn null_value_opt(&self) -> ::__pb::Optional<crate::NullValue> {
    if self.has_null_value() {
      ::__pb::Optional::Set(self.null_value())
    } else {
      ::__pb::Optional::Unset(crate::NullValue::NullValue)
    }
  }
  pub fn set_null_value(&mut self, val: crate::NullValue) {
    unsafe { __rust_proto_thunk__google_protobuf_Value_set_null_value(self.raw_msg(), val) }
  }
  pub fn has_null_value(&self) -> bool {
    unsafe { __rust_proto_thunk__google_protobuf_Value_has_null_value(self.raw_msg()) }
  }
  pub fn clear_null_value(&mut self) {
    unsafe { __rust_proto_thunk__google_protobuf_Value_clear_null_value(self.raw_msg()) }
  }

  // number_value: optional double
  pub fn number_value(&self) -> f64 {
    unsafe { __rust_proto_thunk__google_protobuf_Value_get_number_value(self.raw_msg()) }
  }
  pub fn number_value_opt(&self) -> ::__pb::Optional<f64> {
    if self.has_number_value() {
      ::__pb::Optional::Set(self.number_value())
    } else {
      ::__pb::Optional::Unset(0f64)
    }
  }
  pub fn set_number_value(&mut self, val: f64) {
    unsafe { __rust_proto_thunk__google_protobuf_Value_set_number_value(self.raw_msg(), val) }
  }
  pub fn has_number_value(&self) -> bool {
    unsafe { __rust_proto_thunk__google_protobuf_Value_has_number_value(self.raw_msg()) }
  }
  pub fn clear_number_value(&mut self) {
    unsafe { __rust_proto_thunk__google_protobuf_Value_clear_number_value(self.raw_msg()) }
  }

  // string_value: optional string
  pub fn string_value(&self) -> &'_ ::__pb::ProtoStr {
    let view = unsafe { __rust_proto_thunk__google_protobuf_Value_get_string_value(self.raw_msg()).as_ref() };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::__pb::ProtoStr::from_utf8_unchecked(view) }
  }
  pub fn string_value_opt(&self) -> ::__pb::Optional<&'_ ::__pb::ProtoStr> {
      ::__pb::Optional::new(
        self.string_value(),
        self.has_string_value()
      )
    }
  pub fn set_string_value(&mut self, val: impl ::__pb::SettableValue<::__pb::ProtoStr>) {
    self.string_value_mut().set(val);
  }
  pub fn has_string_value(&self) -> bool {
    unsafe { __rust_proto_thunk__google_protobuf_Value_has_string_value(self.raw_msg()) }
  }
  pub fn clear_string_value(&mut self) {
    unsafe { __rust_proto_thunk__google_protobuf_Value_clear_string_value(self.raw_msg()) }
  }
  // SAFETY: for `string` fields, the default value is verified as valid UTF-8
  const __STRING_VALUE_VTABLE: &'static ::__pb::__internal::BytesOptionalMutVTable = &unsafe {
      ::__pb::__internal::BytesOptionalMutVTable::new(
        ::__pb::__internal::Private,
        __rust_proto_thunk__google_protobuf_Value_get_string_value,
        __rust_proto_thunk__google_protobuf_Value_set_string_value,
        __rust_proto_thunk__google_protobuf_Value_clear_string_value,
        b"",
      )
    };
  fn string_value_mut(&mut self) -> ::__pb::FieldEntry<'_, ::__pb::ProtoStr> {
    let out = unsafe {
      let has = __rust_proto_thunk__google_protobuf_Value_has_string_value(self.raw_msg());
      ::__pb::__internal::new_vtable_field_entry(
        ::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        Value::__STRING_VALUE_VTABLE,
        has,
      )
    };
    ::__pb::ProtoStrMut::field_entry_from_bytes(
      ::__pb::__internal::Private, out
    )
  }

  // bool_value: optional bool
  pub fn bool_value(&self) -> bool {
    unsafe { __rust_proto_thunk__google_protobuf_Value_get_bool_value(self.raw_msg()) }
  }
  pub fn bool_value_opt(&self) -> ::__pb::Optional<bool> {
    if self.has_bool_value() {
      ::__pb::Optional::Set(self.bool_value())
    } else {
      ::__pb::Optional::Unset(false)
    }
  }
  pub fn set_bool_value(&mut self, val: bool) {
    unsafe { __rust_proto_thunk__google_protobuf_Value_set_bool_value(self.raw_msg(), val) }
  }
  pub fn has_bool_value(&self) -> bool {
    unsafe { __rust_proto_thunk__google_protobuf_Value_has_bool_value(self.raw_msg()) }
  }
  pub fn clear_bool_value(&mut self) {
    unsafe { __rust_proto_thunk__google_protobuf_Value_clear_bool_value(self.raw_msg()) }
  }

  // struct_value: optional message google.protobuf.Struct
  pub fn struct_value(&self) -> crate::StructView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__google_protobuf_Value_get_struct_value(self.raw_msg()) };
    crate::StructView::new(::__pb::__internal::Private, submsg)
  }
  pub fn struct_value_mut(&mut self) -> crate::StructMut<'_> {
    self.struct_value_entry().or_default()
  }
  fn struct_value_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::Struct> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__google_protobuf_Value_get_struct_value,
                                __rust_proto_thunk__google_protobuf_Value_get_mut_struct_value,
                                __rust_proto_thunk__google_protobuf_Value_clear_struct_value);
    unsafe {
      let has = self.has_struct_value();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn struct_value_opt(&self) ->
  ::__pb::Optional<crate::StructView<'_>> {
    let view = self.struct_value();
    ::__pb::Optional::new(view, self.has_struct_value())
  }
  pub fn set_struct_value(&mut self, val: impl ::__pb::SettableValue<crate::Struct>) {
    self.struct_value_entry().set(val);
  }
  pub fn has_struct_value(&self) -> bool {
    unsafe { __rust_proto_thunk__google_protobuf_Value_has_struct_value(self.raw_msg()) }
  }
  pub fn clear_struct_value(&mut self) {
    unsafe { __rust_proto_thunk__google_protobuf_Value_clear_struct_value(self.raw_msg()) }
  }

  // list_value: optional message google.protobuf.ListValue
  pub fn list_value(&self) -> crate::ListValueView<'_> {
                  let submsg = unsafe { __rust_proto_thunk__google_protobuf_Value_get_list_value(self.raw_msg()) };
    crate::ListValueView::new(::__pb::__internal::Private, submsg)
  }
  pub fn list_value_mut(&mut self) -> crate::ListValueMut<'_> {
    self.list_value_entry().or_default()
  }
  fn list_value_entry(&mut self)
      -> ::__pb::FieldEntry<'_, crate::ListValue> {
    static VTABLE: ::__pb::__runtime::MessageVTable =
      ::__pb::__runtime::MessageVTable::new(::__pb::__internal::Private,
                                __rust_proto_thunk__google_protobuf_Value_get_list_value,
                                __rust_proto_thunk__google_protobuf_Value_get_mut_list_value,
                                __rust_proto_thunk__google_protobuf_Value_clear_list_value);
    unsafe {
      let has = self.has_list_value();
      ::__pb::__internal::new_vtable_field_entry(::__pb::__internal::Private,
        self.as_mutator_message_ref(),
        &VTABLE,
        has)
    }
  }
  pub fn list_value_opt(&self) ->
  ::__pb::Optional<crate::ListValueView<'_>> {
    let view = self.list_value();
    ::__pb::Optional::new(view, self.has_list_value())
  }
  pub fn set_list_value(&mut self, val: impl ::__pb::SettableValue<crate::ListValue>) {
    self.list_value_entry().set(val);
  }
  pub fn has_list_value(&self) -> bool {
    unsafe { __rust_proto_thunk__google_protobuf_Value_has_list_value(self.raw_msg()) }
  }
  pub fn clear_list_value(&mut self) {
    unsafe { __rust_proto_thunk__google_protobuf_Value_clear_list_value(self.raw_msg()) }
  }

  pub fn kind(&self) -> crate::value::Kind<'_> {
    match &self.kind_case() {
      crate::value::KindCase::NullValue =>
          crate::value::Kind::NullValue(self.null_value()),
      crate::value::KindCase::NumberValue =>
          crate::value::Kind::NumberValue(self.number_value()),
      crate::value::KindCase::StringValue =>
          crate::value::Kind::StringValue(self.string_value()),
      crate::value::KindCase::BoolValue =>
          crate::value::Kind::BoolValue(self.bool_value()),
      crate::value::KindCase::StructValue =>
          crate::value::Kind::StructValue(self.struct_value()),
      crate::value::KindCase::ListValue =>
          crate::value::Kind::ListValue(self.list_value()),
      _ => crate::value::Kind::not_set(std::marker::PhantomData)
    }
  }

  pub fn kind_case(&self) -> crate::value::KindCase {
    unsafe { __rust_proto_thunk__google_protobuf_Value_case_kind(self.raw_msg()) }
  }
}  // impl Value

impl ::__std::ops::Drop for Value {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__google_protobuf_Value_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__google_protobuf_Value_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_protobuf_Value_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_protobuf_Value_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__google_protobuf_Value_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__google_protobuf_Value_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_protobuf_Value_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__google_protobuf_Value_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_protobuf_Value_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_protobuf_Value_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_protobuf_Value_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__google_protobuf_Value_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__google_protobuf_Value_has_null_value(raw_msg: ::__pb::__runtime::RawMessage) -> bool;
  fn __rust_proto_thunk__google_protobuf_Value_clear_null_value(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_protobuf_Value_get_null_value(raw_msg: ::__pb::__runtime::RawMessage) -> crate::NullValue;
  fn __rust_proto_thunk__google_protobuf_Value_set_null_value(raw_msg: ::__pb::__runtime::RawMessage, val: crate::NullValue);

  fn __rust_proto_thunk__google_protobuf_Value_has_number_value(raw_msg: ::__pb::__runtime::RawMessage) -> bool;
  fn __rust_proto_thunk__google_protobuf_Value_clear_number_value(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_protobuf_Value_get_number_value(raw_msg: ::__pb::__runtime::RawMessage) -> f64;
  fn __rust_proto_thunk__google_protobuf_Value_set_number_value(raw_msg: ::__pb::__runtime::RawMessage, val: f64);

  fn __rust_proto_thunk__google_protobuf_Value_has_string_value(raw_msg: ::__pb::__runtime::RawMessage) -> bool;
  fn __rust_proto_thunk__google_protobuf_Value_clear_string_value(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_protobuf_Value_get_string_value(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::PtrAndLen;
  fn __rust_proto_thunk__google_protobuf_Value_set_string_value(raw_msg: ::__pb::__runtime::RawMessage, val: ::__pb::__runtime::PtrAndLen);

  fn __rust_proto_thunk__google_protobuf_Value_has_bool_value(raw_msg: ::__pb::__runtime::RawMessage) -> bool;
  fn __rust_proto_thunk__google_protobuf_Value_clear_bool_value(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_protobuf_Value_get_bool_value(raw_msg: ::__pb::__runtime::RawMessage) -> bool;
  fn __rust_proto_thunk__google_protobuf_Value_set_bool_value(raw_msg: ::__pb::__runtime::RawMessage, val: bool);

  fn __rust_proto_thunk__google_protobuf_Value_get_struct_value(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_protobuf_Value_get_mut_struct_value(raw_msg: ::__pb::__runtime::RawMessage)
     -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_protobuf_Value_clear_struct_value(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_protobuf_Value_has_struct_value(raw_msg: ::__pb::__runtime::RawMessage) -> bool;

  fn __rust_proto_thunk__google_protobuf_Value_get_list_value(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_protobuf_Value_get_mut_list_value(raw_msg: ::__pb::__runtime::RawMessage)
     -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_protobuf_Value_clear_list_value(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_protobuf_Value_has_list_value(raw_msg: ::__pb::__runtime::RawMessage) -> bool;


  fn __rust_proto_thunk__google_protobuf_Value_case_kind(raw_msg: ::__pb::__runtime::RawMessage) -> crate::value::KindCase;
}  // extern "C" for Value

pub mod value {

  #[non_exhaustive]
  #[derive(Debug, Clone, Copy)]
  #[allow(dead_code)]
  #[repr(isize)]
  pub enum Kind<'msg> {
    NullValue(::__pb::View<'msg, crate::NullValue>) = 1,
    NumberValue(f64) = 2,
    StringValue(&'msg ::__pb::ProtoStr) = 3,
    BoolValue(bool) = 4,
    StructValue(::__pb::View<'msg, crate::Struct>) = 5,
    ListValue(::__pb::View<'msg, crate::ListValue>) = 6,

    #[allow(non_camel_case_types)]
    not_set(std::marker::PhantomData<&'msg ()>) = 0
  }
  #[repr(C)]
  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  #[allow(dead_code)]
  pub enum KindCase {
    NullValue = 1,
    NumberValue = 2,
    StringValue = 3,
    BoolValue = 4,
    StructValue = 5,
    ListValue = 6,

    #[allow(non_camel_case_types)]
    not_set = 0
  }
}  // mod value

impl Value {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> ValueMut<'a> {
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

impl<'a> ValueView<'a> {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(
    msg: &'a ::__pb::__runtime::RawMessage) -> Self {
    Self::new(::__pb::__internal::Private, *msg)
  }
  pub fn __unstable_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }
}

#[allow(non_camel_case_types)]
pub struct ListValue {
  inner: ::__pb::__runtime::MessageInner
}

impl std::fmt::Debug for ListValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

impl std::default::Default for ListValue {
  fn default() -> Self {
    Self::new()
  }
}

// SAFETY:
// - `ListValue` is `Sync` because it does not implement interior mutability.
//    Neither does `ListValueMut`.
unsafe impl Sync for ListValue {}

// SAFETY:
// - `ListValue` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for ListValue {}

impl ::__pb::Proxied for ListValue {
  type View<'msg> = ListValueView<'msg>;
  type Mut<'msg> = ListValueMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ListValueView<'msg> {
  msg: ::__pb::__runtime::RawMessage,
  _phantom: ::__std::marker::PhantomData<&'msg ()>,
}

impl std::fmt::Debug for ListValueView<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> ListValueView<'msg> {
  #[doc(hidden)]
  pub fn new(_private: ::__pb::__internal::Private, msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { msg, _phantom: ::__std::marker::PhantomData }
  }

  fn raw_msg(&self) -> ::__pb::__runtime::RawMessage {
    self.msg
  }

  pub fn serialize(&self) -> ::__pb::__runtime::SerializedData {
    unsafe { __rust_proto_thunk__google_protobuf_ListValue_serialize(self.raw_msg()) }
  }

  // values: repeated message google.protobuf.Value
  pub fn values(self) -> ::__pb::RepeatedView<'msg, crate::Value> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__google_protobuf_ListValue_get_values(self.raw_msg()),
      )
    }
  }

}

// SAFETY:
// - `ListValueView` is `Sync` because it does not support mutation.
unsafe impl Sync for ListValueView<'_> {}

// SAFETY:
// - `ListValueView` is `Send` because while its alive a `ListValueMut` cannot.
// - `ListValueView` does not use thread-local data.
unsafe impl Send for ListValueView<'_> {}

impl<'msg> ::__pb::ViewProxy<'msg> for ListValueView<'msg> {
  type Proxied = ListValue;

  fn as_view(&self) -> ::__pb::View<'msg, ListValue> {
    *self
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, ListValue> where 'msg: 'shorter {
    self
  }
}

impl ::__pb::__internal::ProxiedWithRawVTable for ListValue {
  type VTable = ::__pb::__runtime::MessageVTable;

  fn make_view(_private: ::__pb::__internal::Private,
              mut_inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::View<'_, Self> {
    let msg = unsafe {
      (mut_inner.vtable().getter)(mut_inner.msg_ref().msg())
    };
    ListValueView::new(::__pb::__internal::Private, msg)
  }

  fn make_mut(_private: ::__pb::__internal::Private,
              inner: ::__pb::__internal::RawVTableMutator<'_, Self>)
              -> ::__pb::Mut<'_, Self> {
    let raw_submsg = unsafe {
      (inner.vtable().mut_getter)(inner.msg_ref().msg())
    };
    ListValueMut::from_parent(::__pb::__internal::Private, inner.msg_ref(), raw_submsg)
  }
}

impl ::__pb::__internal::ProxiedWithRawOptionalVTable for ListValue {
  type OptionalVTable = ::__pb::__runtime::MessageVTable;

  fn upcast_vtable(_private: ::__pb::__internal::Private,
                   optional_vtable: &'static Self::OptionalVTable)
                  -> &'static Self::VTable {
    &optional_vtable
  }
}

impl ::__pb::ProxiedWithPresence for ListValue {
  type PresentMutData<'a> = ::__pb::__runtime::MessagePresentMutData<'a, ListValue>;
  type AbsentMutData<'a> = ::__pb::__runtime::MessageAbsentMutData<'a, ListValue>;

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

impl<'msg> ::__pb::SettableValue<ListValue> for ListValueView<'msg> {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, ListValue>)
    where ListValue: 'dst {
    unsafe { __rust_proto_thunk__google_protobuf_ListValue_copy_from(mutator.inner.msg(), self.msg) };
  }
}

impl ::__pb::SettableValue<ListValue> for ListValue {
  fn set_on<'dst>(
    self, _private: ::__pb::__internal::Private, mutator: ::__pb::Mut<'dst, ListValue>)
    where ListValue: 'dst {
    self.as_view().set_on(::__pb::__internal::Private, mutator);
  }
}

unsafe impl ::__pb::ProxiedInRepeated for ListValue {
  fn repeated_len(f: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    // SAFETY: `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__google_protobuf_ListValue_repeated_len(f.as_raw(::__pb::__internal::Private)) }
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
      __rust_proto_thunk__google_protobuf_ListValue_copy_from(
        __rust_proto_thunk__google_protobuf_ListValue_repeated_get_mut(f.as_raw(::__pb::__internal::Private), i),
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
    let msg = unsafe { __rust_proto_thunk__google_protobuf_ListValue_repeated_get(f.as_raw(::__pb::__internal::Private), i) };
    ::__pb::View::<Self>::new(::__pb::__internal::Private, msg)
  }
  fn repeated_clear(mut f: ::__pb::Mut<::__pb::Repeated<Self>>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    unsafe { __rust_proto_thunk__google_protobuf_ListValue_repeated_clear(f.as_raw(::__pb::__internal::Private)) };
  }

  fn repeated_push(mut f: ::__pb::Mut<::__pb::Repeated<Self>>, v: ::__pb::View<Self>) {
    // SAFETY:
    // - `f.as_raw()` is a valid `RepeatedPtrField*`.
    // - `v.raw_msg()` is a valid `const Message&`.
    unsafe {
      let new_elem = __rust_proto_thunk__google_protobuf_ListValue_repeated_add(f.as_raw(::__pb::__internal::Private));
      __rust_proto_thunk__google_protobuf_ListValue_copy_from(new_elem, v.raw_msg());
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
      __rust_proto_thunk__google_protobuf_ListValue_repeated_copy_from(dest.as_raw(::__pb::__internal::Private), src.as_raw(::__pb::__internal::Private));
    }
  }
}
extern "C" {
    fn __rust_proto_thunk__Map_i32_google_protobuf_ListValue_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i32_google_protobuf_ListValue_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_google_protobuf_ListValue_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i32_google_protobuf_ListValue_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i32_google_protobuf_ListValue_insert(m: ::__pb::__runtime::RawMap, key: i32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_google_protobuf_ListValue_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_google_protobuf_ListValue_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i32_google_protobuf_ListValue_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i32_google_protobuf_ListValue_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i32> for ListValue {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_google_protobuf_ListValue_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
        unsafe { __rust_proto_thunk__Map_i32_google_protobuf_ListValue_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
        unsafe { __rust_proto_thunk__Map_i32_google_protobuf_ListValue_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i32_google_protobuf_ListValue_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i32_google_protobuf_ListValue_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i32_google_protobuf_ListValue_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ListValueView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i32_google_protobuf_ListValue_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i32_google_protobuf_ListValue_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i32_google_protobuf_ListValue_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ListValueView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u32_google_protobuf_ListValue_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u32_google_protobuf_ListValue_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_google_protobuf_ListValue_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u32_google_protobuf_ListValue_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u32_google_protobuf_ListValue_insert(m: ::__pb::__runtime::RawMap, key: u32, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_google_protobuf_ListValue_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_google_protobuf_ListValue_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u32_google_protobuf_ListValue_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u32_google_protobuf_ListValue_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u32> for ListValue {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_google_protobuf_ListValue_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
        unsafe { __rust_proto_thunk__Map_u32_google_protobuf_ListValue_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
        unsafe { __rust_proto_thunk__Map_u32_google_protobuf_ListValue_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u32_google_protobuf_ListValue_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u32_google_protobuf_ListValue_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u32_google_protobuf_ListValue_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ListValueView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u32_google_protobuf_ListValue_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u32_google_protobuf_ListValue_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u32_google_protobuf_ListValue_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ListValueView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_i64_google_protobuf_ListValue_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_i64_google_protobuf_ListValue_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_google_protobuf_ListValue_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_i64_google_protobuf_ListValue_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_i64_google_protobuf_ListValue_insert(m: ::__pb::__runtime::RawMap, key: i64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_google_protobuf_ListValue_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_google_protobuf_ListValue_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_i64_google_protobuf_ListValue_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_i64_google_protobuf_ListValue_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<i64> for ListValue {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_google_protobuf_ListValue_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
        unsafe { __rust_proto_thunk__Map_i64_google_protobuf_ListValue_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
        unsafe { __rust_proto_thunk__Map_i64_google_protobuf_ListValue_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_i64_google_protobuf_ListValue_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_i64_google_protobuf_ListValue_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_i64_google_protobuf_ListValue_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ListValueView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_i64_google_protobuf_ListValue_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_i64_google_protobuf_ListValue_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_i64_google_protobuf_ListValue_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ListValueView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_u64_google_protobuf_ListValue_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_u64_google_protobuf_ListValue_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_google_protobuf_ListValue_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_u64_google_protobuf_ListValue_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_u64_google_protobuf_ListValue_insert(m: ::__pb::__runtime::RawMap, key: u64, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_google_protobuf_ListValue_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_google_protobuf_ListValue_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_u64_google_protobuf_ListValue_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_u64_google_protobuf_ListValue_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<u64> for ListValue {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_google_protobuf_ListValue_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
        unsafe { __rust_proto_thunk__Map_u64_google_protobuf_ListValue_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
        unsafe { __rust_proto_thunk__Map_u64_google_protobuf_ListValue_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_u64_google_protobuf_ListValue_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_u64_google_protobuf_ListValue_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_u64_google_protobuf_ListValue_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ListValueView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_u64_google_protobuf_ListValue_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_u64_google_protobuf_ListValue_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_u64_google_protobuf_ListValue_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ListValueView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_bool_google_protobuf_ListValue_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_bool_google_protobuf_ListValue_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_google_protobuf_ListValue_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_bool_google_protobuf_ListValue_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_bool_google_protobuf_ListValue_insert(m: ::__pb::__runtime::RawMap, key: bool, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_google_protobuf_ListValue_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_google_protobuf_ListValue_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_bool_google_protobuf_ListValue_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_bool_google_protobuf_ListValue_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<bool> for ListValue {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_google_protobuf_ListValue_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
        unsafe { __rust_proto_thunk__Map_bool_google_protobuf_ListValue_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
        unsafe { __rust_proto_thunk__Map_bool_google_protobuf_ListValue_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_bool_google_protobuf_ListValue_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_bool_google_protobuf_ListValue_insert(map.as_raw(::__pb::__internal::Private), key, value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
        let key = key;
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_bool_google_protobuf_ListValue_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ListValueView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_bool_google_protobuf_ListValue_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_bool_google_protobuf_ListValue_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_bool_google_protobuf_ListValue_iter_get,
                |ffi_key| ffi_key,
                |raw_msg| ListValueView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}
extern "C" {
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_new() -> ::__pb::__runtime::RawMap;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_free(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_clear(m: ::__pb::__runtime::RawMap);
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_size(m: ::__pb::__runtime::RawMap) -> usize;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage) -> bool;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
    fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut ::__pb::__runtime::RawMessage);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for ListValue {
    fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
        unsafe {
            ::__pb::Map::from_inner(
                ::__pb::__internal::Private,
                ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_new())
            )
        }
    }

    unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_free(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_clear(map.as_raw(::__pb::__internal::Private)); }
    }

    fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_size(map.as_raw(::__pb::__internal::Private)) }
    }

    fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.raw_msg()) }
    }

    fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
        let key = key.as_bytes().into();
        let mut value = ::__std::mem::MaybeUninit::uninit();
        let found = unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
        if !found {
            return None;
        }
        Some(ListValueView::new(::__pb::__internal::Private, unsafe { value.assume_init() }))
    }

    fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
        let mut value = ::__std::mem::MaybeUninit::uninit();
        unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
                __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_iter(map.as_raw(::__pb::__internal::Private))
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
                __rust_proto_thunk__Map_ProtoStr_google_protobuf_ListValue_iter_get,
                |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
                |raw_msg| ListValueView::new(::__pb::__internal::Private, raw_msg)
            )
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ListValueMut<'msg> {
  inner: ::__pb::__runtime::MutatorMessageRef<'msg>,
}

impl std::fmt::Debug for ListValueMut<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ::__pb::__runtime::debug_string(::__pb::__internal::Private, self.raw_msg(), f)
  }
}

#[allow(dead_code)]
impl<'msg> ListValueMut<'msg> {
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


  // values: repeated message google.protobuf.Value
  pub fn values(&self) -> ::__pb::RepeatedView<'_, crate::Value> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__google_protobuf_ListValue_get_values(self.raw_msg()),
      )
    }
  }
  pub fn values_mut(&mut self) -> ::__pb::RepeatedMut<'_, crate::Value> {
    unsafe {
      ::__pb::RepeatedMut::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__runtime::InnerRepeatedMut::new(
          ::__pb::__internal::Private,
          __rust_proto_thunk__google_protobuf_ListValue_get_mut_values(self.raw_msg()),
        ),
      )
    }
  }

}

// SAFETY:
// - `ListValueMut` does not perform any shared mutation.
// - `ListValueMut` is not `Send`, and so even in the presence of mutator
//   splitting, synchronous access of an arena is impossible.
unsafe impl Sync for ListValueMut<'_> {}

impl<'msg> ::__pb::MutProxy<'msg> for ListValueMut<'msg> {
  fn as_mut(&mut self) -> ::__pb::Mut<'_, ListValue> {
    ListValueMut { inner: self.inner }
  }
  fn into_mut<'shorter>(self) -> ::__pb::Mut<'shorter, ListValue> where 'msg : 'shorter { self }
}

impl<'msg> ::__pb::ViewProxy<'msg> for ListValueMut<'msg> {
  type Proxied = ListValue;
  fn as_view(&self) -> ::__pb::View<'_, ListValue> {
    ListValueView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, ListValue> where 'msg: 'shorter {
    ListValueView { msg: self.raw_msg(), _phantom: ::__std::marker::PhantomData }
  }
}

#[allow(dead_code)]
impl ListValue {
  pub fn new() -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg: unsafe { __rust_proto_thunk__google_protobuf_ListValue_new() } } }
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

      __rust_proto_thunk__google_protobuf_ListValue_parse(self.raw_msg(), data)
    };
    success.then_some(()).ok_or(::__pb::ParseError)
  }
  pub fn parse(data: &[u8]) -> Result<Self, ::__pb::ParseError> {
    let mut msg = Self::new();
    msg.clear_and_parse(data).map(|_| msg)
  }

  pub fn as_view(&self) -> ListValueView {
    ListValueView::new(::__pb::__internal::Private, self.inner.msg)
  }

  pub fn as_mut(&mut self) -> ListValueMut {
    ListValueMut::new(::__pb::__internal::Private, &mut self.inner)
  }

  // values: repeated message google.protobuf.Value
  pub fn values(&self) -> ::__pb::RepeatedView<'_, crate::Value> {
    unsafe {
      ::__pb::RepeatedView::from_raw(
        ::__pb::__internal::Private,
        __rust_proto_thunk__google_protobuf_ListValue_get_values(self.raw_msg()),
      )
    }
  }
  pub fn values_mut(&mut self) -> ::__pb::RepeatedMut<'_, crate::Value> {
    unsafe {
      ::__pb::RepeatedMut::from_inner(
        ::__pb::__internal::Private,
        ::__pb::__runtime::InnerRepeatedMut::new(
          ::__pb::__internal::Private,
          __rust_proto_thunk__google_protobuf_ListValue_get_mut_values(self.raw_msg()),
        ),
      )
    }
  }

}  // impl ListValue

impl ::__std::ops::Drop for ListValue {
  fn drop(&mut self) {
    unsafe { __rust_proto_thunk__google_protobuf_ListValue_delete(self.raw_msg()); }
  }
}

extern "C" {
  fn __rust_proto_thunk__google_protobuf_ListValue_new() -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_protobuf_ListValue_delete(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_protobuf_ListValue_serialize(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::SerializedData;
  fn __rust_proto_thunk__google_protobuf_ListValue_parse(raw_msg: ::__pb::__runtime::RawMessage, data: ::__pb::__runtime::SerializedData) -> bool;
  fn __rust_proto_thunk__google_protobuf_ListValue_copy_from(dst: ::__pb::__runtime::RawMessage, src: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_protobuf_ListValue_repeated_len(raw: ::__pb::__runtime::RawRepeatedField) -> usize;
  fn __rust_proto_thunk__google_protobuf_ListValue_repeated_add(raw: ::__pb::__runtime::RawRepeatedField) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_protobuf_ListValue_repeated_get(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_protobuf_ListValue_repeated_get_mut(raw: ::__pb::__runtime::RawRepeatedField, index: usize) -> ::__pb::__runtime::RawMessage;
  fn __rust_proto_thunk__google_protobuf_ListValue_repeated_clear(raw: ::__pb::__runtime::RawRepeatedField);
  fn __rust_proto_thunk__google_protobuf_ListValue_repeated_copy_from(dst: ::__pb::__runtime::RawRepeatedField, src: ::__pb::__runtime::RawRepeatedField);

  fn __rust_proto_thunk__google_protobuf_ListValue_clear_values(raw_msg: ::__pb::__runtime::RawMessage);
  fn __rust_proto_thunk__google_protobuf_ListValue_get_mut_values(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawRepeatedField;
  fn __rust_proto_thunk__google_protobuf_ListValue_get_values(raw_msg: ::__pb::__runtime::RawMessage) -> ::__pb::__runtime::RawRepeatedField;


}  // extern "C" for ListValue


impl ListValue {
  pub fn __unstable_wrap_cpp_grant_permission_to_break(msg: ::__pb::__runtime::RawMessage) -> Self {
    Self { inner: ::__pb::__runtime::MessageInner { msg } }
  }
  pub fn __unstable_leak_cpp_repr_grant_permission_to_break(self) -> ::__pb::__runtime::RawMessage {
    let s = std::mem::ManuallyDrop::new(self);
    s.raw_msg()
  }
}

impl<'a> ListValueMut<'a> {
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

impl<'a> ListValueView<'a> {
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
pub struct NullValue(i32);

#[allow(non_upper_case_globals)]
impl NullValue {
  pub const NullValue: NullValue = NullValue(0);
}

impl ::__std::convert::From<NullValue> for i32 {
  fn from(val: NullValue) -> i32 {
    val.0
  }
}

impl ::__std::convert::From<i32> for NullValue {
  fn from(val: i32) -> NullValue {
    Self(val)
  }
}

impl ::__std::default::Default for NullValue {
  fn default() -> Self {
    Self(0)
  }
}

impl ::__std::fmt::Debug for NullValue {
  fn fmt(&self, f: &mut ::__std::fmt::Formatter<'_>) -> ::__std::fmt::Result {
    f.debug_tuple(stringify!(NullValue)).field(&self.0).finish()
  }
}

impl ::__pb::Proxied for NullValue {
  type View<'a> = NullValue;
  type Mut<'a> = ::__pb::PrimitiveMut<'a, NullValue>;
}

impl ::__pb::ViewProxy<'_> for NullValue {
  type Proxied = NullValue;

  fn as_view(&self) -> NullValue {
    *self
  }

  fn into_view<'shorter>(self) -> ::__pb::View<'shorter, NullValue> {
    self
  }
}

impl ::__pb::SettableValue<NullValue> for NullValue {
  fn set_on<'msg>(
      self,
      private: ::__pb::__internal::Private,
      mut mutator: ::__pb::Mut<'msg, NullValue>
  ) where NullValue: 'msg {
    mutator.set_primitive(private, self)
  }
}

impl ::__pb::ProxiedWithPresence for NullValue {
  type PresentMutData<'a> = ::__pb::__internal::RawVTableOptionalMutatorData<'a, NullValue>;
  type AbsentMutData<'a> = ::__pb::__internal::RawVTableOptionalMutatorData<'a, NullValue>;

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

unsafe impl ::__pb::ProxiedInRepeated for NullValue {
  fn repeated_len(r: ::__pb::View<::__pb::Repeated<Self>>) -> usize {
    ::__pb::__runtime::cast_enum_repeated_view(::__pb::__internal::Private, r).len()
  }

  fn repeated_push(r: ::__pb::Mut<::__pb::Repeated<Self>>, val: NullValue) {
    ::__pb::__runtime::cast_enum_repeated_mut(::__pb::__internal::Private, r).push(val.into())
  }

  fn repeated_clear(r: ::__pb::Mut<::__pb::Repeated<Self>>) {
    ::__pb::__runtime::cast_enum_repeated_mut(::__pb::__internal::Private, r).clear()
  }

  unsafe fn repeated_get_unchecked(
      r: ::__pb::View<::__pb::Repeated<Self>>,
      index: usize,
  ) -> ::__pb::View<NullValue> {
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
      val: NullValue,
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

impl ::__pb::__internal::PrimitiveWithRawVTable for NullValue {}

// SAFETY: this is an enum type
unsafe impl ::__pb::__internal::Enum for NullValue {
  const NAME: &'static str = "NullValue";
}

extern "C" {
  fn __rust_proto_thunk__Map_i32_google_protobuf_NullValue_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_i32_google_protobuf_NullValue_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_i32_google_protobuf_NullValue_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_i32_google_protobuf_NullValue_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_i32_google_protobuf_NullValue_insert(m: ::__pb::__runtime::RawMap, key: i32, value: NullValue) -> bool;
  fn __rust_proto_thunk__Map_i32_google_protobuf_NullValue_get(m: ::__pb::__runtime::RawMap, key: i32, value: *mut NullValue) -> bool;
  fn __rust_proto_thunk__Map_i32_google_protobuf_NullValue_remove(m: ::__pb::__runtime::RawMap, key: i32, value: *mut NullValue) -> bool;
  fn __rust_proto_thunk__Map_i32_google_protobuf_NullValue_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_i32_google_protobuf_NullValue_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i32, value: *mut NullValue);
}
impl ::__pb::ProxiedInMapValue<i32> for NullValue {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i32, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i32_google_protobuf_NullValue_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i32, Self>) {
      unsafe { __rust_proto_thunk__Map_i32_google_protobuf_NullValue_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>) {
      unsafe { __rust_proto_thunk__Map_i32_google_protobuf_NullValue_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<i32, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_i32_google_protobuf_NullValue_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_i32_google_protobuf_NullValue_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_i32_google_protobuf_NullValue_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i32, Self>>, key: ::__pb::View<'_, i32>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_i32_google_protobuf_NullValue_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_i32_google_protobuf_NullValue_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_i32_google_protobuf_NullValue_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_u32_google_protobuf_NullValue_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_u32_google_protobuf_NullValue_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_u32_google_protobuf_NullValue_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_u32_google_protobuf_NullValue_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_u32_google_protobuf_NullValue_insert(m: ::__pb::__runtime::RawMap, key: u32, value: NullValue) -> bool;
  fn __rust_proto_thunk__Map_u32_google_protobuf_NullValue_get(m: ::__pb::__runtime::RawMap, key: u32, value: *mut NullValue) -> bool;
  fn __rust_proto_thunk__Map_u32_google_protobuf_NullValue_remove(m: ::__pb::__runtime::RawMap, key: u32, value: *mut NullValue) -> bool;
  fn __rust_proto_thunk__Map_u32_google_protobuf_NullValue_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_u32_google_protobuf_NullValue_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u32, value: *mut NullValue);
}
impl ::__pb::ProxiedInMapValue<u32> for NullValue {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u32, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u32_google_protobuf_NullValue_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u32, Self>) {
      unsafe { __rust_proto_thunk__Map_u32_google_protobuf_NullValue_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>) {
      unsafe { __rust_proto_thunk__Map_u32_google_protobuf_NullValue_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<u32, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_u32_google_protobuf_NullValue_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_u32_google_protobuf_NullValue_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_u32_google_protobuf_NullValue_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u32, Self>>, key: ::__pb::View<'_, u32>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_u32_google_protobuf_NullValue_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_u32_google_protobuf_NullValue_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_u32_google_protobuf_NullValue_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_i64_google_protobuf_NullValue_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_i64_google_protobuf_NullValue_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_i64_google_protobuf_NullValue_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_i64_google_protobuf_NullValue_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_i64_google_protobuf_NullValue_insert(m: ::__pb::__runtime::RawMap, key: i64, value: NullValue) -> bool;
  fn __rust_proto_thunk__Map_i64_google_protobuf_NullValue_get(m: ::__pb::__runtime::RawMap, key: i64, value: *mut NullValue) -> bool;
  fn __rust_proto_thunk__Map_i64_google_protobuf_NullValue_remove(m: ::__pb::__runtime::RawMap, key: i64, value: *mut NullValue) -> bool;
  fn __rust_proto_thunk__Map_i64_google_protobuf_NullValue_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_i64_google_protobuf_NullValue_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut i64, value: *mut NullValue);
}
impl ::__pb::ProxiedInMapValue<i64> for NullValue {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<i64, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_i64_google_protobuf_NullValue_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<i64, Self>) {
      unsafe { __rust_proto_thunk__Map_i64_google_protobuf_NullValue_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>) {
      unsafe { __rust_proto_thunk__Map_i64_google_protobuf_NullValue_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<i64, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_i64_google_protobuf_NullValue_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_i64_google_protobuf_NullValue_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_i64_google_protobuf_NullValue_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<i64, Self>>, key: ::__pb::View<'_, i64>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_i64_google_protobuf_NullValue_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_i64_google_protobuf_NullValue_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_i64_google_protobuf_NullValue_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_u64_google_protobuf_NullValue_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_u64_google_protobuf_NullValue_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_u64_google_protobuf_NullValue_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_u64_google_protobuf_NullValue_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_u64_google_protobuf_NullValue_insert(m: ::__pb::__runtime::RawMap, key: u64, value: NullValue) -> bool;
  fn __rust_proto_thunk__Map_u64_google_protobuf_NullValue_get(m: ::__pb::__runtime::RawMap, key: u64, value: *mut NullValue) -> bool;
  fn __rust_proto_thunk__Map_u64_google_protobuf_NullValue_remove(m: ::__pb::__runtime::RawMap, key: u64, value: *mut NullValue) -> bool;
  fn __rust_proto_thunk__Map_u64_google_protobuf_NullValue_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_u64_google_protobuf_NullValue_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut u64, value: *mut NullValue);
}
impl ::__pb::ProxiedInMapValue<u64> for NullValue {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<u64, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_u64_google_protobuf_NullValue_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<u64, Self>) {
      unsafe { __rust_proto_thunk__Map_u64_google_protobuf_NullValue_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>) {
      unsafe { __rust_proto_thunk__Map_u64_google_protobuf_NullValue_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<u64, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_u64_google_protobuf_NullValue_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_u64_google_protobuf_NullValue_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_u64_google_protobuf_NullValue_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<u64, Self>>, key: ::__pb::View<'_, u64>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_u64_google_protobuf_NullValue_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_u64_google_protobuf_NullValue_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_u64_google_protobuf_NullValue_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_bool_google_protobuf_NullValue_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_bool_google_protobuf_NullValue_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_bool_google_protobuf_NullValue_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_bool_google_protobuf_NullValue_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_bool_google_protobuf_NullValue_insert(m: ::__pb::__runtime::RawMap, key: bool, value: NullValue) -> bool;
  fn __rust_proto_thunk__Map_bool_google_protobuf_NullValue_get(m: ::__pb::__runtime::RawMap, key: bool, value: *mut NullValue) -> bool;
  fn __rust_proto_thunk__Map_bool_google_protobuf_NullValue_remove(m: ::__pb::__runtime::RawMap, key: bool, value: *mut NullValue) -> bool;
  fn __rust_proto_thunk__Map_bool_google_protobuf_NullValue_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_bool_google_protobuf_NullValue_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut bool, value: *mut NullValue);
}
impl ::__pb::ProxiedInMapValue<bool> for NullValue {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<bool, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_bool_google_protobuf_NullValue_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<bool, Self>) {
      unsafe { __rust_proto_thunk__Map_bool_google_protobuf_NullValue_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>) {
      unsafe { __rust_proto_thunk__Map_bool_google_protobuf_NullValue_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<bool, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_bool_google_protobuf_NullValue_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_bool_google_protobuf_NullValue_insert(map.as_raw(::__pb::__internal::Private), key, value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> Option<::__pb::View<'a, Self>> {
      let key = key;
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_bool_google_protobuf_NullValue_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<bool, Self>>, key: ::__pb::View<'_, bool>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_bool_google_protobuf_NullValue_remove(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_bool_google_protobuf_NullValue_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_bool_google_protobuf_NullValue_iter_get,
              |ffi_key| ffi_key,
              ::__std::convert::identity,
          )
      }
  }
}
extern "C" {
  fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_NullValue_new() -> ::__pb::__runtime::RawMap;
  fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_NullValue_free(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_NullValue_clear(m: ::__pb::__runtime::RawMap);
  fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_NullValue_size(m: ::__pb::__runtime::RawMap) -> usize;
  fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_NullValue_insert(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: NullValue) -> bool;
  fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_NullValue_get(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut NullValue) -> bool;
  fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_NullValue_remove(m: ::__pb::__runtime::RawMap, key: ::__pb::__runtime::PtrAndLen, value: *mut NullValue) -> bool;
  fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_NullValue_iter(m: ::__pb::__runtime::RawMap) -> ::__pb::__runtime::UntypedMapIterator;
  fn __rust_proto_thunk__Map_ProtoStr_google_protobuf_NullValue_iter_get(iter: &mut ::__pb::__runtime::UntypedMapIterator, key: *mut ::__pb::__runtime::PtrAndLen, value: *mut NullValue);
}
impl ::__pb::ProxiedInMapValue<::__pb::ProtoStr> for NullValue {
  fn map_new(_private: ::__pb::__internal::Private) -> ::__pb::Map<::__pb::ProtoStr, Self> {
      unsafe {
          ::__pb::Map::from_inner(
              ::__pb::__internal::Private,
              ::__pb::__runtime::InnerMap::new(::__pb::__internal::Private, __rust_proto_thunk__Map_ProtoStr_google_protobuf_NullValue_new())
          )
      }
  }

  unsafe fn map_free(_private: ::__pb::__internal::Private, map: &mut ::__pb::Map<::__pb::ProtoStr, Self>) {
      unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_NullValue_free(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_clear(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) {
      unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_NullValue_clear(map.as_raw(::__pb::__internal::Private)); }
  }

  fn map_len(map: ::__pb::View<'_, ::__pb::Map<::__pb::ProtoStr, Self>>) -> usize {
      unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_NullValue_size(map.as_raw(::__pb::__internal::Private)) }
  }

  fn map_insert(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>, value: ::__pb::View<'_, Self>) -> bool {
      unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_NullValue_insert(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value) }
  }

  fn map_get<'a>(map: ::__pb::View<'a, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> Option<::__pb::View<'a, Self>> {
      let key = key.as_bytes().into();
      let mut value = ::__std::mem::MaybeUninit::uninit();
      let found = unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_NullValue_get(map.as_raw(::__pb::__internal::Private), key, value.as_mut_ptr()) };
      if !found {
          return None;
      }
      Some(unsafe { value.assume_init() })
  }

  fn map_remove(mut map: ::__pb::Mut<'_, ::__pb::Map<::__pb::ProtoStr, Self>>, key: ::__pb::View<'_, ::__pb::ProtoStr>) -> bool {
      let mut value = ::__std::mem::MaybeUninit::uninit();
      unsafe { __rust_proto_thunk__Map_ProtoStr_google_protobuf_NullValue_remove(map.as_raw(::__pb::__internal::Private), key.as_bytes().into(), value.as_mut_ptr()) }
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
              __rust_proto_thunk__Map_ProtoStr_google_protobuf_NullValue_iter(map.as_raw(::__pb::__internal::Private))
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
              __rust_proto_thunk__Map_ProtoStr_google_protobuf_NullValue_iter_get,
              |ffi_key| ::__pb::ProtoStr::from_utf8_unchecked(ffi_key.as_ref()),
              ::__std::convert::identity,
          )
      }
  }
}

