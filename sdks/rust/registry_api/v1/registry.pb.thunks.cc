#include "registry_api/v1/registry.proto.h"
#include "google/protobuf/rust/cpp_kernel/cpp_api.h"
// registry_api.v1.CreateRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_CreateRequest_new() { return new ::registry_api::v1::CreateRequest(); }
void __rust_proto_thunk__registry_api_v1_CreateRequest_delete(void* ptr) { delete static_cast<::registry_api::v1::CreateRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_CreateRequest_serialize(::registry_api::v1::CreateRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_CreateRequest_parse(::registry_api::v1::CreateRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_CreateRequest_copy_from(::registry_api::v1::CreateRequest* dst, const ::registry_api::v1::CreateRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_CreateRequest_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::CreateRequest>* field) {
  return field->size();
}
const ::registry_api::v1::CreateRequest& __rust_proto_thunk__registry_api_v1_CreateRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::CreateRequest>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::CreateRequest* __rust_proto_thunk__registry_api_v1_CreateRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::CreateRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::CreateRequest* __rust_proto_thunk__registry_api_v1_CreateRequest_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::CreateRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_CreateRequest_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::CreateRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_CreateRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::CreateRequest>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::CreateRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_CreateRequest_get_did(::registry_api::v1::CreateRequest* msg) {
  absl::string_view val = msg->did();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_CreateRequest_set_did(::registry_api::v1::CreateRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_did(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_CreateRequest_get_document(::registry_api::v1::CreateRequest* msg) {
  absl::string_view val = msg->document();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_CreateRequest_set_document(::registry_api::v1::CreateRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_document(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::CreateRequest>* __rust_proto_thunk__Map_i32_registry_api_v1_CreateRequest_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::CreateRequest>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_CreateRequest_free(const google::protobuf::Map<int32_t, ::registry_api::v1::CreateRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_CreateRequest_clear(google::protobuf::Map<int32_t, ::registry_api::v1::CreateRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_CreateRequest_size(const google::protobuf::Map<int32_t, ::registry_api::v1::CreateRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_CreateRequest_insert(google::protobuf::Map<int32_t, ::registry_api::v1::CreateRequest> * m,
                          int32_t key, ::registry_api::v1::CreateRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_CreateRequest_get(const google::protobuf::Map<int32_t, ::registry_api::v1::CreateRequest>* m,
                       int32_t key, const ::registry_api::v1::CreateRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_CreateRequest_remove(google::protobuf::Map<int32_t, ::registry_api::v1::CreateRequest> * m,
                          int32_t key, ::registry_api::v1::CreateRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_CreateRequest_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::CreateRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_CreateRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::CreateRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::CreateRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateRequest>* __rust_proto_thunk__Map_u32_registry_api_v1_CreateRequest_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::CreateRequest>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_CreateRequest_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_CreateRequest_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::CreateRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_CreateRequest_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_CreateRequest_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::CreateRequest> * m,
                          uint32_t key, ::registry_api::v1::CreateRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_CreateRequest_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateRequest>* m,
                       uint32_t key, const ::registry_api::v1::CreateRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_CreateRequest_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::CreateRequest> * m,
                          uint32_t key, ::registry_api::v1::CreateRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_CreateRequest_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_CreateRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::CreateRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::CreateRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::CreateRequest>* __rust_proto_thunk__Map_i64_registry_api_v1_CreateRequest_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::CreateRequest>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_CreateRequest_free(const google::protobuf::Map<int64_t, ::registry_api::v1::CreateRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_CreateRequest_clear(google::protobuf::Map<int64_t, ::registry_api::v1::CreateRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_CreateRequest_size(const google::protobuf::Map<int64_t, ::registry_api::v1::CreateRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_CreateRequest_insert(google::protobuf::Map<int64_t, ::registry_api::v1::CreateRequest> * m,
                          int64_t key, ::registry_api::v1::CreateRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_CreateRequest_get(const google::protobuf::Map<int64_t, ::registry_api::v1::CreateRequest>* m,
                       int64_t key, const ::registry_api::v1::CreateRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_CreateRequest_remove(google::protobuf::Map<int64_t, ::registry_api::v1::CreateRequest> * m,
                          int64_t key, ::registry_api::v1::CreateRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_CreateRequest_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::CreateRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_CreateRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::CreateRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::CreateRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateRequest>* __rust_proto_thunk__Map_u64_registry_api_v1_CreateRequest_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::CreateRequest>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_CreateRequest_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_CreateRequest_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::CreateRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_CreateRequest_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_CreateRequest_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::CreateRequest> * m,
                          uint64_t key, ::registry_api::v1::CreateRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_CreateRequest_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateRequest>* m,
                       uint64_t key, const ::registry_api::v1::CreateRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_CreateRequest_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::CreateRequest> * m,
                          uint64_t key, ::registry_api::v1::CreateRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_CreateRequest_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_CreateRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::CreateRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::CreateRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::CreateRequest>* __rust_proto_thunk__Map_bool_registry_api_v1_CreateRequest_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::CreateRequest>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_CreateRequest_free(const google::protobuf::Map<bool, ::registry_api::v1::CreateRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_CreateRequest_clear(google::protobuf::Map<bool, ::registry_api::v1::CreateRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_CreateRequest_size(const google::protobuf::Map<bool, ::registry_api::v1::CreateRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_CreateRequest_insert(google::protobuf::Map<bool, ::registry_api::v1::CreateRequest> * m,
                          bool key, ::registry_api::v1::CreateRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_CreateRequest_get(const google::protobuf::Map<bool, ::registry_api::v1::CreateRequest>* m,
                       bool key, const ::registry_api::v1::CreateRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_CreateRequest_remove(google::protobuf::Map<bool, ::registry_api::v1::CreateRequest> * m,
                          bool key, ::registry_api::v1::CreateRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_CreateRequest_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::CreateRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_CreateRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::CreateRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::CreateRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::CreateRequest>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateRequest_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::CreateRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateRequest_free(const google::protobuf::Map<std::string, ::registry_api::v1::CreateRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateRequest_clear(google::protobuf::Map<std::string, ::registry_api::v1::CreateRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateRequest_size(const google::protobuf::Map<std::string, ::registry_api::v1::CreateRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateRequest_insert(google::protobuf::Map<std::string, ::registry_api::v1::CreateRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::CreateRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateRequest_get(const google::protobuf::Map<std::string, ::registry_api::v1::CreateRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::CreateRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateRequest_remove(google::protobuf::Map<std::string, ::registry_api::v1::CreateRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::CreateRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateRequest_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::CreateRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::CreateRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::CreateRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.CreateResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_CreateResponse_new() { return new ::registry_api::v1::CreateResponse(); }
void __rust_proto_thunk__registry_api_v1_CreateResponse_delete(void* ptr) { delete static_cast<::registry_api::v1::CreateResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_CreateResponse_serialize(::registry_api::v1::CreateResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_CreateResponse_parse(::registry_api::v1::CreateResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_CreateResponse_copy_from(::registry_api::v1::CreateResponse* dst, const ::registry_api::v1::CreateResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_CreateResponse_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::CreateResponse>* field) {
  return field->size();
}
const ::registry_api::v1::CreateResponse& __rust_proto_thunk__registry_api_v1_CreateResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::CreateResponse>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::CreateResponse* __rust_proto_thunk__registry_api_v1_CreateResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::CreateResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::CreateResponse* __rust_proto_thunk__registry_api_v1_CreateResponse_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::CreateResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_CreateResponse_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::CreateResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_CreateResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::CreateResponse>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::CreateResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::CreateResponse>* __rust_proto_thunk__Map_i32_registry_api_v1_CreateResponse_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::CreateResponse>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_CreateResponse_free(const google::protobuf::Map<int32_t, ::registry_api::v1::CreateResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_CreateResponse_clear(google::protobuf::Map<int32_t, ::registry_api::v1::CreateResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_CreateResponse_size(const google::protobuf::Map<int32_t, ::registry_api::v1::CreateResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_CreateResponse_insert(google::protobuf::Map<int32_t, ::registry_api::v1::CreateResponse> * m,
                          int32_t key, ::registry_api::v1::CreateResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_CreateResponse_get(const google::protobuf::Map<int32_t, ::registry_api::v1::CreateResponse>* m,
                       int32_t key, const ::registry_api::v1::CreateResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_CreateResponse_remove(google::protobuf::Map<int32_t, ::registry_api::v1::CreateResponse> * m,
                          int32_t key, ::registry_api::v1::CreateResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_CreateResponse_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::CreateResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_CreateResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::CreateResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::CreateResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateResponse>* __rust_proto_thunk__Map_u32_registry_api_v1_CreateResponse_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::CreateResponse>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_CreateResponse_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_CreateResponse_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::CreateResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_CreateResponse_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_CreateResponse_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::CreateResponse> * m,
                          uint32_t key, ::registry_api::v1::CreateResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_CreateResponse_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateResponse>* m,
                       uint32_t key, const ::registry_api::v1::CreateResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_CreateResponse_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::CreateResponse> * m,
                          uint32_t key, ::registry_api::v1::CreateResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_CreateResponse_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_CreateResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::CreateResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::CreateResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::CreateResponse>* __rust_proto_thunk__Map_i64_registry_api_v1_CreateResponse_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::CreateResponse>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_CreateResponse_free(const google::protobuf::Map<int64_t, ::registry_api::v1::CreateResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_CreateResponse_clear(google::protobuf::Map<int64_t, ::registry_api::v1::CreateResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_CreateResponse_size(const google::protobuf::Map<int64_t, ::registry_api::v1::CreateResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_CreateResponse_insert(google::protobuf::Map<int64_t, ::registry_api::v1::CreateResponse> * m,
                          int64_t key, ::registry_api::v1::CreateResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_CreateResponse_get(const google::protobuf::Map<int64_t, ::registry_api::v1::CreateResponse>* m,
                       int64_t key, const ::registry_api::v1::CreateResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_CreateResponse_remove(google::protobuf::Map<int64_t, ::registry_api::v1::CreateResponse> * m,
                          int64_t key, ::registry_api::v1::CreateResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_CreateResponse_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::CreateResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_CreateResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::CreateResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::CreateResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateResponse>* __rust_proto_thunk__Map_u64_registry_api_v1_CreateResponse_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::CreateResponse>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_CreateResponse_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_CreateResponse_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::CreateResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_CreateResponse_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_CreateResponse_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::CreateResponse> * m,
                          uint64_t key, ::registry_api::v1::CreateResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_CreateResponse_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateResponse>* m,
                       uint64_t key, const ::registry_api::v1::CreateResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_CreateResponse_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::CreateResponse> * m,
                          uint64_t key, ::registry_api::v1::CreateResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_CreateResponse_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_CreateResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::CreateResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::CreateResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::CreateResponse>* __rust_proto_thunk__Map_bool_registry_api_v1_CreateResponse_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::CreateResponse>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_CreateResponse_free(const google::protobuf::Map<bool, ::registry_api::v1::CreateResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_CreateResponse_clear(google::protobuf::Map<bool, ::registry_api::v1::CreateResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_CreateResponse_size(const google::protobuf::Map<bool, ::registry_api::v1::CreateResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_CreateResponse_insert(google::protobuf::Map<bool, ::registry_api::v1::CreateResponse> * m,
                          bool key, ::registry_api::v1::CreateResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_CreateResponse_get(const google::protobuf::Map<bool, ::registry_api::v1::CreateResponse>* m,
                       bool key, const ::registry_api::v1::CreateResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_CreateResponse_remove(google::protobuf::Map<bool, ::registry_api::v1::CreateResponse> * m,
                          bool key, ::registry_api::v1::CreateResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_CreateResponse_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::CreateResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_CreateResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::CreateResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::CreateResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::CreateResponse>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateResponse_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::CreateResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateResponse_free(const google::protobuf::Map<std::string, ::registry_api::v1::CreateResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateResponse_clear(google::protobuf::Map<std::string, ::registry_api::v1::CreateResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateResponse_size(const google::protobuf::Map<std::string, ::registry_api::v1::CreateResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateResponse_insert(google::protobuf::Map<std::string, ::registry_api::v1::CreateResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::CreateResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateResponse_get(const google::protobuf::Map<std::string, ::registry_api::v1::CreateResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::CreateResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateResponse_remove(google::protobuf::Map<std::string, ::registry_api::v1::CreateResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::CreateResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateResponse_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::CreateResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::CreateResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::CreateResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.ResolveRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_ResolveRequest_new() { return new ::registry_api::v1::ResolveRequest(); }
void __rust_proto_thunk__registry_api_v1_ResolveRequest_delete(void* ptr) { delete static_cast<::registry_api::v1::ResolveRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_ResolveRequest_serialize(::registry_api::v1::ResolveRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_ResolveRequest_parse(::registry_api::v1::ResolveRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_ResolveRequest_copy_from(::registry_api::v1::ResolveRequest* dst, const ::registry_api::v1::ResolveRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_ResolveRequest_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveRequest>* field) {
  return field->size();
}
const ::registry_api::v1::ResolveRequest& __rust_proto_thunk__registry_api_v1_ResolveRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveRequest>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::ResolveRequest* __rust_proto_thunk__registry_api_v1_ResolveRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::ResolveRequest* __rust_proto_thunk__registry_api_v1_ResolveRequest_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_ResolveRequest_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_ResolveRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveRequest>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_ResolveRequest_get_did(::registry_api::v1::ResolveRequest* msg) {
  absl::string_view val = msg->did();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_ResolveRequest_set_did(::registry_api::v1::ResolveRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_did(absl::string_view(s.ptr, s.len));
}

const void* __rust_proto_thunk__registry_api_v1_ResolveRequest_get_resolutionOption(::registry_api::v1::ResolveRequest* msg) {
  return static_cast<const void*>(&msg->resolutionoption());
}
void* __rust_proto_thunk__registry_api_v1_ResolveRequest_get_mut_resolutionOption(::registry_api::v1::ResolveRequest* msg) {
  return static_cast<void*>(msg->mutable_resolutionoption());
}
void __rust_proto_thunk__registry_api_v1_ResolveRequest_clear_resolutionOption(::registry_api::v1::ResolveRequest* msg) { msg->clear_resolutionoption(); }
bool __rust_proto_thunk__registry_api_v1_ResolveRequest_has_resolutionOption(::registry_api::v1::ResolveRequest* msg) { return msg->has_resolutionoption(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRequest>* __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRequest_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRequest>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRequest_free(const google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRequest_clear(google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRequest_size(const google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRequest_insert(google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRequest> * m,
                          int32_t key, ::registry_api::v1::ResolveRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRequest_get(const google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRequest>* m,
                       int32_t key, const ::registry_api::v1::ResolveRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRequest_remove(google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRequest> * m,
                          int32_t key, ::registry_api::v1::ResolveRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRequest_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::ResolveRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRequest>* __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRequest_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRequest>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRequest_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRequest_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRequest_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRequest_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRequest> * m,
                          uint32_t key, ::registry_api::v1::ResolveRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRequest_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRequest>* m,
                       uint32_t key, const ::registry_api::v1::ResolveRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRequest_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRequest> * m,
                          uint32_t key, ::registry_api::v1::ResolveRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRequest_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::ResolveRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRequest>* __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRequest_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRequest>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRequest_free(const google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRequest_clear(google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRequest_size(const google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRequest_insert(google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRequest> * m,
                          int64_t key, ::registry_api::v1::ResolveRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRequest_get(const google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRequest>* m,
                       int64_t key, const ::registry_api::v1::ResolveRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRequest_remove(google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRequest> * m,
                          int64_t key, ::registry_api::v1::ResolveRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRequest_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::ResolveRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRequest>* __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRequest_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRequest>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRequest_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRequest_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRequest_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRequest_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRequest> * m,
                          uint64_t key, ::registry_api::v1::ResolveRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRequest_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRequest>* m,
                       uint64_t key, const ::registry_api::v1::ResolveRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRequest_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRequest> * m,
                          uint64_t key, ::registry_api::v1::ResolveRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRequest_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::ResolveRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::ResolveRequest>* __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRequest_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::ResolveRequest>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRequest_free(const google::protobuf::Map<bool, ::registry_api::v1::ResolveRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRequest_clear(google::protobuf::Map<bool, ::registry_api::v1::ResolveRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRequest_size(const google::protobuf::Map<bool, ::registry_api::v1::ResolveRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRequest_insert(google::protobuf::Map<bool, ::registry_api::v1::ResolveRequest> * m,
                          bool key, ::registry_api::v1::ResolveRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRequest_get(const google::protobuf::Map<bool, ::registry_api::v1::ResolveRequest>* m,
                       bool key, const ::registry_api::v1::ResolveRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRequest_remove(google::protobuf::Map<bool, ::registry_api::v1::ResolveRequest> * m,
                          bool key, ::registry_api::v1::ResolveRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRequest_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::ResolveRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::ResolveRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::ResolveRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::ResolveRequest>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRequest_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::ResolveRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRequest_free(const google::protobuf::Map<std::string, ::registry_api::v1::ResolveRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRequest_clear(google::protobuf::Map<std::string, ::registry_api::v1::ResolveRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRequest_size(const google::protobuf::Map<std::string, ::registry_api::v1::ResolveRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRequest_insert(google::protobuf::Map<std::string, ::registry_api::v1::ResolveRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::ResolveRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRequest_get(const google::protobuf::Map<std::string, ::registry_api::v1::ResolveRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::ResolveRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRequest_remove(google::protobuf::Map<std::string, ::registry_api::v1::ResolveRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::ResolveRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRequest_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::ResolveRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::ResolveRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::ResolveRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.ResolveRepresentationRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_ResolveRepresentationRequest_new() { return new ::registry_api::v1::ResolveRepresentationRequest(); }
void __rust_proto_thunk__registry_api_v1_ResolveRepresentationRequest_delete(void* ptr) { delete static_cast<::registry_api::v1::ResolveRepresentationRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_ResolveRepresentationRequest_serialize(::registry_api::v1::ResolveRepresentationRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_ResolveRepresentationRequest_parse(::registry_api::v1::ResolveRepresentationRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_ResolveRepresentationRequest_copy_from(::registry_api::v1::ResolveRepresentationRequest* dst, const ::registry_api::v1::ResolveRepresentationRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_ResolveRepresentationRequest_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveRepresentationRequest>* field) {
  return field->size();
}
const ::registry_api::v1::ResolveRepresentationRequest& __rust_proto_thunk__registry_api_v1_ResolveRepresentationRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveRepresentationRequest>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::ResolveRepresentationRequest* __rust_proto_thunk__registry_api_v1_ResolveRepresentationRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveRepresentationRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::ResolveRepresentationRequest* __rust_proto_thunk__registry_api_v1_ResolveRepresentationRequest_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveRepresentationRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_ResolveRepresentationRequest_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveRepresentationRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_ResolveRepresentationRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveRepresentationRequest>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveRepresentationRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_ResolveRepresentationRequest_get_did(::registry_api::v1::ResolveRepresentationRequest* msg) {
  absl::string_view val = msg->did();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_ResolveRepresentationRequest_set_did(::registry_api::v1::ResolveRepresentationRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_did(absl::string_view(s.ptr, s.len));
}

const void* __rust_proto_thunk__registry_api_v1_ResolveRepresentationRequest_get_resolutionOption(::registry_api::v1::ResolveRepresentationRequest* msg) {
  return static_cast<const void*>(&msg->resolutionoption());
}
void* __rust_proto_thunk__registry_api_v1_ResolveRepresentationRequest_get_mut_resolutionOption(::registry_api::v1::ResolveRepresentationRequest* msg) {
  return static_cast<void*>(msg->mutable_resolutionoption());
}
void __rust_proto_thunk__registry_api_v1_ResolveRepresentationRequest_clear_resolutionOption(::registry_api::v1::ResolveRepresentationRequest* msg) { msg->clear_resolutionoption(); }
bool __rust_proto_thunk__registry_api_v1_ResolveRepresentationRequest_has_resolutionOption(::registry_api::v1::ResolveRepresentationRequest* msg) { return msg->has_resolutionoption(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRepresentationRequest>* __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRepresentationRequest_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRepresentationRequest>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRepresentationRequest_free(const google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRepresentationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRepresentationRequest_clear(google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRepresentationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRepresentationRequest_size(const google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRepresentationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRepresentationRequest_insert(google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRepresentationRequest> * m,
                          int32_t key, ::registry_api::v1::ResolveRepresentationRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRepresentationRequest_get(const google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRepresentationRequest>* m,
                       int32_t key, const ::registry_api::v1::ResolveRepresentationRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveRepresentationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRepresentationRequest_remove(google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRepresentationRequest> * m,
                          int32_t key, ::registry_api::v1::ResolveRepresentationRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRepresentationRequest_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRepresentationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRepresentationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::ResolveRepresentationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRepresentationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRepresentationRequest>* __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRepresentationRequest_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRepresentationRequest>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRepresentationRequest_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRepresentationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRepresentationRequest_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRepresentationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRepresentationRequest_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRepresentationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRepresentationRequest_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRepresentationRequest> * m,
                          uint32_t key, ::registry_api::v1::ResolveRepresentationRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRepresentationRequest_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRepresentationRequest>* m,
                       uint32_t key, const ::registry_api::v1::ResolveRepresentationRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveRepresentationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRepresentationRequest_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRepresentationRequest> * m,
                          uint32_t key, ::registry_api::v1::ResolveRepresentationRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRepresentationRequest_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRepresentationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRepresentationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::ResolveRepresentationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRepresentationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRepresentationRequest>* __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRepresentationRequest_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRepresentationRequest>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRepresentationRequest_free(const google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRepresentationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRepresentationRequest_clear(google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRepresentationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRepresentationRequest_size(const google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRepresentationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRepresentationRequest_insert(google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRepresentationRequest> * m,
                          int64_t key, ::registry_api::v1::ResolveRepresentationRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRepresentationRequest_get(const google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRepresentationRequest>* m,
                       int64_t key, const ::registry_api::v1::ResolveRepresentationRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveRepresentationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRepresentationRequest_remove(google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRepresentationRequest> * m,
                          int64_t key, ::registry_api::v1::ResolveRepresentationRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRepresentationRequest_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRepresentationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRepresentationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::ResolveRepresentationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRepresentationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRepresentationRequest>* __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRepresentationRequest_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRepresentationRequest>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRepresentationRequest_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRepresentationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRepresentationRequest_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRepresentationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRepresentationRequest_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRepresentationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRepresentationRequest_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRepresentationRequest> * m,
                          uint64_t key, ::registry_api::v1::ResolveRepresentationRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRepresentationRequest_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRepresentationRequest>* m,
                       uint64_t key, const ::registry_api::v1::ResolveRepresentationRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveRepresentationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRepresentationRequest_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRepresentationRequest> * m,
                          uint64_t key, ::registry_api::v1::ResolveRepresentationRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRepresentationRequest_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRepresentationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRepresentationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::ResolveRepresentationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRepresentationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::ResolveRepresentationRequest>* __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRepresentationRequest_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::ResolveRepresentationRequest>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRepresentationRequest_free(const google::protobuf::Map<bool, ::registry_api::v1::ResolveRepresentationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRepresentationRequest_clear(google::protobuf::Map<bool, ::registry_api::v1::ResolveRepresentationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRepresentationRequest_size(const google::protobuf::Map<bool, ::registry_api::v1::ResolveRepresentationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRepresentationRequest_insert(google::protobuf::Map<bool, ::registry_api::v1::ResolveRepresentationRequest> * m,
                          bool key, ::registry_api::v1::ResolveRepresentationRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRepresentationRequest_get(const google::protobuf::Map<bool, ::registry_api::v1::ResolveRepresentationRequest>* m,
                       bool key, const ::registry_api::v1::ResolveRepresentationRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveRepresentationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRepresentationRequest_remove(google::protobuf::Map<bool, ::registry_api::v1::ResolveRepresentationRequest> * m,
                          bool key, ::registry_api::v1::ResolveRepresentationRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRepresentationRequest_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::ResolveRepresentationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRepresentationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::ResolveRepresentationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::ResolveRepresentationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::ResolveRepresentationRequest>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRepresentationRequest_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::ResolveRepresentationRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRepresentationRequest_free(const google::protobuf::Map<std::string, ::registry_api::v1::ResolveRepresentationRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRepresentationRequest_clear(google::protobuf::Map<std::string, ::registry_api::v1::ResolveRepresentationRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRepresentationRequest_size(const google::protobuf::Map<std::string, ::registry_api::v1::ResolveRepresentationRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRepresentationRequest_insert(google::protobuf::Map<std::string, ::registry_api::v1::ResolveRepresentationRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::ResolveRepresentationRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRepresentationRequest_get(const google::protobuf::Map<std::string, ::registry_api::v1::ResolveRepresentationRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::ResolveRepresentationRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveRepresentationRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRepresentationRequest_remove(google::protobuf::Map<std::string, ::registry_api::v1::ResolveRepresentationRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::ResolveRepresentationRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRepresentationRequest_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::ResolveRepresentationRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRepresentationRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::ResolveRepresentationRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::ResolveRepresentationRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.ResolutionOption
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_ResolutionOption_new() { return new ::registry_api::v1::ResolutionOption(); }
void __rust_proto_thunk__registry_api_v1_ResolutionOption_delete(void* ptr) { delete static_cast<::registry_api::v1::ResolutionOption*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_ResolutionOption_serialize(::registry_api::v1::ResolutionOption* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_ResolutionOption_parse(::registry_api::v1::ResolutionOption* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_ResolutionOption_copy_from(::registry_api::v1::ResolutionOption* dst, const ::registry_api::v1::ResolutionOption* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_ResolutionOption_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::ResolutionOption>* field) {
  return field->size();
}
const ::registry_api::v1::ResolutionOption& __rust_proto_thunk__registry_api_v1_ResolutionOption_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::ResolutionOption>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::ResolutionOption* __rust_proto_thunk__registry_api_v1_ResolutionOption_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::ResolutionOption>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::ResolutionOption* __rust_proto_thunk__registry_api_v1_ResolutionOption_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::ResolutionOption>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_ResolutionOption_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::ResolutionOption>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_ResolutionOption_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::ResolutionOption>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::ResolutionOption>& src) {
  dst = src;
}

::registry_api::v1::ResolutionOptionAccept __rust_proto_thunk__registry_api_v1_ResolutionOption_get_accept(::registry_api::v1::ResolutionOption* msg) { return msg->accept(); }
void __rust_proto_thunk__registry_api_v1_ResolutionOption_set_accept(::registry_api::v1::ResolutionOption* msg, ::registry_api::v1::ResolutionOptionAccept val) {
  msg->set_accept(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionOption>* __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionOption_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionOption>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionOption_free(const google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionOption>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionOption_clear(google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionOption> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionOption_size(const google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionOption>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionOption_insert(google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionOption> * m,
                          int32_t key, ::registry_api::v1::ResolutionOption value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionOption_get(const google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionOption>* m,
                       int32_t key, const ::registry_api::v1::ResolutionOption** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolutionOption* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionOption_remove(google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionOption> * m,
                          int32_t key, ::registry_api::v1::ResolutionOption * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionOption_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionOption>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionOption_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::ResolutionOption** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionOption>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionOption>* __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionOption_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionOption>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionOption_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionOption>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionOption_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionOption> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionOption_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionOption>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionOption_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionOption> * m,
                          uint32_t key, ::registry_api::v1::ResolutionOption value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionOption_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionOption>* m,
                       uint32_t key, const ::registry_api::v1::ResolutionOption** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolutionOption* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionOption_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionOption> * m,
                          uint32_t key, ::registry_api::v1::ResolutionOption * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionOption_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionOption>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionOption_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::ResolutionOption** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionOption>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionOption>* __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionOption_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionOption>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionOption_free(const google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionOption>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionOption_clear(google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionOption> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionOption_size(const google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionOption>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionOption_insert(google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionOption> * m,
                          int64_t key, ::registry_api::v1::ResolutionOption value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionOption_get(const google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionOption>* m,
                       int64_t key, const ::registry_api::v1::ResolutionOption** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolutionOption* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionOption_remove(google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionOption> * m,
                          int64_t key, ::registry_api::v1::ResolutionOption * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionOption_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionOption>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionOption_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::ResolutionOption** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionOption>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionOption>* __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionOption_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionOption>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionOption_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionOption>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionOption_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionOption> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionOption_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionOption>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionOption_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionOption> * m,
                          uint64_t key, ::registry_api::v1::ResolutionOption value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionOption_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionOption>* m,
                       uint64_t key, const ::registry_api::v1::ResolutionOption** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolutionOption* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionOption_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionOption> * m,
                          uint64_t key, ::registry_api::v1::ResolutionOption * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionOption_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionOption>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionOption_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::ResolutionOption** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionOption>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::ResolutionOption>* __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionOption_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::ResolutionOption>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionOption_free(const google::protobuf::Map<bool, ::registry_api::v1::ResolutionOption>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionOption_clear(google::protobuf::Map<bool, ::registry_api::v1::ResolutionOption> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionOption_size(const google::protobuf::Map<bool, ::registry_api::v1::ResolutionOption>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionOption_insert(google::protobuf::Map<bool, ::registry_api::v1::ResolutionOption> * m,
                          bool key, ::registry_api::v1::ResolutionOption value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionOption_get(const google::protobuf::Map<bool, ::registry_api::v1::ResolutionOption>* m,
                       bool key, const ::registry_api::v1::ResolutionOption** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolutionOption* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionOption_remove(google::protobuf::Map<bool, ::registry_api::v1::ResolutionOption> * m,
                          bool key, ::registry_api::v1::ResolutionOption * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionOption_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::ResolutionOption>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionOption_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::ResolutionOption** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::ResolutionOption>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::ResolutionOption>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionOption_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::ResolutionOption>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionOption_free(const google::protobuf::Map<std::string, ::registry_api::v1::ResolutionOption>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionOption_clear(google::protobuf::Map<std::string, ::registry_api::v1::ResolutionOption> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionOption_size(const google::protobuf::Map<std::string, ::registry_api::v1::ResolutionOption>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionOption_insert(google::protobuf::Map<std::string, ::registry_api::v1::ResolutionOption> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::ResolutionOption value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionOption_get(const google::protobuf::Map<std::string, ::registry_api::v1::ResolutionOption>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::ResolutionOption** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolutionOption* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionOption_remove(google::protobuf::Map<std::string, ::registry_api::v1::ResolutionOption> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::ResolutionOption * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionOption_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::ResolutionOption>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionOption_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::ResolutionOption** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::ResolutionOption>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.DidDocumentMetadata
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_DidDocumentMetadata_new() { return new ::registry_api::v1::DidDocumentMetadata(); }
void __rust_proto_thunk__registry_api_v1_DidDocumentMetadata_delete(void* ptr) { delete static_cast<::registry_api::v1::DidDocumentMetadata*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_DidDocumentMetadata_serialize(::registry_api::v1::DidDocumentMetadata* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_DidDocumentMetadata_parse(::registry_api::v1::DidDocumentMetadata* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_DidDocumentMetadata_copy_from(::registry_api::v1::DidDocumentMetadata* dst, const ::registry_api::v1::DidDocumentMetadata* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_DidDocumentMetadata_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::DidDocumentMetadata>* field) {
  return field->size();
}
const ::registry_api::v1::DidDocumentMetadata& __rust_proto_thunk__registry_api_v1_DidDocumentMetadata_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::DidDocumentMetadata>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::DidDocumentMetadata* __rust_proto_thunk__registry_api_v1_DidDocumentMetadata_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::DidDocumentMetadata>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::DidDocumentMetadata* __rust_proto_thunk__registry_api_v1_DidDocumentMetadata_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::DidDocumentMetadata>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_DidDocumentMetadata_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::DidDocumentMetadata>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_DidDocumentMetadata_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::DidDocumentMetadata>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::DidDocumentMetadata>& src) {
  dst = src;
}

const void* __rust_proto_thunk__registry_api_v1_DidDocumentMetadata_get_created(::registry_api::v1::DidDocumentMetadata* msg) {
  return static_cast<const void*>(&msg->created());
}
void* __rust_proto_thunk__registry_api_v1_DidDocumentMetadata_get_mut_created(::registry_api::v1::DidDocumentMetadata* msg) {
  return static_cast<void*>(msg->mutable_created());
}
void __rust_proto_thunk__registry_api_v1_DidDocumentMetadata_clear_created(::registry_api::v1::DidDocumentMetadata* msg) { msg->clear_created(); }
bool __rust_proto_thunk__registry_api_v1_DidDocumentMetadata_has_created(::registry_api::v1::DidDocumentMetadata* msg) { return msg->has_created(); }

const void* __rust_proto_thunk__registry_api_v1_DidDocumentMetadata_get_updated(::registry_api::v1::DidDocumentMetadata* msg) {
  return static_cast<const void*>(&msg->updated());
}
void* __rust_proto_thunk__registry_api_v1_DidDocumentMetadata_get_mut_updated(::registry_api::v1::DidDocumentMetadata* msg) {
  return static_cast<void*>(msg->mutable_updated());
}
void __rust_proto_thunk__registry_api_v1_DidDocumentMetadata_clear_updated(::registry_api::v1::DidDocumentMetadata* msg) { msg->clear_updated(); }
bool __rust_proto_thunk__registry_api_v1_DidDocumentMetadata_has_updated(::registry_api::v1::DidDocumentMetadata* msg) { return msg->has_updated(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::DidDocumentMetadata>* __rust_proto_thunk__Map_i32_registry_api_v1_DidDocumentMetadata_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::DidDocumentMetadata>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_DidDocumentMetadata_free(const google::protobuf::Map<int32_t, ::registry_api::v1::DidDocumentMetadata>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_DidDocumentMetadata_clear(google::protobuf::Map<int32_t, ::registry_api::v1::DidDocumentMetadata> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_DidDocumentMetadata_size(const google::protobuf::Map<int32_t, ::registry_api::v1::DidDocumentMetadata>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_DidDocumentMetadata_insert(google::protobuf::Map<int32_t, ::registry_api::v1::DidDocumentMetadata> * m,
                          int32_t key, ::registry_api::v1::DidDocumentMetadata value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_DidDocumentMetadata_get(const google::protobuf::Map<int32_t, ::registry_api::v1::DidDocumentMetadata>* m,
                       int32_t key, const ::registry_api::v1::DidDocumentMetadata** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::DidDocumentMetadata* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_DidDocumentMetadata_remove(google::protobuf::Map<int32_t, ::registry_api::v1::DidDocumentMetadata> * m,
                          int32_t key, ::registry_api::v1::DidDocumentMetadata * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_DidDocumentMetadata_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::DidDocumentMetadata>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_DidDocumentMetadata_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::DidDocumentMetadata** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::DidDocumentMetadata>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::DidDocumentMetadata>* __rust_proto_thunk__Map_u32_registry_api_v1_DidDocumentMetadata_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::DidDocumentMetadata>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_DidDocumentMetadata_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::DidDocumentMetadata>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_DidDocumentMetadata_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::DidDocumentMetadata> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_DidDocumentMetadata_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::DidDocumentMetadata>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_DidDocumentMetadata_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::DidDocumentMetadata> * m,
                          uint32_t key, ::registry_api::v1::DidDocumentMetadata value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_DidDocumentMetadata_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::DidDocumentMetadata>* m,
                       uint32_t key, const ::registry_api::v1::DidDocumentMetadata** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::DidDocumentMetadata* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_DidDocumentMetadata_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::DidDocumentMetadata> * m,
                          uint32_t key, ::registry_api::v1::DidDocumentMetadata * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_DidDocumentMetadata_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::DidDocumentMetadata>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_DidDocumentMetadata_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::DidDocumentMetadata** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::DidDocumentMetadata>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::DidDocumentMetadata>* __rust_proto_thunk__Map_i64_registry_api_v1_DidDocumentMetadata_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::DidDocumentMetadata>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_DidDocumentMetadata_free(const google::protobuf::Map<int64_t, ::registry_api::v1::DidDocumentMetadata>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_DidDocumentMetadata_clear(google::protobuf::Map<int64_t, ::registry_api::v1::DidDocumentMetadata> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_DidDocumentMetadata_size(const google::protobuf::Map<int64_t, ::registry_api::v1::DidDocumentMetadata>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_DidDocumentMetadata_insert(google::protobuf::Map<int64_t, ::registry_api::v1::DidDocumentMetadata> * m,
                          int64_t key, ::registry_api::v1::DidDocumentMetadata value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_DidDocumentMetadata_get(const google::protobuf::Map<int64_t, ::registry_api::v1::DidDocumentMetadata>* m,
                       int64_t key, const ::registry_api::v1::DidDocumentMetadata** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::DidDocumentMetadata* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_DidDocumentMetadata_remove(google::protobuf::Map<int64_t, ::registry_api::v1::DidDocumentMetadata> * m,
                          int64_t key, ::registry_api::v1::DidDocumentMetadata * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_DidDocumentMetadata_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::DidDocumentMetadata>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_DidDocumentMetadata_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::DidDocumentMetadata** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::DidDocumentMetadata>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::DidDocumentMetadata>* __rust_proto_thunk__Map_u64_registry_api_v1_DidDocumentMetadata_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::DidDocumentMetadata>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_DidDocumentMetadata_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::DidDocumentMetadata>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_DidDocumentMetadata_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::DidDocumentMetadata> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_DidDocumentMetadata_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::DidDocumentMetadata>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_DidDocumentMetadata_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::DidDocumentMetadata> * m,
                          uint64_t key, ::registry_api::v1::DidDocumentMetadata value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_DidDocumentMetadata_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::DidDocumentMetadata>* m,
                       uint64_t key, const ::registry_api::v1::DidDocumentMetadata** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::DidDocumentMetadata* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_DidDocumentMetadata_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::DidDocumentMetadata> * m,
                          uint64_t key, ::registry_api::v1::DidDocumentMetadata * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_DidDocumentMetadata_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::DidDocumentMetadata>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_DidDocumentMetadata_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::DidDocumentMetadata** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::DidDocumentMetadata>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::DidDocumentMetadata>* __rust_proto_thunk__Map_bool_registry_api_v1_DidDocumentMetadata_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::DidDocumentMetadata>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_DidDocumentMetadata_free(const google::protobuf::Map<bool, ::registry_api::v1::DidDocumentMetadata>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_DidDocumentMetadata_clear(google::protobuf::Map<bool, ::registry_api::v1::DidDocumentMetadata> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_DidDocumentMetadata_size(const google::protobuf::Map<bool, ::registry_api::v1::DidDocumentMetadata>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_DidDocumentMetadata_insert(google::protobuf::Map<bool, ::registry_api::v1::DidDocumentMetadata> * m,
                          bool key, ::registry_api::v1::DidDocumentMetadata value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_DidDocumentMetadata_get(const google::protobuf::Map<bool, ::registry_api::v1::DidDocumentMetadata>* m,
                       bool key, const ::registry_api::v1::DidDocumentMetadata** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::DidDocumentMetadata* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_DidDocumentMetadata_remove(google::protobuf::Map<bool, ::registry_api::v1::DidDocumentMetadata> * m,
                          bool key, ::registry_api::v1::DidDocumentMetadata * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_DidDocumentMetadata_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::DidDocumentMetadata>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_DidDocumentMetadata_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::DidDocumentMetadata** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::DidDocumentMetadata>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::DidDocumentMetadata>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_DidDocumentMetadata_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::DidDocumentMetadata>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_DidDocumentMetadata_free(const google::protobuf::Map<std::string, ::registry_api::v1::DidDocumentMetadata>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_DidDocumentMetadata_clear(google::protobuf::Map<std::string, ::registry_api::v1::DidDocumentMetadata> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_DidDocumentMetadata_size(const google::protobuf::Map<std::string, ::registry_api::v1::DidDocumentMetadata>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_DidDocumentMetadata_insert(google::protobuf::Map<std::string, ::registry_api::v1::DidDocumentMetadata> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::DidDocumentMetadata value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_DidDocumentMetadata_get(const google::protobuf::Map<std::string, ::registry_api::v1::DidDocumentMetadata>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::DidDocumentMetadata** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::DidDocumentMetadata* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_DidDocumentMetadata_remove(google::protobuf::Map<std::string, ::registry_api::v1::DidDocumentMetadata> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::DidDocumentMetadata * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_DidDocumentMetadata_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::DidDocumentMetadata>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_DidDocumentMetadata_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::DidDocumentMetadata** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::DidDocumentMetadata>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.ResolveResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_ResolveResponse_new() { return new ::registry_api::v1::ResolveResponse(); }
void __rust_proto_thunk__registry_api_v1_ResolveResponse_delete(void* ptr) { delete static_cast<::registry_api::v1::ResolveResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_ResolveResponse_serialize(::registry_api::v1::ResolveResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_ResolveResponse_parse(::registry_api::v1::ResolveResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_ResolveResponse_copy_from(::registry_api::v1::ResolveResponse* dst, const ::registry_api::v1::ResolveResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_ResolveResponse_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveResponse>* field) {
  return field->size();
}
const ::registry_api::v1::ResolveResponse& __rust_proto_thunk__registry_api_v1_ResolveResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveResponse>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::ResolveResponse* __rust_proto_thunk__registry_api_v1_ResolveResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::ResolveResponse* __rust_proto_thunk__registry_api_v1_ResolveResponse_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_ResolveResponse_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_ResolveResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveResponse>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveResponse>& src) {
  dst = src;
}

const void* __rust_proto_thunk__registry_api_v1_ResolveResponse_get_didResolutionMetadata(::registry_api::v1::ResolveResponse* msg) {
  return static_cast<const void*>(&msg->didresolutionmetadata());
}
void* __rust_proto_thunk__registry_api_v1_ResolveResponse_get_mut_didResolutionMetadata(::registry_api::v1::ResolveResponse* msg) {
  return static_cast<void*>(msg->mutable_didresolutionmetadata());
}
void __rust_proto_thunk__registry_api_v1_ResolveResponse_clear_didResolutionMetadata(::registry_api::v1::ResolveResponse* msg) { msg->clear_didresolutionmetadata(); }
bool __rust_proto_thunk__registry_api_v1_ResolveResponse_has_didResolutionMetadata(::registry_api::v1::ResolveResponse* msg) { return msg->has_didresolutionmetadata(); }

const void* __rust_proto_thunk__registry_api_v1_ResolveResponse_get_didDocument(::registry_api::v1::ResolveResponse* msg) {
  return static_cast<const void*>(&msg->diddocument());
}
void* __rust_proto_thunk__registry_api_v1_ResolveResponse_get_mut_didDocument(::registry_api::v1::ResolveResponse* msg) {
  return static_cast<void*>(msg->mutable_diddocument());
}
void __rust_proto_thunk__registry_api_v1_ResolveResponse_clear_didDocument(::registry_api::v1::ResolveResponse* msg) { msg->clear_diddocument(); }
bool __rust_proto_thunk__registry_api_v1_ResolveResponse_has_didDocument(::registry_api::v1::ResolveResponse* msg) { return msg->has_diddocument(); }

const void* __rust_proto_thunk__registry_api_v1_ResolveResponse_get_didDocumentMetadata(::registry_api::v1::ResolveResponse* msg) {
  return static_cast<const void*>(&msg->diddocumentmetadata());
}
void* __rust_proto_thunk__registry_api_v1_ResolveResponse_get_mut_didDocumentMetadata(::registry_api::v1::ResolveResponse* msg) {
  return static_cast<void*>(msg->mutable_diddocumentmetadata());
}
void __rust_proto_thunk__registry_api_v1_ResolveResponse_clear_didDocumentMetadata(::registry_api::v1::ResolveResponse* msg) { msg->clear_diddocumentmetadata(); }
bool __rust_proto_thunk__registry_api_v1_ResolveResponse_has_didDocumentMetadata(::registry_api::v1::ResolveResponse* msg) { return msg->has_diddocumentmetadata(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::ResolveResponse>* __rust_proto_thunk__Map_i32_registry_api_v1_ResolveResponse_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::ResolveResponse>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_ResolveResponse_free(const google::protobuf::Map<int32_t, ::registry_api::v1::ResolveResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_ResolveResponse_clear(google::protobuf::Map<int32_t, ::registry_api::v1::ResolveResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_ResolveResponse_size(const google::protobuf::Map<int32_t, ::registry_api::v1::ResolveResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_ResolveResponse_insert(google::protobuf::Map<int32_t, ::registry_api::v1::ResolveResponse> * m,
                          int32_t key, ::registry_api::v1::ResolveResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_ResolveResponse_get(const google::protobuf::Map<int32_t, ::registry_api::v1::ResolveResponse>* m,
                       int32_t key, const ::registry_api::v1::ResolveResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_ResolveResponse_remove(google::protobuf::Map<int32_t, ::registry_api::v1::ResolveResponse> * m,
                          int32_t key, ::registry_api::v1::ResolveResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_ResolveResponse_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::ResolveResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_ResolveResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::ResolveResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::ResolveResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveResponse>* __rust_proto_thunk__Map_u32_registry_api_v1_ResolveResponse_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveResponse>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_ResolveResponse_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_ResolveResponse_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_ResolveResponse_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_ResolveResponse_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveResponse> * m,
                          uint32_t key, ::registry_api::v1::ResolveResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_ResolveResponse_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveResponse>* m,
                       uint32_t key, const ::registry_api::v1::ResolveResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_ResolveResponse_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveResponse> * m,
                          uint32_t key, ::registry_api::v1::ResolveResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_ResolveResponse_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_ResolveResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::ResolveResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::ResolveResponse>* __rust_proto_thunk__Map_i64_registry_api_v1_ResolveResponse_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::ResolveResponse>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_ResolveResponse_free(const google::protobuf::Map<int64_t, ::registry_api::v1::ResolveResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_ResolveResponse_clear(google::protobuf::Map<int64_t, ::registry_api::v1::ResolveResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_ResolveResponse_size(const google::protobuf::Map<int64_t, ::registry_api::v1::ResolveResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_ResolveResponse_insert(google::protobuf::Map<int64_t, ::registry_api::v1::ResolveResponse> * m,
                          int64_t key, ::registry_api::v1::ResolveResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_ResolveResponse_get(const google::protobuf::Map<int64_t, ::registry_api::v1::ResolveResponse>* m,
                       int64_t key, const ::registry_api::v1::ResolveResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_ResolveResponse_remove(google::protobuf::Map<int64_t, ::registry_api::v1::ResolveResponse> * m,
                          int64_t key, ::registry_api::v1::ResolveResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_ResolveResponse_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::ResolveResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_ResolveResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::ResolveResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::ResolveResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveResponse>* __rust_proto_thunk__Map_u64_registry_api_v1_ResolveResponse_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveResponse>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_ResolveResponse_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_ResolveResponse_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_ResolveResponse_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_ResolveResponse_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveResponse> * m,
                          uint64_t key, ::registry_api::v1::ResolveResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_ResolveResponse_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveResponse>* m,
                       uint64_t key, const ::registry_api::v1::ResolveResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_ResolveResponse_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveResponse> * m,
                          uint64_t key, ::registry_api::v1::ResolveResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_ResolveResponse_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_ResolveResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::ResolveResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::ResolveResponse>* __rust_proto_thunk__Map_bool_registry_api_v1_ResolveResponse_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::ResolveResponse>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_ResolveResponse_free(const google::protobuf::Map<bool, ::registry_api::v1::ResolveResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_ResolveResponse_clear(google::protobuf::Map<bool, ::registry_api::v1::ResolveResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_ResolveResponse_size(const google::protobuf::Map<bool, ::registry_api::v1::ResolveResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_ResolveResponse_insert(google::protobuf::Map<bool, ::registry_api::v1::ResolveResponse> * m,
                          bool key, ::registry_api::v1::ResolveResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_ResolveResponse_get(const google::protobuf::Map<bool, ::registry_api::v1::ResolveResponse>* m,
                       bool key, const ::registry_api::v1::ResolveResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_ResolveResponse_remove(google::protobuf::Map<bool, ::registry_api::v1::ResolveResponse> * m,
                          bool key, ::registry_api::v1::ResolveResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_ResolveResponse_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::ResolveResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_ResolveResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::ResolveResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::ResolveResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::ResolveResponse>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveResponse_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::ResolveResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveResponse_free(const google::protobuf::Map<std::string, ::registry_api::v1::ResolveResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveResponse_clear(google::protobuf::Map<std::string, ::registry_api::v1::ResolveResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveResponse_size(const google::protobuf::Map<std::string, ::registry_api::v1::ResolveResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveResponse_insert(google::protobuf::Map<std::string, ::registry_api::v1::ResolveResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::ResolveResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveResponse_get(const google::protobuf::Map<std::string, ::registry_api::v1::ResolveResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::ResolveResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveResponse_remove(google::protobuf::Map<std::string, ::registry_api::v1::ResolveResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::ResolveResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveResponse_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::ResolveResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::ResolveResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::ResolveResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.ResolveRepresentationResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_ResolveRepresentationResponse_new() { return new ::registry_api::v1::ResolveRepresentationResponse(); }
void __rust_proto_thunk__registry_api_v1_ResolveRepresentationResponse_delete(void* ptr) { delete static_cast<::registry_api::v1::ResolveRepresentationResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_ResolveRepresentationResponse_serialize(::registry_api::v1::ResolveRepresentationResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_ResolveRepresentationResponse_parse(::registry_api::v1::ResolveRepresentationResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_ResolveRepresentationResponse_copy_from(::registry_api::v1::ResolveRepresentationResponse* dst, const ::registry_api::v1::ResolveRepresentationResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_ResolveRepresentationResponse_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveRepresentationResponse>* field) {
  return field->size();
}
const ::registry_api::v1::ResolveRepresentationResponse& __rust_proto_thunk__registry_api_v1_ResolveRepresentationResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveRepresentationResponse>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::ResolveRepresentationResponse* __rust_proto_thunk__registry_api_v1_ResolveRepresentationResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveRepresentationResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::ResolveRepresentationResponse* __rust_proto_thunk__registry_api_v1_ResolveRepresentationResponse_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveRepresentationResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_ResolveRepresentationResponse_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveRepresentationResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_ResolveRepresentationResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveRepresentationResponse>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::ResolveRepresentationResponse>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_ResolveRepresentationResponse_get_didDocumentStream(::registry_api::v1::ResolveRepresentationResponse* msg) {
  absl::string_view val = msg->diddocumentstream();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_ResolveRepresentationResponse_set_didDocumentStream(::registry_api::v1::ResolveRepresentationResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_diddocumentstream(absl::string_view(s.ptr, s.len));
}

const void* __rust_proto_thunk__registry_api_v1_ResolveRepresentationResponse_get_didResolutionMetadata(::registry_api::v1::ResolveRepresentationResponse* msg) {
  return static_cast<const void*>(&msg->didresolutionmetadata());
}
void* __rust_proto_thunk__registry_api_v1_ResolveRepresentationResponse_get_mut_didResolutionMetadata(::registry_api::v1::ResolveRepresentationResponse* msg) {
  return static_cast<void*>(msg->mutable_didresolutionmetadata());
}
void __rust_proto_thunk__registry_api_v1_ResolveRepresentationResponse_clear_didResolutionMetadata(::registry_api::v1::ResolveRepresentationResponse* msg) { msg->clear_didresolutionmetadata(); }
bool __rust_proto_thunk__registry_api_v1_ResolveRepresentationResponse_has_didResolutionMetadata(::registry_api::v1::ResolveRepresentationResponse* msg) { return msg->has_didresolutionmetadata(); }

const void* __rust_proto_thunk__registry_api_v1_ResolveRepresentationResponse_get_didDocumentMetadata(::registry_api::v1::ResolveRepresentationResponse* msg) {
  return static_cast<const void*>(&msg->diddocumentmetadata());
}
void* __rust_proto_thunk__registry_api_v1_ResolveRepresentationResponse_get_mut_didDocumentMetadata(::registry_api::v1::ResolveRepresentationResponse* msg) {
  return static_cast<void*>(msg->mutable_diddocumentmetadata());
}
void __rust_proto_thunk__registry_api_v1_ResolveRepresentationResponse_clear_didDocumentMetadata(::registry_api::v1::ResolveRepresentationResponse* msg) { msg->clear_diddocumentmetadata(); }
bool __rust_proto_thunk__registry_api_v1_ResolveRepresentationResponse_has_didDocumentMetadata(::registry_api::v1::ResolveRepresentationResponse* msg) { return msg->has_diddocumentmetadata(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRepresentationResponse>* __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRepresentationResponse_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRepresentationResponse>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRepresentationResponse_free(const google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRepresentationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRepresentationResponse_clear(google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRepresentationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRepresentationResponse_size(const google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRepresentationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRepresentationResponse_insert(google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRepresentationResponse> * m,
                          int32_t key, ::registry_api::v1::ResolveRepresentationResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRepresentationResponse_get(const google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRepresentationResponse>* m,
                       int32_t key, const ::registry_api::v1::ResolveRepresentationResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveRepresentationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRepresentationResponse_remove(google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRepresentationResponse> * m,
                          int32_t key, ::registry_api::v1::ResolveRepresentationResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRepresentationResponse_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRepresentationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_ResolveRepresentationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::ResolveRepresentationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::ResolveRepresentationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRepresentationResponse>* __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRepresentationResponse_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRepresentationResponse>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRepresentationResponse_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRepresentationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRepresentationResponse_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRepresentationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRepresentationResponse_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRepresentationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRepresentationResponse_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRepresentationResponse> * m,
                          uint32_t key, ::registry_api::v1::ResolveRepresentationResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRepresentationResponse_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRepresentationResponse>* m,
                       uint32_t key, const ::registry_api::v1::ResolveRepresentationResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveRepresentationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRepresentationResponse_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRepresentationResponse> * m,
                          uint32_t key, ::registry_api::v1::ResolveRepresentationResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRepresentationResponse_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRepresentationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_ResolveRepresentationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::ResolveRepresentationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::ResolveRepresentationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRepresentationResponse>* __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRepresentationResponse_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRepresentationResponse>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRepresentationResponse_free(const google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRepresentationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRepresentationResponse_clear(google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRepresentationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRepresentationResponse_size(const google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRepresentationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRepresentationResponse_insert(google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRepresentationResponse> * m,
                          int64_t key, ::registry_api::v1::ResolveRepresentationResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRepresentationResponse_get(const google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRepresentationResponse>* m,
                       int64_t key, const ::registry_api::v1::ResolveRepresentationResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveRepresentationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRepresentationResponse_remove(google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRepresentationResponse> * m,
                          int64_t key, ::registry_api::v1::ResolveRepresentationResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRepresentationResponse_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRepresentationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_ResolveRepresentationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::ResolveRepresentationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::ResolveRepresentationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRepresentationResponse>* __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRepresentationResponse_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRepresentationResponse>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRepresentationResponse_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRepresentationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRepresentationResponse_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRepresentationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRepresentationResponse_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRepresentationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRepresentationResponse_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRepresentationResponse> * m,
                          uint64_t key, ::registry_api::v1::ResolveRepresentationResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRepresentationResponse_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRepresentationResponse>* m,
                       uint64_t key, const ::registry_api::v1::ResolveRepresentationResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveRepresentationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRepresentationResponse_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRepresentationResponse> * m,
                          uint64_t key, ::registry_api::v1::ResolveRepresentationResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRepresentationResponse_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRepresentationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_ResolveRepresentationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::ResolveRepresentationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::ResolveRepresentationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::ResolveRepresentationResponse>* __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRepresentationResponse_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::ResolveRepresentationResponse>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRepresentationResponse_free(const google::protobuf::Map<bool, ::registry_api::v1::ResolveRepresentationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRepresentationResponse_clear(google::protobuf::Map<bool, ::registry_api::v1::ResolveRepresentationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRepresentationResponse_size(const google::protobuf::Map<bool, ::registry_api::v1::ResolveRepresentationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRepresentationResponse_insert(google::protobuf::Map<bool, ::registry_api::v1::ResolveRepresentationResponse> * m,
                          bool key, ::registry_api::v1::ResolveRepresentationResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRepresentationResponse_get(const google::protobuf::Map<bool, ::registry_api::v1::ResolveRepresentationResponse>* m,
                       bool key, const ::registry_api::v1::ResolveRepresentationResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveRepresentationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRepresentationResponse_remove(google::protobuf::Map<bool, ::registry_api::v1::ResolveRepresentationResponse> * m,
                          bool key, ::registry_api::v1::ResolveRepresentationResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRepresentationResponse_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::ResolveRepresentationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_ResolveRepresentationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::ResolveRepresentationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::ResolveRepresentationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::ResolveRepresentationResponse>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRepresentationResponse_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::ResolveRepresentationResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRepresentationResponse_free(const google::protobuf::Map<std::string, ::registry_api::v1::ResolveRepresentationResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRepresentationResponse_clear(google::protobuf::Map<std::string, ::registry_api::v1::ResolveRepresentationResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRepresentationResponse_size(const google::protobuf::Map<std::string, ::registry_api::v1::ResolveRepresentationResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRepresentationResponse_insert(google::protobuf::Map<std::string, ::registry_api::v1::ResolveRepresentationResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::ResolveRepresentationResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRepresentationResponse_get(const google::protobuf::Map<std::string, ::registry_api::v1::ResolveRepresentationResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::ResolveRepresentationResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolveRepresentationResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRepresentationResponse_remove(google::protobuf::Map<std::string, ::registry_api::v1::ResolveRepresentationResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::ResolveRepresentationResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRepresentationResponse_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::ResolveRepresentationResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolveRepresentationResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::ResolveRepresentationResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::ResolveRepresentationResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.ResolutionMetadata
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_ResolutionMetadata_new() { return new ::registry_api::v1::ResolutionMetadata(); }
void __rust_proto_thunk__registry_api_v1_ResolutionMetadata_delete(void* ptr) { delete static_cast<::registry_api::v1::ResolutionMetadata*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_ResolutionMetadata_serialize(::registry_api::v1::ResolutionMetadata* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_ResolutionMetadata_parse(::registry_api::v1::ResolutionMetadata* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_ResolutionMetadata_copy_from(::registry_api::v1::ResolutionMetadata* dst, const ::registry_api::v1::ResolutionMetadata* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_ResolutionMetadata_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::ResolutionMetadata>* field) {
  return field->size();
}
const ::registry_api::v1::ResolutionMetadata& __rust_proto_thunk__registry_api_v1_ResolutionMetadata_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::ResolutionMetadata>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::ResolutionMetadata* __rust_proto_thunk__registry_api_v1_ResolutionMetadata_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::ResolutionMetadata>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::ResolutionMetadata* __rust_proto_thunk__registry_api_v1_ResolutionMetadata_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::ResolutionMetadata>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_ResolutionMetadata_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::ResolutionMetadata>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_ResolutionMetadata_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::ResolutionMetadata>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::ResolutionMetadata>& src) {
  dst = src;
}

bool __rust_proto_thunk__registry_api_v1_ResolutionMetadata_has_contentType(::registry_api::v1::ResolutionMetadata* msg) {
  return msg->has_contenttype();
}
void __rust_proto_thunk__registry_api_v1_ResolutionMetadata_clear_contentType(::registry_api::v1::ResolutionMetadata* msg) { msg->clear_contenttype(); }
::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_ResolutionMetadata_get_contentType(::registry_api::v1::ResolutionMetadata* msg) {
  absl::string_view val = msg->contenttype();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_ResolutionMetadata_set_contentType(::registry_api::v1::ResolutionMetadata* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_contenttype(absl::string_view(s.ptr, s.len));
}

bool __rust_proto_thunk__registry_api_v1_ResolutionMetadata_has_duration(::registry_api::v1::ResolutionMetadata* msg) {
  return msg->has_duration();
}
void __rust_proto_thunk__registry_api_v1_ResolutionMetadata_clear_duration(::registry_api::v1::ResolutionMetadata* msg) { msg->clear_duration(); }
::int64_t __rust_proto_thunk__registry_api_v1_ResolutionMetadata_get_duration(::registry_api::v1::ResolutionMetadata* msg) { return msg->duration(); }
void __rust_proto_thunk__registry_api_v1_ResolutionMetadata_set_duration(::registry_api::v1::ResolutionMetadata* msg, ::int64_t val) {
  msg->set_duration(val);
}

const void* __rust_proto_thunk__registry_api_v1_ResolutionMetadata_get_didUrl(::registry_api::v1::ResolutionMetadata* msg) {
  return static_cast<const void*>(&msg->didurl());
}
void* __rust_proto_thunk__registry_api_v1_ResolutionMetadata_get_mut_didUrl(::registry_api::v1::ResolutionMetadata* msg) {
  return static_cast<void*>(msg->mutable_didurl());
}
void __rust_proto_thunk__registry_api_v1_ResolutionMetadata_clear_didUrl(::registry_api::v1::ResolutionMetadata* msg) { msg->clear_didurl(); }
bool __rust_proto_thunk__registry_api_v1_ResolutionMetadata_has_didUrl(::registry_api::v1::ResolutionMetadata* msg) { return msg->has_didurl(); }

bool __rust_proto_thunk__registry_api_v1_ResolutionMetadata_has_error(::registry_api::v1::ResolutionMetadata* msg) {
  return msg->has_error();
}
void __rust_proto_thunk__registry_api_v1_ResolutionMetadata_clear_error(::registry_api::v1::ResolutionMetadata* msg) { msg->clear_error(); }
::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_ResolutionMetadata_get_error(::registry_api::v1::ResolutionMetadata* msg) {
  absl::string_view val = msg->error();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_ResolutionMetadata_set_error(::registry_api::v1::ResolutionMetadata* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_error(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionMetadata>* __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionMetadata_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionMetadata>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionMetadata_free(const google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionMetadata>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionMetadata_clear(google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionMetadata> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionMetadata_size(const google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionMetadata>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionMetadata_insert(google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionMetadata> * m,
                          int32_t key, ::registry_api::v1::ResolutionMetadata value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionMetadata_get(const google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionMetadata>* m,
                       int32_t key, const ::registry_api::v1::ResolutionMetadata** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolutionMetadata* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionMetadata_remove(google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionMetadata> * m,
                          int32_t key, ::registry_api::v1::ResolutionMetadata * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionMetadata_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionMetadata>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionMetadata_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::ResolutionMetadata** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionMetadata>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionMetadata>* __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionMetadata_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionMetadata>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionMetadata_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionMetadata>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionMetadata_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionMetadata> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionMetadata_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionMetadata>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionMetadata_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionMetadata> * m,
                          uint32_t key, ::registry_api::v1::ResolutionMetadata value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionMetadata_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionMetadata>* m,
                       uint32_t key, const ::registry_api::v1::ResolutionMetadata** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolutionMetadata* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionMetadata_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionMetadata> * m,
                          uint32_t key, ::registry_api::v1::ResolutionMetadata * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionMetadata_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionMetadata>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionMetadata_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::ResolutionMetadata** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionMetadata>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionMetadata>* __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionMetadata_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionMetadata>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionMetadata_free(const google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionMetadata>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionMetadata_clear(google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionMetadata> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionMetadata_size(const google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionMetadata>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionMetadata_insert(google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionMetadata> * m,
                          int64_t key, ::registry_api::v1::ResolutionMetadata value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionMetadata_get(const google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionMetadata>* m,
                       int64_t key, const ::registry_api::v1::ResolutionMetadata** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolutionMetadata* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionMetadata_remove(google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionMetadata> * m,
                          int64_t key, ::registry_api::v1::ResolutionMetadata * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionMetadata_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionMetadata>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionMetadata_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::ResolutionMetadata** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionMetadata>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionMetadata>* __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionMetadata_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionMetadata>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionMetadata_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionMetadata>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionMetadata_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionMetadata> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionMetadata_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionMetadata>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionMetadata_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionMetadata> * m,
                          uint64_t key, ::registry_api::v1::ResolutionMetadata value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionMetadata_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionMetadata>* m,
                       uint64_t key, const ::registry_api::v1::ResolutionMetadata** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolutionMetadata* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionMetadata_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionMetadata> * m,
                          uint64_t key, ::registry_api::v1::ResolutionMetadata * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionMetadata_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionMetadata>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionMetadata_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::ResolutionMetadata** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionMetadata>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::ResolutionMetadata>* __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionMetadata_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::ResolutionMetadata>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionMetadata_free(const google::protobuf::Map<bool, ::registry_api::v1::ResolutionMetadata>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionMetadata_clear(google::protobuf::Map<bool, ::registry_api::v1::ResolutionMetadata> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionMetadata_size(const google::protobuf::Map<bool, ::registry_api::v1::ResolutionMetadata>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionMetadata_insert(google::protobuf::Map<bool, ::registry_api::v1::ResolutionMetadata> * m,
                          bool key, ::registry_api::v1::ResolutionMetadata value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionMetadata_get(const google::protobuf::Map<bool, ::registry_api::v1::ResolutionMetadata>* m,
                       bool key, const ::registry_api::v1::ResolutionMetadata** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolutionMetadata* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionMetadata_remove(google::protobuf::Map<bool, ::registry_api::v1::ResolutionMetadata> * m,
                          bool key, ::registry_api::v1::ResolutionMetadata * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionMetadata_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::ResolutionMetadata>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionMetadata_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::ResolutionMetadata** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::ResolutionMetadata>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::ResolutionMetadata>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionMetadata_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::ResolutionMetadata>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionMetadata_free(const google::protobuf::Map<std::string, ::registry_api::v1::ResolutionMetadata>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionMetadata_clear(google::protobuf::Map<std::string, ::registry_api::v1::ResolutionMetadata> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionMetadata_size(const google::protobuf::Map<std::string, ::registry_api::v1::ResolutionMetadata>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionMetadata_insert(google::protobuf::Map<std::string, ::registry_api::v1::ResolutionMetadata> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::ResolutionMetadata value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionMetadata_get(const google::protobuf::Map<std::string, ::registry_api::v1::ResolutionMetadata>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::ResolutionMetadata** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolutionMetadata* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionMetadata_remove(google::protobuf::Map<std::string, ::registry_api::v1::ResolutionMetadata> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::ResolutionMetadata * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionMetadata_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::ResolutionMetadata>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionMetadata_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::ResolutionMetadata** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::ResolutionMetadata>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.ResolutionMetadataDidUrl
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_ResolutionMetadataDidUrl_new() { return new ::registry_api::v1::ResolutionMetadataDidUrl(); }
void __rust_proto_thunk__registry_api_v1_ResolutionMetadataDidUrl_delete(void* ptr) { delete static_cast<::registry_api::v1::ResolutionMetadataDidUrl*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_ResolutionMetadataDidUrl_serialize(::registry_api::v1::ResolutionMetadataDidUrl* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_ResolutionMetadataDidUrl_parse(::registry_api::v1::ResolutionMetadataDidUrl* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_ResolutionMetadataDidUrl_copy_from(::registry_api::v1::ResolutionMetadataDidUrl* dst, const ::registry_api::v1::ResolutionMetadataDidUrl* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_ResolutionMetadataDidUrl_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::ResolutionMetadataDidUrl>* field) {
  return field->size();
}
const ::registry_api::v1::ResolutionMetadataDidUrl& __rust_proto_thunk__registry_api_v1_ResolutionMetadataDidUrl_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::ResolutionMetadataDidUrl>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::ResolutionMetadataDidUrl* __rust_proto_thunk__registry_api_v1_ResolutionMetadataDidUrl_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::ResolutionMetadataDidUrl>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::ResolutionMetadataDidUrl* __rust_proto_thunk__registry_api_v1_ResolutionMetadataDidUrl_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::ResolutionMetadataDidUrl>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_ResolutionMetadataDidUrl_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::ResolutionMetadataDidUrl>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_ResolutionMetadataDidUrl_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::ResolutionMetadataDidUrl>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::ResolutionMetadataDidUrl>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_ResolutionMetadataDidUrl_get_did(::registry_api::v1::ResolutionMetadataDidUrl* msg) {
  absl::string_view val = msg->did();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_ResolutionMetadataDidUrl_set_did(::registry_api::v1::ResolutionMetadataDidUrl* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_did(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_ResolutionMetadataDidUrl_get_methodName(::registry_api::v1::ResolutionMetadataDidUrl* msg) {
  absl::string_view val = msg->methodname();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_ResolutionMetadataDidUrl_set_methodName(::registry_api::v1::ResolutionMetadataDidUrl* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_methodname(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_ResolutionMetadataDidUrl_get_methodSpecificId(::registry_api::v1::ResolutionMetadataDidUrl* msg) {
  absl::string_view val = msg->methodspecificid();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_ResolutionMetadataDidUrl_set_methodSpecificId(::registry_api::v1::ResolutionMetadataDidUrl* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_methodspecificid(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionMetadataDidUrl>* __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionMetadataDidUrl_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionMetadataDidUrl>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionMetadataDidUrl_free(const google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionMetadataDidUrl>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionMetadataDidUrl_clear(google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionMetadataDidUrl> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionMetadataDidUrl_size(const google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionMetadataDidUrl>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionMetadataDidUrl_insert(google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionMetadataDidUrl> * m,
                          int32_t key, ::registry_api::v1::ResolutionMetadataDidUrl value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionMetadataDidUrl_get(const google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionMetadataDidUrl>* m,
                       int32_t key, const ::registry_api::v1::ResolutionMetadataDidUrl** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolutionMetadataDidUrl* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionMetadataDidUrl_remove(google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionMetadataDidUrl> * m,
                          int32_t key, ::registry_api::v1::ResolutionMetadataDidUrl * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionMetadataDidUrl_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionMetadataDidUrl>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_ResolutionMetadataDidUrl_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::ResolutionMetadataDidUrl** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::ResolutionMetadataDidUrl>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionMetadataDidUrl>* __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionMetadataDidUrl_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionMetadataDidUrl>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionMetadataDidUrl_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionMetadataDidUrl>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionMetadataDidUrl_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionMetadataDidUrl> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionMetadataDidUrl_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionMetadataDidUrl>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionMetadataDidUrl_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionMetadataDidUrl> * m,
                          uint32_t key, ::registry_api::v1::ResolutionMetadataDidUrl value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionMetadataDidUrl_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionMetadataDidUrl>* m,
                       uint32_t key, const ::registry_api::v1::ResolutionMetadataDidUrl** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolutionMetadataDidUrl* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionMetadataDidUrl_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionMetadataDidUrl> * m,
                          uint32_t key, ::registry_api::v1::ResolutionMetadataDidUrl * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionMetadataDidUrl_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionMetadataDidUrl>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_ResolutionMetadataDidUrl_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::ResolutionMetadataDidUrl** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::ResolutionMetadataDidUrl>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionMetadataDidUrl>* __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionMetadataDidUrl_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionMetadataDidUrl>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionMetadataDidUrl_free(const google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionMetadataDidUrl>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionMetadataDidUrl_clear(google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionMetadataDidUrl> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionMetadataDidUrl_size(const google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionMetadataDidUrl>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionMetadataDidUrl_insert(google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionMetadataDidUrl> * m,
                          int64_t key, ::registry_api::v1::ResolutionMetadataDidUrl value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionMetadataDidUrl_get(const google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionMetadataDidUrl>* m,
                       int64_t key, const ::registry_api::v1::ResolutionMetadataDidUrl** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolutionMetadataDidUrl* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionMetadataDidUrl_remove(google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionMetadataDidUrl> * m,
                          int64_t key, ::registry_api::v1::ResolutionMetadataDidUrl * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionMetadataDidUrl_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionMetadataDidUrl>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_ResolutionMetadataDidUrl_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::ResolutionMetadataDidUrl** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::ResolutionMetadataDidUrl>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionMetadataDidUrl>* __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionMetadataDidUrl_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionMetadataDidUrl>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionMetadataDidUrl_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionMetadataDidUrl>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionMetadataDidUrl_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionMetadataDidUrl> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionMetadataDidUrl_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionMetadataDidUrl>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionMetadataDidUrl_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionMetadataDidUrl> * m,
                          uint64_t key, ::registry_api::v1::ResolutionMetadataDidUrl value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionMetadataDidUrl_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionMetadataDidUrl>* m,
                       uint64_t key, const ::registry_api::v1::ResolutionMetadataDidUrl** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolutionMetadataDidUrl* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionMetadataDidUrl_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionMetadataDidUrl> * m,
                          uint64_t key, ::registry_api::v1::ResolutionMetadataDidUrl * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionMetadataDidUrl_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionMetadataDidUrl>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_ResolutionMetadataDidUrl_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::ResolutionMetadataDidUrl** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::ResolutionMetadataDidUrl>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::ResolutionMetadataDidUrl>* __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionMetadataDidUrl_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::ResolutionMetadataDidUrl>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionMetadataDidUrl_free(const google::protobuf::Map<bool, ::registry_api::v1::ResolutionMetadataDidUrl>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionMetadataDidUrl_clear(google::protobuf::Map<bool, ::registry_api::v1::ResolutionMetadataDidUrl> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionMetadataDidUrl_size(const google::protobuf::Map<bool, ::registry_api::v1::ResolutionMetadataDidUrl>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionMetadataDidUrl_insert(google::protobuf::Map<bool, ::registry_api::v1::ResolutionMetadataDidUrl> * m,
                          bool key, ::registry_api::v1::ResolutionMetadataDidUrl value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionMetadataDidUrl_get(const google::protobuf::Map<bool, ::registry_api::v1::ResolutionMetadataDidUrl>* m,
                       bool key, const ::registry_api::v1::ResolutionMetadataDidUrl** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolutionMetadataDidUrl* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionMetadataDidUrl_remove(google::protobuf::Map<bool, ::registry_api::v1::ResolutionMetadataDidUrl> * m,
                          bool key, ::registry_api::v1::ResolutionMetadataDidUrl * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionMetadataDidUrl_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::ResolutionMetadataDidUrl>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_ResolutionMetadataDidUrl_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::ResolutionMetadataDidUrl** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::ResolutionMetadataDidUrl>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::ResolutionMetadataDidUrl>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionMetadataDidUrl_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::ResolutionMetadataDidUrl>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionMetadataDidUrl_free(const google::protobuf::Map<std::string, ::registry_api::v1::ResolutionMetadataDidUrl>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionMetadataDidUrl_clear(google::protobuf::Map<std::string, ::registry_api::v1::ResolutionMetadataDidUrl> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionMetadataDidUrl_size(const google::protobuf::Map<std::string, ::registry_api::v1::ResolutionMetadataDidUrl>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionMetadataDidUrl_insert(google::protobuf::Map<std::string, ::registry_api::v1::ResolutionMetadataDidUrl> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::ResolutionMetadataDidUrl value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionMetadataDidUrl_get(const google::protobuf::Map<std::string, ::registry_api::v1::ResolutionMetadataDidUrl>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::ResolutionMetadataDidUrl** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::ResolutionMetadataDidUrl* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionMetadataDidUrl_remove(google::protobuf::Map<std::string, ::registry_api::v1::ResolutionMetadataDidUrl> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::ResolutionMetadataDidUrl * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionMetadataDidUrl_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::ResolutionMetadataDidUrl>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_ResolutionMetadataDidUrl_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::ResolutionMetadataDidUrl** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::ResolutionMetadataDidUrl>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.UpdateRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_UpdateRequest_new() { return new ::registry_api::v1::UpdateRequest(); }
void __rust_proto_thunk__registry_api_v1_UpdateRequest_delete(void* ptr) { delete static_cast<::registry_api::v1::UpdateRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_UpdateRequest_serialize(::registry_api::v1::UpdateRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_UpdateRequest_parse(::registry_api::v1::UpdateRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_UpdateRequest_copy_from(::registry_api::v1::UpdateRequest* dst, const ::registry_api::v1::UpdateRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_UpdateRequest_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateRequest>* field) {
  return field->size();
}
const ::registry_api::v1::UpdateRequest& __rust_proto_thunk__registry_api_v1_UpdateRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateRequest>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::UpdateRequest* __rust_proto_thunk__registry_api_v1_UpdateRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::UpdateRequest* __rust_proto_thunk__registry_api_v1_UpdateRequest_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_UpdateRequest_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_UpdateRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateRequest>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_UpdateRequest_get_did(::registry_api::v1::UpdateRequest* msg) {
  absl::string_view val = msg->did();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_UpdateRequest_set_did(::registry_api::v1::UpdateRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_did(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_UpdateRequest_get_document(::registry_api::v1::UpdateRequest* msg) {
  absl::string_view val = msg->document();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_UpdateRequest_set_document(::registry_api::v1::UpdateRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_document(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateRequest>* __rust_proto_thunk__Map_i32_registry_api_v1_UpdateRequest_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::UpdateRequest>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_UpdateRequest_free(const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_UpdateRequest_clear(google::protobuf::Map<int32_t, ::registry_api::v1::UpdateRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_UpdateRequest_size(const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_UpdateRequest_insert(google::protobuf::Map<int32_t, ::registry_api::v1::UpdateRequest> * m,
                          int32_t key, ::registry_api::v1::UpdateRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_UpdateRequest_get(const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateRequest>* m,
                       int32_t key, const ::registry_api::v1::UpdateRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_UpdateRequest_remove(google::protobuf::Map<int32_t, ::registry_api::v1::UpdateRequest> * m,
                          int32_t key, ::registry_api::v1::UpdateRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_UpdateRequest_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_UpdateRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::UpdateRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::UpdateRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateRequest>* __rust_proto_thunk__Map_u32_registry_api_v1_UpdateRequest_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateRequest>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_UpdateRequest_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_UpdateRequest_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_UpdateRequest_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_UpdateRequest_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateRequest> * m,
                          uint32_t key, ::registry_api::v1::UpdateRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_UpdateRequest_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateRequest>* m,
                       uint32_t key, const ::registry_api::v1::UpdateRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_UpdateRequest_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateRequest> * m,
                          uint32_t key, ::registry_api::v1::UpdateRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_UpdateRequest_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_UpdateRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::UpdateRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateRequest>* __rust_proto_thunk__Map_i64_registry_api_v1_UpdateRequest_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::UpdateRequest>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_UpdateRequest_free(const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_UpdateRequest_clear(google::protobuf::Map<int64_t, ::registry_api::v1::UpdateRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_UpdateRequest_size(const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_UpdateRequest_insert(google::protobuf::Map<int64_t, ::registry_api::v1::UpdateRequest> * m,
                          int64_t key, ::registry_api::v1::UpdateRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_UpdateRequest_get(const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateRequest>* m,
                       int64_t key, const ::registry_api::v1::UpdateRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_UpdateRequest_remove(google::protobuf::Map<int64_t, ::registry_api::v1::UpdateRequest> * m,
                          int64_t key, ::registry_api::v1::UpdateRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_UpdateRequest_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_UpdateRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::UpdateRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::UpdateRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateRequest>* __rust_proto_thunk__Map_u64_registry_api_v1_UpdateRequest_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateRequest>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_UpdateRequest_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_UpdateRequest_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_UpdateRequest_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_UpdateRequest_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateRequest> * m,
                          uint64_t key, ::registry_api::v1::UpdateRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_UpdateRequest_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateRequest>* m,
                       uint64_t key, const ::registry_api::v1::UpdateRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_UpdateRequest_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateRequest> * m,
                          uint64_t key, ::registry_api::v1::UpdateRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_UpdateRequest_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_UpdateRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::UpdateRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::UpdateRequest>* __rust_proto_thunk__Map_bool_registry_api_v1_UpdateRequest_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::UpdateRequest>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_UpdateRequest_free(const google::protobuf::Map<bool, ::registry_api::v1::UpdateRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_UpdateRequest_clear(google::protobuf::Map<bool, ::registry_api::v1::UpdateRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_UpdateRequest_size(const google::protobuf::Map<bool, ::registry_api::v1::UpdateRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_UpdateRequest_insert(google::protobuf::Map<bool, ::registry_api::v1::UpdateRequest> * m,
                          bool key, ::registry_api::v1::UpdateRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_UpdateRequest_get(const google::protobuf::Map<bool, ::registry_api::v1::UpdateRequest>* m,
                       bool key, const ::registry_api::v1::UpdateRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_UpdateRequest_remove(google::protobuf::Map<bool, ::registry_api::v1::UpdateRequest> * m,
                          bool key, ::registry_api::v1::UpdateRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_UpdateRequest_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::UpdateRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_UpdateRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::UpdateRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::UpdateRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::UpdateRequest>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateRequest_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::UpdateRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateRequest_free(const google::protobuf::Map<std::string, ::registry_api::v1::UpdateRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateRequest_clear(google::protobuf::Map<std::string, ::registry_api::v1::UpdateRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateRequest_size(const google::protobuf::Map<std::string, ::registry_api::v1::UpdateRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateRequest_insert(google::protobuf::Map<std::string, ::registry_api::v1::UpdateRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::UpdateRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateRequest_get(const google::protobuf::Map<std::string, ::registry_api::v1::UpdateRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::UpdateRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateRequest_remove(google::protobuf::Map<std::string, ::registry_api::v1::UpdateRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::UpdateRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateRequest_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::UpdateRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::UpdateRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::UpdateRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.UpdateResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_UpdateResponse_new() { return new ::registry_api::v1::UpdateResponse(); }
void __rust_proto_thunk__registry_api_v1_UpdateResponse_delete(void* ptr) { delete static_cast<::registry_api::v1::UpdateResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_UpdateResponse_serialize(::registry_api::v1::UpdateResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_UpdateResponse_parse(::registry_api::v1::UpdateResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_UpdateResponse_copy_from(::registry_api::v1::UpdateResponse* dst, const ::registry_api::v1::UpdateResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_UpdateResponse_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateResponse>* field) {
  return field->size();
}
const ::registry_api::v1::UpdateResponse& __rust_proto_thunk__registry_api_v1_UpdateResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateResponse>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::UpdateResponse* __rust_proto_thunk__registry_api_v1_UpdateResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::UpdateResponse* __rust_proto_thunk__registry_api_v1_UpdateResponse_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_UpdateResponse_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_UpdateResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateResponse>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateResponse>* __rust_proto_thunk__Map_i32_registry_api_v1_UpdateResponse_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::UpdateResponse>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_UpdateResponse_free(const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_UpdateResponse_clear(google::protobuf::Map<int32_t, ::registry_api::v1::UpdateResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_UpdateResponse_size(const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_UpdateResponse_insert(google::protobuf::Map<int32_t, ::registry_api::v1::UpdateResponse> * m,
                          int32_t key, ::registry_api::v1::UpdateResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_UpdateResponse_get(const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateResponse>* m,
                       int32_t key, const ::registry_api::v1::UpdateResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_UpdateResponse_remove(google::protobuf::Map<int32_t, ::registry_api::v1::UpdateResponse> * m,
                          int32_t key, ::registry_api::v1::UpdateResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_UpdateResponse_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_UpdateResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::UpdateResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::UpdateResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateResponse>* __rust_proto_thunk__Map_u32_registry_api_v1_UpdateResponse_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateResponse>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_UpdateResponse_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_UpdateResponse_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_UpdateResponse_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_UpdateResponse_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateResponse> * m,
                          uint32_t key, ::registry_api::v1::UpdateResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_UpdateResponse_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateResponse>* m,
                       uint32_t key, const ::registry_api::v1::UpdateResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_UpdateResponse_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateResponse> * m,
                          uint32_t key, ::registry_api::v1::UpdateResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_UpdateResponse_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_UpdateResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::UpdateResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateResponse>* __rust_proto_thunk__Map_i64_registry_api_v1_UpdateResponse_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::UpdateResponse>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_UpdateResponse_free(const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_UpdateResponse_clear(google::protobuf::Map<int64_t, ::registry_api::v1::UpdateResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_UpdateResponse_size(const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_UpdateResponse_insert(google::protobuf::Map<int64_t, ::registry_api::v1::UpdateResponse> * m,
                          int64_t key, ::registry_api::v1::UpdateResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_UpdateResponse_get(const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateResponse>* m,
                       int64_t key, const ::registry_api::v1::UpdateResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_UpdateResponse_remove(google::protobuf::Map<int64_t, ::registry_api::v1::UpdateResponse> * m,
                          int64_t key, ::registry_api::v1::UpdateResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_UpdateResponse_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_UpdateResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::UpdateResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::UpdateResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateResponse>* __rust_proto_thunk__Map_u64_registry_api_v1_UpdateResponse_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateResponse>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_UpdateResponse_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_UpdateResponse_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_UpdateResponse_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_UpdateResponse_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateResponse> * m,
                          uint64_t key, ::registry_api::v1::UpdateResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_UpdateResponse_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateResponse>* m,
                       uint64_t key, const ::registry_api::v1::UpdateResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_UpdateResponse_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateResponse> * m,
                          uint64_t key, ::registry_api::v1::UpdateResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_UpdateResponse_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_UpdateResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::UpdateResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::UpdateResponse>* __rust_proto_thunk__Map_bool_registry_api_v1_UpdateResponse_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::UpdateResponse>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_UpdateResponse_free(const google::protobuf::Map<bool, ::registry_api::v1::UpdateResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_UpdateResponse_clear(google::protobuf::Map<bool, ::registry_api::v1::UpdateResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_UpdateResponse_size(const google::protobuf::Map<bool, ::registry_api::v1::UpdateResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_UpdateResponse_insert(google::protobuf::Map<bool, ::registry_api::v1::UpdateResponse> * m,
                          bool key, ::registry_api::v1::UpdateResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_UpdateResponse_get(const google::protobuf::Map<bool, ::registry_api::v1::UpdateResponse>* m,
                       bool key, const ::registry_api::v1::UpdateResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_UpdateResponse_remove(google::protobuf::Map<bool, ::registry_api::v1::UpdateResponse> * m,
                          bool key, ::registry_api::v1::UpdateResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_UpdateResponse_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::UpdateResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_UpdateResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::UpdateResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::UpdateResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::UpdateResponse>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateResponse_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::UpdateResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateResponse_free(const google::protobuf::Map<std::string, ::registry_api::v1::UpdateResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateResponse_clear(google::protobuf::Map<std::string, ::registry_api::v1::UpdateResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateResponse_size(const google::protobuf::Map<std::string, ::registry_api::v1::UpdateResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateResponse_insert(google::protobuf::Map<std::string, ::registry_api::v1::UpdateResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::UpdateResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateResponse_get(const google::protobuf::Map<std::string, ::registry_api::v1::UpdateResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::UpdateResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateResponse_remove(google::protobuf::Map<std::string, ::registry_api::v1::UpdateResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::UpdateResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateResponse_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::UpdateResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::UpdateResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::UpdateResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.RevokeRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_RevokeRequest_new() { return new ::registry_api::v1::RevokeRequest(); }
void __rust_proto_thunk__registry_api_v1_RevokeRequest_delete(void* ptr) { delete static_cast<::registry_api::v1::RevokeRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_RevokeRequest_serialize(::registry_api::v1::RevokeRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_RevokeRequest_parse(::registry_api::v1::RevokeRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_RevokeRequest_copy_from(::registry_api::v1::RevokeRequest* dst, const ::registry_api::v1::RevokeRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_RevokeRequest_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::RevokeRequest>* field) {
  return field->size();
}
const ::registry_api::v1::RevokeRequest& __rust_proto_thunk__registry_api_v1_RevokeRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::RevokeRequest>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::RevokeRequest* __rust_proto_thunk__registry_api_v1_RevokeRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::RevokeRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::RevokeRequest* __rust_proto_thunk__registry_api_v1_RevokeRequest_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::RevokeRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_RevokeRequest_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::RevokeRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_RevokeRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::RevokeRequest>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::RevokeRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_RevokeRequest_get_did(::registry_api::v1::RevokeRequest* msg) {
  absl::string_view val = msg->did();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_RevokeRequest_set_did(::registry_api::v1::RevokeRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_did(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_RevokeRequest_get_document(::registry_api::v1::RevokeRequest* msg) {
  absl::string_view val = msg->document();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_RevokeRequest_set_document(::registry_api::v1::RevokeRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_document(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::RevokeRequest>* __rust_proto_thunk__Map_i32_registry_api_v1_RevokeRequest_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::RevokeRequest>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_RevokeRequest_free(const google::protobuf::Map<int32_t, ::registry_api::v1::RevokeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_RevokeRequest_clear(google::protobuf::Map<int32_t, ::registry_api::v1::RevokeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_RevokeRequest_size(const google::protobuf::Map<int32_t, ::registry_api::v1::RevokeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_RevokeRequest_insert(google::protobuf::Map<int32_t, ::registry_api::v1::RevokeRequest> * m,
                          int32_t key, ::registry_api::v1::RevokeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_RevokeRequest_get(const google::protobuf::Map<int32_t, ::registry_api::v1::RevokeRequest>* m,
                       int32_t key, const ::registry_api::v1::RevokeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::RevokeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_RevokeRequest_remove(google::protobuf::Map<int32_t, ::registry_api::v1::RevokeRequest> * m,
                          int32_t key, ::registry_api::v1::RevokeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_RevokeRequest_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::RevokeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_RevokeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::RevokeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::RevokeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::RevokeRequest>* __rust_proto_thunk__Map_u32_registry_api_v1_RevokeRequest_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::RevokeRequest>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_RevokeRequest_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::RevokeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_RevokeRequest_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::RevokeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_RevokeRequest_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::RevokeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_RevokeRequest_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::RevokeRequest> * m,
                          uint32_t key, ::registry_api::v1::RevokeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_RevokeRequest_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::RevokeRequest>* m,
                       uint32_t key, const ::registry_api::v1::RevokeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::RevokeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_RevokeRequest_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::RevokeRequest> * m,
                          uint32_t key, ::registry_api::v1::RevokeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_RevokeRequest_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::RevokeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_RevokeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::RevokeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::RevokeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::RevokeRequest>* __rust_proto_thunk__Map_i64_registry_api_v1_RevokeRequest_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::RevokeRequest>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_RevokeRequest_free(const google::protobuf::Map<int64_t, ::registry_api::v1::RevokeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_RevokeRequest_clear(google::protobuf::Map<int64_t, ::registry_api::v1::RevokeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_RevokeRequest_size(const google::protobuf::Map<int64_t, ::registry_api::v1::RevokeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_RevokeRequest_insert(google::protobuf::Map<int64_t, ::registry_api::v1::RevokeRequest> * m,
                          int64_t key, ::registry_api::v1::RevokeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_RevokeRequest_get(const google::protobuf::Map<int64_t, ::registry_api::v1::RevokeRequest>* m,
                       int64_t key, const ::registry_api::v1::RevokeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::RevokeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_RevokeRequest_remove(google::protobuf::Map<int64_t, ::registry_api::v1::RevokeRequest> * m,
                          int64_t key, ::registry_api::v1::RevokeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_RevokeRequest_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::RevokeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_RevokeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::RevokeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::RevokeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::RevokeRequest>* __rust_proto_thunk__Map_u64_registry_api_v1_RevokeRequest_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::RevokeRequest>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_RevokeRequest_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::RevokeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_RevokeRequest_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::RevokeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_RevokeRequest_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::RevokeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_RevokeRequest_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::RevokeRequest> * m,
                          uint64_t key, ::registry_api::v1::RevokeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_RevokeRequest_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::RevokeRequest>* m,
                       uint64_t key, const ::registry_api::v1::RevokeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::RevokeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_RevokeRequest_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::RevokeRequest> * m,
                          uint64_t key, ::registry_api::v1::RevokeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_RevokeRequest_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::RevokeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_RevokeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::RevokeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::RevokeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::RevokeRequest>* __rust_proto_thunk__Map_bool_registry_api_v1_RevokeRequest_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::RevokeRequest>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_RevokeRequest_free(const google::protobuf::Map<bool, ::registry_api::v1::RevokeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_RevokeRequest_clear(google::protobuf::Map<bool, ::registry_api::v1::RevokeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_RevokeRequest_size(const google::protobuf::Map<bool, ::registry_api::v1::RevokeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_RevokeRequest_insert(google::protobuf::Map<bool, ::registry_api::v1::RevokeRequest> * m,
                          bool key, ::registry_api::v1::RevokeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_RevokeRequest_get(const google::protobuf::Map<bool, ::registry_api::v1::RevokeRequest>* m,
                       bool key, const ::registry_api::v1::RevokeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::RevokeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_RevokeRequest_remove(google::protobuf::Map<bool, ::registry_api::v1::RevokeRequest> * m,
                          bool key, ::registry_api::v1::RevokeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_RevokeRequest_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::RevokeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_RevokeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::RevokeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::RevokeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::RevokeRequest>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_RevokeRequest_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::RevokeRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_RevokeRequest_free(const google::protobuf::Map<std::string, ::registry_api::v1::RevokeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_RevokeRequest_clear(google::protobuf::Map<std::string, ::registry_api::v1::RevokeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_RevokeRequest_size(const google::protobuf::Map<std::string, ::registry_api::v1::RevokeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_RevokeRequest_insert(google::protobuf::Map<std::string, ::registry_api::v1::RevokeRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::RevokeRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_RevokeRequest_get(const google::protobuf::Map<std::string, ::registry_api::v1::RevokeRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::RevokeRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::RevokeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_RevokeRequest_remove(google::protobuf::Map<std::string, ::registry_api::v1::RevokeRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::RevokeRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_RevokeRequest_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::RevokeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_RevokeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::RevokeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::RevokeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.RevokeResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_RevokeResponse_new() { return new ::registry_api::v1::RevokeResponse(); }
void __rust_proto_thunk__registry_api_v1_RevokeResponse_delete(void* ptr) { delete static_cast<::registry_api::v1::RevokeResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_RevokeResponse_serialize(::registry_api::v1::RevokeResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_RevokeResponse_parse(::registry_api::v1::RevokeResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_RevokeResponse_copy_from(::registry_api::v1::RevokeResponse* dst, const ::registry_api::v1::RevokeResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_RevokeResponse_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::RevokeResponse>* field) {
  return field->size();
}
const ::registry_api::v1::RevokeResponse& __rust_proto_thunk__registry_api_v1_RevokeResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::RevokeResponse>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::RevokeResponse* __rust_proto_thunk__registry_api_v1_RevokeResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::RevokeResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::RevokeResponse* __rust_proto_thunk__registry_api_v1_RevokeResponse_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::RevokeResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_RevokeResponse_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::RevokeResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_RevokeResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::RevokeResponse>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::RevokeResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::RevokeResponse>* __rust_proto_thunk__Map_i32_registry_api_v1_RevokeResponse_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::RevokeResponse>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_RevokeResponse_free(const google::protobuf::Map<int32_t, ::registry_api::v1::RevokeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_RevokeResponse_clear(google::protobuf::Map<int32_t, ::registry_api::v1::RevokeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_RevokeResponse_size(const google::protobuf::Map<int32_t, ::registry_api::v1::RevokeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_RevokeResponse_insert(google::protobuf::Map<int32_t, ::registry_api::v1::RevokeResponse> * m,
                          int32_t key, ::registry_api::v1::RevokeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_RevokeResponse_get(const google::protobuf::Map<int32_t, ::registry_api::v1::RevokeResponse>* m,
                       int32_t key, const ::registry_api::v1::RevokeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::RevokeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_RevokeResponse_remove(google::protobuf::Map<int32_t, ::registry_api::v1::RevokeResponse> * m,
                          int32_t key, ::registry_api::v1::RevokeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_RevokeResponse_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::RevokeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_RevokeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::RevokeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::RevokeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::RevokeResponse>* __rust_proto_thunk__Map_u32_registry_api_v1_RevokeResponse_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::RevokeResponse>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_RevokeResponse_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::RevokeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_RevokeResponse_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::RevokeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_RevokeResponse_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::RevokeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_RevokeResponse_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::RevokeResponse> * m,
                          uint32_t key, ::registry_api::v1::RevokeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_RevokeResponse_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::RevokeResponse>* m,
                       uint32_t key, const ::registry_api::v1::RevokeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::RevokeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_RevokeResponse_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::RevokeResponse> * m,
                          uint32_t key, ::registry_api::v1::RevokeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_RevokeResponse_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::RevokeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_RevokeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::RevokeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::RevokeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::RevokeResponse>* __rust_proto_thunk__Map_i64_registry_api_v1_RevokeResponse_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::RevokeResponse>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_RevokeResponse_free(const google::protobuf::Map<int64_t, ::registry_api::v1::RevokeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_RevokeResponse_clear(google::protobuf::Map<int64_t, ::registry_api::v1::RevokeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_RevokeResponse_size(const google::protobuf::Map<int64_t, ::registry_api::v1::RevokeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_RevokeResponse_insert(google::protobuf::Map<int64_t, ::registry_api::v1::RevokeResponse> * m,
                          int64_t key, ::registry_api::v1::RevokeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_RevokeResponse_get(const google::protobuf::Map<int64_t, ::registry_api::v1::RevokeResponse>* m,
                       int64_t key, const ::registry_api::v1::RevokeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::RevokeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_RevokeResponse_remove(google::protobuf::Map<int64_t, ::registry_api::v1::RevokeResponse> * m,
                          int64_t key, ::registry_api::v1::RevokeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_RevokeResponse_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::RevokeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_RevokeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::RevokeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::RevokeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::RevokeResponse>* __rust_proto_thunk__Map_u64_registry_api_v1_RevokeResponse_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::RevokeResponse>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_RevokeResponse_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::RevokeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_RevokeResponse_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::RevokeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_RevokeResponse_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::RevokeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_RevokeResponse_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::RevokeResponse> * m,
                          uint64_t key, ::registry_api::v1::RevokeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_RevokeResponse_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::RevokeResponse>* m,
                       uint64_t key, const ::registry_api::v1::RevokeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::RevokeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_RevokeResponse_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::RevokeResponse> * m,
                          uint64_t key, ::registry_api::v1::RevokeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_RevokeResponse_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::RevokeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_RevokeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::RevokeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::RevokeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::RevokeResponse>* __rust_proto_thunk__Map_bool_registry_api_v1_RevokeResponse_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::RevokeResponse>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_RevokeResponse_free(const google::protobuf::Map<bool, ::registry_api::v1::RevokeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_RevokeResponse_clear(google::protobuf::Map<bool, ::registry_api::v1::RevokeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_RevokeResponse_size(const google::protobuf::Map<bool, ::registry_api::v1::RevokeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_RevokeResponse_insert(google::protobuf::Map<bool, ::registry_api::v1::RevokeResponse> * m,
                          bool key, ::registry_api::v1::RevokeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_RevokeResponse_get(const google::protobuf::Map<bool, ::registry_api::v1::RevokeResponse>* m,
                       bool key, const ::registry_api::v1::RevokeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::RevokeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_RevokeResponse_remove(google::protobuf::Map<bool, ::registry_api::v1::RevokeResponse> * m,
                          bool key, ::registry_api::v1::RevokeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_RevokeResponse_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::RevokeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_RevokeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::RevokeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::RevokeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::RevokeResponse>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_RevokeResponse_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::RevokeResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_RevokeResponse_free(const google::protobuf::Map<std::string, ::registry_api::v1::RevokeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_RevokeResponse_clear(google::protobuf::Map<std::string, ::registry_api::v1::RevokeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_RevokeResponse_size(const google::protobuf::Map<std::string, ::registry_api::v1::RevokeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_RevokeResponse_insert(google::protobuf::Map<std::string, ::registry_api::v1::RevokeResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::RevokeResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_RevokeResponse_get(const google::protobuf::Map<std::string, ::registry_api::v1::RevokeResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::RevokeResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::RevokeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_RevokeResponse_remove(google::protobuf::Map<std::string, ::registry_api::v1::RevokeResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::RevokeResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_RevokeResponse_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::RevokeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_RevokeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::RevokeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::RevokeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.PostalAddress
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_PostalAddress_new() { return new ::registry_api::v1::PostalAddress(); }
void __rust_proto_thunk__registry_api_v1_PostalAddress_delete(void* ptr) { delete static_cast<::registry_api::v1::PostalAddress*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_PostalAddress_serialize(::registry_api::v1::PostalAddress* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_PostalAddress_parse(::registry_api::v1::PostalAddress* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_PostalAddress_copy_from(::registry_api::v1::PostalAddress* dst, const ::registry_api::v1::PostalAddress* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_PostalAddress_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::PostalAddress>* field) {
  return field->size();
}
const ::registry_api::v1::PostalAddress& __rust_proto_thunk__registry_api_v1_PostalAddress_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::PostalAddress>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::PostalAddress* __rust_proto_thunk__registry_api_v1_PostalAddress_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::PostalAddress>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::PostalAddress* __rust_proto_thunk__registry_api_v1_PostalAddress_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::PostalAddress>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_PostalAddress_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::PostalAddress>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_PostalAddress_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::PostalAddress>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::PostalAddress>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_PostalAddress_get_country(::registry_api::v1::PostalAddress* msg) {
  absl::string_view val = msg->country();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_PostalAddress_set_country(::registry_api::v1::PostalAddress* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_country(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_PostalAddress_get_city(::registry_api::v1::PostalAddress* msg) {
  absl::string_view val = msg->city();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_PostalAddress_set_city(::registry_api::v1::PostalAddress* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_city(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_PostalAddress_get_state(::registry_api::v1::PostalAddress* msg) {
  absl::string_view val = msg->state();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_PostalAddress_set_state(::registry_api::v1::PostalAddress* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_state(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_PostalAddress_get_postal_code(::registry_api::v1::PostalAddress* msg) {
  absl::string_view val = msg->postal_code();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_PostalAddress_set_postal_code(::registry_api::v1::PostalAddress* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_postal_code(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_PostalAddress_get_street(::registry_api::v1::PostalAddress* msg) {
  absl::string_view val = msg->street();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_PostalAddress_set_street(::registry_api::v1::PostalAddress* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_street(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::PostalAddress>* __rust_proto_thunk__Map_i32_registry_api_v1_PostalAddress_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::PostalAddress>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_PostalAddress_free(const google::protobuf::Map<int32_t, ::registry_api::v1::PostalAddress>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_PostalAddress_clear(google::protobuf::Map<int32_t, ::registry_api::v1::PostalAddress> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_PostalAddress_size(const google::protobuf::Map<int32_t, ::registry_api::v1::PostalAddress>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_PostalAddress_insert(google::protobuf::Map<int32_t, ::registry_api::v1::PostalAddress> * m,
                          int32_t key, ::registry_api::v1::PostalAddress value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_PostalAddress_get(const google::protobuf::Map<int32_t, ::registry_api::v1::PostalAddress>* m,
                       int32_t key, const ::registry_api::v1::PostalAddress** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::PostalAddress* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_PostalAddress_remove(google::protobuf::Map<int32_t, ::registry_api::v1::PostalAddress> * m,
                          int32_t key, ::registry_api::v1::PostalAddress * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_PostalAddress_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::PostalAddress>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_PostalAddress_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::PostalAddress** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::PostalAddress>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::PostalAddress>* __rust_proto_thunk__Map_u32_registry_api_v1_PostalAddress_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::PostalAddress>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_PostalAddress_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::PostalAddress>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_PostalAddress_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::PostalAddress> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_PostalAddress_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::PostalAddress>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_PostalAddress_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::PostalAddress> * m,
                          uint32_t key, ::registry_api::v1::PostalAddress value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_PostalAddress_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::PostalAddress>* m,
                       uint32_t key, const ::registry_api::v1::PostalAddress** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::PostalAddress* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_PostalAddress_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::PostalAddress> * m,
                          uint32_t key, ::registry_api::v1::PostalAddress * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_PostalAddress_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::PostalAddress>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_PostalAddress_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::PostalAddress** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::PostalAddress>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::PostalAddress>* __rust_proto_thunk__Map_i64_registry_api_v1_PostalAddress_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::PostalAddress>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_PostalAddress_free(const google::protobuf::Map<int64_t, ::registry_api::v1::PostalAddress>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_PostalAddress_clear(google::protobuf::Map<int64_t, ::registry_api::v1::PostalAddress> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_PostalAddress_size(const google::protobuf::Map<int64_t, ::registry_api::v1::PostalAddress>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_PostalAddress_insert(google::protobuf::Map<int64_t, ::registry_api::v1::PostalAddress> * m,
                          int64_t key, ::registry_api::v1::PostalAddress value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_PostalAddress_get(const google::protobuf::Map<int64_t, ::registry_api::v1::PostalAddress>* m,
                       int64_t key, const ::registry_api::v1::PostalAddress** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::PostalAddress* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_PostalAddress_remove(google::protobuf::Map<int64_t, ::registry_api::v1::PostalAddress> * m,
                          int64_t key, ::registry_api::v1::PostalAddress * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_PostalAddress_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::PostalAddress>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_PostalAddress_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::PostalAddress** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::PostalAddress>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::PostalAddress>* __rust_proto_thunk__Map_u64_registry_api_v1_PostalAddress_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::PostalAddress>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_PostalAddress_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::PostalAddress>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_PostalAddress_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::PostalAddress> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_PostalAddress_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::PostalAddress>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_PostalAddress_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::PostalAddress> * m,
                          uint64_t key, ::registry_api::v1::PostalAddress value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_PostalAddress_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::PostalAddress>* m,
                       uint64_t key, const ::registry_api::v1::PostalAddress** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::PostalAddress* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_PostalAddress_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::PostalAddress> * m,
                          uint64_t key, ::registry_api::v1::PostalAddress * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_PostalAddress_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::PostalAddress>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_PostalAddress_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::PostalAddress** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::PostalAddress>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::PostalAddress>* __rust_proto_thunk__Map_bool_registry_api_v1_PostalAddress_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::PostalAddress>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_PostalAddress_free(const google::protobuf::Map<bool, ::registry_api::v1::PostalAddress>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_PostalAddress_clear(google::protobuf::Map<bool, ::registry_api::v1::PostalAddress> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_PostalAddress_size(const google::protobuf::Map<bool, ::registry_api::v1::PostalAddress>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_PostalAddress_insert(google::protobuf::Map<bool, ::registry_api::v1::PostalAddress> * m,
                          bool key, ::registry_api::v1::PostalAddress value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_PostalAddress_get(const google::protobuf::Map<bool, ::registry_api::v1::PostalAddress>* m,
                       bool key, const ::registry_api::v1::PostalAddress** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::PostalAddress* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_PostalAddress_remove(google::protobuf::Map<bool, ::registry_api::v1::PostalAddress> * m,
                          bool key, ::registry_api::v1::PostalAddress * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_PostalAddress_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::PostalAddress>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_PostalAddress_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::PostalAddress** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::PostalAddress>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::PostalAddress>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_PostalAddress_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::PostalAddress>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_PostalAddress_free(const google::protobuf::Map<std::string, ::registry_api::v1::PostalAddress>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_PostalAddress_clear(google::protobuf::Map<std::string, ::registry_api::v1::PostalAddress> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_PostalAddress_size(const google::protobuf::Map<std::string, ::registry_api::v1::PostalAddress>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_PostalAddress_insert(google::protobuf::Map<std::string, ::registry_api::v1::PostalAddress> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::PostalAddress value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_PostalAddress_get(const google::protobuf::Map<std::string, ::registry_api::v1::PostalAddress>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::PostalAddress** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::PostalAddress* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_PostalAddress_remove(google::protobuf::Map<std::string, ::registry_api::v1::PostalAddress> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::PostalAddress * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_PostalAddress_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::PostalAddress>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_PostalAddress_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::PostalAddress** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::PostalAddress>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.CredentialIssuer
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_CredentialIssuer_new() { return new ::registry_api::v1::CredentialIssuer(); }
void __rust_proto_thunk__registry_api_v1_CredentialIssuer_delete(void* ptr) { delete static_cast<::registry_api::v1::CredentialIssuer*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_CredentialIssuer_serialize(::registry_api::v1::CredentialIssuer* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_CredentialIssuer_parse(::registry_api::v1::CredentialIssuer* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_CredentialIssuer_copy_from(::registry_api::v1::CredentialIssuer* dst, const ::registry_api::v1::CredentialIssuer* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_CredentialIssuer_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::CredentialIssuer>* field) {
  return field->size();
}
const ::registry_api::v1::CredentialIssuer& __rust_proto_thunk__registry_api_v1_CredentialIssuer_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::CredentialIssuer>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::CredentialIssuer* __rust_proto_thunk__registry_api_v1_CredentialIssuer_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::CredentialIssuer>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::CredentialIssuer* __rust_proto_thunk__registry_api_v1_CredentialIssuer_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::CredentialIssuer>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_CredentialIssuer_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::CredentialIssuer>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_CredentialIssuer_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::CredentialIssuer>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::CredentialIssuer>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_CredentialIssuer_get_issuer_did(::registry_api::v1::CredentialIssuer* msg) {
  absl::string_view val = msg->issuer_did();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_CredentialIssuer_set_issuer_did(::registry_api::v1::CredentialIssuer* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_issuer_did(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_CredentialIssuer_get_issuer_name(::registry_api::v1::CredentialIssuer* msg) {
  absl::string_view val = msg->issuer_name();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_CredentialIssuer_set_issuer_name(::registry_api::v1::CredentialIssuer* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_issuer_name(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_CredentialIssuer_get_email(::registry_api::v1::CredentialIssuer* msg) {
  absl::string_view val = msg->email();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_CredentialIssuer_set_email(::registry_api::v1::CredentialIssuer* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_email(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_CredentialIssuer_get_website(::registry_api::v1::CredentialIssuer* msg) {
  absl::string_view val = msg->website();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_CredentialIssuer_set_website(::registry_api::v1::CredentialIssuer* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_website(absl::string_view(s.ptr, s.len));
}

const void* __rust_proto_thunk__registry_api_v1_CredentialIssuer_get_postal_address(::registry_api::v1::CredentialIssuer* msg) {
  return static_cast<const void*>(&msg->postal_address());
}
void* __rust_proto_thunk__registry_api_v1_CredentialIssuer_get_mut_postal_address(::registry_api::v1::CredentialIssuer* msg) {
  return static_cast<void*>(msg->mutable_postal_address());
}
void __rust_proto_thunk__registry_api_v1_CredentialIssuer_clear_postal_address(::registry_api::v1::CredentialIssuer* msg) { msg->clear_postal_address(); }
bool __rust_proto_thunk__registry_api_v1_CredentialIssuer_has_postal_address(::registry_api::v1::CredentialIssuer* msg) { return msg->has_postal_address(); }

::registry_api::v1::IssuerStatus __rust_proto_thunk__registry_api_v1_CredentialIssuer_get_status(::registry_api::v1::CredentialIssuer* msg) { return msg->status(); }
void __rust_proto_thunk__registry_api_v1_CredentialIssuer_set_status(::registry_api::v1::CredentialIssuer* msg, ::registry_api::v1::IssuerStatus val) {
  msg->set_status(val);
}

const void* __rust_proto_thunk__registry_api_v1_CredentialIssuer_get_metadata(::registry_api::v1::CredentialIssuer* msg) {
  return static_cast<const void*>(&msg->metadata());
}
void* __rust_proto_thunk__registry_api_v1_CredentialIssuer_get_mut_metadata(::registry_api::v1::CredentialIssuer* msg) {
  return static_cast<void*>(msg->mutable_metadata());
}
void __rust_proto_thunk__registry_api_v1_CredentialIssuer_clear_metadata(::registry_api::v1::CredentialIssuer* msg) { msg->clear_metadata(); }
bool __rust_proto_thunk__registry_api_v1_CredentialIssuer_has_metadata(::registry_api::v1::CredentialIssuer* msg) { return msg->has_metadata(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::CredentialIssuer>* __rust_proto_thunk__Map_i32_registry_api_v1_CredentialIssuer_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::CredentialIssuer>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_CredentialIssuer_free(const google::protobuf::Map<int32_t, ::registry_api::v1::CredentialIssuer>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_CredentialIssuer_clear(google::protobuf::Map<int32_t, ::registry_api::v1::CredentialIssuer> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_CredentialIssuer_size(const google::protobuf::Map<int32_t, ::registry_api::v1::CredentialIssuer>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_CredentialIssuer_insert(google::protobuf::Map<int32_t, ::registry_api::v1::CredentialIssuer> * m,
                          int32_t key, ::registry_api::v1::CredentialIssuer value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_CredentialIssuer_get(const google::protobuf::Map<int32_t, ::registry_api::v1::CredentialIssuer>* m,
                       int32_t key, const ::registry_api::v1::CredentialIssuer** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CredentialIssuer* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_CredentialIssuer_remove(google::protobuf::Map<int32_t, ::registry_api::v1::CredentialIssuer> * m,
                          int32_t key, ::registry_api::v1::CredentialIssuer * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_CredentialIssuer_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::CredentialIssuer>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_CredentialIssuer_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::CredentialIssuer** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::CredentialIssuer>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::CredentialIssuer>* __rust_proto_thunk__Map_u32_registry_api_v1_CredentialIssuer_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::CredentialIssuer>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_CredentialIssuer_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::CredentialIssuer>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_CredentialIssuer_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::CredentialIssuer> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_CredentialIssuer_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::CredentialIssuer>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_CredentialIssuer_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::CredentialIssuer> * m,
                          uint32_t key, ::registry_api::v1::CredentialIssuer value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_CredentialIssuer_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::CredentialIssuer>* m,
                       uint32_t key, const ::registry_api::v1::CredentialIssuer** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CredentialIssuer* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_CredentialIssuer_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::CredentialIssuer> * m,
                          uint32_t key, ::registry_api::v1::CredentialIssuer * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_CredentialIssuer_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::CredentialIssuer>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_CredentialIssuer_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::CredentialIssuer** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::CredentialIssuer>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::CredentialIssuer>* __rust_proto_thunk__Map_i64_registry_api_v1_CredentialIssuer_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::CredentialIssuer>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_CredentialIssuer_free(const google::protobuf::Map<int64_t, ::registry_api::v1::CredentialIssuer>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_CredentialIssuer_clear(google::protobuf::Map<int64_t, ::registry_api::v1::CredentialIssuer> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_CredentialIssuer_size(const google::protobuf::Map<int64_t, ::registry_api::v1::CredentialIssuer>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_CredentialIssuer_insert(google::protobuf::Map<int64_t, ::registry_api::v1::CredentialIssuer> * m,
                          int64_t key, ::registry_api::v1::CredentialIssuer value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_CredentialIssuer_get(const google::protobuf::Map<int64_t, ::registry_api::v1::CredentialIssuer>* m,
                       int64_t key, const ::registry_api::v1::CredentialIssuer** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CredentialIssuer* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_CredentialIssuer_remove(google::protobuf::Map<int64_t, ::registry_api::v1::CredentialIssuer> * m,
                          int64_t key, ::registry_api::v1::CredentialIssuer * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_CredentialIssuer_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::CredentialIssuer>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_CredentialIssuer_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::CredentialIssuer** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::CredentialIssuer>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::CredentialIssuer>* __rust_proto_thunk__Map_u64_registry_api_v1_CredentialIssuer_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::CredentialIssuer>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_CredentialIssuer_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::CredentialIssuer>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_CredentialIssuer_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::CredentialIssuer> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_CredentialIssuer_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::CredentialIssuer>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_CredentialIssuer_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::CredentialIssuer> * m,
                          uint64_t key, ::registry_api::v1::CredentialIssuer value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_CredentialIssuer_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::CredentialIssuer>* m,
                       uint64_t key, const ::registry_api::v1::CredentialIssuer** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CredentialIssuer* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_CredentialIssuer_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::CredentialIssuer> * m,
                          uint64_t key, ::registry_api::v1::CredentialIssuer * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_CredentialIssuer_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::CredentialIssuer>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_CredentialIssuer_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::CredentialIssuer** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::CredentialIssuer>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::CredentialIssuer>* __rust_proto_thunk__Map_bool_registry_api_v1_CredentialIssuer_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::CredentialIssuer>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_CredentialIssuer_free(const google::protobuf::Map<bool, ::registry_api::v1::CredentialIssuer>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_CredentialIssuer_clear(google::protobuf::Map<bool, ::registry_api::v1::CredentialIssuer> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_CredentialIssuer_size(const google::protobuf::Map<bool, ::registry_api::v1::CredentialIssuer>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_CredentialIssuer_insert(google::protobuf::Map<bool, ::registry_api::v1::CredentialIssuer> * m,
                          bool key, ::registry_api::v1::CredentialIssuer value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_CredentialIssuer_get(const google::protobuf::Map<bool, ::registry_api::v1::CredentialIssuer>* m,
                       bool key, const ::registry_api::v1::CredentialIssuer** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CredentialIssuer* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_CredentialIssuer_remove(google::protobuf::Map<bool, ::registry_api::v1::CredentialIssuer> * m,
                          bool key, ::registry_api::v1::CredentialIssuer * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_CredentialIssuer_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::CredentialIssuer>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_CredentialIssuer_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::CredentialIssuer** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::CredentialIssuer>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::CredentialIssuer>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CredentialIssuer_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::CredentialIssuer>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CredentialIssuer_free(const google::protobuf::Map<std::string, ::registry_api::v1::CredentialIssuer>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CredentialIssuer_clear(google::protobuf::Map<std::string, ::registry_api::v1::CredentialIssuer> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CredentialIssuer_size(const google::protobuf::Map<std::string, ::registry_api::v1::CredentialIssuer>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CredentialIssuer_insert(google::protobuf::Map<std::string, ::registry_api::v1::CredentialIssuer> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::CredentialIssuer value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CredentialIssuer_get(const google::protobuf::Map<std::string, ::registry_api::v1::CredentialIssuer>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::CredentialIssuer** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CredentialIssuer* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CredentialIssuer_remove(google::protobuf::Map<std::string, ::registry_api::v1::CredentialIssuer> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::CredentialIssuer * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CredentialIssuer_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::CredentialIssuer>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CredentialIssuer_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::CredentialIssuer** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::CredentialIssuer>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.CreateCredentialIssuerRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerRequest_new() { return new ::registry_api::v1::CreateCredentialIssuerRequest(); }
void __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerRequest_delete(void* ptr) { delete static_cast<::registry_api::v1::CreateCredentialIssuerRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerRequest_serialize(::registry_api::v1::CreateCredentialIssuerRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerRequest_parse(::registry_api::v1::CreateCredentialIssuerRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerRequest_copy_from(::registry_api::v1::CreateCredentialIssuerRequest* dst, const ::registry_api::v1::CreateCredentialIssuerRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerRequest_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::CreateCredentialIssuerRequest>* field) {
  return field->size();
}
const ::registry_api::v1::CreateCredentialIssuerRequest& __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::CreateCredentialIssuerRequest>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::CreateCredentialIssuerRequest* __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::CreateCredentialIssuerRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::CreateCredentialIssuerRequest* __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerRequest_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::CreateCredentialIssuerRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerRequest_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::CreateCredentialIssuerRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::CreateCredentialIssuerRequest>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::CreateCredentialIssuerRequest>& src) {
  dst = src;
}

const void* __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerRequest_get_credential_issuer(::registry_api::v1::CreateCredentialIssuerRequest* msg) {
  return static_cast<const void*>(&msg->credential_issuer());
}
void* __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerRequest_get_mut_credential_issuer(::registry_api::v1::CreateCredentialIssuerRequest* msg) {
  return static_cast<void*>(msg->mutable_credential_issuer());
}
void __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerRequest_clear_credential_issuer(::registry_api::v1::CreateCredentialIssuerRequest* msg) { msg->clear_credential_issuer(); }
bool __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerRequest_has_credential_issuer(::registry_api::v1::CreateCredentialIssuerRequest* msg) { return msg->has_credential_issuer(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::CreateCredentialIssuerRequest>* __rust_proto_thunk__Map_i32_registry_api_v1_CreateCredentialIssuerRequest_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::CreateCredentialIssuerRequest>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_CreateCredentialIssuerRequest_free(const google::protobuf::Map<int32_t, ::registry_api::v1::CreateCredentialIssuerRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_CreateCredentialIssuerRequest_clear(google::protobuf::Map<int32_t, ::registry_api::v1::CreateCredentialIssuerRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_CreateCredentialIssuerRequest_size(const google::protobuf::Map<int32_t, ::registry_api::v1::CreateCredentialIssuerRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_CreateCredentialIssuerRequest_insert(google::protobuf::Map<int32_t, ::registry_api::v1::CreateCredentialIssuerRequest> * m,
                          int32_t key, ::registry_api::v1::CreateCredentialIssuerRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_CreateCredentialIssuerRequest_get(const google::protobuf::Map<int32_t, ::registry_api::v1::CreateCredentialIssuerRequest>* m,
                       int32_t key, const ::registry_api::v1::CreateCredentialIssuerRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateCredentialIssuerRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_CreateCredentialIssuerRequest_remove(google::protobuf::Map<int32_t, ::registry_api::v1::CreateCredentialIssuerRequest> * m,
                          int32_t key, ::registry_api::v1::CreateCredentialIssuerRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_CreateCredentialIssuerRequest_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::CreateCredentialIssuerRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_CreateCredentialIssuerRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::CreateCredentialIssuerRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::CreateCredentialIssuerRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateCredentialIssuerRequest>* __rust_proto_thunk__Map_u32_registry_api_v1_CreateCredentialIssuerRequest_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::CreateCredentialIssuerRequest>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_CreateCredentialIssuerRequest_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateCredentialIssuerRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_CreateCredentialIssuerRequest_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::CreateCredentialIssuerRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_CreateCredentialIssuerRequest_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateCredentialIssuerRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_CreateCredentialIssuerRequest_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::CreateCredentialIssuerRequest> * m,
                          uint32_t key, ::registry_api::v1::CreateCredentialIssuerRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_CreateCredentialIssuerRequest_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateCredentialIssuerRequest>* m,
                       uint32_t key, const ::registry_api::v1::CreateCredentialIssuerRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateCredentialIssuerRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_CreateCredentialIssuerRequest_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::CreateCredentialIssuerRequest> * m,
                          uint32_t key, ::registry_api::v1::CreateCredentialIssuerRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_CreateCredentialIssuerRequest_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateCredentialIssuerRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_CreateCredentialIssuerRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::CreateCredentialIssuerRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::CreateCredentialIssuerRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::CreateCredentialIssuerRequest>* __rust_proto_thunk__Map_i64_registry_api_v1_CreateCredentialIssuerRequest_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::CreateCredentialIssuerRequest>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_CreateCredentialIssuerRequest_free(const google::protobuf::Map<int64_t, ::registry_api::v1::CreateCredentialIssuerRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_CreateCredentialIssuerRequest_clear(google::protobuf::Map<int64_t, ::registry_api::v1::CreateCredentialIssuerRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_CreateCredentialIssuerRequest_size(const google::protobuf::Map<int64_t, ::registry_api::v1::CreateCredentialIssuerRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_CreateCredentialIssuerRequest_insert(google::protobuf::Map<int64_t, ::registry_api::v1::CreateCredentialIssuerRequest> * m,
                          int64_t key, ::registry_api::v1::CreateCredentialIssuerRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_CreateCredentialIssuerRequest_get(const google::protobuf::Map<int64_t, ::registry_api::v1::CreateCredentialIssuerRequest>* m,
                       int64_t key, const ::registry_api::v1::CreateCredentialIssuerRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateCredentialIssuerRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_CreateCredentialIssuerRequest_remove(google::protobuf::Map<int64_t, ::registry_api::v1::CreateCredentialIssuerRequest> * m,
                          int64_t key, ::registry_api::v1::CreateCredentialIssuerRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_CreateCredentialIssuerRequest_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::CreateCredentialIssuerRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_CreateCredentialIssuerRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::CreateCredentialIssuerRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::CreateCredentialIssuerRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateCredentialIssuerRequest>* __rust_proto_thunk__Map_u64_registry_api_v1_CreateCredentialIssuerRequest_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::CreateCredentialIssuerRequest>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_CreateCredentialIssuerRequest_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateCredentialIssuerRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_CreateCredentialIssuerRequest_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::CreateCredentialIssuerRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_CreateCredentialIssuerRequest_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateCredentialIssuerRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_CreateCredentialIssuerRequest_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::CreateCredentialIssuerRequest> * m,
                          uint64_t key, ::registry_api::v1::CreateCredentialIssuerRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_CreateCredentialIssuerRequest_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateCredentialIssuerRequest>* m,
                       uint64_t key, const ::registry_api::v1::CreateCredentialIssuerRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateCredentialIssuerRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_CreateCredentialIssuerRequest_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::CreateCredentialIssuerRequest> * m,
                          uint64_t key, ::registry_api::v1::CreateCredentialIssuerRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_CreateCredentialIssuerRequest_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateCredentialIssuerRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_CreateCredentialIssuerRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::CreateCredentialIssuerRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::CreateCredentialIssuerRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::CreateCredentialIssuerRequest>* __rust_proto_thunk__Map_bool_registry_api_v1_CreateCredentialIssuerRequest_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::CreateCredentialIssuerRequest>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_CreateCredentialIssuerRequest_free(const google::protobuf::Map<bool, ::registry_api::v1::CreateCredentialIssuerRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_CreateCredentialIssuerRequest_clear(google::protobuf::Map<bool, ::registry_api::v1::CreateCredentialIssuerRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_CreateCredentialIssuerRequest_size(const google::protobuf::Map<bool, ::registry_api::v1::CreateCredentialIssuerRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_CreateCredentialIssuerRequest_insert(google::protobuf::Map<bool, ::registry_api::v1::CreateCredentialIssuerRequest> * m,
                          bool key, ::registry_api::v1::CreateCredentialIssuerRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_CreateCredentialIssuerRequest_get(const google::protobuf::Map<bool, ::registry_api::v1::CreateCredentialIssuerRequest>* m,
                       bool key, const ::registry_api::v1::CreateCredentialIssuerRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateCredentialIssuerRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_CreateCredentialIssuerRequest_remove(google::protobuf::Map<bool, ::registry_api::v1::CreateCredentialIssuerRequest> * m,
                          bool key, ::registry_api::v1::CreateCredentialIssuerRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_CreateCredentialIssuerRequest_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::CreateCredentialIssuerRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_CreateCredentialIssuerRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::CreateCredentialIssuerRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::CreateCredentialIssuerRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::CreateCredentialIssuerRequest>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateCredentialIssuerRequest_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::CreateCredentialIssuerRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateCredentialIssuerRequest_free(const google::protobuf::Map<std::string, ::registry_api::v1::CreateCredentialIssuerRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateCredentialIssuerRequest_clear(google::protobuf::Map<std::string, ::registry_api::v1::CreateCredentialIssuerRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateCredentialIssuerRequest_size(const google::protobuf::Map<std::string, ::registry_api::v1::CreateCredentialIssuerRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateCredentialIssuerRequest_insert(google::protobuf::Map<std::string, ::registry_api::v1::CreateCredentialIssuerRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::CreateCredentialIssuerRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateCredentialIssuerRequest_get(const google::protobuf::Map<std::string, ::registry_api::v1::CreateCredentialIssuerRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::CreateCredentialIssuerRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateCredentialIssuerRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateCredentialIssuerRequest_remove(google::protobuf::Map<std::string, ::registry_api::v1::CreateCredentialIssuerRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::CreateCredentialIssuerRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateCredentialIssuerRequest_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::CreateCredentialIssuerRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateCredentialIssuerRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::CreateCredentialIssuerRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::CreateCredentialIssuerRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.CreateCredentialIssuerResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerResponse_new() { return new ::registry_api::v1::CreateCredentialIssuerResponse(); }
void __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerResponse_delete(void* ptr) { delete static_cast<::registry_api::v1::CreateCredentialIssuerResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerResponse_serialize(::registry_api::v1::CreateCredentialIssuerResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerResponse_parse(::registry_api::v1::CreateCredentialIssuerResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerResponse_copy_from(::registry_api::v1::CreateCredentialIssuerResponse* dst, const ::registry_api::v1::CreateCredentialIssuerResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerResponse_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::CreateCredentialIssuerResponse>* field) {
  return field->size();
}
const ::registry_api::v1::CreateCredentialIssuerResponse& __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::CreateCredentialIssuerResponse>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::CreateCredentialIssuerResponse* __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::CreateCredentialIssuerResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::CreateCredentialIssuerResponse* __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerResponse_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::CreateCredentialIssuerResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerResponse_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::CreateCredentialIssuerResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::CreateCredentialIssuerResponse>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::CreateCredentialIssuerResponse>& src) {
  dst = src;
}

bool __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerResponse_get_ok(::registry_api::v1::CreateCredentialIssuerResponse* msg) { return msg->ok(); }
void __rust_proto_thunk__registry_api_v1_CreateCredentialIssuerResponse_set_ok(::registry_api::v1::CreateCredentialIssuerResponse* msg, bool val) {
  msg->set_ok(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::CreateCredentialIssuerResponse>* __rust_proto_thunk__Map_i32_registry_api_v1_CreateCredentialIssuerResponse_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::CreateCredentialIssuerResponse>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_CreateCredentialIssuerResponse_free(const google::protobuf::Map<int32_t, ::registry_api::v1::CreateCredentialIssuerResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_CreateCredentialIssuerResponse_clear(google::protobuf::Map<int32_t, ::registry_api::v1::CreateCredentialIssuerResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_CreateCredentialIssuerResponse_size(const google::protobuf::Map<int32_t, ::registry_api::v1::CreateCredentialIssuerResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_CreateCredentialIssuerResponse_insert(google::protobuf::Map<int32_t, ::registry_api::v1::CreateCredentialIssuerResponse> * m,
                          int32_t key, ::registry_api::v1::CreateCredentialIssuerResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_CreateCredentialIssuerResponse_get(const google::protobuf::Map<int32_t, ::registry_api::v1::CreateCredentialIssuerResponse>* m,
                       int32_t key, const ::registry_api::v1::CreateCredentialIssuerResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateCredentialIssuerResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_CreateCredentialIssuerResponse_remove(google::protobuf::Map<int32_t, ::registry_api::v1::CreateCredentialIssuerResponse> * m,
                          int32_t key, ::registry_api::v1::CreateCredentialIssuerResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_CreateCredentialIssuerResponse_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::CreateCredentialIssuerResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_CreateCredentialIssuerResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::CreateCredentialIssuerResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::CreateCredentialIssuerResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateCredentialIssuerResponse>* __rust_proto_thunk__Map_u32_registry_api_v1_CreateCredentialIssuerResponse_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::CreateCredentialIssuerResponse>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_CreateCredentialIssuerResponse_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateCredentialIssuerResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_CreateCredentialIssuerResponse_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::CreateCredentialIssuerResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_CreateCredentialIssuerResponse_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateCredentialIssuerResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_CreateCredentialIssuerResponse_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::CreateCredentialIssuerResponse> * m,
                          uint32_t key, ::registry_api::v1::CreateCredentialIssuerResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_CreateCredentialIssuerResponse_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateCredentialIssuerResponse>* m,
                       uint32_t key, const ::registry_api::v1::CreateCredentialIssuerResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateCredentialIssuerResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_CreateCredentialIssuerResponse_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::CreateCredentialIssuerResponse> * m,
                          uint32_t key, ::registry_api::v1::CreateCredentialIssuerResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_CreateCredentialIssuerResponse_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateCredentialIssuerResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_CreateCredentialIssuerResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::CreateCredentialIssuerResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::CreateCredentialIssuerResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::CreateCredentialIssuerResponse>* __rust_proto_thunk__Map_i64_registry_api_v1_CreateCredentialIssuerResponse_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::CreateCredentialIssuerResponse>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_CreateCredentialIssuerResponse_free(const google::protobuf::Map<int64_t, ::registry_api::v1::CreateCredentialIssuerResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_CreateCredentialIssuerResponse_clear(google::protobuf::Map<int64_t, ::registry_api::v1::CreateCredentialIssuerResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_CreateCredentialIssuerResponse_size(const google::protobuf::Map<int64_t, ::registry_api::v1::CreateCredentialIssuerResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_CreateCredentialIssuerResponse_insert(google::protobuf::Map<int64_t, ::registry_api::v1::CreateCredentialIssuerResponse> * m,
                          int64_t key, ::registry_api::v1::CreateCredentialIssuerResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_CreateCredentialIssuerResponse_get(const google::protobuf::Map<int64_t, ::registry_api::v1::CreateCredentialIssuerResponse>* m,
                       int64_t key, const ::registry_api::v1::CreateCredentialIssuerResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateCredentialIssuerResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_CreateCredentialIssuerResponse_remove(google::protobuf::Map<int64_t, ::registry_api::v1::CreateCredentialIssuerResponse> * m,
                          int64_t key, ::registry_api::v1::CreateCredentialIssuerResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_CreateCredentialIssuerResponse_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::CreateCredentialIssuerResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_CreateCredentialIssuerResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::CreateCredentialIssuerResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::CreateCredentialIssuerResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateCredentialIssuerResponse>* __rust_proto_thunk__Map_u64_registry_api_v1_CreateCredentialIssuerResponse_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::CreateCredentialIssuerResponse>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_CreateCredentialIssuerResponse_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateCredentialIssuerResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_CreateCredentialIssuerResponse_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::CreateCredentialIssuerResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_CreateCredentialIssuerResponse_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateCredentialIssuerResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_CreateCredentialIssuerResponse_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::CreateCredentialIssuerResponse> * m,
                          uint64_t key, ::registry_api::v1::CreateCredentialIssuerResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_CreateCredentialIssuerResponse_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateCredentialIssuerResponse>* m,
                       uint64_t key, const ::registry_api::v1::CreateCredentialIssuerResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateCredentialIssuerResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_CreateCredentialIssuerResponse_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::CreateCredentialIssuerResponse> * m,
                          uint64_t key, ::registry_api::v1::CreateCredentialIssuerResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_CreateCredentialIssuerResponse_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateCredentialIssuerResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_CreateCredentialIssuerResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::CreateCredentialIssuerResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::CreateCredentialIssuerResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::CreateCredentialIssuerResponse>* __rust_proto_thunk__Map_bool_registry_api_v1_CreateCredentialIssuerResponse_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::CreateCredentialIssuerResponse>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_CreateCredentialIssuerResponse_free(const google::protobuf::Map<bool, ::registry_api::v1::CreateCredentialIssuerResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_CreateCredentialIssuerResponse_clear(google::protobuf::Map<bool, ::registry_api::v1::CreateCredentialIssuerResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_CreateCredentialIssuerResponse_size(const google::protobuf::Map<bool, ::registry_api::v1::CreateCredentialIssuerResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_CreateCredentialIssuerResponse_insert(google::protobuf::Map<bool, ::registry_api::v1::CreateCredentialIssuerResponse> * m,
                          bool key, ::registry_api::v1::CreateCredentialIssuerResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_CreateCredentialIssuerResponse_get(const google::protobuf::Map<bool, ::registry_api::v1::CreateCredentialIssuerResponse>* m,
                       bool key, const ::registry_api::v1::CreateCredentialIssuerResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateCredentialIssuerResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_CreateCredentialIssuerResponse_remove(google::protobuf::Map<bool, ::registry_api::v1::CreateCredentialIssuerResponse> * m,
                          bool key, ::registry_api::v1::CreateCredentialIssuerResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_CreateCredentialIssuerResponse_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::CreateCredentialIssuerResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_CreateCredentialIssuerResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::CreateCredentialIssuerResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::CreateCredentialIssuerResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::CreateCredentialIssuerResponse>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateCredentialIssuerResponse_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::CreateCredentialIssuerResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateCredentialIssuerResponse_free(const google::protobuf::Map<std::string, ::registry_api::v1::CreateCredentialIssuerResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateCredentialIssuerResponse_clear(google::protobuf::Map<std::string, ::registry_api::v1::CreateCredentialIssuerResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateCredentialIssuerResponse_size(const google::protobuf::Map<std::string, ::registry_api::v1::CreateCredentialIssuerResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateCredentialIssuerResponse_insert(google::protobuf::Map<std::string, ::registry_api::v1::CreateCredentialIssuerResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::CreateCredentialIssuerResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateCredentialIssuerResponse_get(const google::protobuf::Map<std::string, ::registry_api::v1::CreateCredentialIssuerResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::CreateCredentialIssuerResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateCredentialIssuerResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateCredentialIssuerResponse_remove(google::protobuf::Map<std::string, ::registry_api::v1::CreateCredentialIssuerResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::CreateCredentialIssuerResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateCredentialIssuerResponse_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::CreateCredentialIssuerResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateCredentialIssuerResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::CreateCredentialIssuerResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::CreateCredentialIssuerResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.GetCredentialIssuerRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_GetCredentialIssuerRequest_new() { return new ::registry_api::v1::GetCredentialIssuerRequest(); }
void __rust_proto_thunk__registry_api_v1_GetCredentialIssuerRequest_delete(void* ptr) { delete static_cast<::registry_api::v1::GetCredentialIssuerRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_GetCredentialIssuerRequest_serialize(::registry_api::v1::GetCredentialIssuerRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_GetCredentialIssuerRequest_parse(::registry_api::v1::GetCredentialIssuerRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_GetCredentialIssuerRequest_copy_from(::registry_api::v1::GetCredentialIssuerRequest* dst, const ::registry_api::v1::GetCredentialIssuerRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_GetCredentialIssuerRequest_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::GetCredentialIssuerRequest>* field) {
  return field->size();
}
const ::registry_api::v1::GetCredentialIssuerRequest& __rust_proto_thunk__registry_api_v1_GetCredentialIssuerRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::GetCredentialIssuerRequest>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::GetCredentialIssuerRequest* __rust_proto_thunk__registry_api_v1_GetCredentialIssuerRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::GetCredentialIssuerRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::GetCredentialIssuerRequest* __rust_proto_thunk__registry_api_v1_GetCredentialIssuerRequest_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::GetCredentialIssuerRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_GetCredentialIssuerRequest_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::GetCredentialIssuerRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_GetCredentialIssuerRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::GetCredentialIssuerRequest>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::GetCredentialIssuerRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_GetCredentialIssuerRequest_get_issuer_did(::registry_api::v1::GetCredentialIssuerRequest* msg) {
  absl::string_view val = msg->issuer_did();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_GetCredentialIssuerRequest_set_issuer_did(::registry_api::v1::GetCredentialIssuerRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_issuer_did(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::GetCredentialIssuerRequest>* __rust_proto_thunk__Map_i32_registry_api_v1_GetCredentialIssuerRequest_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::GetCredentialIssuerRequest>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_GetCredentialIssuerRequest_free(const google::protobuf::Map<int32_t, ::registry_api::v1::GetCredentialIssuerRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_GetCredentialIssuerRequest_clear(google::protobuf::Map<int32_t, ::registry_api::v1::GetCredentialIssuerRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_GetCredentialIssuerRequest_size(const google::protobuf::Map<int32_t, ::registry_api::v1::GetCredentialIssuerRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_GetCredentialIssuerRequest_insert(google::protobuf::Map<int32_t, ::registry_api::v1::GetCredentialIssuerRequest> * m,
                          int32_t key, ::registry_api::v1::GetCredentialIssuerRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_GetCredentialIssuerRequest_get(const google::protobuf::Map<int32_t, ::registry_api::v1::GetCredentialIssuerRequest>* m,
                       int32_t key, const ::registry_api::v1::GetCredentialIssuerRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetCredentialIssuerRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_GetCredentialIssuerRequest_remove(google::protobuf::Map<int32_t, ::registry_api::v1::GetCredentialIssuerRequest> * m,
                          int32_t key, ::registry_api::v1::GetCredentialIssuerRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_GetCredentialIssuerRequest_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::GetCredentialIssuerRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_GetCredentialIssuerRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::GetCredentialIssuerRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::GetCredentialIssuerRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::GetCredentialIssuerRequest>* __rust_proto_thunk__Map_u32_registry_api_v1_GetCredentialIssuerRequest_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::GetCredentialIssuerRequest>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_GetCredentialIssuerRequest_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::GetCredentialIssuerRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_GetCredentialIssuerRequest_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::GetCredentialIssuerRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_GetCredentialIssuerRequest_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::GetCredentialIssuerRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_GetCredentialIssuerRequest_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::GetCredentialIssuerRequest> * m,
                          uint32_t key, ::registry_api::v1::GetCredentialIssuerRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_GetCredentialIssuerRequest_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::GetCredentialIssuerRequest>* m,
                       uint32_t key, const ::registry_api::v1::GetCredentialIssuerRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetCredentialIssuerRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_GetCredentialIssuerRequest_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::GetCredentialIssuerRequest> * m,
                          uint32_t key, ::registry_api::v1::GetCredentialIssuerRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_GetCredentialIssuerRequest_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::GetCredentialIssuerRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_GetCredentialIssuerRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::GetCredentialIssuerRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::GetCredentialIssuerRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::GetCredentialIssuerRequest>* __rust_proto_thunk__Map_i64_registry_api_v1_GetCredentialIssuerRequest_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::GetCredentialIssuerRequest>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_GetCredentialIssuerRequest_free(const google::protobuf::Map<int64_t, ::registry_api::v1::GetCredentialIssuerRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_GetCredentialIssuerRequest_clear(google::protobuf::Map<int64_t, ::registry_api::v1::GetCredentialIssuerRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_GetCredentialIssuerRequest_size(const google::protobuf::Map<int64_t, ::registry_api::v1::GetCredentialIssuerRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_GetCredentialIssuerRequest_insert(google::protobuf::Map<int64_t, ::registry_api::v1::GetCredentialIssuerRequest> * m,
                          int64_t key, ::registry_api::v1::GetCredentialIssuerRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_GetCredentialIssuerRequest_get(const google::protobuf::Map<int64_t, ::registry_api::v1::GetCredentialIssuerRequest>* m,
                       int64_t key, const ::registry_api::v1::GetCredentialIssuerRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetCredentialIssuerRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_GetCredentialIssuerRequest_remove(google::protobuf::Map<int64_t, ::registry_api::v1::GetCredentialIssuerRequest> * m,
                          int64_t key, ::registry_api::v1::GetCredentialIssuerRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_GetCredentialIssuerRequest_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::GetCredentialIssuerRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_GetCredentialIssuerRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::GetCredentialIssuerRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::GetCredentialIssuerRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::GetCredentialIssuerRequest>* __rust_proto_thunk__Map_u64_registry_api_v1_GetCredentialIssuerRequest_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::GetCredentialIssuerRequest>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_GetCredentialIssuerRequest_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::GetCredentialIssuerRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_GetCredentialIssuerRequest_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::GetCredentialIssuerRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_GetCredentialIssuerRequest_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::GetCredentialIssuerRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_GetCredentialIssuerRequest_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::GetCredentialIssuerRequest> * m,
                          uint64_t key, ::registry_api::v1::GetCredentialIssuerRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_GetCredentialIssuerRequest_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::GetCredentialIssuerRequest>* m,
                       uint64_t key, const ::registry_api::v1::GetCredentialIssuerRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetCredentialIssuerRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_GetCredentialIssuerRequest_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::GetCredentialIssuerRequest> * m,
                          uint64_t key, ::registry_api::v1::GetCredentialIssuerRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_GetCredentialIssuerRequest_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::GetCredentialIssuerRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_GetCredentialIssuerRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::GetCredentialIssuerRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::GetCredentialIssuerRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::GetCredentialIssuerRequest>* __rust_proto_thunk__Map_bool_registry_api_v1_GetCredentialIssuerRequest_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::GetCredentialIssuerRequest>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_GetCredentialIssuerRequest_free(const google::protobuf::Map<bool, ::registry_api::v1::GetCredentialIssuerRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_GetCredentialIssuerRequest_clear(google::protobuf::Map<bool, ::registry_api::v1::GetCredentialIssuerRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_GetCredentialIssuerRequest_size(const google::protobuf::Map<bool, ::registry_api::v1::GetCredentialIssuerRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_GetCredentialIssuerRequest_insert(google::protobuf::Map<bool, ::registry_api::v1::GetCredentialIssuerRequest> * m,
                          bool key, ::registry_api::v1::GetCredentialIssuerRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_GetCredentialIssuerRequest_get(const google::protobuf::Map<bool, ::registry_api::v1::GetCredentialIssuerRequest>* m,
                       bool key, const ::registry_api::v1::GetCredentialIssuerRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetCredentialIssuerRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_GetCredentialIssuerRequest_remove(google::protobuf::Map<bool, ::registry_api::v1::GetCredentialIssuerRequest> * m,
                          bool key, ::registry_api::v1::GetCredentialIssuerRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_GetCredentialIssuerRequest_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::GetCredentialIssuerRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_GetCredentialIssuerRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::GetCredentialIssuerRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::GetCredentialIssuerRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::GetCredentialIssuerRequest>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetCredentialIssuerRequest_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::GetCredentialIssuerRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetCredentialIssuerRequest_free(const google::protobuf::Map<std::string, ::registry_api::v1::GetCredentialIssuerRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetCredentialIssuerRequest_clear(google::protobuf::Map<std::string, ::registry_api::v1::GetCredentialIssuerRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetCredentialIssuerRequest_size(const google::protobuf::Map<std::string, ::registry_api::v1::GetCredentialIssuerRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetCredentialIssuerRequest_insert(google::protobuf::Map<std::string, ::registry_api::v1::GetCredentialIssuerRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::GetCredentialIssuerRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetCredentialIssuerRequest_get(const google::protobuf::Map<std::string, ::registry_api::v1::GetCredentialIssuerRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::GetCredentialIssuerRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetCredentialIssuerRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetCredentialIssuerRequest_remove(google::protobuf::Map<std::string, ::registry_api::v1::GetCredentialIssuerRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::GetCredentialIssuerRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetCredentialIssuerRequest_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::GetCredentialIssuerRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetCredentialIssuerRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::GetCredentialIssuerRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::GetCredentialIssuerRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.GetCredentialIssuerResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_GetCredentialIssuerResponse_new() { return new ::registry_api::v1::GetCredentialIssuerResponse(); }
void __rust_proto_thunk__registry_api_v1_GetCredentialIssuerResponse_delete(void* ptr) { delete static_cast<::registry_api::v1::GetCredentialIssuerResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_GetCredentialIssuerResponse_serialize(::registry_api::v1::GetCredentialIssuerResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_GetCredentialIssuerResponse_parse(::registry_api::v1::GetCredentialIssuerResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_GetCredentialIssuerResponse_copy_from(::registry_api::v1::GetCredentialIssuerResponse* dst, const ::registry_api::v1::GetCredentialIssuerResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_GetCredentialIssuerResponse_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::GetCredentialIssuerResponse>* field) {
  return field->size();
}
const ::registry_api::v1::GetCredentialIssuerResponse& __rust_proto_thunk__registry_api_v1_GetCredentialIssuerResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::GetCredentialIssuerResponse>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::GetCredentialIssuerResponse* __rust_proto_thunk__registry_api_v1_GetCredentialIssuerResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::GetCredentialIssuerResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::GetCredentialIssuerResponse* __rust_proto_thunk__registry_api_v1_GetCredentialIssuerResponse_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::GetCredentialIssuerResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_GetCredentialIssuerResponse_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::GetCredentialIssuerResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_GetCredentialIssuerResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::GetCredentialIssuerResponse>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::GetCredentialIssuerResponse>& src) {
  dst = src;
}

const void* __rust_proto_thunk__registry_api_v1_GetCredentialIssuerResponse_get_credential_issuer(::registry_api::v1::GetCredentialIssuerResponse* msg) {
  return static_cast<const void*>(&msg->credential_issuer());
}
void* __rust_proto_thunk__registry_api_v1_GetCredentialIssuerResponse_get_mut_credential_issuer(::registry_api::v1::GetCredentialIssuerResponse* msg) {
  return static_cast<void*>(msg->mutable_credential_issuer());
}
void __rust_proto_thunk__registry_api_v1_GetCredentialIssuerResponse_clear_credential_issuer(::registry_api::v1::GetCredentialIssuerResponse* msg) { msg->clear_credential_issuer(); }
bool __rust_proto_thunk__registry_api_v1_GetCredentialIssuerResponse_has_credential_issuer(::registry_api::v1::GetCredentialIssuerResponse* msg) { return msg->has_credential_issuer(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::GetCredentialIssuerResponse>* __rust_proto_thunk__Map_i32_registry_api_v1_GetCredentialIssuerResponse_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::GetCredentialIssuerResponse>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_GetCredentialIssuerResponse_free(const google::protobuf::Map<int32_t, ::registry_api::v1::GetCredentialIssuerResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_GetCredentialIssuerResponse_clear(google::protobuf::Map<int32_t, ::registry_api::v1::GetCredentialIssuerResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_GetCredentialIssuerResponse_size(const google::protobuf::Map<int32_t, ::registry_api::v1::GetCredentialIssuerResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_GetCredentialIssuerResponse_insert(google::protobuf::Map<int32_t, ::registry_api::v1::GetCredentialIssuerResponse> * m,
                          int32_t key, ::registry_api::v1::GetCredentialIssuerResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_GetCredentialIssuerResponse_get(const google::protobuf::Map<int32_t, ::registry_api::v1::GetCredentialIssuerResponse>* m,
                       int32_t key, const ::registry_api::v1::GetCredentialIssuerResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetCredentialIssuerResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_GetCredentialIssuerResponse_remove(google::protobuf::Map<int32_t, ::registry_api::v1::GetCredentialIssuerResponse> * m,
                          int32_t key, ::registry_api::v1::GetCredentialIssuerResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_GetCredentialIssuerResponse_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::GetCredentialIssuerResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_GetCredentialIssuerResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::GetCredentialIssuerResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::GetCredentialIssuerResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::GetCredentialIssuerResponse>* __rust_proto_thunk__Map_u32_registry_api_v1_GetCredentialIssuerResponse_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::GetCredentialIssuerResponse>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_GetCredentialIssuerResponse_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::GetCredentialIssuerResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_GetCredentialIssuerResponse_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::GetCredentialIssuerResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_GetCredentialIssuerResponse_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::GetCredentialIssuerResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_GetCredentialIssuerResponse_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::GetCredentialIssuerResponse> * m,
                          uint32_t key, ::registry_api::v1::GetCredentialIssuerResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_GetCredentialIssuerResponse_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::GetCredentialIssuerResponse>* m,
                       uint32_t key, const ::registry_api::v1::GetCredentialIssuerResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetCredentialIssuerResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_GetCredentialIssuerResponse_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::GetCredentialIssuerResponse> * m,
                          uint32_t key, ::registry_api::v1::GetCredentialIssuerResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_GetCredentialIssuerResponse_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::GetCredentialIssuerResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_GetCredentialIssuerResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::GetCredentialIssuerResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::GetCredentialIssuerResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::GetCredentialIssuerResponse>* __rust_proto_thunk__Map_i64_registry_api_v1_GetCredentialIssuerResponse_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::GetCredentialIssuerResponse>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_GetCredentialIssuerResponse_free(const google::protobuf::Map<int64_t, ::registry_api::v1::GetCredentialIssuerResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_GetCredentialIssuerResponse_clear(google::protobuf::Map<int64_t, ::registry_api::v1::GetCredentialIssuerResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_GetCredentialIssuerResponse_size(const google::protobuf::Map<int64_t, ::registry_api::v1::GetCredentialIssuerResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_GetCredentialIssuerResponse_insert(google::protobuf::Map<int64_t, ::registry_api::v1::GetCredentialIssuerResponse> * m,
                          int64_t key, ::registry_api::v1::GetCredentialIssuerResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_GetCredentialIssuerResponse_get(const google::protobuf::Map<int64_t, ::registry_api::v1::GetCredentialIssuerResponse>* m,
                       int64_t key, const ::registry_api::v1::GetCredentialIssuerResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetCredentialIssuerResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_GetCredentialIssuerResponse_remove(google::protobuf::Map<int64_t, ::registry_api::v1::GetCredentialIssuerResponse> * m,
                          int64_t key, ::registry_api::v1::GetCredentialIssuerResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_GetCredentialIssuerResponse_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::GetCredentialIssuerResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_GetCredentialIssuerResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::GetCredentialIssuerResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::GetCredentialIssuerResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::GetCredentialIssuerResponse>* __rust_proto_thunk__Map_u64_registry_api_v1_GetCredentialIssuerResponse_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::GetCredentialIssuerResponse>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_GetCredentialIssuerResponse_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::GetCredentialIssuerResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_GetCredentialIssuerResponse_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::GetCredentialIssuerResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_GetCredentialIssuerResponse_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::GetCredentialIssuerResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_GetCredentialIssuerResponse_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::GetCredentialIssuerResponse> * m,
                          uint64_t key, ::registry_api::v1::GetCredentialIssuerResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_GetCredentialIssuerResponse_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::GetCredentialIssuerResponse>* m,
                       uint64_t key, const ::registry_api::v1::GetCredentialIssuerResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetCredentialIssuerResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_GetCredentialIssuerResponse_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::GetCredentialIssuerResponse> * m,
                          uint64_t key, ::registry_api::v1::GetCredentialIssuerResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_GetCredentialIssuerResponse_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::GetCredentialIssuerResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_GetCredentialIssuerResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::GetCredentialIssuerResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::GetCredentialIssuerResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::GetCredentialIssuerResponse>* __rust_proto_thunk__Map_bool_registry_api_v1_GetCredentialIssuerResponse_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::GetCredentialIssuerResponse>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_GetCredentialIssuerResponse_free(const google::protobuf::Map<bool, ::registry_api::v1::GetCredentialIssuerResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_GetCredentialIssuerResponse_clear(google::protobuf::Map<bool, ::registry_api::v1::GetCredentialIssuerResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_GetCredentialIssuerResponse_size(const google::protobuf::Map<bool, ::registry_api::v1::GetCredentialIssuerResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_GetCredentialIssuerResponse_insert(google::protobuf::Map<bool, ::registry_api::v1::GetCredentialIssuerResponse> * m,
                          bool key, ::registry_api::v1::GetCredentialIssuerResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_GetCredentialIssuerResponse_get(const google::protobuf::Map<bool, ::registry_api::v1::GetCredentialIssuerResponse>* m,
                       bool key, const ::registry_api::v1::GetCredentialIssuerResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetCredentialIssuerResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_GetCredentialIssuerResponse_remove(google::protobuf::Map<bool, ::registry_api::v1::GetCredentialIssuerResponse> * m,
                          bool key, ::registry_api::v1::GetCredentialIssuerResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_GetCredentialIssuerResponse_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::GetCredentialIssuerResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_GetCredentialIssuerResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::GetCredentialIssuerResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::GetCredentialIssuerResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::GetCredentialIssuerResponse>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetCredentialIssuerResponse_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::GetCredentialIssuerResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetCredentialIssuerResponse_free(const google::protobuf::Map<std::string, ::registry_api::v1::GetCredentialIssuerResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetCredentialIssuerResponse_clear(google::protobuf::Map<std::string, ::registry_api::v1::GetCredentialIssuerResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetCredentialIssuerResponse_size(const google::protobuf::Map<std::string, ::registry_api::v1::GetCredentialIssuerResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetCredentialIssuerResponse_insert(google::protobuf::Map<std::string, ::registry_api::v1::GetCredentialIssuerResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::GetCredentialIssuerResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetCredentialIssuerResponse_get(const google::protobuf::Map<std::string, ::registry_api::v1::GetCredentialIssuerResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::GetCredentialIssuerResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetCredentialIssuerResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetCredentialIssuerResponse_remove(google::protobuf::Map<std::string, ::registry_api::v1::GetCredentialIssuerResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::GetCredentialIssuerResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetCredentialIssuerResponse_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::GetCredentialIssuerResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetCredentialIssuerResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::GetCredentialIssuerResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::GetCredentialIssuerResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.UpdateCredentialIssuerRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerRequest_new() { return new ::registry_api::v1::UpdateCredentialIssuerRequest(); }
void __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerRequest_delete(void* ptr) { delete static_cast<::registry_api::v1::UpdateCredentialIssuerRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerRequest_serialize(::registry_api::v1::UpdateCredentialIssuerRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerRequest_parse(::registry_api::v1::UpdateCredentialIssuerRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerRequest_copy_from(::registry_api::v1::UpdateCredentialIssuerRequest* dst, const ::registry_api::v1::UpdateCredentialIssuerRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerRequest_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateCredentialIssuerRequest>* field) {
  return field->size();
}
const ::registry_api::v1::UpdateCredentialIssuerRequest& __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateCredentialIssuerRequest>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::UpdateCredentialIssuerRequest* __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateCredentialIssuerRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::UpdateCredentialIssuerRequest* __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerRequest_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateCredentialIssuerRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerRequest_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateCredentialIssuerRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateCredentialIssuerRequest>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateCredentialIssuerRequest>& src) {
  dst = src;
}

const void* __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerRequest_get_credential_issuer(::registry_api::v1::UpdateCredentialIssuerRequest* msg) {
  return static_cast<const void*>(&msg->credential_issuer());
}
void* __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerRequest_get_mut_credential_issuer(::registry_api::v1::UpdateCredentialIssuerRequest* msg) {
  return static_cast<void*>(msg->mutable_credential_issuer());
}
void __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerRequest_clear_credential_issuer(::registry_api::v1::UpdateCredentialIssuerRequest* msg) { msg->clear_credential_issuer(); }
bool __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerRequest_has_credential_issuer(::registry_api::v1::UpdateCredentialIssuerRequest* msg) { return msg->has_credential_issuer(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateCredentialIssuerRequest>* __rust_proto_thunk__Map_i32_registry_api_v1_UpdateCredentialIssuerRequest_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::UpdateCredentialIssuerRequest>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_UpdateCredentialIssuerRequest_free(const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateCredentialIssuerRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_UpdateCredentialIssuerRequest_clear(google::protobuf::Map<int32_t, ::registry_api::v1::UpdateCredentialIssuerRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_UpdateCredentialIssuerRequest_size(const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateCredentialIssuerRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_UpdateCredentialIssuerRequest_insert(google::protobuf::Map<int32_t, ::registry_api::v1::UpdateCredentialIssuerRequest> * m,
                          int32_t key, ::registry_api::v1::UpdateCredentialIssuerRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_UpdateCredentialIssuerRequest_get(const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateCredentialIssuerRequest>* m,
                       int32_t key, const ::registry_api::v1::UpdateCredentialIssuerRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateCredentialIssuerRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_UpdateCredentialIssuerRequest_remove(google::protobuf::Map<int32_t, ::registry_api::v1::UpdateCredentialIssuerRequest> * m,
                          int32_t key, ::registry_api::v1::UpdateCredentialIssuerRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_UpdateCredentialIssuerRequest_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateCredentialIssuerRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_UpdateCredentialIssuerRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::UpdateCredentialIssuerRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::UpdateCredentialIssuerRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateCredentialIssuerRequest>* __rust_proto_thunk__Map_u32_registry_api_v1_UpdateCredentialIssuerRequest_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateCredentialIssuerRequest>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_UpdateCredentialIssuerRequest_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateCredentialIssuerRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_UpdateCredentialIssuerRequest_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateCredentialIssuerRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_UpdateCredentialIssuerRequest_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateCredentialIssuerRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_UpdateCredentialIssuerRequest_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateCredentialIssuerRequest> * m,
                          uint32_t key, ::registry_api::v1::UpdateCredentialIssuerRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_UpdateCredentialIssuerRequest_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateCredentialIssuerRequest>* m,
                       uint32_t key, const ::registry_api::v1::UpdateCredentialIssuerRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateCredentialIssuerRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_UpdateCredentialIssuerRequest_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateCredentialIssuerRequest> * m,
                          uint32_t key, ::registry_api::v1::UpdateCredentialIssuerRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_UpdateCredentialIssuerRequest_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateCredentialIssuerRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_UpdateCredentialIssuerRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::UpdateCredentialIssuerRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateCredentialIssuerRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateCredentialIssuerRequest>* __rust_proto_thunk__Map_i64_registry_api_v1_UpdateCredentialIssuerRequest_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::UpdateCredentialIssuerRequest>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_UpdateCredentialIssuerRequest_free(const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateCredentialIssuerRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_UpdateCredentialIssuerRequest_clear(google::protobuf::Map<int64_t, ::registry_api::v1::UpdateCredentialIssuerRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_UpdateCredentialIssuerRequest_size(const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateCredentialIssuerRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_UpdateCredentialIssuerRequest_insert(google::protobuf::Map<int64_t, ::registry_api::v1::UpdateCredentialIssuerRequest> * m,
                          int64_t key, ::registry_api::v1::UpdateCredentialIssuerRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_UpdateCredentialIssuerRequest_get(const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateCredentialIssuerRequest>* m,
                       int64_t key, const ::registry_api::v1::UpdateCredentialIssuerRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateCredentialIssuerRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_UpdateCredentialIssuerRequest_remove(google::protobuf::Map<int64_t, ::registry_api::v1::UpdateCredentialIssuerRequest> * m,
                          int64_t key, ::registry_api::v1::UpdateCredentialIssuerRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_UpdateCredentialIssuerRequest_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateCredentialIssuerRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_UpdateCredentialIssuerRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::UpdateCredentialIssuerRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::UpdateCredentialIssuerRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateCredentialIssuerRequest>* __rust_proto_thunk__Map_u64_registry_api_v1_UpdateCredentialIssuerRequest_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateCredentialIssuerRequest>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_UpdateCredentialIssuerRequest_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateCredentialIssuerRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_UpdateCredentialIssuerRequest_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateCredentialIssuerRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_UpdateCredentialIssuerRequest_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateCredentialIssuerRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_UpdateCredentialIssuerRequest_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateCredentialIssuerRequest> * m,
                          uint64_t key, ::registry_api::v1::UpdateCredentialIssuerRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_UpdateCredentialIssuerRequest_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateCredentialIssuerRequest>* m,
                       uint64_t key, const ::registry_api::v1::UpdateCredentialIssuerRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateCredentialIssuerRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_UpdateCredentialIssuerRequest_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateCredentialIssuerRequest> * m,
                          uint64_t key, ::registry_api::v1::UpdateCredentialIssuerRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_UpdateCredentialIssuerRequest_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateCredentialIssuerRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_UpdateCredentialIssuerRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::UpdateCredentialIssuerRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateCredentialIssuerRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::UpdateCredentialIssuerRequest>* __rust_proto_thunk__Map_bool_registry_api_v1_UpdateCredentialIssuerRequest_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::UpdateCredentialIssuerRequest>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_UpdateCredentialIssuerRequest_free(const google::protobuf::Map<bool, ::registry_api::v1::UpdateCredentialIssuerRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_UpdateCredentialIssuerRequest_clear(google::protobuf::Map<bool, ::registry_api::v1::UpdateCredentialIssuerRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_UpdateCredentialIssuerRequest_size(const google::protobuf::Map<bool, ::registry_api::v1::UpdateCredentialIssuerRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_UpdateCredentialIssuerRequest_insert(google::protobuf::Map<bool, ::registry_api::v1::UpdateCredentialIssuerRequest> * m,
                          bool key, ::registry_api::v1::UpdateCredentialIssuerRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_UpdateCredentialIssuerRequest_get(const google::protobuf::Map<bool, ::registry_api::v1::UpdateCredentialIssuerRequest>* m,
                       bool key, const ::registry_api::v1::UpdateCredentialIssuerRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateCredentialIssuerRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_UpdateCredentialIssuerRequest_remove(google::protobuf::Map<bool, ::registry_api::v1::UpdateCredentialIssuerRequest> * m,
                          bool key, ::registry_api::v1::UpdateCredentialIssuerRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_UpdateCredentialIssuerRequest_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::UpdateCredentialIssuerRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_UpdateCredentialIssuerRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::UpdateCredentialIssuerRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::UpdateCredentialIssuerRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::UpdateCredentialIssuerRequest>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateCredentialIssuerRequest_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::UpdateCredentialIssuerRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateCredentialIssuerRequest_free(const google::protobuf::Map<std::string, ::registry_api::v1::UpdateCredentialIssuerRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateCredentialIssuerRequest_clear(google::protobuf::Map<std::string, ::registry_api::v1::UpdateCredentialIssuerRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateCredentialIssuerRequest_size(const google::protobuf::Map<std::string, ::registry_api::v1::UpdateCredentialIssuerRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateCredentialIssuerRequest_insert(google::protobuf::Map<std::string, ::registry_api::v1::UpdateCredentialIssuerRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::UpdateCredentialIssuerRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateCredentialIssuerRequest_get(const google::protobuf::Map<std::string, ::registry_api::v1::UpdateCredentialIssuerRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::UpdateCredentialIssuerRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateCredentialIssuerRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateCredentialIssuerRequest_remove(google::protobuf::Map<std::string, ::registry_api::v1::UpdateCredentialIssuerRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::UpdateCredentialIssuerRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateCredentialIssuerRequest_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::UpdateCredentialIssuerRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateCredentialIssuerRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::UpdateCredentialIssuerRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::UpdateCredentialIssuerRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.UpdateCredentialIssuerResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerResponse_new() { return new ::registry_api::v1::UpdateCredentialIssuerResponse(); }
void __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerResponse_delete(void* ptr) { delete static_cast<::registry_api::v1::UpdateCredentialIssuerResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerResponse_serialize(::registry_api::v1::UpdateCredentialIssuerResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerResponse_parse(::registry_api::v1::UpdateCredentialIssuerResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerResponse_copy_from(::registry_api::v1::UpdateCredentialIssuerResponse* dst, const ::registry_api::v1::UpdateCredentialIssuerResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerResponse_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateCredentialIssuerResponse>* field) {
  return field->size();
}
const ::registry_api::v1::UpdateCredentialIssuerResponse& __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateCredentialIssuerResponse>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::UpdateCredentialIssuerResponse* __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateCredentialIssuerResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::UpdateCredentialIssuerResponse* __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerResponse_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateCredentialIssuerResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerResponse_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateCredentialIssuerResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateCredentialIssuerResponse>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateCredentialIssuerResponse>& src) {
  dst = src;
}

bool __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerResponse_get_ok(::registry_api::v1::UpdateCredentialIssuerResponse* msg) { return msg->ok(); }
void __rust_proto_thunk__registry_api_v1_UpdateCredentialIssuerResponse_set_ok(::registry_api::v1::UpdateCredentialIssuerResponse* msg, bool val) {
  msg->set_ok(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateCredentialIssuerResponse>* __rust_proto_thunk__Map_i32_registry_api_v1_UpdateCredentialIssuerResponse_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::UpdateCredentialIssuerResponse>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_UpdateCredentialIssuerResponse_free(const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateCredentialIssuerResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_UpdateCredentialIssuerResponse_clear(google::protobuf::Map<int32_t, ::registry_api::v1::UpdateCredentialIssuerResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_UpdateCredentialIssuerResponse_size(const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateCredentialIssuerResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_UpdateCredentialIssuerResponse_insert(google::protobuf::Map<int32_t, ::registry_api::v1::UpdateCredentialIssuerResponse> * m,
                          int32_t key, ::registry_api::v1::UpdateCredentialIssuerResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_UpdateCredentialIssuerResponse_get(const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateCredentialIssuerResponse>* m,
                       int32_t key, const ::registry_api::v1::UpdateCredentialIssuerResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateCredentialIssuerResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_UpdateCredentialIssuerResponse_remove(google::protobuf::Map<int32_t, ::registry_api::v1::UpdateCredentialIssuerResponse> * m,
                          int32_t key, ::registry_api::v1::UpdateCredentialIssuerResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_UpdateCredentialIssuerResponse_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateCredentialIssuerResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_UpdateCredentialIssuerResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::UpdateCredentialIssuerResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::UpdateCredentialIssuerResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateCredentialIssuerResponse>* __rust_proto_thunk__Map_u32_registry_api_v1_UpdateCredentialIssuerResponse_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateCredentialIssuerResponse>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_UpdateCredentialIssuerResponse_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateCredentialIssuerResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_UpdateCredentialIssuerResponse_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateCredentialIssuerResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_UpdateCredentialIssuerResponse_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateCredentialIssuerResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_UpdateCredentialIssuerResponse_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateCredentialIssuerResponse> * m,
                          uint32_t key, ::registry_api::v1::UpdateCredentialIssuerResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_UpdateCredentialIssuerResponse_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateCredentialIssuerResponse>* m,
                       uint32_t key, const ::registry_api::v1::UpdateCredentialIssuerResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateCredentialIssuerResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_UpdateCredentialIssuerResponse_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateCredentialIssuerResponse> * m,
                          uint32_t key, ::registry_api::v1::UpdateCredentialIssuerResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_UpdateCredentialIssuerResponse_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateCredentialIssuerResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_UpdateCredentialIssuerResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::UpdateCredentialIssuerResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateCredentialIssuerResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateCredentialIssuerResponse>* __rust_proto_thunk__Map_i64_registry_api_v1_UpdateCredentialIssuerResponse_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::UpdateCredentialIssuerResponse>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_UpdateCredentialIssuerResponse_free(const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateCredentialIssuerResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_UpdateCredentialIssuerResponse_clear(google::protobuf::Map<int64_t, ::registry_api::v1::UpdateCredentialIssuerResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_UpdateCredentialIssuerResponse_size(const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateCredentialIssuerResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_UpdateCredentialIssuerResponse_insert(google::protobuf::Map<int64_t, ::registry_api::v1::UpdateCredentialIssuerResponse> * m,
                          int64_t key, ::registry_api::v1::UpdateCredentialIssuerResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_UpdateCredentialIssuerResponse_get(const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateCredentialIssuerResponse>* m,
                       int64_t key, const ::registry_api::v1::UpdateCredentialIssuerResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateCredentialIssuerResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_UpdateCredentialIssuerResponse_remove(google::protobuf::Map<int64_t, ::registry_api::v1::UpdateCredentialIssuerResponse> * m,
                          int64_t key, ::registry_api::v1::UpdateCredentialIssuerResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_UpdateCredentialIssuerResponse_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateCredentialIssuerResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_UpdateCredentialIssuerResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::UpdateCredentialIssuerResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::UpdateCredentialIssuerResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateCredentialIssuerResponse>* __rust_proto_thunk__Map_u64_registry_api_v1_UpdateCredentialIssuerResponse_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateCredentialIssuerResponse>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_UpdateCredentialIssuerResponse_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateCredentialIssuerResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_UpdateCredentialIssuerResponse_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateCredentialIssuerResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_UpdateCredentialIssuerResponse_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateCredentialIssuerResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_UpdateCredentialIssuerResponse_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateCredentialIssuerResponse> * m,
                          uint64_t key, ::registry_api::v1::UpdateCredentialIssuerResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_UpdateCredentialIssuerResponse_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateCredentialIssuerResponse>* m,
                       uint64_t key, const ::registry_api::v1::UpdateCredentialIssuerResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateCredentialIssuerResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_UpdateCredentialIssuerResponse_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateCredentialIssuerResponse> * m,
                          uint64_t key, ::registry_api::v1::UpdateCredentialIssuerResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_UpdateCredentialIssuerResponse_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateCredentialIssuerResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_UpdateCredentialIssuerResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::UpdateCredentialIssuerResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateCredentialIssuerResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::UpdateCredentialIssuerResponse>* __rust_proto_thunk__Map_bool_registry_api_v1_UpdateCredentialIssuerResponse_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::UpdateCredentialIssuerResponse>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_UpdateCredentialIssuerResponse_free(const google::protobuf::Map<bool, ::registry_api::v1::UpdateCredentialIssuerResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_UpdateCredentialIssuerResponse_clear(google::protobuf::Map<bool, ::registry_api::v1::UpdateCredentialIssuerResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_UpdateCredentialIssuerResponse_size(const google::protobuf::Map<bool, ::registry_api::v1::UpdateCredentialIssuerResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_UpdateCredentialIssuerResponse_insert(google::protobuf::Map<bool, ::registry_api::v1::UpdateCredentialIssuerResponse> * m,
                          bool key, ::registry_api::v1::UpdateCredentialIssuerResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_UpdateCredentialIssuerResponse_get(const google::protobuf::Map<bool, ::registry_api::v1::UpdateCredentialIssuerResponse>* m,
                       bool key, const ::registry_api::v1::UpdateCredentialIssuerResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateCredentialIssuerResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_UpdateCredentialIssuerResponse_remove(google::protobuf::Map<bool, ::registry_api::v1::UpdateCredentialIssuerResponse> * m,
                          bool key, ::registry_api::v1::UpdateCredentialIssuerResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_UpdateCredentialIssuerResponse_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::UpdateCredentialIssuerResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_UpdateCredentialIssuerResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::UpdateCredentialIssuerResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::UpdateCredentialIssuerResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::UpdateCredentialIssuerResponse>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateCredentialIssuerResponse_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::UpdateCredentialIssuerResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateCredentialIssuerResponse_free(const google::protobuf::Map<std::string, ::registry_api::v1::UpdateCredentialIssuerResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateCredentialIssuerResponse_clear(google::protobuf::Map<std::string, ::registry_api::v1::UpdateCredentialIssuerResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateCredentialIssuerResponse_size(const google::protobuf::Map<std::string, ::registry_api::v1::UpdateCredentialIssuerResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateCredentialIssuerResponse_insert(google::protobuf::Map<std::string, ::registry_api::v1::UpdateCredentialIssuerResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::UpdateCredentialIssuerResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateCredentialIssuerResponse_get(const google::protobuf::Map<std::string, ::registry_api::v1::UpdateCredentialIssuerResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::UpdateCredentialIssuerResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateCredentialIssuerResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateCredentialIssuerResponse_remove(google::protobuf::Map<std::string, ::registry_api::v1::UpdateCredentialIssuerResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::UpdateCredentialIssuerResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateCredentialIssuerResponse_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::UpdateCredentialIssuerResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateCredentialIssuerResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::UpdateCredentialIssuerResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::UpdateCredentialIssuerResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.StatusListEntry
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_StatusListEntry_new() { return new ::registry_api::v1::StatusListEntry(); }
void __rust_proto_thunk__registry_api_v1_StatusListEntry_delete(void* ptr) { delete static_cast<::registry_api::v1::StatusListEntry*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_StatusListEntry_serialize(::registry_api::v1::StatusListEntry* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_StatusListEntry_parse(::registry_api::v1::StatusListEntry* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_StatusListEntry_copy_from(::registry_api::v1::StatusListEntry* dst, const ::registry_api::v1::StatusListEntry* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_StatusListEntry_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::StatusListEntry>* field) {
  return field->size();
}
const ::registry_api::v1::StatusListEntry& __rust_proto_thunk__registry_api_v1_StatusListEntry_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::StatusListEntry>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::StatusListEntry* __rust_proto_thunk__registry_api_v1_StatusListEntry_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::StatusListEntry>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::StatusListEntry* __rust_proto_thunk__registry_api_v1_StatusListEntry_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::StatusListEntry>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_StatusListEntry_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::StatusListEntry>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_StatusListEntry_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::StatusListEntry>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::StatusListEntry>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_StatusListEntry_get_credential_id(::registry_api::v1::StatusListEntry* msg) {
  absl::string_view val = msg->credential_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_StatusListEntry_set_credential_id(::registry_api::v1::StatusListEntry* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_credential_id(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_StatusListEntry_get_credential_issuer_id(::registry_api::v1::StatusListEntry* msg) {
  absl::string_view val = msg->credential_issuer_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_StatusListEntry_set_credential_issuer_id(::registry_api::v1::StatusListEntry* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_credential_issuer_id(absl::string_view(s.ptr, s.len));
}

::registry_api::v1::StatusPurpose __rust_proto_thunk__registry_api_v1_StatusListEntry_get_status_purpose(::registry_api::v1::StatusListEntry* msg) { return msg->status_purpose(); }
void __rust_proto_thunk__registry_api_v1_StatusListEntry_set_status_purpose(::registry_api::v1::StatusListEntry* msg, ::registry_api::v1::StatusPurpose val) {
  msg->set_status_purpose(val);
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_StatusListEntry_get_credential_type(::registry_api::v1::StatusListEntry* msg) {
  absl::string_view val = msg->credential_type();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_StatusListEntry_set_credential_type(::registry_api::v1::StatusListEntry* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_credential_type(absl::string_view(s.ptr, s.len));
}

::int64_t __rust_proto_thunk__registry_api_v1_StatusListEntry_get_status_list_index(::registry_api::v1::StatusListEntry* msg) { return msg->status_list_index(); }
void __rust_proto_thunk__registry_api_v1_StatusListEntry_set_status_list_index(::registry_api::v1::StatusListEntry* msg, ::int64_t val) {
  msg->set_status_list_index(val);
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_StatusListEntry_get_status_list_credential_id(::registry_api::v1::StatusListEntry* msg) {
  absl::string_view val = msg->status_list_credential_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_StatusListEntry_set_status_list_credential_id(::registry_api::v1::StatusListEntry* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_status_list_credential_id(absl::string_view(s.ptr, s.len));
}

bool __rust_proto_thunk__registry_api_v1_StatusListEntry_get_status(::registry_api::v1::StatusListEntry* msg) { return msg->status(); }
void __rust_proto_thunk__registry_api_v1_StatusListEntry_set_status(::registry_api::v1::StatusListEntry* msg, bool val) {
  msg->set_status(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::StatusListEntry>* __rust_proto_thunk__Map_i32_registry_api_v1_StatusListEntry_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::StatusListEntry>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_StatusListEntry_free(const google::protobuf::Map<int32_t, ::registry_api::v1::StatusListEntry>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_StatusListEntry_clear(google::protobuf::Map<int32_t, ::registry_api::v1::StatusListEntry> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_StatusListEntry_size(const google::protobuf::Map<int32_t, ::registry_api::v1::StatusListEntry>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_StatusListEntry_insert(google::protobuf::Map<int32_t, ::registry_api::v1::StatusListEntry> * m,
                          int32_t key, ::registry_api::v1::StatusListEntry value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_StatusListEntry_get(const google::protobuf::Map<int32_t, ::registry_api::v1::StatusListEntry>* m,
                       int32_t key, const ::registry_api::v1::StatusListEntry** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::StatusListEntry* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_StatusListEntry_remove(google::protobuf::Map<int32_t, ::registry_api::v1::StatusListEntry> * m,
                          int32_t key, ::registry_api::v1::StatusListEntry * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_StatusListEntry_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::StatusListEntry>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_StatusListEntry_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::StatusListEntry** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::StatusListEntry>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::StatusListEntry>* __rust_proto_thunk__Map_u32_registry_api_v1_StatusListEntry_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::StatusListEntry>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_StatusListEntry_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::StatusListEntry>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_StatusListEntry_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::StatusListEntry> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_StatusListEntry_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::StatusListEntry>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_StatusListEntry_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::StatusListEntry> * m,
                          uint32_t key, ::registry_api::v1::StatusListEntry value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_StatusListEntry_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::StatusListEntry>* m,
                       uint32_t key, const ::registry_api::v1::StatusListEntry** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::StatusListEntry* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_StatusListEntry_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::StatusListEntry> * m,
                          uint32_t key, ::registry_api::v1::StatusListEntry * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_StatusListEntry_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::StatusListEntry>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_StatusListEntry_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::StatusListEntry** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::StatusListEntry>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::StatusListEntry>* __rust_proto_thunk__Map_i64_registry_api_v1_StatusListEntry_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::StatusListEntry>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_StatusListEntry_free(const google::protobuf::Map<int64_t, ::registry_api::v1::StatusListEntry>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_StatusListEntry_clear(google::protobuf::Map<int64_t, ::registry_api::v1::StatusListEntry> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_StatusListEntry_size(const google::protobuf::Map<int64_t, ::registry_api::v1::StatusListEntry>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_StatusListEntry_insert(google::protobuf::Map<int64_t, ::registry_api::v1::StatusListEntry> * m,
                          int64_t key, ::registry_api::v1::StatusListEntry value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_StatusListEntry_get(const google::protobuf::Map<int64_t, ::registry_api::v1::StatusListEntry>* m,
                       int64_t key, const ::registry_api::v1::StatusListEntry** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::StatusListEntry* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_StatusListEntry_remove(google::protobuf::Map<int64_t, ::registry_api::v1::StatusListEntry> * m,
                          int64_t key, ::registry_api::v1::StatusListEntry * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_StatusListEntry_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::StatusListEntry>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_StatusListEntry_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::StatusListEntry** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::StatusListEntry>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::StatusListEntry>* __rust_proto_thunk__Map_u64_registry_api_v1_StatusListEntry_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::StatusListEntry>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_StatusListEntry_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::StatusListEntry>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_StatusListEntry_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::StatusListEntry> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_StatusListEntry_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::StatusListEntry>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_StatusListEntry_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::StatusListEntry> * m,
                          uint64_t key, ::registry_api::v1::StatusListEntry value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_StatusListEntry_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::StatusListEntry>* m,
                       uint64_t key, const ::registry_api::v1::StatusListEntry** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::StatusListEntry* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_StatusListEntry_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::StatusListEntry> * m,
                          uint64_t key, ::registry_api::v1::StatusListEntry * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_StatusListEntry_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::StatusListEntry>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_StatusListEntry_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::StatusListEntry** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::StatusListEntry>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::StatusListEntry>* __rust_proto_thunk__Map_bool_registry_api_v1_StatusListEntry_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::StatusListEntry>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_StatusListEntry_free(const google::protobuf::Map<bool, ::registry_api::v1::StatusListEntry>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_StatusListEntry_clear(google::protobuf::Map<bool, ::registry_api::v1::StatusListEntry> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_StatusListEntry_size(const google::protobuf::Map<bool, ::registry_api::v1::StatusListEntry>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_StatusListEntry_insert(google::protobuf::Map<bool, ::registry_api::v1::StatusListEntry> * m,
                          bool key, ::registry_api::v1::StatusListEntry value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_StatusListEntry_get(const google::protobuf::Map<bool, ::registry_api::v1::StatusListEntry>* m,
                       bool key, const ::registry_api::v1::StatusListEntry** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::StatusListEntry* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_StatusListEntry_remove(google::protobuf::Map<bool, ::registry_api::v1::StatusListEntry> * m,
                          bool key, ::registry_api::v1::StatusListEntry * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_StatusListEntry_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::StatusListEntry>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_StatusListEntry_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::StatusListEntry** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::StatusListEntry>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::StatusListEntry>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_StatusListEntry_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::StatusListEntry>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_StatusListEntry_free(const google::protobuf::Map<std::string, ::registry_api::v1::StatusListEntry>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_StatusListEntry_clear(google::protobuf::Map<std::string, ::registry_api::v1::StatusListEntry> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_StatusListEntry_size(const google::protobuf::Map<std::string, ::registry_api::v1::StatusListEntry>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_StatusListEntry_insert(google::protobuf::Map<std::string, ::registry_api::v1::StatusListEntry> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::StatusListEntry value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_StatusListEntry_get(const google::protobuf::Map<std::string, ::registry_api::v1::StatusListEntry>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::StatusListEntry** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::StatusListEntry* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_StatusListEntry_remove(google::protobuf::Map<std::string, ::registry_api::v1::StatusListEntry> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::StatusListEntry * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_StatusListEntry_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::StatusListEntry>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_StatusListEntry_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::StatusListEntry** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::StatusListEntry>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.CreateStatusListEntryRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_CreateStatusListEntryRequest_new() { return new ::registry_api::v1::CreateStatusListEntryRequest(); }
void __rust_proto_thunk__registry_api_v1_CreateStatusListEntryRequest_delete(void* ptr) { delete static_cast<::registry_api::v1::CreateStatusListEntryRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_CreateStatusListEntryRequest_serialize(::registry_api::v1::CreateStatusListEntryRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_CreateStatusListEntryRequest_parse(::registry_api::v1::CreateStatusListEntryRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_CreateStatusListEntryRequest_copy_from(::registry_api::v1::CreateStatusListEntryRequest* dst, const ::registry_api::v1::CreateStatusListEntryRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_CreateStatusListEntryRequest_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::CreateStatusListEntryRequest>* field) {
  return field->size();
}
const ::registry_api::v1::CreateStatusListEntryRequest& __rust_proto_thunk__registry_api_v1_CreateStatusListEntryRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::CreateStatusListEntryRequest>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::CreateStatusListEntryRequest* __rust_proto_thunk__registry_api_v1_CreateStatusListEntryRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::CreateStatusListEntryRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::CreateStatusListEntryRequest* __rust_proto_thunk__registry_api_v1_CreateStatusListEntryRequest_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::CreateStatusListEntryRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_CreateStatusListEntryRequest_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::CreateStatusListEntryRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_CreateStatusListEntryRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::CreateStatusListEntryRequest>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::CreateStatusListEntryRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_CreateStatusListEntryRequest_get_credential_id(::registry_api::v1::CreateStatusListEntryRequest* msg) {
  absl::string_view val = msg->credential_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_CreateStatusListEntryRequest_set_credential_id(::registry_api::v1::CreateStatusListEntryRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_credential_id(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_CreateStatusListEntryRequest_get_credential_issuer_id(::registry_api::v1::CreateStatusListEntryRequest* msg) {
  absl::string_view val = msg->credential_issuer_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_CreateStatusListEntryRequest_set_credential_issuer_id(::registry_api::v1::CreateStatusListEntryRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_credential_issuer_id(absl::string_view(s.ptr, s.len));
}

::registry_api::v1::StatusPurpose __rust_proto_thunk__registry_api_v1_CreateStatusListEntryRequest_get_status_purpose(::registry_api::v1::CreateStatusListEntryRequest* msg) { return msg->status_purpose(); }
void __rust_proto_thunk__registry_api_v1_CreateStatusListEntryRequest_set_status_purpose(::registry_api::v1::CreateStatusListEntryRequest* msg, ::registry_api::v1::StatusPurpose val) {
  msg->set_status_purpose(val);
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_CreateStatusListEntryRequest_get_credential_type(::registry_api::v1::CreateStatusListEntryRequest* msg) {
  absl::string_view val = msg->credential_type();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_CreateStatusListEntryRequest_set_credential_type(::registry_api::v1::CreateStatusListEntryRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_credential_type(absl::string_view(s.ptr, s.len));
}

bool __rust_proto_thunk__registry_api_v1_CreateStatusListEntryRequest_get_status(::registry_api::v1::CreateStatusListEntryRequest* msg) { return msg->status(); }
void __rust_proto_thunk__registry_api_v1_CreateStatusListEntryRequest_set_status(::registry_api::v1::CreateStatusListEntryRequest* msg, bool val) {
  msg->set_status(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::CreateStatusListEntryRequest>* __rust_proto_thunk__Map_i32_registry_api_v1_CreateStatusListEntryRequest_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::CreateStatusListEntryRequest>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_CreateStatusListEntryRequest_free(const google::protobuf::Map<int32_t, ::registry_api::v1::CreateStatusListEntryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_CreateStatusListEntryRequest_clear(google::protobuf::Map<int32_t, ::registry_api::v1::CreateStatusListEntryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_CreateStatusListEntryRequest_size(const google::protobuf::Map<int32_t, ::registry_api::v1::CreateStatusListEntryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_CreateStatusListEntryRequest_insert(google::protobuf::Map<int32_t, ::registry_api::v1::CreateStatusListEntryRequest> * m,
                          int32_t key, ::registry_api::v1::CreateStatusListEntryRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_CreateStatusListEntryRequest_get(const google::protobuf::Map<int32_t, ::registry_api::v1::CreateStatusListEntryRequest>* m,
                       int32_t key, const ::registry_api::v1::CreateStatusListEntryRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateStatusListEntryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_CreateStatusListEntryRequest_remove(google::protobuf::Map<int32_t, ::registry_api::v1::CreateStatusListEntryRequest> * m,
                          int32_t key, ::registry_api::v1::CreateStatusListEntryRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_CreateStatusListEntryRequest_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::CreateStatusListEntryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_CreateStatusListEntryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::CreateStatusListEntryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::CreateStatusListEntryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateStatusListEntryRequest>* __rust_proto_thunk__Map_u32_registry_api_v1_CreateStatusListEntryRequest_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::CreateStatusListEntryRequest>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_CreateStatusListEntryRequest_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateStatusListEntryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_CreateStatusListEntryRequest_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::CreateStatusListEntryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_CreateStatusListEntryRequest_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateStatusListEntryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_CreateStatusListEntryRequest_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::CreateStatusListEntryRequest> * m,
                          uint32_t key, ::registry_api::v1::CreateStatusListEntryRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_CreateStatusListEntryRequest_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateStatusListEntryRequest>* m,
                       uint32_t key, const ::registry_api::v1::CreateStatusListEntryRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateStatusListEntryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_CreateStatusListEntryRequest_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::CreateStatusListEntryRequest> * m,
                          uint32_t key, ::registry_api::v1::CreateStatusListEntryRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_CreateStatusListEntryRequest_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateStatusListEntryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_CreateStatusListEntryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::CreateStatusListEntryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::CreateStatusListEntryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::CreateStatusListEntryRequest>* __rust_proto_thunk__Map_i64_registry_api_v1_CreateStatusListEntryRequest_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::CreateStatusListEntryRequest>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_CreateStatusListEntryRequest_free(const google::protobuf::Map<int64_t, ::registry_api::v1::CreateStatusListEntryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_CreateStatusListEntryRequest_clear(google::protobuf::Map<int64_t, ::registry_api::v1::CreateStatusListEntryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_CreateStatusListEntryRequest_size(const google::protobuf::Map<int64_t, ::registry_api::v1::CreateStatusListEntryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_CreateStatusListEntryRequest_insert(google::protobuf::Map<int64_t, ::registry_api::v1::CreateStatusListEntryRequest> * m,
                          int64_t key, ::registry_api::v1::CreateStatusListEntryRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_CreateStatusListEntryRequest_get(const google::protobuf::Map<int64_t, ::registry_api::v1::CreateStatusListEntryRequest>* m,
                       int64_t key, const ::registry_api::v1::CreateStatusListEntryRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateStatusListEntryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_CreateStatusListEntryRequest_remove(google::protobuf::Map<int64_t, ::registry_api::v1::CreateStatusListEntryRequest> * m,
                          int64_t key, ::registry_api::v1::CreateStatusListEntryRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_CreateStatusListEntryRequest_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::CreateStatusListEntryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_CreateStatusListEntryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::CreateStatusListEntryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::CreateStatusListEntryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateStatusListEntryRequest>* __rust_proto_thunk__Map_u64_registry_api_v1_CreateStatusListEntryRequest_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::CreateStatusListEntryRequest>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_CreateStatusListEntryRequest_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateStatusListEntryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_CreateStatusListEntryRequest_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::CreateStatusListEntryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_CreateStatusListEntryRequest_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateStatusListEntryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_CreateStatusListEntryRequest_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::CreateStatusListEntryRequest> * m,
                          uint64_t key, ::registry_api::v1::CreateStatusListEntryRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_CreateStatusListEntryRequest_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateStatusListEntryRequest>* m,
                       uint64_t key, const ::registry_api::v1::CreateStatusListEntryRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateStatusListEntryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_CreateStatusListEntryRequest_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::CreateStatusListEntryRequest> * m,
                          uint64_t key, ::registry_api::v1::CreateStatusListEntryRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_CreateStatusListEntryRequest_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateStatusListEntryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_CreateStatusListEntryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::CreateStatusListEntryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::CreateStatusListEntryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::CreateStatusListEntryRequest>* __rust_proto_thunk__Map_bool_registry_api_v1_CreateStatusListEntryRequest_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::CreateStatusListEntryRequest>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_CreateStatusListEntryRequest_free(const google::protobuf::Map<bool, ::registry_api::v1::CreateStatusListEntryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_CreateStatusListEntryRequest_clear(google::protobuf::Map<bool, ::registry_api::v1::CreateStatusListEntryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_CreateStatusListEntryRequest_size(const google::protobuf::Map<bool, ::registry_api::v1::CreateStatusListEntryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_CreateStatusListEntryRequest_insert(google::protobuf::Map<bool, ::registry_api::v1::CreateStatusListEntryRequest> * m,
                          bool key, ::registry_api::v1::CreateStatusListEntryRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_CreateStatusListEntryRequest_get(const google::protobuf::Map<bool, ::registry_api::v1::CreateStatusListEntryRequest>* m,
                       bool key, const ::registry_api::v1::CreateStatusListEntryRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateStatusListEntryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_CreateStatusListEntryRequest_remove(google::protobuf::Map<bool, ::registry_api::v1::CreateStatusListEntryRequest> * m,
                          bool key, ::registry_api::v1::CreateStatusListEntryRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_CreateStatusListEntryRequest_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::CreateStatusListEntryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_CreateStatusListEntryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::CreateStatusListEntryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::CreateStatusListEntryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::CreateStatusListEntryRequest>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateStatusListEntryRequest_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::CreateStatusListEntryRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateStatusListEntryRequest_free(const google::protobuf::Map<std::string, ::registry_api::v1::CreateStatusListEntryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateStatusListEntryRequest_clear(google::protobuf::Map<std::string, ::registry_api::v1::CreateStatusListEntryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateStatusListEntryRequest_size(const google::protobuf::Map<std::string, ::registry_api::v1::CreateStatusListEntryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateStatusListEntryRequest_insert(google::protobuf::Map<std::string, ::registry_api::v1::CreateStatusListEntryRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::CreateStatusListEntryRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateStatusListEntryRequest_get(const google::protobuf::Map<std::string, ::registry_api::v1::CreateStatusListEntryRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::CreateStatusListEntryRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateStatusListEntryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateStatusListEntryRequest_remove(google::protobuf::Map<std::string, ::registry_api::v1::CreateStatusListEntryRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::CreateStatusListEntryRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateStatusListEntryRequest_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::CreateStatusListEntryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateStatusListEntryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::CreateStatusListEntryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::CreateStatusListEntryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.CreateStatusListEntryResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_CreateStatusListEntryResponse_new() { return new ::registry_api::v1::CreateStatusListEntryResponse(); }
void __rust_proto_thunk__registry_api_v1_CreateStatusListEntryResponse_delete(void* ptr) { delete static_cast<::registry_api::v1::CreateStatusListEntryResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_CreateStatusListEntryResponse_serialize(::registry_api::v1::CreateStatusListEntryResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_CreateStatusListEntryResponse_parse(::registry_api::v1::CreateStatusListEntryResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_CreateStatusListEntryResponse_copy_from(::registry_api::v1::CreateStatusListEntryResponse* dst, const ::registry_api::v1::CreateStatusListEntryResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_CreateStatusListEntryResponse_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::CreateStatusListEntryResponse>* field) {
  return field->size();
}
const ::registry_api::v1::CreateStatusListEntryResponse& __rust_proto_thunk__registry_api_v1_CreateStatusListEntryResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::CreateStatusListEntryResponse>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::CreateStatusListEntryResponse* __rust_proto_thunk__registry_api_v1_CreateStatusListEntryResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::CreateStatusListEntryResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::CreateStatusListEntryResponse* __rust_proto_thunk__registry_api_v1_CreateStatusListEntryResponse_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::CreateStatusListEntryResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_CreateStatusListEntryResponse_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::CreateStatusListEntryResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_CreateStatusListEntryResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::CreateStatusListEntryResponse>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::CreateStatusListEntryResponse>& src) {
  dst = src;
}

const void* __rust_proto_thunk__registry_api_v1_CreateStatusListEntryResponse_get_status_list_entry(::registry_api::v1::CreateStatusListEntryResponse* msg) {
  return static_cast<const void*>(&msg->status_list_entry());
}
void* __rust_proto_thunk__registry_api_v1_CreateStatusListEntryResponse_get_mut_status_list_entry(::registry_api::v1::CreateStatusListEntryResponse* msg) {
  return static_cast<void*>(msg->mutable_status_list_entry());
}
void __rust_proto_thunk__registry_api_v1_CreateStatusListEntryResponse_clear_status_list_entry(::registry_api::v1::CreateStatusListEntryResponse* msg) { msg->clear_status_list_entry(); }
bool __rust_proto_thunk__registry_api_v1_CreateStatusListEntryResponse_has_status_list_entry(::registry_api::v1::CreateStatusListEntryResponse* msg) { return msg->has_status_list_entry(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::CreateStatusListEntryResponse>* __rust_proto_thunk__Map_i32_registry_api_v1_CreateStatusListEntryResponse_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::CreateStatusListEntryResponse>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_CreateStatusListEntryResponse_free(const google::protobuf::Map<int32_t, ::registry_api::v1::CreateStatusListEntryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_CreateStatusListEntryResponse_clear(google::protobuf::Map<int32_t, ::registry_api::v1::CreateStatusListEntryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_CreateStatusListEntryResponse_size(const google::protobuf::Map<int32_t, ::registry_api::v1::CreateStatusListEntryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_CreateStatusListEntryResponse_insert(google::protobuf::Map<int32_t, ::registry_api::v1::CreateStatusListEntryResponse> * m,
                          int32_t key, ::registry_api::v1::CreateStatusListEntryResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_CreateStatusListEntryResponse_get(const google::protobuf::Map<int32_t, ::registry_api::v1::CreateStatusListEntryResponse>* m,
                       int32_t key, const ::registry_api::v1::CreateStatusListEntryResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateStatusListEntryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_CreateStatusListEntryResponse_remove(google::protobuf::Map<int32_t, ::registry_api::v1::CreateStatusListEntryResponse> * m,
                          int32_t key, ::registry_api::v1::CreateStatusListEntryResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_CreateStatusListEntryResponse_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::CreateStatusListEntryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_CreateStatusListEntryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::CreateStatusListEntryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::CreateStatusListEntryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateStatusListEntryResponse>* __rust_proto_thunk__Map_u32_registry_api_v1_CreateStatusListEntryResponse_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::CreateStatusListEntryResponse>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_CreateStatusListEntryResponse_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateStatusListEntryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_CreateStatusListEntryResponse_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::CreateStatusListEntryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_CreateStatusListEntryResponse_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateStatusListEntryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_CreateStatusListEntryResponse_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::CreateStatusListEntryResponse> * m,
                          uint32_t key, ::registry_api::v1::CreateStatusListEntryResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_CreateStatusListEntryResponse_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateStatusListEntryResponse>* m,
                       uint32_t key, const ::registry_api::v1::CreateStatusListEntryResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateStatusListEntryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_CreateStatusListEntryResponse_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::CreateStatusListEntryResponse> * m,
                          uint32_t key, ::registry_api::v1::CreateStatusListEntryResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_CreateStatusListEntryResponse_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::CreateStatusListEntryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_CreateStatusListEntryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::CreateStatusListEntryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::CreateStatusListEntryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::CreateStatusListEntryResponse>* __rust_proto_thunk__Map_i64_registry_api_v1_CreateStatusListEntryResponse_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::CreateStatusListEntryResponse>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_CreateStatusListEntryResponse_free(const google::protobuf::Map<int64_t, ::registry_api::v1::CreateStatusListEntryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_CreateStatusListEntryResponse_clear(google::protobuf::Map<int64_t, ::registry_api::v1::CreateStatusListEntryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_CreateStatusListEntryResponse_size(const google::protobuf::Map<int64_t, ::registry_api::v1::CreateStatusListEntryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_CreateStatusListEntryResponse_insert(google::protobuf::Map<int64_t, ::registry_api::v1::CreateStatusListEntryResponse> * m,
                          int64_t key, ::registry_api::v1::CreateStatusListEntryResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_CreateStatusListEntryResponse_get(const google::protobuf::Map<int64_t, ::registry_api::v1::CreateStatusListEntryResponse>* m,
                       int64_t key, const ::registry_api::v1::CreateStatusListEntryResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateStatusListEntryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_CreateStatusListEntryResponse_remove(google::protobuf::Map<int64_t, ::registry_api::v1::CreateStatusListEntryResponse> * m,
                          int64_t key, ::registry_api::v1::CreateStatusListEntryResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_CreateStatusListEntryResponse_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::CreateStatusListEntryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_CreateStatusListEntryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::CreateStatusListEntryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::CreateStatusListEntryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateStatusListEntryResponse>* __rust_proto_thunk__Map_u64_registry_api_v1_CreateStatusListEntryResponse_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::CreateStatusListEntryResponse>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_CreateStatusListEntryResponse_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateStatusListEntryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_CreateStatusListEntryResponse_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::CreateStatusListEntryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_CreateStatusListEntryResponse_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateStatusListEntryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_CreateStatusListEntryResponse_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::CreateStatusListEntryResponse> * m,
                          uint64_t key, ::registry_api::v1::CreateStatusListEntryResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_CreateStatusListEntryResponse_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateStatusListEntryResponse>* m,
                       uint64_t key, const ::registry_api::v1::CreateStatusListEntryResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateStatusListEntryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_CreateStatusListEntryResponse_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::CreateStatusListEntryResponse> * m,
                          uint64_t key, ::registry_api::v1::CreateStatusListEntryResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_CreateStatusListEntryResponse_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::CreateStatusListEntryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_CreateStatusListEntryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::CreateStatusListEntryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::CreateStatusListEntryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::CreateStatusListEntryResponse>* __rust_proto_thunk__Map_bool_registry_api_v1_CreateStatusListEntryResponse_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::CreateStatusListEntryResponse>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_CreateStatusListEntryResponse_free(const google::protobuf::Map<bool, ::registry_api::v1::CreateStatusListEntryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_CreateStatusListEntryResponse_clear(google::protobuf::Map<bool, ::registry_api::v1::CreateStatusListEntryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_CreateStatusListEntryResponse_size(const google::protobuf::Map<bool, ::registry_api::v1::CreateStatusListEntryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_CreateStatusListEntryResponse_insert(google::protobuf::Map<bool, ::registry_api::v1::CreateStatusListEntryResponse> * m,
                          bool key, ::registry_api::v1::CreateStatusListEntryResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_CreateStatusListEntryResponse_get(const google::protobuf::Map<bool, ::registry_api::v1::CreateStatusListEntryResponse>* m,
                       bool key, const ::registry_api::v1::CreateStatusListEntryResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateStatusListEntryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_CreateStatusListEntryResponse_remove(google::protobuf::Map<bool, ::registry_api::v1::CreateStatusListEntryResponse> * m,
                          bool key, ::registry_api::v1::CreateStatusListEntryResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_CreateStatusListEntryResponse_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::CreateStatusListEntryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_CreateStatusListEntryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::CreateStatusListEntryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::CreateStatusListEntryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::CreateStatusListEntryResponse>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateStatusListEntryResponse_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::CreateStatusListEntryResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateStatusListEntryResponse_free(const google::protobuf::Map<std::string, ::registry_api::v1::CreateStatusListEntryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateStatusListEntryResponse_clear(google::protobuf::Map<std::string, ::registry_api::v1::CreateStatusListEntryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateStatusListEntryResponse_size(const google::protobuf::Map<std::string, ::registry_api::v1::CreateStatusListEntryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateStatusListEntryResponse_insert(google::protobuf::Map<std::string, ::registry_api::v1::CreateStatusListEntryResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::CreateStatusListEntryResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateStatusListEntryResponse_get(const google::protobuf::Map<std::string, ::registry_api::v1::CreateStatusListEntryResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::CreateStatusListEntryResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::CreateStatusListEntryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateStatusListEntryResponse_remove(google::protobuf::Map<std::string, ::registry_api::v1::CreateStatusListEntryResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::CreateStatusListEntryResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateStatusListEntryResponse_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::CreateStatusListEntryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_CreateStatusListEntryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::CreateStatusListEntryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::CreateStatusListEntryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.GetStatusListCredentialRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_GetStatusListCredentialRequest_new() { return new ::registry_api::v1::GetStatusListCredentialRequest(); }
void __rust_proto_thunk__registry_api_v1_GetStatusListCredentialRequest_delete(void* ptr) { delete static_cast<::registry_api::v1::GetStatusListCredentialRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_GetStatusListCredentialRequest_serialize(::registry_api::v1::GetStatusListCredentialRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_GetStatusListCredentialRequest_parse(::registry_api::v1::GetStatusListCredentialRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_GetStatusListCredentialRequest_copy_from(::registry_api::v1::GetStatusListCredentialRequest* dst, const ::registry_api::v1::GetStatusListCredentialRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_GetStatusListCredentialRequest_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::GetStatusListCredentialRequest>* field) {
  return field->size();
}
const ::registry_api::v1::GetStatusListCredentialRequest& __rust_proto_thunk__registry_api_v1_GetStatusListCredentialRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::GetStatusListCredentialRequest>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::GetStatusListCredentialRequest* __rust_proto_thunk__registry_api_v1_GetStatusListCredentialRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::GetStatusListCredentialRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::GetStatusListCredentialRequest* __rust_proto_thunk__registry_api_v1_GetStatusListCredentialRequest_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::GetStatusListCredentialRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_GetStatusListCredentialRequest_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::GetStatusListCredentialRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_GetStatusListCredentialRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::GetStatusListCredentialRequest>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::GetStatusListCredentialRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_GetStatusListCredentialRequest_get_status_list_credential_id(::registry_api::v1::GetStatusListCredentialRequest* msg) {
  absl::string_view val = msg->status_list_credential_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_GetStatusListCredentialRequest_set_status_list_credential_id(::registry_api::v1::GetStatusListCredentialRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_status_list_credential_id(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::GetStatusListCredentialRequest>* __rust_proto_thunk__Map_i32_registry_api_v1_GetStatusListCredentialRequest_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::GetStatusListCredentialRequest>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_GetStatusListCredentialRequest_free(const google::protobuf::Map<int32_t, ::registry_api::v1::GetStatusListCredentialRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_GetStatusListCredentialRequest_clear(google::protobuf::Map<int32_t, ::registry_api::v1::GetStatusListCredentialRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_GetStatusListCredentialRequest_size(const google::protobuf::Map<int32_t, ::registry_api::v1::GetStatusListCredentialRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_GetStatusListCredentialRequest_insert(google::protobuf::Map<int32_t, ::registry_api::v1::GetStatusListCredentialRequest> * m,
                          int32_t key, ::registry_api::v1::GetStatusListCredentialRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_GetStatusListCredentialRequest_get(const google::protobuf::Map<int32_t, ::registry_api::v1::GetStatusListCredentialRequest>* m,
                       int32_t key, const ::registry_api::v1::GetStatusListCredentialRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetStatusListCredentialRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_GetStatusListCredentialRequest_remove(google::protobuf::Map<int32_t, ::registry_api::v1::GetStatusListCredentialRequest> * m,
                          int32_t key, ::registry_api::v1::GetStatusListCredentialRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_GetStatusListCredentialRequest_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::GetStatusListCredentialRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_GetStatusListCredentialRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::GetStatusListCredentialRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::GetStatusListCredentialRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::GetStatusListCredentialRequest>* __rust_proto_thunk__Map_u32_registry_api_v1_GetStatusListCredentialRequest_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::GetStatusListCredentialRequest>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_GetStatusListCredentialRequest_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::GetStatusListCredentialRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_GetStatusListCredentialRequest_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::GetStatusListCredentialRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_GetStatusListCredentialRequest_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::GetStatusListCredentialRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_GetStatusListCredentialRequest_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::GetStatusListCredentialRequest> * m,
                          uint32_t key, ::registry_api::v1::GetStatusListCredentialRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_GetStatusListCredentialRequest_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::GetStatusListCredentialRequest>* m,
                       uint32_t key, const ::registry_api::v1::GetStatusListCredentialRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetStatusListCredentialRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_GetStatusListCredentialRequest_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::GetStatusListCredentialRequest> * m,
                          uint32_t key, ::registry_api::v1::GetStatusListCredentialRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_GetStatusListCredentialRequest_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::GetStatusListCredentialRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_GetStatusListCredentialRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::GetStatusListCredentialRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::GetStatusListCredentialRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::GetStatusListCredentialRequest>* __rust_proto_thunk__Map_i64_registry_api_v1_GetStatusListCredentialRequest_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::GetStatusListCredentialRequest>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_GetStatusListCredentialRequest_free(const google::protobuf::Map<int64_t, ::registry_api::v1::GetStatusListCredentialRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_GetStatusListCredentialRequest_clear(google::protobuf::Map<int64_t, ::registry_api::v1::GetStatusListCredentialRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_GetStatusListCredentialRequest_size(const google::protobuf::Map<int64_t, ::registry_api::v1::GetStatusListCredentialRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_GetStatusListCredentialRequest_insert(google::protobuf::Map<int64_t, ::registry_api::v1::GetStatusListCredentialRequest> * m,
                          int64_t key, ::registry_api::v1::GetStatusListCredentialRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_GetStatusListCredentialRequest_get(const google::protobuf::Map<int64_t, ::registry_api::v1::GetStatusListCredentialRequest>* m,
                       int64_t key, const ::registry_api::v1::GetStatusListCredentialRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetStatusListCredentialRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_GetStatusListCredentialRequest_remove(google::protobuf::Map<int64_t, ::registry_api::v1::GetStatusListCredentialRequest> * m,
                          int64_t key, ::registry_api::v1::GetStatusListCredentialRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_GetStatusListCredentialRequest_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::GetStatusListCredentialRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_GetStatusListCredentialRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::GetStatusListCredentialRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::GetStatusListCredentialRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::GetStatusListCredentialRequest>* __rust_proto_thunk__Map_u64_registry_api_v1_GetStatusListCredentialRequest_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::GetStatusListCredentialRequest>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_GetStatusListCredentialRequest_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::GetStatusListCredentialRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_GetStatusListCredentialRequest_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::GetStatusListCredentialRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_GetStatusListCredentialRequest_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::GetStatusListCredentialRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_GetStatusListCredentialRequest_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::GetStatusListCredentialRequest> * m,
                          uint64_t key, ::registry_api::v1::GetStatusListCredentialRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_GetStatusListCredentialRequest_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::GetStatusListCredentialRequest>* m,
                       uint64_t key, const ::registry_api::v1::GetStatusListCredentialRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetStatusListCredentialRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_GetStatusListCredentialRequest_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::GetStatusListCredentialRequest> * m,
                          uint64_t key, ::registry_api::v1::GetStatusListCredentialRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_GetStatusListCredentialRequest_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::GetStatusListCredentialRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_GetStatusListCredentialRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::GetStatusListCredentialRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::GetStatusListCredentialRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::GetStatusListCredentialRequest>* __rust_proto_thunk__Map_bool_registry_api_v1_GetStatusListCredentialRequest_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::GetStatusListCredentialRequest>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_GetStatusListCredentialRequest_free(const google::protobuf::Map<bool, ::registry_api::v1::GetStatusListCredentialRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_GetStatusListCredentialRequest_clear(google::protobuf::Map<bool, ::registry_api::v1::GetStatusListCredentialRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_GetStatusListCredentialRequest_size(const google::protobuf::Map<bool, ::registry_api::v1::GetStatusListCredentialRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_GetStatusListCredentialRequest_insert(google::protobuf::Map<bool, ::registry_api::v1::GetStatusListCredentialRequest> * m,
                          bool key, ::registry_api::v1::GetStatusListCredentialRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_GetStatusListCredentialRequest_get(const google::protobuf::Map<bool, ::registry_api::v1::GetStatusListCredentialRequest>* m,
                       bool key, const ::registry_api::v1::GetStatusListCredentialRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetStatusListCredentialRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_GetStatusListCredentialRequest_remove(google::protobuf::Map<bool, ::registry_api::v1::GetStatusListCredentialRequest> * m,
                          bool key, ::registry_api::v1::GetStatusListCredentialRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_GetStatusListCredentialRequest_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::GetStatusListCredentialRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_GetStatusListCredentialRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::GetStatusListCredentialRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::GetStatusListCredentialRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::GetStatusListCredentialRequest>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetStatusListCredentialRequest_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::GetStatusListCredentialRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetStatusListCredentialRequest_free(const google::protobuf::Map<std::string, ::registry_api::v1::GetStatusListCredentialRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetStatusListCredentialRequest_clear(google::protobuf::Map<std::string, ::registry_api::v1::GetStatusListCredentialRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetStatusListCredentialRequest_size(const google::protobuf::Map<std::string, ::registry_api::v1::GetStatusListCredentialRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetStatusListCredentialRequest_insert(google::protobuf::Map<std::string, ::registry_api::v1::GetStatusListCredentialRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::GetStatusListCredentialRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetStatusListCredentialRequest_get(const google::protobuf::Map<std::string, ::registry_api::v1::GetStatusListCredentialRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::GetStatusListCredentialRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetStatusListCredentialRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetStatusListCredentialRequest_remove(google::protobuf::Map<std::string, ::registry_api::v1::GetStatusListCredentialRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::GetStatusListCredentialRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetStatusListCredentialRequest_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::GetStatusListCredentialRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetStatusListCredentialRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::GetStatusListCredentialRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::GetStatusListCredentialRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.GetStatusListCredentialResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_GetStatusListCredentialResponse_new() { return new ::registry_api::v1::GetStatusListCredentialResponse(); }
void __rust_proto_thunk__registry_api_v1_GetStatusListCredentialResponse_delete(void* ptr) { delete static_cast<::registry_api::v1::GetStatusListCredentialResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_GetStatusListCredentialResponse_serialize(::registry_api::v1::GetStatusListCredentialResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_GetStatusListCredentialResponse_parse(::registry_api::v1::GetStatusListCredentialResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_GetStatusListCredentialResponse_copy_from(::registry_api::v1::GetStatusListCredentialResponse* dst, const ::registry_api::v1::GetStatusListCredentialResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_GetStatusListCredentialResponse_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::GetStatusListCredentialResponse>* field) {
  return field->size();
}
const ::registry_api::v1::GetStatusListCredentialResponse& __rust_proto_thunk__registry_api_v1_GetStatusListCredentialResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::GetStatusListCredentialResponse>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::GetStatusListCredentialResponse* __rust_proto_thunk__registry_api_v1_GetStatusListCredentialResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::GetStatusListCredentialResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::GetStatusListCredentialResponse* __rust_proto_thunk__registry_api_v1_GetStatusListCredentialResponse_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::GetStatusListCredentialResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_GetStatusListCredentialResponse_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::GetStatusListCredentialResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_GetStatusListCredentialResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::GetStatusListCredentialResponse>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::GetStatusListCredentialResponse>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_GetStatusListCredentialResponse_get_status_list_credential(::registry_api::v1::GetStatusListCredentialResponse* msg) {
  absl::string_view val = msg->status_list_credential();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_GetStatusListCredentialResponse_set_status_list_credential(::registry_api::v1::GetStatusListCredentialResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_status_list_credential(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::GetStatusListCredentialResponse>* __rust_proto_thunk__Map_i32_registry_api_v1_GetStatusListCredentialResponse_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::GetStatusListCredentialResponse>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_GetStatusListCredentialResponse_free(const google::protobuf::Map<int32_t, ::registry_api::v1::GetStatusListCredentialResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_GetStatusListCredentialResponse_clear(google::protobuf::Map<int32_t, ::registry_api::v1::GetStatusListCredentialResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_GetStatusListCredentialResponse_size(const google::protobuf::Map<int32_t, ::registry_api::v1::GetStatusListCredentialResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_GetStatusListCredentialResponse_insert(google::protobuf::Map<int32_t, ::registry_api::v1::GetStatusListCredentialResponse> * m,
                          int32_t key, ::registry_api::v1::GetStatusListCredentialResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_GetStatusListCredentialResponse_get(const google::protobuf::Map<int32_t, ::registry_api::v1::GetStatusListCredentialResponse>* m,
                       int32_t key, const ::registry_api::v1::GetStatusListCredentialResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetStatusListCredentialResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_GetStatusListCredentialResponse_remove(google::protobuf::Map<int32_t, ::registry_api::v1::GetStatusListCredentialResponse> * m,
                          int32_t key, ::registry_api::v1::GetStatusListCredentialResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_GetStatusListCredentialResponse_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::GetStatusListCredentialResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_GetStatusListCredentialResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::GetStatusListCredentialResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::GetStatusListCredentialResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::GetStatusListCredentialResponse>* __rust_proto_thunk__Map_u32_registry_api_v1_GetStatusListCredentialResponse_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::GetStatusListCredentialResponse>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_GetStatusListCredentialResponse_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::GetStatusListCredentialResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_GetStatusListCredentialResponse_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::GetStatusListCredentialResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_GetStatusListCredentialResponse_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::GetStatusListCredentialResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_GetStatusListCredentialResponse_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::GetStatusListCredentialResponse> * m,
                          uint32_t key, ::registry_api::v1::GetStatusListCredentialResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_GetStatusListCredentialResponse_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::GetStatusListCredentialResponse>* m,
                       uint32_t key, const ::registry_api::v1::GetStatusListCredentialResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetStatusListCredentialResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_GetStatusListCredentialResponse_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::GetStatusListCredentialResponse> * m,
                          uint32_t key, ::registry_api::v1::GetStatusListCredentialResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_GetStatusListCredentialResponse_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::GetStatusListCredentialResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_GetStatusListCredentialResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::GetStatusListCredentialResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::GetStatusListCredentialResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::GetStatusListCredentialResponse>* __rust_proto_thunk__Map_i64_registry_api_v1_GetStatusListCredentialResponse_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::GetStatusListCredentialResponse>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_GetStatusListCredentialResponse_free(const google::protobuf::Map<int64_t, ::registry_api::v1::GetStatusListCredentialResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_GetStatusListCredentialResponse_clear(google::protobuf::Map<int64_t, ::registry_api::v1::GetStatusListCredentialResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_GetStatusListCredentialResponse_size(const google::protobuf::Map<int64_t, ::registry_api::v1::GetStatusListCredentialResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_GetStatusListCredentialResponse_insert(google::protobuf::Map<int64_t, ::registry_api::v1::GetStatusListCredentialResponse> * m,
                          int64_t key, ::registry_api::v1::GetStatusListCredentialResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_GetStatusListCredentialResponse_get(const google::protobuf::Map<int64_t, ::registry_api::v1::GetStatusListCredentialResponse>* m,
                       int64_t key, const ::registry_api::v1::GetStatusListCredentialResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetStatusListCredentialResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_GetStatusListCredentialResponse_remove(google::protobuf::Map<int64_t, ::registry_api::v1::GetStatusListCredentialResponse> * m,
                          int64_t key, ::registry_api::v1::GetStatusListCredentialResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_GetStatusListCredentialResponse_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::GetStatusListCredentialResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_GetStatusListCredentialResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::GetStatusListCredentialResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::GetStatusListCredentialResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::GetStatusListCredentialResponse>* __rust_proto_thunk__Map_u64_registry_api_v1_GetStatusListCredentialResponse_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::GetStatusListCredentialResponse>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_GetStatusListCredentialResponse_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::GetStatusListCredentialResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_GetStatusListCredentialResponse_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::GetStatusListCredentialResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_GetStatusListCredentialResponse_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::GetStatusListCredentialResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_GetStatusListCredentialResponse_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::GetStatusListCredentialResponse> * m,
                          uint64_t key, ::registry_api::v1::GetStatusListCredentialResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_GetStatusListCredentialResponse_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::GetStatusListCredentialResponse>* m,
                       uint64_t key, const ::registry_api::v1::GetStatusListCredentialResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetStatusListCredentialResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_GetStatusListCredentialResponse_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::GetStatusListCredentialResponse> * m,
                          uint64_t key, ::registry_api::v1::GetStatusListCredentialResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_GetStatusListCredentialResponse_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::GetStatusListCredentialResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_GetStatusListCredentialResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::GetStatusListCredentialResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::GetStatusListCredentialResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::GetStatusListCredentialResponse>* __rust_proto_thunk__Map_bool_registry_api_v1_GetStatusListCredentialResponse_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::GetStatusListCredentialResponse>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_GetStatusListCredentialResponse_free(const google::protobuf::Map<bool, ::registry_api::v1::GetStatusListCredentialResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_GetStatusListCredentialResponse_clear(google::protobuf::Map<bool, ::registry_api::v1::GetStatusListCredentialResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_GetStatusListCredentialResponse_size(const google::protobuf::Map<bool, ::registry_api::v1::GetStatusListCredentialResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_GetStatusListCredentialResponse_insert(google::protobuf::Map<bool, ::registry_api::v1::GetStatusListCredentialResponse> * m,
                          bool key, ::registry_api::v1::GetStatusListCredentialResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_GetStatusListCredentialResponse_get(const google::protobuf::Map<bool, ::registry_api::v1::GetStatusListCredentialResponse>* m,
                       bool key, const ::registry_api::v1::GetStatusListCredentialResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetStatusListCredentialResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_GetStatusListCredentialResponse_remove(google::protobuf::Map<bool, ::registry_api::v1::GetStatusListCredentialResponse> * m,
                          bool key, ::registry_api::v1::GetStatusListCredentialResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_GetStatusListCredentialResponse_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::GetStatusListCredentialResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_GetStatusListCredentialResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::GetStatusListCredentialResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::GetStatusListCredentialResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::GetStatusListCredentialResponse>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetStatusListCredentialResponse_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::GetStatusListCredentialResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetStatusListCredentialResponse_free(const google::protobuf::Map<std::string, ::registry_api::v1::GetStatusListCredentialResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetStatusListCredentialResponse_clear(google::protobuf::Map<std::string, ::registry_api::v1::GetStatusListCredentialResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetStatusListCredentialResponse_size(const google::protobuf::Map<std::string, ::registry_api::v1::GetStatusListCredentialResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetStatusListCredentialResponse_insert(google::protobuf::Map<std::string, ::registry_api::v1::GetStatusListCredentialResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::GetStatusListCredentialResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetStatusListCredentialResponse_get(const google::protobuf::Map<std::string, ::registry_api::v1::GetStatusListCredentialResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::GetStatusListCredentialResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::GetStatusListCredentialResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetStatusListCredentialResponse_remove(google::protobuf::Map<std::string, ::registry_api::v1::GetStatusListCredentialResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::GetStatusListCredentialResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetStatusListCredentialResponse_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::GetStatusListCredentialResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_GetStatusListCredentialResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::GetStatusListCredentialResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::GetStatusListCredentialResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.UpdateStatusListEntryRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryRequest_new() { return new ::registry_api::v1::UpdateStatusListEntryRequest(); }
void __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryRequest_delete(void* ptr) { delete static_cast<::registry_api::v1::UpdateStatusListEntryRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryRequest_serialize(::registry_api::v1::UpdateStatusListEntryRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryRequest_parse(::registry_api::v1::UpdateStatusListEntryRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryRequest_copy_from(::registry_api::v1::UpdateStatusListEntryRequest* dst, const ::registry_api::v1::UpdateStatusListEntryRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryRequest_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateStatusListEntryRequest>* field) {
  return field->size();
}
const ::registry_api::v1::UpdateStatusListEntryRequest& __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateStatusListEntryRequest>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::UpdateStatusListEntryRequest* __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateStatusListEntryRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::UpdateStatusListEntryRequest* __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryRequest_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateStatusListEntryRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryRequest_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateStatusListEntryRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateStatusListEntryRequest>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateStatusListEntryRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryRequest_get_credential_id(::registry_api::v1::UpdateStatusListEntryRequest* msg) {
  absl::string_view val = msg->credential_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryRequest_set_credential_id(::registry_api::v1::UpdateStatusListEntryRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_credential_id(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryRequest_get_credential_issuer_id(::registry_api::v1::UpdateStatusListEntryRequest* msg) {
  absl::string_view val = msg->credential_issuer_id();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryRequest_set_credential_issuer_id(::registry_api::v1::UpdateStatusListEntryRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_credential_issuer_id(absl::string_view(s.ptr, s.len));
}

::registry_api::v1::StatusPurpose __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryRequest_get_status_purpose(::registry_api::v1::UpdateStatusListEntryRequest* msg) { return msg->status_purpose(); }
void __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryRequest_set_status_purpose(::registry_api::v1::UpdateStatusListEntryRequest* msg, ::registry_api::v1::StatusPurpose val) {
  msg->set_status_purpose(val);
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryRequest_get_credential_type(::registry_api::v1::UpdateStatusListEntryRequest* msg) {
  absl::string_view val = msg->credential_type();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryRequest_set_credential_type(::registry_api::v1::UpdateStatusListEntryRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_credential_type(absl::string_view(s.ptr, s.len));
}

bool __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryRequest_get_status(::registry_api::v1::UpdateStatusListEntryRequest* msg) { return msg->status(); }
void __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryRequest_set_status(::registry_api::v1::UpdateStatusListEntryRequest* msg, bool val) {
  msg->set_status(val);
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateStatusListEntryRequest>* __rust_proto_thunk__Map_i32_registry_api_v1_UpdateStatusListEntryRequest_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::UpdateStatusListEntryRequest>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_UpdateStatusListEntryRequest_free(const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateStatusListEntryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_UpdateStatusListEntryRequest_clear(google::protobuf::Map<int32_t, ::registry_api::v1::UpdateStatusListEntryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_UpdateStatusListEntryRequest_size(const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateStatusListEntryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_UpdateStatusListEntryRequest_insert(google::protobuf::Map<int32_t, ::registry_api::v1::UpdateStatusListEntryRequest> * m,
                          int32_t key, ::registry_api::v1::UpdateStatusListEntryRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_UpdateStatusListEntryRequest_get(const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateStatusListEntryRequest>* m,
                       int32_t key, const ::registry_api::v1::UpdateStatusListEntryRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateStatusListEntryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_UpdateStatusListEntryRequest_remove(google::protobuf::Map<int32_t, ::registry_api::v1::UpdateStatusListEntryRequest> * m,
                          int32_t key, ::registry_api::v1::UpdateStatusListEntryRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_UpdateStatusListEntryRequest_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateStatusListEntryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_UpdateStatusListEntryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::UpdateStatusListEntryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::UpdateStatusListEntryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateStatusListEntryRequest>* __rust_proto_thunk__Map_u32_registry_api_v1_UpdateStatusListEntryRequest_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateStatusListEntryRequest>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_UpdateStatusListEntryRequest_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateStatusListEntryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_UpdateStatusListEntryRequest_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateStatusListEntryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_UpdateStatusListEntryRequest_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateStatusListEntryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_UpdateStatusListEntryRequest_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateStatusListEntryRequest> * m,
                          uint32_t key, ::registry_api::v1::UpdateStatusListEntryRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_UpdateStatusListEntryRequest_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateStatusListEntryRequest>* m,
                       uint32_t key, const ::registry_api::v1::UpdateStatusListEntryRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateStatusListEntryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_UpdateStatusListEntryRequest_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateStatusListEntryRequest> * m,
                          uint32_t key, ::registry_api::v1::UpdateStatusListEntryRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_UpdateStatusListEntryRequest_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateStatusListEntryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_UpdateStatusListEntryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::UpdateStatusListEntryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateStatusListEntryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateStatusListEntryRequest>* __rust_proto_thunk__Map_i64_registry_api_v1_UpdateStatusListEntryRequest_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::UpdateStatusListEntryRequest>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_UpdateStatusListEntryRequest_free(const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateStatusListEntryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_UpdateStatusListEntryRequest_clear(google::protobuf::Map<int64_t, ::registry_api::v1::UpdateStatusListEntryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_UpdateStatusListEntryRequest_size(const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateStatusListEntryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_UpdateStatusListEntryRequest_insert(google::protobuf::Map<int64_t, ::registry_api::v1::UpdateStatusListEntryRequest> * m,
                          int64_t key, ::registry_api::v1::UpdateStatusListEntryRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_UpdateStatusListEntryRequest_get(const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateStatusListEntryRequest>* m,
                       int64_t key, const ::registry_api::v1::UpdateStatusListEntryRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateStatusListEntryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_UpdateStatusListEntryRequest_remove(google::protobuf::Map<int64_t, ::registry_api::v1::UpdateStatusListEntryRequest> * m,
                          int64_t key, ::registry_api::v1::UpdateStatusListEntryRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_UpdateStatusListEntryRequest_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateStatusListEntryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_UpdateStatusListEntryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::UpdateStatusListEntryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::UpdateStatusListEntryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateStatusListEntryRequest>* __rust_proto_thunk__Map_u64_registry_api_v1_UpdateStatusListEntryRequest_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateStatusListEntryRequest>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_UpdateStatusListEntryRequest_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateStatusListEntryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_UpdateStatusListEntryRequest_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateStatusListEntryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_UpdateStatusListEntryRequest_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateStatusListEntryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_UpdateStatusListEntryRequest_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateStatusListEntryRequest> * m,
                          uint64_t key, ::registry_api::v1::UpdateStatusListEntryRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_UpdateStatusListEntryRequest_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateStatusListEntryRequest>* m,
                       uint64_t key, const ::registry_api::v1::UpdateStatusListEntryRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateStatusListEntryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_UpdateStatusListEntryRequest_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateStatusListEntryRequest> * m,
                          uint64_t key, ::registry_api::v1::UpdateStatusListEntryRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_UpdateStatusListEntryRequest_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateStatusListEntryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_UpdateStatusListEntryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::UpdateStatusListEntryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateStatusListEntryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::UpdateStatusListEntryRequest>* __rust_proto_thunk__Map_bool_registry_api_v1_UpdateStatusListEntryRequest_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::UpdateStatusListEntryRequest>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_UpdateStatusListEntryRequest_free(const google::protobuf::Map<bool, ::registry_api::v1::UpdateStatusListEntryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_UpdateStatusListEntryRequest_clear(google::protobuf::Map<bool, ::registry_api::v1::UpdateStatusListEntryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_UpdateStatusListEntryRequest_size(const google::protobuf::Map<bool, ::registry_api::v1::UpdateStatusListEntryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_UpdateStatusListEntryRequest_insert(google::protobuf::Map<bool, ::registry_api::v1::UpdateStatusListEntryRequest> * m,
                          bool key, ::registry_api::v1::UpdateStatusListEntryRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_UpdateStatusListEntryRequest_get(const google::protobuf::Map<bool, ::registry_api::v1::UpdateStatusListEntryRequest>* m,
                       bool key, const ::registry_api::v1::UpdateStatusListEntryRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateStatusListEntryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_UpdateStatusListEntryRequest_remove(google::protobuf::Map<bool, ::registry_api::v1::UpdateStatusListEntryRequest> * m,
                          bool key, ::registry_api::v1::UpdateStatusListEntryRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_UpdateStatusListEntryRequest_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::UpdateStatusListEntryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_UpdateStatusListEntryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::UpdateStatusListEntryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::UpdateStatusListEntryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::UpdateStatusListEntryRequest>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateStatusListEntryRequest_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::UpdateStatusListEntryRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateStatusListEntryRequest_free(const google::protobuf::Map<std::string, ::registry_api::v1::UpdateStatusListEntryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateStatusListEntryRequest_clear(google::protobuf::Map<std::string, ::registry_api::v1::UpdateStatusListEntryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateStatusListEntryRequest_size(const google::protobuf::Map<std::string, ::registry_api::v1::UpdateStatusListEntryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateStatusListEntryRequest_insert(google::protobuf::Map<std::string, ::registry_api::v1::UpdateStatusListEntryRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::UpdateStatusListEntryRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateStatusListEntryRequest_get(const google::protobuf::Map<std::string, ::registry_api::v1::UpdateStatusListEntryRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::UpdateStatusListEntryRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateStatusListEntryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateStatusListEntryRequest_remove(google::protobuf::Map<std::string, ::registry_api::v1::UpdateStatusListEntryRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::UpdateStatusListEntryRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateStatusListEntryRequest_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::UpdateStatusListEntryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateStatusListEntryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::UpdateStatusListEntryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::UpdateStatusListEntryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.UpdateStatusListEntryResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryResponse_new() { return new ::registry_api::v1::UpdateStatusListEntryResponse(); }
void __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryResponse_delete(void* ptr) { delete static_cast<::registry_api::v1::UpdateStatusListEntryResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryResponse_serialize(::registry_api::v1::UpdateStatusListEntryResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryResponse_parse(::registry_api::v1::UpdateStatusListEntryResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryResponse_copy_from(::registry_api::v1::UpdateStatusListEntryResponse* dst, const ::registry_api::v1::UpdateStatusListEntryResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryResponse_repeated_len(google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateStatusListEntryResponse>* field) {
  return field->size();
}
const ::registry_api::v1::UpdateStatusListEntryResponse& __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateStatusListEntryResponse>* field,
  size_t index) {
  return field->Get(index);
}
::registry_api::v1::UpdateStatusListEntryResponse* __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateStatusListEntryResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::registry_api::v1::UpdateStatusListEntryResponse* __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryResponse_repeated_add(google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateStatusListEntryResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryResponse_repeated_clear(google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateStatusListEntryResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__registry_api_v1_UpdateStatusListEntryResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateStatusListEntryResponse>& dst,
  const google::protobuf::RepeatedPtrField<::registry_api::v1::UpdateStatusListEntryResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateStatusListEntryResponse>* __rust_proto_thunk__Map_i32_registry_api_v1_UpdateStatusListEntryResponse_new() {
    return new google::protobuf::Map<int32_t, ::registry_api::v1::UpdateStatusListEntryResponse>();
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_UpdateStatusListEntryResponse_free(const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateStatusListEntryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_registry_api_v1_UpdateStatusListEntryResponse_clear(google::protobuf::Map<int32_t, ::registry_api::v1::UpdateStatusListEntryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_registry_api_v1_UpdateStatusListEntryResponse_size(const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateStatusListEntryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_UpdateStatusListEntryResponse_insert(google::protobuf::Map<int32_t, ::registry_api::v1::UpdateStatusListEntryResponse> * m,
                          int32_t key, ::registry_api::v1::UpdateStatusListEntryResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_UpdateStatusListEntryResponse_get(const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateStatusListEntryResponse>* m,
                       int32_t key, const ::registry_api::v1::UpdateStatusListEntryResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateStatusListEntryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_registry_api_v1_UpdateStatusListEntryResponse_remove(google::protobuf::Map<int32_t, ::registry_api::v1::UpdateStatusListEntryResponse> * m,
                          int32_t key, ::registry_api::v1::UpdateStatusListEntryResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_registry_api_v1_UpdateStatusListEntryResponse_iter(
      const google::protobuf::Map<int32_t, ::registry_api::v1::UpdateStatusListEntryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_registry_api_v1_UpdateStatusListEntryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::registry_api::v1::UpdateStatusListEntryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::registry_api::v1::UpdateStatusListEntryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateStatusListEntryResponse>* __rust_proto_thunk__Map_u32_registry_api_v1_UpdateStatusListEntryResponse_new() {
    return new google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateStatusListEntryResponse>();
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_UpdateStatusListEntryResponse_free(const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateStatusListEntryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_registry_api_v1_UpdateStatusListEntryResponse_clear(google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateStatusListEntryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_registry_api_v1_UpdateStatusListEntryResponse_size(const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateStatusListEntryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_UpdateStatusListEntryResponse_insert(google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateStatusListEntryResponse> * m,
                          uint32_t key, ::registry_api::v1::UpdateStatusListEntryResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_UpdateStatusListEntryResponse_get(const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateStatusListEntryResponse>* m,
                       uint32_t key, const ::registry_api::v1::UpdateStatusListEntryResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateStatusListEntryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_registry_api_v1_UpdateStatusListEntryResponse_remove(google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateStatusListEntryResponse> * m,
                          uint32_t key, ::registry_api::v1::UpdateStatusListEntryResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_registry_api_v1_UpdateStatusListEntryResponse_iter(
      const google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateStatusListEntryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_registry_api_v1_UpdateStatusListEntryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::registry_api::v1::UpdateStatusListEntryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::registry_api::v1::UpdateStatusListEntryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateStatusListEntryResponse>* __rust_proto_thunk__Map_i64_registry_api_v1_UpdateStatusListEntryResponse_new() {
    return new google::protobuf::Map<int64_t, ::registry_api::v1::UpdateStatusListEntryResponse>();
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_UpdateStatusListEntryResponse_free(const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateStatusListEntryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_registry_api_v1_UpdateStatusListEntryResponse_clear(google::protobuf::Map<int64_t, ::registry_api::v1::UpdateStatusListEntryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_registry_api_v1_UpdateStatusListEntryResponse_size(const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateStatusListEntryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_UpdateStatusListEntryResponse_insert(google::protobuf::Map<int64_t, ::registry_api::v1::UpdateStatusListEntryResponse> * m,
                          int64_t key, ::registry_api::v1::UpdateStatusListEntryResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_UpdateStatusListEntryResponse_get(const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateStatusListEntryResponse>* m,
                       int64_t key, const ::registry_api::v1::UpdateStatusListEntryResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateStatusListEntryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_registry_api_v1_UpdateStatusListEntryResponse_remove(google::protobuf::Map<int64_t, ::registry_api::v1::UpdateStatusListEntryResponse> * m,
                          int64_t key, ::registry_api::v1::UpdateStatusListEntryResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_registry_api_v1_UpdateStatusListEntryResponse_iter(
      const google::protobuf::Map<int64_t, ::registry_api::v1::UpdateStatusListEntryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_registry_api_v1_UpdateStatusListEntryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::registry_api::v1::UpdateStatusListEntryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::registry_api::v1::UpdateStatusListEntryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateStatusListEntryResponse>* __rust_proto_thunk__Map_u64_registry_api_v1_UpdateStatusListEntryResponse_new() {
    return new google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateStatusListEntryResponse>();
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_UpdateStatusListEntryResponse_free(const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateStatusListEntryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_registry_api_v1_UpdateStatusListEntryResponse_clear(google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateStatusListEntryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_registry_api_v1_UpdateStatusListEntryResponse_size(const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateStatusListEntryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_UpdateStatusListEntryResponse_insert(google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateStatusListEntryResponse> * m,
                          uint64_t key, ::registry_api::v1::UpdateStatusListEntryResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_UpdateStatusListEntryResponse_get(const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateStatusListEntryResponse>* m,
                       uint64_t key, const ::registry_api::v1::UpdateStatusListEntryResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateStatusListEntryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_registry_api_v1_UpdateStatusListEntryResponse_remove(google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateStatusListEntryResponse> * m,
                          uint64_t key, ::registry_api::v1::UpdateStatusListEntryResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_registry_api_v1_UpdateStatusListEntryResponse_iter(
      const google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateStatusListEntryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_registry_api_v1_UpdateStatusListEntryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::registry_api::v1::UpdateStatusListEntryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::registry_api::v1::UpdateStatusListEntryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::registry_api::v1::UpdateStatusListEntryResponse>* __rust_proto_thunk__Map_bool_registry_api_v1_UpdateStatusListEntryResponse_new() {
    return new google::protobuf::Map<bool, ::registry_api::v1::UpdateStatusListEntryResponse>();
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_UpdateStatusListEntryResponse_free(const google::protobuf::Map<bool, ::registry_api::v1::UpdateStatusListEntryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_registry_api_v1_UpdateStatusListEntryResponse_clear(google::protobuf::Map<bool, ::registry_api::v1::UpdateStatusListEntryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_registry_api_v1_UpdateStatusListEntryResponse_size(const google::protobuf::Map<bool, ::registry_api::v1::UpdateStatusListEntryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_UpdateStatusListEntryResponse_insert(google::protobuf::Map<bool, ::registry_api::v1::UpdateStatusListEntryResponse> * m,
                          bool key, ::registry_api::v1::UpdateStatusListEntryResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_UpdateStatusListEntryResponse_get(const google::protobuf::Map<bool, ::registry_api::v1::UpdateStatusListEntryResponse>* m,
                       bool key, const ::registry_api::v1::UpdateStatusListEntryResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateStatusListEntryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_registry_api_v1_UpdateStatusListEntryResponse_remove(google::protobuf::Map<bool, ::registry_api::v1::UpdateStatusListEntryResponse> * m,
                          bool key, ::registry_api::v1::UpdateStatusListEntryResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_registry_api_v1_UpdateStatusListEntryResponse_iter(
      const google::protobuf::Map<bool, ::registry_api::v1::UpdateStatusListEntryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_registry_api_v1_UpdateStatusListEntryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::registry_api::v1::UpdateStatusListEntryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::registry_api::v1::UpdateStatusListEntryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::registry_api::v1::UpdateStatusListEntryResponse>* __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateStatusListEntryResponse_new() {
    return new google::protobuf::Map<std::string, ::registry_api::v1::UpdateStatusListEntryResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateStatusListEntryResponse_free(const google::protobuf::Map<std::string, ::registry_api::v1::UpdateStatusListEntryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateStatusListEntryResponse_clear(google::protobuf::Map<std::string, ::registry_api::v1::UpdateStatusListEntryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateStatusListEntryResponse_size(const google::protobuf::Map<std::string, ::registry_api::v1::UpdateStatusListEntryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateStatusListEntryResponse_insert(google::protobuf::Map<std::string, ::registry_api::v1::UpdateStatusListEntryResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::UpdateStatusListEntryResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateStatusListEntryResponse_get(const google::protobuf::Map<std::string, ::registry_api::v1::UpdateStatusListEntryResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::registry_api::v1::UpdateStatusListEntryResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::registry_api::v1::UpdateStatusListEntryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateStatusListEntryResponse_remove(google::protobuf::Map<std::string, ::registry_api::v1::UpdateStatusListEntryResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::registry_api::v1::UpdateStatusListEntryResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateStatusListEntryResponse_iter(
      const google::protobuf::Map<std::string, ::registry_api::v1::UpdateStatusListEntryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_registry_api_v1_UpdateStatusListEntryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::registry_api::v1::UpdateStatusListEntryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::registry_api::v1::UpdateStatusListEntryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// registry_api.v1.ResolutionOptionAccept
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::registry_api::v1::ResolutionOptionAccept, registry_api_v1_ResolutionOptionAccept, ::registry_api::v1::ResolutionOptionAccept, value, cpp_value)
}

// registry_api.v1.IssuerStatus
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::registry_api::v1::IssuerStatus, registry_api_v1_IssuerStatus, ::registry_api::v1::IssuerStatus, value, cpp_value)
}

// registry_api.v1.StatusPurpose
extern "C" {
  __PB_RUST_EXPOSE_SCALAR_MAP_METHODS_FOR_VALUE_TYPE(
      ::registry_api::v1::StatusPurpose, registry_api_v1_StatusPurpose, ::registry_api::v1::StatusPurpose, value, cpp_value)
}

