#include "google/protobuf/timestamp.proto.h"
#include "google/protobuf/rust/cpp_kernel/cpp_api.h"
// google.protobuf.Timestamp
        // clang-format off
extern "C" {
void* __rust_proto_thunk__google_protobuf_Timestamp_new() { return new ::google::protobuf::Timestamp(); }
void __rust_proto_thunk__google_protobuf_Timestamp_delete(void* ptr) { delete static_cast<::google::protobuf::Timestamp*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__google_protobuf_Timestamp_serialize(::google::protobuf::Timestamp* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__google_protobuf_Timestamp_parse(::google::protobuf::Timestamp* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__google_protobuf_Timestamp_copy_from(::google::protobuf::Timestamp* dst, const ::google::protobuf::Timestamp* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__google_protobuf_Timestamp_repeated_len(google::protobuf::RepeatedPtrField<::google::protobuf::Timestamp>* field) {
  return field->size();
}
const ::google::protobuf::Timestamp& __rust_proto_thunk__google_protobuf_Timestamp_repeated_get(
  google::protobuf::RepeatedPtrField<::google::protobuf::Timestamp>* field,
  size_t index) {
  return field->Get(index);
}
::google::protobuf::Timestamp* __rust_proto_thunk__google_protobuf_Timestamp_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::google::protobuf::Timestamp>* field,
  size_t index) {
  return field->Mutable(index);
}
::google::protobuf::Timestamp* __rust_proto_thunk__google_protobuf_Timestamp_repeated_add(google::protobuf::RepeatedPtrField<::google::protobuf::Timestamp>* field) {
  return field->Add();
}
void __rust_proto_thunk__google_protobuf_Timestamp_repeated_clear(google::protobuf::RepeatedPtrField<::google::protobuf::Timestamp>* field) {
  field->Clear();
}
void __rust_proto_thunk__google_protobuf_Timestamp_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::google::protobuf::Timestamp>& dst,
  const google::protobuf::RepeatedPtrField<::google::protobuf::Timestamp>& src) {
  dst = src;
}

::int64_t __rust_proto_thunk__google_protobuf_Timestamp_get_seconds(::google::protobuf::Timestamp* msg) { return msg->seconds(); }
void __rust_proto_thunk__google_protobuf_Timestamp_set_seconds(::google::protobuf::Timestamp* msg, ::int64_t val) {
  msg->set_seconds(val);
}

::int32_t __rust_proto_thunk__google_protobuf_Timestamp_get_nanos(::google::protobuf::Timestamp* msg) { return msg->nanos(); }
void __rust_proto_thunk__google_protobuf_Timestamp_set_nanos(::google::protobuf::Timestamp* msg, ::int32_t val) {
  msg->set_nanos(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::google::protobuf::Timestamp>* __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_new() {
    return new google::protobuf::Map<int32_t, ::google::protobuf::Timestamp>();
  }
  void __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_free(const google::protobuf::Map<int32_t, ::google::protobuf::Timestamp>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_clear(google::protobuf::Map<int32_t, ::google::protobuf::Timestamp> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_size(const google::protobuf::Map<int32_t, ::google::protobuf::Timestamp>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_insert(google::protobuf::Map<int32_t, ::google::protobuf::Timestamp> * m,
                          int32_t key, ::google::protobuf::Timestamp value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_get(const google::protobuf::Map<int32_t, ::google::protobuf::Timestamp>* m,
                       int32_t key, const ::google::protobuf::Timestamp** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::Timestamp* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_remove(google::protobuf::Map<int32_t, ::google::protobuf::Timestamp> * m,
                          int32_t key, ::google::protobuf::Timestamp * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_iter(
      const google::protobuf::Map<int32_t, ::google::protobuf::Timestamp>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_google_protobuf_Timestamp_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::google::protobuf::Timestamp** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::google::protobuf::Timestamp>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::google::protobuf::Timestamp>* __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_new() {
    return new google::protobuf::Map<uint32_t, ::google::protobuf::Timestamp>();
  }
  void __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_free(const google::protobuf::Map<uint32_t, ::google::protobuf::Timestamp>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_clear(google::protobuf::Map<uint32_t, ::google::protobuf::Timestamp> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_size(const google::protobuf::Map<uint32_t, ::google::protobuf::Timestamp>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_insert(google::protobuf::Map<uint32_t, ::google::protobuf::Timestamp> * m,
                          uint32_t key, ::google::protobuf::Timestamp value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_get(const google::protobuf::Map<uint32_t, ::google::protobuf::Timestamp>* m,
                       uint32_t key, const ::google::protobuf::Timestamp** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::Timestamp* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_remove(google::protobuf::Map<uint32_t, ::google::protobuf::Timestamp> * m,
                          uint32_t key, ::google::protobuf::Timestamp * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_iter(
      const google::protobuf::Map<uint32_t, ::google::protobuf::Timestamp>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_google_protobuf_Timestamp_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::google::protobuf::Timestamp** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::google::protobuf::Timestamp>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::google::protobuf::Timestamp>* __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_new() {
    return new google::protobuf::Map<int64_t, ::google::protobuf::Timestamp>();
  }
  void __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_free(const google::protobuf::Map<int64_t, ::google::protobuf::Timestamp>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_clear(google::protobuf::Map<int64_t, ::google::protobuf::Timestamp> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_size(const google::protobuf::Map<int64_t, ::google::protobuf::Timestamp>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_insert(google::protobuf::Map<int64_t, ::google::protobuf::Timestamp> * m,
                          int64_t key, ::google::protobuf::Timestamp value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_get(const google::protobuf::Map<int64_t, ::google::protobuf::Timestamp>* m,
                       int64_t key, const ::google::protobuf::Timestamp** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::Timestamp* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_remove(google::protobuf::Map<int64_t, ::google::protobuf::Timestamp> * m,
                          int64_t key, ::google::protobuf::Timestamp * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_iter(
      const google::protobuf::Map<int64_t, ::google::protobuf::Timestamp>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_google_protobuf_Timestamp_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::google::protobuf::Timestamp** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::google::protobuf::Timestamp>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::google::protobuf::Timestamp>* __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_new() {
    return new google::protobuf::Map<uint64_t, ::google::protobuf::Timestamp>();
  }
  void __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_free(const google::protobuf::Map<uint64_t, ::google::protobuf::Timestamp>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_clear(google::protobuf::Map<uint64_t, ::google::protobuf::Timestamp> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_size(const google::protobuf::Map<uint64_t, ::google::protobuf::Timestamp>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_insert(google::protobuf::Map<uint64_t, ::google::protobuf::Timestamp> * m,
                          uint64_t key, ::google::protobuf::Timestamp value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_get(const google::protobuf::Map<uint64_t, ::google::protobuf::Timestamp>* m,
                       uint64_t key, const ::google::protobuf::Timestamp** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::Timestamp* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_remove(google::protobuf::Map<uint64_t, ::google::protobuf::Timestamp> * m,
                          uint64_t key, ::google::protobuf::Timestamp * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_iter(
      const google::protobuf::Map<uint64_t, ::google::protobuf::Timestamp>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_google_protobuf_Timestamp_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::google::protobuf::Timestamp** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::google::protobuf::Timestamp>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::google::protobuf::Timestamp>* __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_new() {
    return new google::protobuf::Map<bool, ::google::protobuf::Timestamp>();
  }
  void __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_free(const google::protobuf::Map<bool, ::google::protobuf::Timestamp>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_clear(google::protobuf::Map<bool, ::google::protobuf::Timestamp> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_size(const google::protobuf::Map<bool, ::google::protobuf::Timestamp>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_insert(google::protobuf::Map<bool, ::google::protobuf::Timestamp> * m,
                          bool key, ::google::protobuf::Timestamp value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_get(const google::protobuf::Map<bool, ::google::protobuf::Timestamp>* m,
                       bool key, const ::google::protobuf::Timestamp** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::Timestamp* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_remove(google::protobuf::Map<bool, ::google::protobuf::Timestamp> * m,
                          bool key, ::google::protobuf::Timestamp * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_iter(
      const google::protobuf::Map<bool, ::google::protobuf::Timestamp>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_google_protobuf_Timestamp_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::google::protobuf::Timestamp** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::google::protobuf::Timestamp>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::google::protobuf::Timestamp>* __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_new() {
    return new google::protobuf::Map<std::string, ::google::protobuf::Timestamp>();
  }
  void __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_free(const google::protobuf::Map<std::string, ::google::protobuf::Timestamp>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_clear(google::protobuf::Map<std::string, ::google::protobuf::Timestamp> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_size(const google::protobuf::Map<std::string, ::google::protobuf::Timestamp>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_insert(google::protobuf::Map<std::string, ::google::protobuf::Timestamp> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::google::protobuf::Timestamp value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_get(const google::protobuf::Map<std::string, ::google::protobuf::Timestamp>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::google::protobuf::Timestamp** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::google::protobuf::Timestamp* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_remove(google::protobuf::Map<std::string, ::google::protobuf::Timestamp> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::google::protobuf::Timestamp * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_iter(
      const google::protobuf::Map<std::string, ::google::protobuf::Timestamp>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_google_protobuf_Timestamp_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::google::protobuf::Timestamp** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::google::protobuf::Timestamp>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

