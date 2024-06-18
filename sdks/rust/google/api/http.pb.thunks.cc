#include "google/api/http.proto.h"
#include "google/protobuf/rust/cpp_kernel/cpp_api.h"
// google.api.Http
        // clang-format off
extern "C" {
void* __rust_proto_thunk__google_api_Http_new() { return new ::google::api::Http(); }
void __rust_proto_thunk__google_api_Http_delete(void* ptr) { delete static_cast<::google::api::Http*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__google_api_Http_serialize(::google::api::Http* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__google_api_Http_parse(::google::api::Http* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__google_api_Http_copy_from(::google::api::Http* dst, const ::google::api::Http* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__google_api_Http_repeated_len(google::protobuf::RepeatedPtrField<::google::api::Http>* field) {
  return field->size();
}
const ::google::api::Http& __rust_proto_thunk__google_api_Http_repeated_get(
  google::protobuf::RepeatedPtrField<::google::api::Http>* field,
  size_t index) {
  return field->Get(index);
}
::google::api::Http* __rust_proto_thunk__google_api_Http_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::google::api::Http>* field,
  size_t index) {
  return field->Mutable(index);
}
::google::api::Http* __rust_proto_thunk__google_api_Http_repeated_add(google::protobuf::RepeatedPtrField<::google::api::Http>* field) {
  return field->Add();
}
void __rust_proto_thunk__google_api_Http_repeated_clear(google::protobuf::RepeatedPtrField<::google::api::Http>* field) {
  field->Clear();
}
void __rust_proto_thunk__google_api_Http_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::google::api::Http>& dst,
  const google::protobuf::RepeatedPtrField<::google::api::Http>& src) {
  dst = src;
}

void __rust_proto_thunk__google_api_Http_clear_rules(::google::api::Http* msg) {
  msg->clear_rules();
}
google::protobuf::RepeatedPtrField<::google::api::HttpRule>* __rust_proto_thunk__google_api_Http_get_mut_rules(::google::api::Http* msg) {
  return msg->mutable_rules();
}
const google::protobuf::RepeatedPtrField<::google::api::HttpRule>* __rust_proto_thunk__google_api_Http_get_rules(
    const ::google::api::Http* msg) {
  return &msg->rules();
}

bool __rust_proto_thunk__google_api_Http_get_fully_decode_reserved_expansion(::google::api::Http* msg) { return msg->fully_decode_reserved_expansion(); }
void __rust_proto_thunk__google_api_Http_set_fully_decode_reserved_expansion(::google::api::Http* msg, bool val) {
  msg->set_fully_decode_reserved_expansion(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::google::api::Http>* __rust_proto_thunk__Map_i32_google_api_Http_new() {
    return new google::protobuf::Map<int32_t, ::google::api::Http>();
  }
  void __rust_proto_thunk__Map_i32_google_api_Http_free(const google::protobuf::Map<int32_t, ::google::api::Http>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_google_api_Http_clear(google::protobuf::Map<int32_t, ::google::api::Http> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_google_api_Http_size(const google::protobuf::Map<int32_t, ::google::api::Http>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_google_api_Http_insert(google::protobuf::Map<int32_t, ::google::api::Http> * m,
                          int32_t key, ::google::api::Http value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_google_api_Http_get(const google::protobuf::Map<int32_t, ::google::api::Http>* m,
                       int32_t key, const ::google::api::Http** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::api::Http* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_google_api_Http_remove(google::protobuf::Map<int32_t, ::google::api::Http> * m,
                          int32_t key, ::google::api::Http * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_google_api_Http_iter(
      const google::protobuf::Map<int32_t, ::google::api::Http>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_google_api_Http_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::google::api::Http** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::google::api::Http>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::google::api::Http>* __rust_proto_thunk__Map_u32_google_api_Http_new() {
    return new google::protobuf::Map<uint32_t, ::google::api::Http>();
  }
  void __rust_proto_thunk__Map_u32_google_api_Http_free(const google::protobuf::Map<uint32_t, ::google::api::Http>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_google_api_Http_clear(google::protobuf::Map<uint32_t, ::google::api::Http> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_google_api_Http_size(const google::protobuf::Map<uint32_t, ::google::api::Http>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_google_api_Http_insert(google::protobuf::Map<uint32_t, ::google::api::Http> * m,
                          uint32_t key, ::google::api::Http value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_google_api_Http_get(const google::protobuf::Map<uint32_t, ::google::api::Http>* m,
                       uint32_t key, const ::google::api::Http** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::api::Http* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_google_api_Http_remove(google::protobuf::Map<uint32_t, ::google::api::Http> * m,
                          uint32_t key, ::google::api::Http * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_google_api_Http_iter(
      const google::protobuf::Map<uint32_t, ::google::api::Http>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_google_api_Http_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::google::api::Http** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::google::api::Http>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::google::api::Http>* __rust_proto_thunk__Map_i64_google_api_Http_new() {
    return new google::protobuf::Map<int64_t, ::google::api::Http>();
  }
  void __rust_proto_thunk__Map_i64_google_api_Http_free(const google::protobuf::Map<int64_t, ::google::api::Http>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_google_api_Http_clear(google::protobuf::Map<int64_t, ::google::api::Http> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_google_api_Http_size(const google::protobuf::Map<int64_t, ::google::api::Http>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_google_api_Http_insert(google::protobuf::Map<int64_t, ::google::api::Http> * m,
                          int64_t key, ::google::api::Http value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_google_api_Http_get(const google::protobuf::Map<int64_t, ::google::api::Http>* m,
                       int64_t key, const ::google::api::Http** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::api::Http* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_google_api_Http_remove(google::protobuf::Map<int64_t, ::google::api::Http> * m,
                          int64_t key, ::google::api::Http * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_google_api_Http_iter(
      const google::protobuf::Map<int64_t, ::google::api::Http>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_google_api_Http_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::google::api::Http** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::google::api::Http>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::google::api::Http>* __rust_proto_thunk__Map_u64_google_api_Http_new() {
    return new google::protobuf::Map<uint64_t, ::google::api::Http>();
  }
  void __rust_proto_thunk__Map_u64_google_api_Http_free(const google::protobuf::Map<uint64_t, ::google::api::Http>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_google_api_Http_clear(google::protobuf::Map<uint64_t, ::google::api::Http> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_google_api_Http_size(const google::protobuf::Map<uint64_t, ::google::api::Http>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_google_api_Http_insert(google::protobuf::Map<uint64_t, ::google::api::Http> * m,
                          uint64_t key, ::google::api::Http value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_google_api_Http_get(const google::protobuf::Map<uint64_t, ::google::api::Http>* m,
                       uint64_t key, const ::google::api::Http** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::api::Http* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_google_api_Http_remove(google::protobuf::Map<uint64_t, ::google::api::Http> * m,
                          uint64_t key, ::google::api::Http * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_google_api_Http_iter(
      const google::protobuf::Map<uint64_t, ::google::api::Http>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_google_api_Http_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::google::api::Http** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::google::api::Http>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::google::api::Http>* __rust_proto_thunk__Map_bool_google_api_Http_new() {
    return new google::protobuf::Map<bool, ::google::api::Http>();
  }
  void __rust_proto_thunk__Map_bool_google_api_Http_free(const google::protobuf::Map<bool, ::google::api::Http>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_google_api_Http_clear(google::protobuf::Map<bool, ::google::api::Http> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_google_api_Http_size(const google::protobuf::Map<bool, ::google::api::Http>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_google_api_Http_insert(google::protobuf::Map<bool, ::google::api::Http> * m,
                          bool key, ::google::api::Http value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_google_api_Http_get(const google::protobuf::Map<bool, ::google::api::Http>* m,
                       bool key, const ::google::api::Http** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::api::Http* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_google_api_Http_remove(google::protobuf::Map<bool, ::google::api::Http> * m,
                          bool key, ::google::api::Http * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_google_api_Http_iter(
      const google::protobuf::Map<bool, ::google::api::Http>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_google_api_Http_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::google::api::Http** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::google::api::Http>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::google::api::Http>* __rust_proto_thunk__Map_ProtoStr_google_api_Http_new() {
    return new google::protobuf::Map<std::string, ::google::api::Http>();
  }
  void __rust_proto_thunk__Map_ProtoStr_google_api_Http_free(const google::protobuf::Map<std::string, ::google::api::Http>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_google_api_Http_clear(google::protobuf::Map<std::string, ::google::api::Http> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_google_api_Http_size(const google::protobuf::Map<std::string, ::google::api::Http>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_google_api_Http_insert(google::protobuf::Map<std::string, ::google::api::Http> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::google::api::Http value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_google_api_Http_get(const google::protobuf::Map<std::string, ::google::api::Http>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::google::api::Http** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::google::api::Http* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_google_api_Http_remove(google::protobuf::Map<std::string, ::google::api::Http> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::google::api::Http * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_google_api_Http_iter(
      const google::protobuf::Map<std::string, ::google::api::Http>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_google_api_Http_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::google::api::Http** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::google::api::Http>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// google.api.HttpRule
        // clang-format off
extern "C" {
void* __rust_proto_thunk__google_api_HttpRule_new() { return new ::google::api::HttpRule(); }
void __rust_proto_thunk__google_api_HttpRule_delete(void* ptr) { delete static_cast<::google::api::HttpRule*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__google_api_HttpRule_serialize(::google::api::HttpRule* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__google_api_HttpRule_parse(::google::api::HttpRule* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__google_api_HttpRule_copy_from(::google::api::HttpRule* dst, const ::google::api::HttpRule* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__google_api_HttpRule_repeated_len(google::protobuf::RepeatedPtrField<::google::api::HttpRule>* field) {
  return field->size();
}
const ::google::api::HttpRule& __rust_proto_thunk__google_api_HttpRule_repeated_get(
  google::protobuf::RepeatedPtrField<::google::api::HttpRule>* field,
  size_t index) {
  return field->Get(index);
}
::google::api::HttpRule* __rust_proto_thunk__google_api_HttpRule_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::google::api::HttpRule>* field,
  size_t index) {
  return field->Mutable(index);
}
::google::api::HttpRule* __rust_proto_thunk__google_api_HttpRule_repeated_add(google::protobuf::RepeatedPtrField<::google::api::HttpRule>* field) {
  return field->Add();
}
void __rust_proto_thunk__google_api_HttpRule_repeated_clear(google::protobuf::RepeatedPtrField<::google::api::HttpRule>* field) {
  field->Clear();
}
void __rust_proto_thunk__google_api_HttpRule_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::google::api::HttpRule>& dst,
  const google::protobuf::RepeatedPtrField<::google::api::HttpRule>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__google_api_HttpRule_get_selector(::google::api::HttpRule* msg) {
  absl::string_view val = msg->selector();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__google_api_HttpRule_set_selector(::google::api::HttpRule* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_selector(absl::string_view(s.ptr, s.len));
}

bool __rust_proto_thunk__google_api_HttpRule_has_get(::google::api::HttpRule* msg) {
  return msg->has_get();
}
void __rust_proto_thunk__google_api_HttpRule_clear_get(::google::api::HttpRule* msg) { msg->clear_get(); }
::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__google_api_HttpRule_get_get(::google::api::HttpRule* msg) {
  absl::string_view val = msg->get();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__google_api_HttpRule_set_get(::google::api::HttpRule* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_get(absl::string_view(s.ptr, s.len));
}

bool __rust_proto_thunk__google_api_HttpRule_has_put(::google::api::HttpRule* msg) {
  return msg->has_put();
}
void __rust_proto_thunk__google_api_HttpRule_clear_put(::google::api::HttpRule* msg) { msg->clear_put(); }
::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__google_api_HttpRule_get_put(::google::api::HttpRule* msg) {
  absl::string_view val = msg->put();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__google_api_HttpRule_set_put(::google::api::HttpRule* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_put(absl::string_view(s.ptr, s.len));
}

bool __rust_proto_thunk__google_api_HttpRule_has_post(::google::api::HttpRule* msg) {
  return msg->has_post();
}
void __rust_proto_thunk__google_api_HttpRule_clear_post(::google::api::HttpRule* msg) { msg->clear_post(); }
::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__google_api_HttpRule_get_post(::google::api::HttpRule* msg) {
  absl::string_view val = msg->post();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__google_api_HttpRule_set_post(::google::api::HttpRule* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_post(absl::string_view(s.ptr, s.len));
}

bool __rust_proto_thunk__google_api_HttpRule_has_delete(::google::api::HttpRule* msg) {
  return msg->has_delete_();
}
void __rust_proto_thunk__google_api_HttpRule_clear_delete(::google::api::HttpRule* msg) { msg->clear_delete_(); }
::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__google_api_HttpRule_get_delete(::google::api::HttpRule* msg) {
  absl::string_view val = msg->delete_();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__google_api_HttpRule_set_delete(::google::api::HttpRule* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_delete_(absl::string_view(s.ptr, s.len));
}

bool __rust_proto_thunk__google_api_HttpRule_has_patch(::google::api::HttpRule* msg) {
  return msg->has_patch();
}
void __rust_proto_thunk__google_api_HttpRule_clear_patch(::google::api::HttpRule* msg) { msg->clear_patch(); }
::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__google_api_HttpRule_get_patch(::google::api::HttpRule* msg) {
  absl::string_view val = msg->patch();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__google_api_HttpRule_set_patch(::google::api::HttpRule* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_patch(absl::string_view(s.ptr, s.len));
}

const void* __rust_proto_thunk__google_api_HttpRule_get_custom(::google::api::HttpRule* msg) {
  return static_cast<const void*>(&msg->custom());
}
void* __rust_proto_thunk__google_api_HttpRule_get_mut_custom(::google::api::HttpRule* msg) {
  return static_cast<void*>(msg->mutable_custom());
}
void __rust_proto_thunk__google_api_HttpRule_clear_custom(::google::api::HttpRule* msg) { msg->clear_custom(); }
bool __rust_proto_thunk__google_api_HttpRule_has_custom(::google::api::HttpRule* msg) { return msg->has_custom(); }

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__google_api_HttpRule_get_body(::google::api::HttpRule* msg) {
  absl::string_view val = msg->body();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__google_api_HttpRule_set_body(::google::api::HttpRule* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_body(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__google_api_HttpRule_get_response_body(::google::api::HttpRule* msg) {
  absl::string_view val = msg->response_body();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__google_api_HttpRule_set_response_body(::google::api::HttpRule* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_response_body(absl::string_view(s.ptr, s.len));
}

void __rust_proto_thunk__google_api_HttpRule_clear_additional_bindings(::google::api::HttpRule* msg) {
  msg->clear_additional_bindings();
}
google::protobuf::RepeatedPtrField<::google::api::HttpRule>* __rust_proto_thunk__google_api_HttpRule_get_mut_additional_bindings(::google::api::HttpRule* msg) {
  return msg->mutable_additional_bindings();
}
const google::protobuf::RepeatedPtrField<::google::api::HttpRule>* __rust_proto_thunk__google_api_HttpRule_get_additional_bindings(
    const ::google::api::HttpRule* msg) {
  return &msg->additional_bindings();
}


::google::api::HttpRule::PatternCase __rust_proto_thunk__google_api_HttpRule_case_pattern(::google::api::HttpRule* msg) {
  return msg->pattern_case();
}
}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::google::api::HttpRule>* __rust_proto_thunk__Map_i32_google_api_HttpRule_new() {
    return new google::protobuf::Map<int32_t, ::google::api::HttpRule>();
  }
  void __rust_proto_thunk__Map_i32_google_api_HttpRule_free(const google::protobuf::Map<int32_t, ::google::api::HttpRule>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_google_api_HttpRule_clear(google::protobuf::Map<int32_t, ::google::api::HttpRule> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_google_api_HttpRule_size(const google::protobuf::Map<int32_t, ::google::api::HttpRule>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_google_api_HttpRule_insert(google::protobuf::Map<int32_t, ::google::api::HttpRule> * m,
                          int32_t key, ::google::api::HttpRule value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_google_api_HttpRule_get(const google::protobuf::Map<int32_t, ::google::api::HttpRule>* m,
                       int32_t key, const ::google::api::HttpRule** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::api::HttpRule* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_google_api_HttpRule_remove(google::protobuf::Map<int32_t, ::google::api::HttpRule> * m,
                          int32_t key, ::google::api::HttpRule * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_google_api_HttpRule_iter(
      const google::protobuf::Map<int32_t, ::google::api::HttpRule>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_google_api_HttpRule_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::google::api::HttpRule** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::google::api::HttpRule>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::google::api::HttpRule>* __rust_proto_thunk__Map_u32_google_api_HttpRule_new() {
    return new google::protobuf::Map<uint32_t, ::google::api::HttpRule>();
  }
  void __rust_proto_thunk__Map_u32_google_api_HttpRule_free(const google::protobuf::Map<uint32_t, ::google::api::HttpRule>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_google_api_HttpRule_clear(google::protobuf::Map<uint32_t, ::google::api::HttpRule> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_google_api_HttpRule_size(const google::protobuf::Map<uint32_t, ::google::api::HttpRule>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_google_api_HttpRule_insert(google::protobuf::Map<uint32_t, ::google::api::HttpRule> * m,
                          uint32_t key, ::google::api::HttpRule value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_google_api_HttpRule_get(const google::protobuf::Map<uint32_t, ::google::api::HttpRule>* m,
                       uint32_t key, const ::google::api::HttpRule** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::api::HttpRule* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_google_api_HttpRule_remove(google::protobuf::Map<uint32_t, ::google::api::HttpRule> * m,
                          uint32_t key, ::google::api::HttpRule * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_google_api_HttpRule_iter(
      const google::protobuf::Map<uint32_t, ::google::api::HttpRule>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_google_api_HttpRule_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::google::api::HttpRule** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::google::api::HttpRule>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::google::api::HttpRule>* __rust_proto_thunk__Map_i64_google_api_HttpRule_new() {
    return new google::protobuf::Map<int64_t, ::google::api::HttpRule>();
  }
  void __rust_proto_thunk__Map_i64_google_api_HttpRule_free(const google::protobuf::Map<int64_t, ::google::api::HttpRule>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_google_api_HttpRule_clear(google::protobuf::Map<int64_t, ::google::api::HttpRule> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_google_api_HttpRule_size(const google::protobuf::Map<int64_t, ::google::api::HttpRule>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_google_api_HttpRule_insert(google::protobuf::Map<int64_t, ::google::api::HttpRule> * m,
                          int64_t key, ::google::api::HttpRule value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_google_api_HttpRule_get(const google::protobuf::Map<int64_t, ::google::api::HttpRule>* m,
                       int64_t key, const ::google::api::HttpRule** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::api::HttpRule* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_google_api_HttpRule_remove(google::protobuf::Map<int64_t, ::google::api::HttpRule> * m,
                          int64_t key, ::google::api::HttpRule * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_google_api_HttpRule_iter(
      const google::protobuf::Map<int64_t, ::google::api::HttpRule>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_google_api_HttpRule_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::google::api::HttpRule** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::google::api::HttpRule>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::google::api::HttpRule>* __rust_proto_thunk__Map_u64_google_api_HttpRule_new() {
    return new google::protobuf::Map<uint64_t, ::google::api::HttpRule>();
  }
  void __rust_proto_thunk__Map_u64_google_api_HttpRule_free(const google::protobuf::Map<uint64_t, ::google::api::HttpRule>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_google_api_HttpRule_clear(google::protobuf::Map<uint64_t, ::google::api::HttpRule> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_google_api_HttpRule_size(const google::protobuf::Map<uint64_t, ::google::api::HttpRule>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_google_api_HttpRule_insert(google::protobuf::Map<uint64_t, ::google::api::HttpRule> * m,
                          uint64_t key, ::google::api::HttpRule value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_google_api_HttpRule_get(const google::protobuf::Map<uint64_t, ::google::api::HttpRule>* m,
                       uint64_t key, const ::google::api::HttpRule** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::api::HttpRule* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_google_api_HttpRule_remove(google::protobuf::Map<uint64_t, ::google::api::HttpRule> * m,
                          uint64_t key, ::google::api::HttpRule * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_google_api_HttpRule_iter(
      const google::protobuf::Map<uint64_t, ::google::api::HttpRule>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_google_api_HttpRule_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::google::api::HttpRule** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::google::api::HttpRule>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::google::api::HttpRule>* __rust_proto_thunk__Map_bool_google_api_HttpRule_new() {
    return new google::protobuf::Map<bool, ::google::api::HttpRule>();
  }
  void __rust_proto_thunk__Map_bool_google_api_HttpRule_free(const google::protobuf::Map<bool, ::google::api::HttpRule>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_google_api_HttpRule_clear(google::protobuf::Map<bool, ::google::api::HttpRule> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_google_api_HttpRule_size(const google::protobuf::Map<bool, ::google::api::HttpRule>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_google_api_HttpRule_insert(google::protobuf::Map<bool, ::google::api::HttpRule> * m,
                          bool key, ::google::api::HttpRule value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_google_api_HttpRule_get(const google::protobuf::Map<bool, ::google::api::HttpRule>* m,
                       bool key, const ::google::api::HttpRule** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::api::HttpRule* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_google_api_HttpRule_remove(google::protobuf::Map<bool, ::google::api::HttpRule> * m,
                          bool key, ::google::api::HttpRule * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_google_api_HttpRule_iter(
      const google::protobuf::Map<bool, ::google::api::HttpRule>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_google_api_HttpRule_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::google::api::HttpRule** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::google::api::HttpRule>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::google::api::HttpRule>* __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_new() {
    return new google::protobuf::Map<std::string, ::google::api::HttpRule>();
  }
  void __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_free(const google::protobuf::Map<std::string, ::google::api::HttpRule>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_clear(google::protobuf::Map<std::string, ::google::api::HttpRule> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_size(const google::protobuf::Map<std::string, ::google::api::HttpRule>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_insert(google::protobuf::Map<std::string, ::google::api::HttpRule> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::google::api::HttpRule value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_get(const google::protobuf::Map<std::string, ::google::api::HttpRule>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::google::api::HttpRule** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::google::api::HttpRule* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_remove(google::protobuf::Map<std::string, ::google::api::HttpRule> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::google::api::HttpRule * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_iter(
      const google::protobuf::Map<std::string, ::google::api::HttpRule>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_google_api_HttpRule_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::google::api::HttpRule** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::google::api::HttpRule>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// google.api.CustomHttpPattern
        // clang-format off
extern "C" {
void* __rust_proto_thunk__google_api_CustomHttpPattern_new() { return new ::google::api::CustomHttpPattern(); }
void __rust_proto_thunk__google_api_CustomHttpPattern_delete(void* ptr) { delete static_cast<::google::api::CustomHttpPattern*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__google_api_CustomHttpPattern_serialize(::google::api::CustomHttpPattern* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__google_api_CustomHttpPattern_parse(::google::api::CustomHttpPattern* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__google_api_CustomHttpPattern_copy_from(::google::api::CustomHttpPattern* dst, const ::google::api::CustomHttpPattern* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__google_api_CustomHttpPattern_repeated_len(google::protobuf::RepeatedPtrField<::google::api::CustomHttpPattern>* field) {
  return field->size();
}
const ::google::api::CustomHttpPattern& __rust_proto_thunk__google_api_CustomHttpPattern_repeated_get(
  google::protobuf::RepeatedPtrField<::google::api::CustomHttpPattern>* field,
  size_t index) {
  return field->Get(index);
}
::google::api::CustomHttpPattern* __rust_proto_thunk__google_api_CustomHttpPattern_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::google::api::CustomHttpPattern>* field,
  size_t index) {
  return field->Mutable(index);
}
::google::api::CustomHttpPattern* __rust_proto_thunk__google_api_CustomHttpPattern_repeated_add(google::protobuf::RepeatedPtrField<::google::api::CustomHttpPattern>* field) {
  return field->Add();
}
void __rust_proto_thunk__google_api_CustomHttpPattern_repeated_clear(google::protobuf::RepeatedPtrField<::google::api::CustomHttpPattern>* field) {
  field->Clear();
}
void __rust_proto_thunk__google_api_CustomHttpPattern_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::google::api::CustomHttpPattern>& dst,
  const google::protobuf::RepeatedPtrField<::google::api::CustomHttpPattern>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__google_api_CustomHttpPattern_get_kind(::google::api::CustomHttpPattern* msg) {
  absl::string_view val = msg->kind();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__google_api_CustomHttpPattern_set_kind(::google::api::CustomHttpPattern* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_kind(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__google_api_CustomHttpPattern_get_path(::google::api::CustomHttpPattern* msg) {
  absl::string_view val = msg->path();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__google_api_CustomHttpPattern_set_path(::google::api::CustomHttpPattern* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_path(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::google::api::CustomHttpPattern>* __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_new() {
    return new google::protobuf::Map<int32_t, ::google::api::CustomHttpPattern>();
  }
  void __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_free(const google::protobuf::Map<int32_t, ::google::api::CustomHttpPattern>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_clear(google::protobuf::Map<int32_t, ::google::api::CustomHttpPattern> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_size(const google::protobuf::Map<int32_t, ::google::api::CustomHttpPattern>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_insert(google::protobuf::Map<int32_t, ::google::api::CustomHttpPattern> * m,
                          int32_t key, ::google::api::CustomHttpPattern value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_get(const google::protobuf::Map<int32_t, ::google::api::CustomHttpPattern>* m,
                       int32_t key, const ::google::api::CustomHttpPattern** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::api::CustomHttpPattern* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_remove(google::protobuf::Map<int32_t, ::google::api::CustomHttpPattern> * m,
                          int32_t key, ::google::api::CustomHttpPattern * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_iter(
      const google::protobuf::Map<int32_t, ::google::api::CustomHttpPattern>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_google_api_CustomHttpPattern_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::google::api::CustomHttpPattern** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::google::api::CustomHttpPattern>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::google::api::CustomHttpPattern>* __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_new() {
    return new google::protobuf::Map<uint32_t, ::google::api::CustomHttpPattern>();
  }
  void __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_free(const google::protobuf::Map<uint32_t, ::google::api::CustomHttpPattern>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_clear(google::protobuf::Map<uint32_t, ::google::api::CustomHttpPattern> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_size(const google::protobuf::Map<uint32_t, ::google::api::CustomHttpPattern>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_insert(google::protobuf::Map<uint32_t, ::google::api::CustomHttpPattern> * m,
                          uint32_t key, ::google::api::CustomHttpPattern value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_get(const google::protobuf::Map<uint32_t, ::google::api::CustomHttpPattern>* m,
                       uint32_t key, const ::google::api::CustomHttpPattern** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::api::CustomHttpPattern* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_remove(google::protobuf::Map<uint32_t, ::google::api::CustomHttpPattern> * m,
                          uint32_t key, ::google::api::CustomHttpPattern * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_iter(
      const google::protobuf::Map<uint32_t, ::google::api::CustomHttpPattern>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_google_api_CustomHttpPattern_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::google::api::CustomHttpPattern** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::google::api::CustomHttpPattern>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::google::api::CustomHttpPattern>* __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_new() {
    return new google::protobuf::Map<int64_t, ::google::api::CustomHttpPattern>();
  }
  void __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_free(const google::protobuf::Map<int64_t, ::google::api::CustomHttpPattern>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_clear(google::protobuf::Map<int64_t, ::google::api::CustomHttpPattern> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_size(const google::protobuf::Map<int64_t, ::google::api::CustomHttpPattern>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_insert(google::protobuf::Map<int64_t, ::google::api::CustomHttpPattern> * m,
                          int64_t key, ::google::api::CustomHttpPattern value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_get(const google::protobuf::Map<int64_t, ::google::api::CustomHttpPattern>* m,
                       int64_t key, const ::google::api::CustomHttpPattern** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::api::CustomHttpPattern* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_remove(google::protobuf::Map<int64_t, ::google::api::CustomHttpPattern> * m,
                          int64_t key, ::google::api::CustomHttpPattern * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_iter(
      const google::protobuf::Map<int64_t, ::google::api::CustomHttpPattern>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_google_api_CustomHttpPattern_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::google::api::CustomHttpPattern** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::google::api::CustomHttpPattern>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::google::api::CustomHttpPattern>* __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_new() {
    return new google::protobuf::Map<uint64_t, ::google::api::CustomHttpPattern>();
  }
  void __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_free(const google::protobuf::Map<uint64_t, ::google::api::CustomHttpPattern>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_clear(google::protobuf::Map<uint64_t, ::google::api::CustomHttpPattern> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_size(const google::protobuf::Map<uint64_t, ::google::api::CustomHttpPattern>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_insert(google::protobuf::Map<uint64_t, ::google::api::CustomHttpPattern> * m,
                          uint64_t key, ::google::api::CustomHttpPattern value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_get(const google::protobuf::Map<uint64_t, ::google::api::CustomHttpPattern>* m,
                       uint64_t key, const ::google::api::CustomHttpPattern** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::api::CustomHttpPattern* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_remove(google::protobuf::Map<uint64_t, ::google::api::CustomHttpPattern> * m,
                          uint64_t key, ::google::api::CustomHttpPattern * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_iter(
      const google::protobuf::Map<uint64_t, ::google::api::CustomHttpPattern>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_google_api_CustomHttpPattern_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::google::api::CustomHttpPattern** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::google::api::CustomHttpPattern>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::google::api::CustomHttpPattern>* __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_new() {
    return new google::protobuf::Map<bool, ::google::api::CustomHttpPattern>();
  }
  void __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_free(const google::protobuf::Map<bool, ::google::api::CustomHttpPattern>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_clear(google::protobuf::Map<bool, ::google::api::CustomHttpPattern> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_size(const google::protobuf::Map<bool, ::google::api::CustomHttpPattern>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_insert(google::protobuf::Map<bool, ::google::api::CustomHttpPattern> * m,
                          bool key, ::google::api::CustomHttpPattern value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_get(const google::protobuf::Map<bool, ::google::api::CustomHttpPattern>* m,
                       bool key, const ::google::api::CustomHttpPattern** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::google::api::CustomHttpPattern* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_remove(google::protobuf::Map<bool, ::google::api::CustomHttpPattern> * m,
                          bool key, ::google::api::CustomHttpPattern * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_iter(
      const google::protobuf::Map<bool, ::google::api::CustomHttpPattern>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_google_api_CustomHttpPattern_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::google::api::CustomHttpPattern** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::google::api::CustomHttpPattern>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::google::api::CustomHttpPattern>* __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_new() {
    return new google::protobuf::Map<std::string, ::google::api::CustomHttpPattern>();
  }
  void __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_free(const google::protobuf::Map<std::string, ::google::api::CustomHttpPattern>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_clear(google::protobuf::Map<std::string, ::google::api::CustomHttpPattern> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_size(const google::protobuf::Map<std::string, ::google::api::CustomHttpPattern>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_insert(google::protobuf::Map<std::string, ::google::api::CustomHttpPattern> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::google::api::CustomHttpPattern value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_get(const google::protobuf::Map<std::string, ::google::api::CustomHttpPattern>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::google::api::CustomHttpPattern** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::google::api::CustomHttpPattern* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_remove(google::protobuf::Map<std::string, ::google::api::CustomHttpPattern> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::google::api::CustomHttpPattern * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_iter(
      const google::protobuf::Map<std::string, ::google::api::CustomHttpPattern>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_google_api_CustomHttpPattern_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::google::api::CustomHttpPattern** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::google::api::CustomHttpPattern>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

