#include "authority_api/v1/authority.proto.h"
#include "google/protobuf/rust/cpp_kernel/cpp_api.h"
// authority_api.v1.SetIssuerLimitRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_new() { return new ::authority_api::v1::SetIssuerLimitRequest(); }
void __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_delete(void* ptr) { delete static_cast<::authority_api::v1::SetIssuerLimitRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_serialize(::authority_api::v1::SetIssuerLimitRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_parse(::authority_api::v1::SetIssuerLimitRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_copy_from(::authority_api::v1::SetIssuerLimitRequest* dst, const ::authority_api::v1::SetIssuerLimitRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_repeated_len(google::protobuf::RepeatedPtrField<::authority_api::v1::SetIssuerLimitRequest>* field) {
  return field->size();
}
const ::authority_api::v1::SetIssuerLimitRequest& __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::authority_api::v1::SetIssuerLimitRequest>* field,
  size_t index) {
  return field->Get(index);
}
::authority_api::v1::SetIssuerLimitRequest* __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::authority_api::v1::SetIssuerLimitRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::authority_api::v1::SetIssuerLimitRequest* __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_repeated_add(google::protobuf::RepeatedPtrField<::authority_api::v1::SetIssuerLimitRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_repeated_clear(google::protobuf::RepeatedPtrField<::authority_api::v1::SetIssuerLimitRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::authority_api::v1::SetIssuerLimitRequest>& dst,
  const google::protobuf::RepeatedPtrField<::authority_api::v1::SetIssuerLimitRequest>& src) {
  dst = src;
}

const void* __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_get_limit(::authority_api::v1::SetIssuerLimitRequest* msg) {
  return static_cast<const void*>(&msg->limit());
}
void* __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_get_mut_limit(::authority_api::v1::SetIssuerLimitRequest* msg) {
  return static_cast<void*>(msg->mutable_limit());
}
void __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_clear_limit(::authority_api::v1::SetIssuerLimitRequest* msg) { msg->clear_limit(); }
bool __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_has_limit(::authority_api::v1::SetIssuerLimitRequest* msg) { return msg->has_limit(); }

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_get_issuer_public_key(::authority_api::v1::SetIssuerLimitRequest* msg) {
  absl::string_view val = msg->issuer_public_key();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__authority_api_v1_SetIssuerLimitRequest_set_issuer_public_key(::authority_api::v1::SetIssuerLimitRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_issuer_public_key(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::authority_api::v1::SetIssuerLimitRequest>* __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_new() {
    return new google::protobuf::Map<int32_t, ::authority_api::v1::SetIssuerLimitRequest>();
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_free(const google::protobuf::Map<int32_t, ::authority_api::v1::SetIssuerLimitRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_clear(google::protobuf::Map<int32_t, ::authority_api::v1::SetIssuerLimitRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_size(const google::protobuf::Map<int32_t, ::authority_api::v1::SetIssuerLimitRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_insert(google::protobuf::Map<int32_t, ::authority_api::v1::SetIssuerLimitRequest> * m,
                          int32_t key, ::authority_api::v1::SetIssuerLimitRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_get(const google::protobuf::Map<int32_t, ::authority_api::v1::SetIssuerLimitRequest>* m,
                       int32_t key, const ::authority_api::v1::SetIssuerLimitRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetIssuerLimitRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_remove(google::protobuf::Map<int32_t, ::authority_api::v1::SetIssuerLimitRequest> * m,
                          int32_t key, ::authority_api::v1::SetIssuerLimitRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_iter(
      const google::protobuf::Map<int32_t, ::authority_api::v1::SetIssuerLimitRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::authority_api::v1::SetIssuerLimitRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::authority_api::v1::SetIssuerLimitRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::authority_api::v1::SetIssuerLimitRequest>* __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_new() {
    return new google::protobuf::Map<uint32_t, ::authority_api::v1::SetIssuerLimitRequest>();
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_free(const google::protobuf::Map<uint32_t, ::authority_api::v1::SetIssuerLimitRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_clear(google::protobuf::Map<uint32_t, ::authority_api::v1::SetIssuerLimitRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_size(const google::protobuf::Map<uint32_t, ::authority_api::v1::SetIssuerLimitRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_insert(google::protobuf::Map<uint32_t, ::authority_api::v1::SetIssuerLimitRequest> * m,
                          uint32_t key, ::authority_api::v1::SetIssuerLimitRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_get(const google::protobuf::Map<uint32_t, ::authority_api::v1::SetIssuerLimitRequest>* m,
                       uint32_t key, const ::authority_api::v1::SetIssuerLimitRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetIssuerLimitRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_remove(google::protobuf::Map<uint32_t, ::authority_api::v1::SetIssuerLimitRequest> * m,
                          uint32_t key, ::authority_api::v1::SetIssuerLimitRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_iter(
      const google::protobuf::Map<uint32_t, ::authority_api::v1::SetIssuerLimitRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::authority_api::v1::SetIssuerLimitRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::authority_api::v1::SetIssuerLimitRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::authority_api::v1::SetIssuerLimitRequest>* __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_new() {
    return new google::protobuf::Map<int64_t, ::authority_api::v1::SetIssuerLimitRequest>();
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_free(const google::protobuf::Map<int64_t, ::authority_api::v1::SetIssuerLimitRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_clear(google::protobuf::Map<int64_t, ::authority_api::v1::SetIssuerLimitRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_size(const google::protobuf::Map<int64_t, ::authority_api::v1::SetIssuerLimitRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_insert(google::protobuf::Map<int64_t, ::authority_api::v1::SetIssuerLimitRequest> * m,
                          int64_t key, ::authority_api::v1::SetIssuerLimitRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_get(const google::protobuf::Map<int64_t, ::authority_api::v1::SetIssuerLimitRequest>* m,
                       int64_t key, const ::authority_api::v1::SetIssuerLimitRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetIssuerLimitRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_remove(google::protobuf::Map<int64_t, ::authority_api::v1::SetIssuerLimitRequest> * m,
                          int64_t key, ::authority_api::v1::SetIssuerLimitRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_iter(
      const google::protobuf::Map<int64_t, ::authority_api::v1::SetIssuerLimitRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::authority_api::v1::SetIssuerLimitRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::authority_api::v1::SetIssuerLimitRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::authority_api::v1::SetIssuerLimitRequest>* __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_new() {
    return new google::protobuf::Map<uint64_t, ::authority_api::v1::SetIssuerLimitRequest>();
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_free(const google::protobuf::Map<uint64_t, ::authority_api::v1::SetIssuerLimitRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_clear(google::protobuf::Map<uint64_t, ::authority_api::v1::SetIssuerLimitRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_size(const google::protobuf::Map<uint64_t, ::authority_api::v1::SetIssuerLimitRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_insert(google::protobuf::Map<uint64_t, ::authority_api::v1::SetIssuerLimitRequest> * m,
                          uint64_t key, ::authority_api::v1::SetIssuerLimitRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_get(const google::protobuf::Map<uint64_t, ::authority_api::v1::SetIssuerLimitRequest>* m,
                       uint64_t key, const ::authority_api::v1::SetIssuerLimitRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetIssuerLimitRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_remove(google::protobuf::Map<uint64_t, ::authority_api::v1::SetIssuerLimitRequest> * m,
                          uint64_t key, ::authority_api::v1::SetIssuerLimitRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_iter(
      const google::protobuf::Map<uint64_t, ::authority_api::v1::SetIssuerLimitRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::authority_api::v1::SetIssuerLimitRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::authority_api::v1::SetIssuerLimitRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::authority_api::v1::SetIssuerLimitRequest>* __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_new() {
    return new google::protobuf::Map<bool, ::authority_api::v1::SetIssuerLimitRequest>();
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_free(const google::protobuf::Map<bool, ::authority_api::v1::SetIssuerLimitRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_clear(google::protobuf::Map<bool, ::authority_api::v1::SetIssuerLimitRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_size(const google::protobuf::Map<bool, ::authority_api::v1::SetIssuerLimitRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_insert(google::protobuf::Map<bool, ::authority_api::v1::SetIssuerLimitRequest> * m,
                          bool key, ::authority_api::v1::SetIssuerLimitRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_get(const google::protobuf::Map<bool, ::authority_api::v1::SetIssuerLimitRequest>* m,
                       bool key, const ::authority_api::v1::SetIssuerLimitRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetIssuerLimitRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_remove(google::protobuf::Map<bool, ::authority_api::v1::SetIssuerLimitRequest> * m,
                          bool key, ::authority_api::v1::SetIssuerLimitRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_iter(
      const google::protobuf::Map<bool, ::authority_api::v1::SetIssuerLimitRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::authority_api::v1::SetIssuerLimitRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::authority_api::v1::SetIssuerLimitRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::authority_api::v1::SetIssuerLimitRequest>* __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_new() {
    return new google::protobuf::Map<std::string, ::authority_api::v1::SetIssuerLimitRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_free(const google::protobuf::Map<std::string, ::authority_api::v1::SetIssuerLimitRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_clear(google::protobuf::Map<std::string, ::authority_api::v1::SetIssuerLimitRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_size(const google::protobuf::Map<std::string, ::authority_api::v1::SetIssuerLimitRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_insert(google::protobuf::Map<std::string, ::authority_api::v1::SetIssuerLimitRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::SetIssuerLimitRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_get(const google::protobuf::Map<std::string, ::authority_api::v1::SetIssuerLimitRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::authority_api::v1::SetIssuerLimitRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetIssuerLimitRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_remove(google::protobuf::Map<std::string, ::authority_api::v1::SetIssuerLimitRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::SetIssuerLimitRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_iter(
      const google::protobuf::Map<std::string, ::authority_api::v1::SetIssuerLimitRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::authority_api::v1::SetIssuerLimitRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::authority_api::v1::SetIssuerLimitRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// authority_api.v1.SetIssuerLimitResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_new() { return new ::authority_api::v1::SetIssuerLimitResponse(); }
void __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_delete(void* ptr) { delete static_cast<::authority_api::v1::SetIssuerLimitResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_serialize(::authority_api::v1::SetIssuerLimitResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_parse(::authority_api::v1::SetIssuerLimitResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_copy_from(::authority_api::v1::SetIssuerLimitResponse* dst, const ::authority_api::v1::SetIssuerLimitResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_repeated_len(google::protobuf::RepeatedPtrField<::authority_api::v1::SetIssuerLimitResponse>* field) {
  return field->size();
}
const ::authority_api::v1::SetIssuerLimitResponse& __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::authority_api::v1::SetIssuerLimitResponse>* field,
  size_t index) {
  return field->Get(index);
}
::authority_api::v1::SetIssuerLimitResponse* __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::authority_api::v1::SetIssuerLimitResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::authority_api::v1::SetIssuerLimitResponse* __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_repeated_add(google::protobuf::RepeatedPtrField<::authority_api::v1::SetIssuerLimitResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_repeated_clear(google::protobuf::RepeatedPtrField<::authority_api::v1::SetIssuerLimitResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::authority_api::v1::SetIssuerLimitResponse>& dst,
  const google::protobuf::RepeatedPtrField<::authority_api::v1::SetIssuerLimitResponse>& src) {
  dst = src;
}

const void* __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_get_limit(::authority_api::v1::SetIssuerLimitResponse* msg) {
  return static_cast<const void*>(&msg->limit());
}
void* __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_get_mut_limit(::authority_api::v1::SetIssuerLimitResponse* msg) {
  return static_cast<void*>(msg->mutable_limit());
}
void __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_clear_limit(::authority_api::v1::SetIssuerLimitResponse* msg) { msg->clear_limit(); }
bool __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_has_limit(::authority_api::v1::SetIssuerLimitResponse* msg) { return msg->has_limit(); }

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_get_signature(::authority_api::v1::SetIssuerLimitResponse* msg) {
  absl::string_view val = msg->signature();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__authority_api_v1_SetIssuerLimitResponse_set_signature(::authority_api::v1::SetIssuerLimitResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_signature(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::authority_api::v1::SetIssuerLimitResponse>* __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_new() {
    return new google::protobuf::Map<int32_t, ::authority_api::v1::SetIssuerLimitResponse>();
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_free(const google::protobuf::Map<int32_t, ::authority_api::v1::SetIssuerLimitResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_clear(google::protobuf::Map<int32_t, ::authority_api::v1::SetIssuerLimitResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_size(const google::protobuf::Map<int32_t, ::authority_api::v1::SetIssuerLimitResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_insert(google::protobuf::Map<int32_t, ::authority_api::v1::SetIssuerLimitResponse> * m,
                          int32_t key, ::authority_api::v1::SetIssuerLimitResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_get(const google::protobuf::Map<int32_t, ::authority_api::v1::SetIssuerLimitResponse>* m,
                       int32_t key, const ::authority_api::v1::SetIssuerLimitResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetIssuerLimitResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_remove(google::protobuf::Map<int32_t, ::authority_api::v1::SetIssuerLimitResponse> * m,
                          int32_t key, ::authority_api::v1::SetIssuerLimitResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_iter(
      const google::protobuf::Map<int32_t, ::authority_api::v1::SetIssuerLimitResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_SetIssuerLimitResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::authority_api::v1::SetIssuerLimitResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::authority_api::v1::SetIssuerLimitResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::authority_api::v1::SetIssuerLimitResponse>* __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_new() {
    return new google::protobuf::Map<uint32_t, ::authority_api::v1::SetIssuerLimitResponse>();
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_free(const google::protobuf::Map<uint32_t, ::authority_api::v1::SetIssuerLimitResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_clear(google::protobuf::Map<uint32_t, ::authority_api::v1::SetIssuerLimitResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_size(const google::protobuf::Map<uint32_t, ::authority_api::v1::SetIssuerLimitResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_insert(google::protobuf::Map<uint32_t, ::authority_api::v1::SetIssuerLimitResponse> * m,
                          uint32_t key, ::authority_api::v1::SetIssuerLimitResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_get(const google::protobuf::Map<uint32_t, ::authority_api::v1::SetIssuerLimitResponse>* m,
                       uint32_t key, const ::authority_api::v1::SetIssuerLimitResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetIssuerLimitResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_remove(google::protobuf::Map<uint32_t, ::authority_api::v1::SetIssuerLimitResponse> * m,
                          uint32_t key, ::authority_api::v1::SetIssuerLimitResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_iter(
      const google::protobuf::Map<uint32_t, ::authority_api::v1::SetIssuerLimitResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_SetIssuerLimitResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::authority_api::v1::SetIssuerLimitResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::authority_api::v1::SetIssuerLimitResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::authority_api::v1::SetIssuerLimitResponse>* __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_new() {
    return new google::protobuf::Map<int64_t, ::authority_api::v1::SetIssuerLimitResponse>();
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_free(const google::protobuf::Map<int64_t, ::authority_api::v1::SetIssuerLimitResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_clear(google::protobuf::Map<int64_t, ::authority_api::v1::SetIssuerLimitResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_size(const google::protobuf::Map<int64_t, ::authority_api::v1::SetIssuerLimitResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_insert(google::protobuf::Map<int64_t, ::authority_api::v1::SetIssuerLimitResponse> * m,
                          int64_t key, ::authority_api::v1::SetIssuerLimitResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_get(const google::protobuf::Map<int64_t, ::authority_api::v1::SetIssuerLimitResponse>* m,
                       int64_t key, const ::authority_api::v1::SetIssuerLimitResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetIssuerLimitResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_remove(google::protobuf::Map<int64_t, ::authority_api::v1::SetIssuerLimitResponse> * m,
                          int64_t key, ::authority_api::v1::SetIssuerLimitResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_iter(
      const google::protobuf::Map<int64_t, ::authority_api::v1::SetIssuerLimitResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_SetIssuerLimitResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::authority_api::v1::SetIssuerLimitResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::authority_api::v1::SetIssuerLimitResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::authority_api::v1::SetIssuerLimitResponse>* __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_new() {
    return new google::protobuf::Map<uint64_t, ::authority_api::v1::SetIssuerLimitResponse>();
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_free(const google::protobuf::Map<uint64_t, ::authority_api::v1::SetIssuerLimitResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_clear(google::protobuf::Map<uint64_t, ::authority_api::v1::SetIssuerLimitResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_size(const google::protobuf::Map<uint64_t, ::authority_api::v1::SetIssuerLimitResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_insert(google::protobuf::Map<uint64_t, ::authority_api::v1::SetIssuerLimitResponse> * m,
                          uint64_t key, ::authority_api::v1::SetIssuerLimitResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_get(const google::protobuf::Map<uint64_t, ::authority_api::v1::SetIssuerLimitResponse>* m,
                       uint64_t key, const ::authority_api::v1::SetIssuerLimitResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetIssuerLimitResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_remove(google::protobuf::Map<uint64_t, ::authority_api::v1::SetIssuerLimitResponse> * m,
                          uint64_t key, ::authority_api::v1::SetIssuerLimitResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_iter(
      const google::protobuf::Map<uint64_t, ::authority_api::v1::SetIssuerLimitResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_SetIssuerLimitResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::authority_api::v1::SetIssuerLimitResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::authority_api::v1::SetIssuerLimitResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::authority_api::v1::SetIssuerLimitResponse>* __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_new() {
    return new google::protobuf::Map<bool, ::authority_api::v1::SetIssuerLimitResponse>();
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_free(const google::protobuf::Map<bool, ::authority_api::v1::SetIssuerLimitResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_clear(google::protobuf::Map<bool, ::authority_api::v1::SetIssuerLimitResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_size(const google::protobuf::Map<bool, ::authority_api::v1::SetIssuerLimitResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_insert(google::protobuf::Map<bool, ::authority_api::v1::SetIssuerLimitResponse> * m,
                          bool key, ::authority_api::v1::SetIssuerLimitResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_get(const google::protobuf::Map<bool, ::authority_api::v1::SetIssuerLimitResponse>* m,
                       bool key, const ::authority_api::v1::SetIssuerLimitResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetIssuerLimitResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_remove(google::protobuf::Map<bool, ::authority_api::v1::SetIssuerLimitResponse> * m,
                          bool key, ::authority_api::v1::SetIssuerLimitResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_iter(
      const google::protobuf::Map<bool, ::authority_api::v1::SetIssuerLimitResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_SetIssuerLimitResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::authority_api::v1::SetIssuerLimitResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::authority_api::v1::SetIssuerLimitResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::authority_api::v1::SetIssuerLimitResponse>* __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_new() {
    return new google::protobuf::Map<std::string, ::authority_api::v1::SetIssuerLimitResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_free(const google::protobuf::Map<std::string, ::authority_api::v1::SetIssuerLimitResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_clear(google::protobuf::Map<std::string, ::authority_api::v1::SetIssuerLimitResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_size(const google::protobuf::Map<std::string, ::authority_api::v1::SetIssuerLimitResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_insert(google::protobuf::Map<std::string, ::authority_api::v1::SetIssuerLimitResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::SetIssuerLimitResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_get(const google::protobuf::Map<std::string, ::authority_api::v1::SetIssuerLimitResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::authority_api::v1::SetIssuerLimitResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetIssuerLimitResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_remove(google::protobuf::Map<std::string, ::authority_api::v1::SetIssuerLimitResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::SetIssuerLimitResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_iter(
      const google::protobuf::Map<std::string, ::authority_api::v1::SetIssuerLimitResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetIssuerLimitResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::authority_api::v1::SetIssuerLimitResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::authority_api::v1::SetIssuerLimitResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// authority_api.v1.GetIssuerLimitsRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_new() { return new ::authority_api::v1::GetIssuerLimitsRequest(); }
void __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_delete(void* ptr) { delete static_cast<::authority_api::v1::GetIssuerLimitsRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_serialize(::authority_api::v1::GetIssuerLimitsRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_parse(::authority_api::v1::GetIssuerLimitsRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_copy_from(::authority_api::v1::GetIssuerLimitsRequest* dst, const ::authority_api::v1::GetIssuerLimitsRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_repeated_len(google::protobuf::RepeatedPtrField<::authority_api::v1::GetIssuerLimitsRequest>* field) {
  return field->size();
}
const ::authority_api::v1::GetIssuerLimitsRequest& __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::authority_api::v1::GetIssuerLimitsRequest>* field,
  size_t index) {
  return field->Get(index);
}
::authority_api::v1::GetIssuerLimitsRequest* __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::authority_api::v1::GetIssuerLimitsRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::authority_api::v1::GetIssuerLimitsRequest* __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_repeated_add(google::protobuf::RepeatedPtrField<::authority_api::v1::GetIssuerLimitsRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_repeated_clear(google::protobuf::RepeatedPtrField<::authority_api::v1::GetIssuerLimitsRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::authority_api::v1::GetIssuerLimitsRequest>& dst,
  const google::protobuf::RepeatedPtrField<::authority_api::v1::GetIssuerLimitsRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_get_issuer_public_key(::authority_api::v1::GetIssuerLimitsRequest* msg) {
  absl::string_view val = msg->issuer_public_key();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__authority_api_v1_GetIssuerLimitsRequest_set_issuer_public_key(::authority_api::v1::GetIssuerLimitsRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_issuer_public_key(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::authority_api::v1::GetIssuerLimitsRequest>* __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_new() {
    return new google::protobuf::Map<int32_t, ::authority_api::v1::GetIssuerLimitsRequest>();
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_free(const google::protobuf::Map<int32_t, ::authority_api::v1::GetIssuerLimitsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_clear(google::protobuf::Map<int32_t, ::authority_api::v1::GetIssuerLimitsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_size(const google::protobuf::Map<int32_t, ::authority_api::v1::GetIssuerLimitsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_insert(google::protobuf::Map<int32_t, ::authority_api::v1::GetIssuerLimitsRequest> * m,
                          int32_t key, ::authority_api::v1::GetIssuerLimitsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_get(const google::protobuf::Map<int32_t, ::authority_api::v1::GetIssuerLimitsRequest>* m,
                       int32_t key, const ::authority_api::v1::GetIssuerLimitsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetIssuerLimitsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_remove(google::protobuf::Map<int32_t, ::authority_api::v1::GetIssuerLimitsRequest> * m,
                          int32_t key, ::authority_api::v1::GetIssuerLimitsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_iter(
      const google::protobuf::Map<int32_t, ::authority_api::v1::GetIssuerLimitsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::authority_api::v1::GetIssuerLimitsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::authority_api::v1::GetIssuerLimitsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::authority_api::v1::GetIssuerLimitsRequest>* __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_new() {
    return new google::protobuf::Map<uint32_t, ::authority_api::v1::GetIssuerLimitsRequest>();
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_free(const google::protobuf::Map<uint32_t, ::authority_api::v1::GetIssuerLimitsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_clear(google::protobuf::Map<uint32_t, ::authority_api::v1::GetIssuerLimitsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_size(const google::protobuf::Map<uint32_t, ::authority_api::v1::GetIssuerLimitsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_insert(google::protobuf::Map<uint32_t, ::authority_api::v1::GetIssuerLimitsRequest> * m,
                          uint32_t key, ::authority_api::v1::GetIssuerLimitsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_get(const google::protobuf::Map<uint32_t, ::authority_api::v1::GetIssuerLimitsRequest>* m,
                       uint32_t key, const ::authority_api::v1::GetIssuerLimitsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetIssuerLimitsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_remove(google::protobuf::Map<uint32_t, ::authority_api::v1::GetIssuerLimitsRequest> * m,
                          uint32_t key, ::authority_api::v1::GetIssuerLimitsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_iter(
      const google::protobuf::Map<uint32_t, ::authority_api::v1::GetIssuerLimitsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::authority_api::v1::GetIssuerLimitsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::authority_api::v1::GetIssuerLimitsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::authority_api::v1::GetIssuerLimitsRequest>* __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_new() {
    return new google::protobuf::Map<int64_t, ::authority_api::v1::GetIssuerLimitsRequest>();
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_free(const google::protobuf::Map<int64_t, ::authority_api::v1::GetIssuerLimitsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_clear(google::protobuf::Map<int64_t, ::authority_api::v1::GetIssuerLimitsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_size(const google::protobuf::Map<int64_t, ::authority_api::v1::GetIssuerLimitsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_insert(google::protobuf::Map<int64_t, ::authority_api::v1::GetIssuerLimitsRequest> * m,
                          int64_t key, ::authority_api::v1::GetIssuerLimitsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_get(const google::protobuf::Map<int64_t, ::authority_api::v1::GetIssuerLimitsRequest>* m,
                       int64_t key, const ::authority_api::v1::GetIssuerLimitsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetIssuerLimitsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_remove(google::protobuf::Map<int64_t, ::authority_api::v1::GetIssuerLimitsRequest> * m,
                          int64_t key, ::authority_api::v1::GetIssuerLimitsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_iter(
      const google::protobuf::Map<int64_t, ::authority_api::v1::GetIssuerLimitsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::authority_api::v1::GetIssuerLimitsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::authority_api::v1::GetIssuerLimitsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::authority_api::v1::GetIssuerLimitsRequest>* __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_new() {
    return new google::protobuf::Map<uint64_t, ::authority_api::v1::GetIssuerLimitsRequest>();
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_free(const google::protobuf::Map<uint64_t, ::authority_api::v1::GetIssuerLimitsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_clear(google::protobuf::Map<uint64_t, ::authority_api::v1::GetIssuerLimitsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_size(const google::protobuf::Map<uint64_t, ::authority_api::v1::GetIssuerLimitsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_insert(google::protobuf::Map<uint64_t, ::authority_api::v1::GetIssuerLimitsRequest> * m,
                          uint64_t key, ::authority_api::v1::GetIssuerLimitsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_get(const google::protobuf::Map<uint64_t, ::authority_api::v1::GetIssuerLimitsRequest>* m,
                       uint64_t key, const ::authority_api::v1::GetIssuerLimitsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetIssuerLimitsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_remove(google::protobuf::Map<uint64_t, ::authority_api::v1::GetIssuerLimitsRequest> * m,
                          uint64_t key, ::authority_api::v1::GetIssuerLimitsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_iter(
      const google::protobuf::Map<uint64_t, ::authority_api::v1::GetIssuerLimitsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::authority_api::v1::GetIssuerLimitsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::authority_api::v1::GetIssuerLimitsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::authority_api::v1::GetIssuerLimitsRequest>* __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_new() {
    return new google::protobuf::Map<bool, ::authority_api::v1::GetIssuerLimitsRequest>();
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_free(const google::protobuf::Map<bool, ::authority_api::v1::GetIssuerLimitsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_clear(google::protobuf::Map<bool, ::authority_api::v1::GetIssuerLimitsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_size(const google::protobuf::Map<bool, ::authority_api::v1::GetIssuerLimitsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_insert(google::protobuf::Map<bool, ::authority_api::v1::GetIssuerLimitsRequest> * m,
                          bool key, ::authority_api::v1::GetIssuerLimitsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_get(const google::protobuf::Map<bool, ::authority_api::v1::GetIssuerLimitsRequest>* m,
                       bool key, const ::authority_api::v1::GetIssuerLimitsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetIssuerLimitsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_remove(google::protobuf::Map<bool, ::authority_api::v1::GetIssuerLimitsRequest> * m,
                          bool key, ::authority_api::v1::GetIssuerLimitsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_iter(
      const google::protobuf::Map<bool, ::authority_api::v1::GetIssuerLimitsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::authority_api::v1::GetIssuerLimitsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::authority_api::v1::GetIssuerLimitsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::authority_api::v1::GetIssuerLimitsRequest>* __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_new() {
    return new google::protobuf::Map<std::string, ::authority_api::v1::GetIssuerLimitsRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_free(const google::protobuf::Map<std::string, ::authority_api::v1::GetIssuerLimitsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_clear(google::protobuf::Map<std::string, ::authority_api::v1::GetIssuerLimitsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_size(const google::protobuf::Map<std::string, ::authority_api::v1::GetIssuerLimitsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_insert(google::protobuf::Map<std::string, ::authority_api::v1::GetIssuerLimitsRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::GetIssuerLimitsRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_get(const google::protobuf::Map<std::string, ::authority_api::v1::GetIssuerLimitsRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::authority_api::v1::GetIssuerLimitsRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetIssuerLimitsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_remove(google::protobuf::Map<std::string, ::authority_api::v1::GetIssuerLimitsRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::GetIssuerLimitsRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_iter(
      const google::protobuf::Map<std::string, ::authority_api::v1::GetIssuerLimitsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::authority_api::v1::GetIssuerLimitsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::authority_api::v1::GetIssuerLimitsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// authority_api.v1.GetIssuerLimitsResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_new() { return new ::authority_api::v1::GetIssuerLimitsResponse(); }
void __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_delete(void* ptr) { delete static_cast<::authority_api::v1::GetIssuerLimitsResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_serialize(::authority_api::v1::GetIssuerLimitsResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_parse(::authority_api::v1::GetIssuerLimitsResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_copy_from(::authority_api::v1::GetIssuerLimitsResponse* dst, const ::authority_api::v1::GetIssuerLimitsResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_repeated_len(google::protobuf::RepeatedPtrField<::authority_api::v1::GetIssuerLimitsResponse>* field) {
  return field->size();
}
const ::authority_api::v1::GetIssuerLimitsResponse& __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::authority_api::v1::GetIssuerLimitsResponse>* field,
  size_t index) {
  return field->Get(index);
}
::authority_api::v1::GetIssuerLimitsResponse* __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::authority_api::v1::GetIssuerLimitsResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::authority_api::v1::GetIssuerLimitsResponse* __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_repeated_add(google::protobuf::RepeatedPtrField<::authority_api::v1::GetIssuerLimitsResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_repeated_clear(google::protobuf::RepeatedPtrField<::authority_api::v1::GetIssuerLimitsResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::authority_api::v1::GetIssuerLimitsResponse>& dst,
  const google::protobuf::RepeatedPtrField<::authority_api::v1::GetIssuerLimitsResponse>& src) {
  dst = src;
}

void __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_clear_limit(::authority_api::v1::GetIssuerLimitsResponse* msg) {
  msg->clear_limit();
}
google::protobuf::RepeatedPtrField<::common::Amount>* __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_get_mut_limit(::authority_api::v1::GetIssuerLimitsResponse* msg) {
  return msg->mutable_limit();
}
const google::protobuf::RepeatedPtrField<::common::Amount>* __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_get_limit(
    const ::authority_api::v1::GetIssuerLimitsResponse* msg) {
  return &msg->limit();
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_get_issuer_public_key(::authority_api::v1::GetIssuerLimitsResponse* msg) {
  absl::string_view val = msg->issuer_public_key();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__authority_api_v1_GetIssuerLimitsResponse_set_issuer_public_key(::authority_api::v1::GetIssuerLimitsResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_issuer_public_key(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::authority_api::v1::GetIssuerLimitsResponse>* __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_new() {
    return new google::protobuf::Map<int32_t, ::authority_api::v1::GetIssuerLimitsResponse>();
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_free(const google::protobuf::Map<int32_t, ::authority_api::v1::GetIssuerLimitsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_clear(google::protobuf::Map<int32_t, ::authority_api::v1::GetIssuerLimitsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_size(const google::protobuf::Map<int32_t, ::authority_api::v1::GetIssuerLimitsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_insert(google::protobuf::Map<int32_t, ::authority_api::v1::GetIssuerLimitsResponse> * m,
                          int32_t key, ::authority_api::v1::GetIssuerLimitsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_get(const google::protobuf::Map<int32_t, ::authority_api::v1::GetIssuerLimitsResponse>* m,
                       int32_t key, const ::authority_api::v1::GetIssuerLimitsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetIssuerLimitsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_remove(google::protobuf::Map<int32_t, ::authority_api::v1::GetIssuerLimitsResponse> * m,
                          int32_t key, ::authority_api::v1::GetIssuerLimitsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_iter(
      const google::protobuf::Map<int32_t, ::authority_api::v1::GetIssuerLimitsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_GetIssuerLimitsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::authority_api::v1::GetIssuerLimitsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::authority_api::v1::GetIssuerLimitsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::authority_api::v1::GetIssuerLimitsResponse>* __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_new() {
    return new google::protobuf::Map<uint32_t, ::authority_api::v1::GetIssuerLimitsResponse>();
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_free(const google::protobuf::Map<uint32_t, ::authority_api::v1::GetIssuerLimitsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_clear(google::protobuf::Map<uint32_t, ::authority_api::v1::GetIssuerLimitsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_size(const google::protobuf::Map<uint32_t, ::authority_api::v1::GetIssuerLimitsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_insert(google::protobuf::Map<uint32_t, ::authority_api::v1::GetIssuerLimitsResponse> * m,
                          uint32_t key, ::authority_api::v1::GetIssuerLimitsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_get(const google::protobuf::Map<uint32_t, ::authority_api::v1::GetIssuerLimitsResponse>* m,
                       uint32_t key, const ::authority_api::v1::GetIssuerLimitsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetIssuerLimitsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_remove(google::protobuf::Map<uint32_t, ::authority_api::v1::GetIssuerLimitsResponse> * m,
                          uint32_t key, ::authority_api::v1::GetIssuerLimitsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_iter(
      const google::protobuf::Map<uint32_t, ::authority_api::v1::GetIssuerLimitsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_GetIssuerLimitsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::authority_api::v1::GetIssuerLimitsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::authority_api::v1::GetIssuerLimitsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::authority_api::v1::GetIssuerLimitsResponse>* __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_new() {
    return new google::protobuf::Map<int64_t, ::authority_api::v1::GetIssuerLimitsResponse>();
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_free(const google::protobuf::Map<int64_t, ::authority_api::v1::GetIssuerLimitsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_clear(google::protobuf::Map<int64_t, ::authority_api::v1::GetIssuerLimitsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_size(const google::protobuf::Map<int64_t, ::authority_api::v1::GetIssuerLimitsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_insert(google::protobuf::Map<int64_t, ::authority_api::v1::GetIssuerLimitsResponse> * m,
                          int64_t key, ::authority_api::v1::GetIssuerLimitsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_get(const google::protobuf::Map<int64_t, ::authority_api::v1::GetIssuerLimitsResponse>* m,
                       int64_t key, const ::authority_api::v1::GetIssuerLimitsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetIssuerLimitsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_remove(google::protobuf::Map<int64_t, ::authority_api::v1::GetIssuerLimitsResponse> * m,
                          int64_t key, ::authority_api::v1::GetIssuerLimitsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_iter(
      const google::protobuf::Map<int64_t, ::authority_api::v1::GetIssuerLimitsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_GetIssuerLimitsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::authority_api::v1::GetIssuerLimitsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::authority_api::v1::GetIssuerLimitsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::authority_api::v1::GetIssuerLimitsResponse>* __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_new() {
    return new google::protobuf::Map<uint64_t, ::authority_api::v1::GetIssuerLimitsResponse>();
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_free(const google::protobuf::Map<uint64_t, ::authority_api::v1::GetIssuerLimitsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_clear(google::protobuf::Map<uint64_t, ::authority_api::v1::GetIssuerLimitsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_size(const google::protobuf::Map<uint64_t, ::authority_api::v1::GetIssuerLimitsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_insert(google::protobuf::Map<uint64_t, ::authority_api::v1::GetIssuerLimitsResponse> * m,
                          uint64_t key, ::authority_api::v1::GetIssuerLimitsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_get(const google::protobuf::Map<uint64_t, ::authority_api::v1::GetIssuerLimitsResponse>* m,
                       uint64_t key, const ::authority_api::v1::GetIssuerLimitsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetIssuerLimitsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_remove(google::protobuf::Map<uint64_t, ::authority_api::v1::GetIssuerLimitsResponse> * m,
                          uint64_t key, ::authority_api::v1::GetIssuerLimitsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_iter(
      const google::protobuf::Map<uint64_t, ::authority_api::v1::GetIssuerLimitsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_GetIssuerLimitsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::authority_api::v1::GetIssuerLimitsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::authority_api::v1::GetIssuerLimitsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::authority_api::v1::GetIssuerLimitsResponse>* __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_new() {
    return new google::protobuf::Map<bool, ::authority_api::v1::GetIssuerLimitsResponse>();
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_free(const google::protobuf::Map<bool, ::authority_api::v1::GetIssuerLimitsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_clear(google::protobuf::Map<bool, ::authority_api::v1::GetIssuerLimitsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_size(const google::protobuf::Map<bool, ::authority_api::v1::GetIssuerLimitsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_insert(google::protobuf::Map<bool, ::authority_api::v1::GetIssuerLimitsResponse> * m,
                          bool key, ::authority_api::v1::GetIssuerLimitsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_get(const google::protobuf::Map<bool, ::authority_api::v1::GetIssuerLimitsResponse>* m,
                       bool key, const ::authority_api::v1::GetIssuerLimitsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetIssuerLimitsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_remove(google::protobuf::Map<bool, ::authority_api::v1::GetIssuerLimitsResponse> * m,
                          bool key, ::authority_api::v1::GetIssuerLimitsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_iter(
      const google::protobuf::Map<bool, ::authority_api::v1::GetIssuerLimitsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_GetIssuerLimitsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::authority_api::v1::GetIssuerLimitsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::authority_api::v1::GetIssuerLimitsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::authority_api::v1::GetIssuerLimitsResponse>* __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_new() {
    return new google::protobuf::Map<std::string, ::authority_api::v1::GetIssuerLimitsResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_free(const google::protobuf::Map<std::string, ::authority_api::v1::GetIssuerLimitsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_clear(google::protobuf::Map<std::string, ::authority_api::v1::GetIssuerLimitsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_size(const google::protobuf::Map<std::string, ::authority_api::v1::GetIssuerLimitsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_insert(google::protobuf::Map<std::string, ::authority_api::v1::GetIssuerLimitsResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::GetIssuerLimitsResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_get(const google::protobuf::Map<std::string, ::authority_api::v1::GetIssuerLimitsResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::authority_api::v1::GetIssuerLimitsResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetIssuerLimitsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_remove(google::protobuf::Map<std::string, ::authority_api::v1::GetIssuerLimitsResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::GetIssuerLimitsResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_iter(
      const google::protobuf::Map<std::string, ::authority_api::v1::GetIssuerLimitsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetIssuerLimitsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::authority_api::v1::GetIssuerLimitsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::authority_api::v1::GetIssuerLimitsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// authority_api.v1.AuthorizeRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__authority_api_v1_AuthorizeRequest_new() { return new ::authority_api::v1::AuthorizeRequest(); }
void __rust_proto_thunk__authority_api_v1_AuthorizeRequest_delete(void* ptr) { delete static_cast<::authority_api::v1::AuthorizeRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__authority_api_v1_AuthorizeRequest_serialize(::authority_api::v1::AuthorizeRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__authority_api_v1_AuthorizeRequest_parse(::authority_api::v1::AuthorizeRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__authority_api_v1_AuthorizeRequest_copy_from(::authority_api::v1::AuthorizeRequest* dst, const ::authority_api::v1::AuthorizeRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__authority_api_v1_AuthorizeRequest_repeated_len(google::protobuf::RepeatedPtrField<::authority_api::v1::AuthorizeRequest>* field) {
  return field->size();
}
const ::authority_api::v1::AuthorizeRequest& __rust_proto_thunk__authority_api_v1_AuthorizeRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::authority_api::v1::AuthorizeRequest>* field,
  size_t index) {
  return field->Get(index);
}
::authority_api::v1::AuthorizeRequest* __rust_proto_thunk__authority_api_v1_AuthorizeRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::authority_api::v1::AuthorizeRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::authority_api::v1::AuthorizeRequest* __rust_proto_thunk__authority_api_v1_AuthorizeRequest_repeated_add(google::protobuf::RepeatedPtrField<::authority_api::v1::AuthorizeRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__authority_api_v1_AuthorizeRequest_repeated_clear(google::protobuf::RepeatedPtrField<::authority_api::v1::AuthorizeRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__authority_api_v1_AuthorizeRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::authority_api::v1::AuthorizeRequest>& dst,
  const google::protobuf::RepeatedPtrField<::authority_api::v1::AuthorizeRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__authority_api_v1_AuthorizeRequest_get_promissory_file(::authority_api::v1::AuthorizeRequest* msg) {
  absl::string_view val = msg->promissory_file();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__authority_api_v1_AuthorizeRequest_set_promissory_file(::authority_api::v1::AuthorizeRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_promissory_file(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::authority_api::v1::AuthorizeRequest>* __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_new() {
    return new google::protobuf::Map<int32_t, ::authority_api::v1::AuthorizeRequest>();
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_free(const google::protobuf::Map<int32_t, ::authority_api::v1::AuthorizeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_clear(google::protobuf::Map<int32_t, ::authority_api::v1::AuthorizeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_size(const google::protobuf::Map<int32_t, ::authority_api::v1::AuthorizeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_insert(google::protobuf::Map<int32_t, ::authority_api::v1::AuthorizeRequest> * m,
                          int32_t key, ::authority_api::v1::AuthorizeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_get(const google::protobuf::Map<int32_t, ::authority_api::v1::AuthorizeRequest>* m,
                       int32_t key, const ::authority_api::v1::AuthorizeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AuthorizeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_remove(google::protobuf::Map<int32_t, ::authority_api::v1::AuthorizeRequest> * m,
                          int32_t key, ::authority_api::v1::AuthorizeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_iter(
      const google::protobuf::Map<int32_t, ::authority_api::v1::AuthorizeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::authority_api::v1::AuthorizeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::authority_api::v1::AuthorizeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::authority_api::v1::AuthorizeRequest>* __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_new() {
    return new google::protobuf::Map<uint32_t, ::authority_api::v1::AuthorizeRequest>();
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_free(const google::protobuf::Map<uint32_t, ::authority_api::v1::AuthorizeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_clear(google::protobuf::Map<uint32_t, ::authority_api::v1::AuthorizeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_size(const google::protobuf::Map<uint32_t, ::authority_api::v1::AuthorizeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_insert(google::protobuf::Map<uint32_t, ::authority_api::v1::AuthorizeRequest> * m,
                          uint32_t key, ::authority_api::v1::AuthorizeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_get(const google::protobuf::Map<uint32_t, ::authority_api::v1::AuthorizeRequest>* m,
                       uint32_t key, const ::authority_api::v1::AuthorizeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AuthorizeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_remove(google::protobuf::Map<uint32_t, ::authority_api::v1::AuthorizeRequest> * m,
                          uint32_t key, ::authority_api::v1::AuthorizeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_iter(
      const google::protobuf::Map<uint32_t, ::authority_api::v1::AuthorizeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::authority_api::v1::AuthorizeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::authority_api::v1::AuthorizeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::authority_api::v1::AuthorizeRequest>* __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_new() {
    return new google::protobuf::Map<int64_t, ::authority_api::v1::AuthorizeRequest>();
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_free(const google::protobuf::Map<int64_t, ::authority_api::v1::AuthorizeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_clear(google::protobuf::Map<int64_t, ::authority_api::v1::AuthorizeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_size(const google::protobuf::Map<int64_t, ::authority_api::v1::AuthorizeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_insert(google::protobuf::Map<int64_t, ::authority_api::v1::AuthorizeRequest> * m,
                          int64_t key, ::authority_api::v1::AuthorizeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_get(const google::protobuf::Map<int64_t, ::authority_api::v1::AuthorizeRequest>* m,
                       int64_t key, const ::authority_api::v1::AuthorizeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AuthorizeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_remove(google::protobuf::Map<int64_t, ::authority_api::v1::AuthorizeRequest> * m,
                          int64_t key, ::authority_api::v1::AuthorizeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_iter(
      const google::protobuf::Map<int64_t, ::authority_api::v1::AuthorizeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::authority_api::v1::AuthorizeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::authority_api::v1::AuthorizeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::authority_api::v1::AuthorizeRequest>* __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_new() {
    return new google::protobuf::Map<uint64_t, ::authority_api::v1::AuthorizeRequest>();
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_free(const google::protobuf::Map<uint64_t, ::authority_api::v1::AuthorizeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_clear(google::protobuf::Map<uint64_t, ::authority_api::v1::AuthorizeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_size(const google::protobuf::Map<uint64_t, ::authority_api::v1::AuthorizeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_insert(google::protobuf::Map<uint64_t, ::authority_api::v1::AuthorizeRequest> * m,
                          uint64_t key, ::authority_api::v1::AuthorizeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_get(const google::protobuf::Map<uint64_t, ::authority_api::v1::AuthorizeRequest>* m,
                       uint64_t key, const ::authority_api::v1::AuthorizeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AuthorizeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_remove(google::protobuf::Map<uint64_t, ::authority_api::v1::AuthorizeRequest> * m,
                          uint64_t key, ::authority_api::v1::AuthorizeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_iter(
      const google::protobuf::Map<uint64_t, ::authority_api::v1::AuthorizeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::authority_api::v1::AuthorizeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::authority_api::v1::AuthorizeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::authority_api::v1::AuthorizeRequest>* __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_new() {
    return new google::protobuf::Map<bool, ::authority_api::v1::AuthorizeRequest>();
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_free(const google::protobuf::Map<bool, ::authority_api::v1::AuthorizeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_clear(google::protobuf::Map<bool, ::authority_api::v1::AuthorizeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_size(const google::protobuf::Map<bool, ::authority_api::v1::AuthorizeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_insert(google::protobuf::Map<bool, ::authority_api::v1::AuthorizeRequest> * m,
                          bool key, ::authority_api::v1::AuthorizeRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_get(const google::protobuf::Map<bool, ::authority_api::v1::AuthorizeRequest>* m,
                       bool key, const ::authority_api::v1::AuthorizeRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AuthorizeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_remove(google::protobuf::Map<bool, ::authority_api::v1::AuthorizeRequest> * m,
                          bool key, ::authority_api::v1::AuthorizeRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_iter(
      const google::protobuf::Map<bool, ::authority_api::v1::AuthorizeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::authority_api::v1::AuthorizeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::authority_api::v1::AuthorizeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::authority_api::v1::AuthorizeRequest>* __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_new() {
    return new google::protobuf::Map<std::string, ::authority_api::v1::AuthorizeRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_free(const google::protobuf::Map<std::string, ::authority_api::v1::AuthorizeRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_clear(google::protobuf::Map<std::string, ::authority_api::v1::AuthorizeRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_size(const google::protobuf::Map<std::string, ::authority_api::v1::AuthorizeRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_insert(google::protobuf::Map<std::string, ::authority_api::v1::AuthorizeRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::AuthorizeRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_get(const google::protobuf::Map<std::string, ::authority_api::v1::AuthorizeRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::authority_api::v1::AuthorizeRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AuthorizeRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_remove(google::protobuf::Map<std::string, ::authority_api::v1::AuthorizeRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::AuthorizeRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_iter(
      const google::protobuf::Map<std::string, ::authority_api::v1::AuthorizeRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::authority_api::v1::AuthorizeRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::authority_api::v1::AuthorizeRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// authority_api.v1.AuthorizeResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__authority_api_v1_AuthorizeResponse_new() { return new ::authority_api::v1::AuthorizeResponse(); }
void __rust_proto_thunk__authority_api_v1_AuthorizeResponse_delete(void* ptr) { delete static_cast<::authority_api::v1::AuthorizeResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__authority_api_v1_AuthorizeResponse_serialize(::authority_api::v1::AuthorizeResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__authority_api_v1_AuthorizeResponse_parse(::authority_api::v1::AuthorizeResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__authority_api_v1_AuthorizeResponse_copy_from(::authority_api::v1::AuthorizeResponse* dst, const ::authority_api::v1::AuthorizeResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__authority_api_v1_AuthorizeResponse_repeated_len(google::protobuf::RepeatedPtrField<::authority_api::v1::AuthorizeResponse>* field) {
  return field->size();
}
const ::authority_api::v1::AuthorizeResponse& __rust_proto_thunk__authority_api_v1_AuthorizeResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::authority_api::v1::AuthorizeResponse>* field,
  size_t index) {
  return field->Get(index);
}
::authority_api::v1::AuthorizeResponse* __rust_proto_thunk__authority_api_v1_AuthorizeResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::authority_api::v1::AuthorizeResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::authority_api::v1::AuthorizeResponse* __rust_proto_thunk__authority_api_v1_AuthorizeResponse_repeated_add(google::protobuf::RepeatedPtrField<::authority_api::v1::AuthorizeResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__authority_api_v1_AuthorizeResponse_repeated_clear(google::protobuf::RepeatedPtrField<::authority_api::v1::AuthorizeResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__authority_api_v1_AuthorizeResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::authority_api::v1::AuthorizeResponse>& dst,
  const google::protobuf::RepeatedPtrField<::authority_api::v1::AuthorizeResponse>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__authority_api_v1_AuthorizeResponse_get_promissory_file(::authority_api::v1::AuthorizeResponse* msg) {
  absl::string_view val = msg->promissory_file();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__authority_api_v1_AuthorizeResponse_set_promissory_file(::authority_api::v1::AuthorizeResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_promissory_file(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::authority_api::v1::AuthorizeResponse>* __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_new() {
    return new google::protobuf::Map<int32_t, ::authority_api::v1::AuthorizeResponse>();
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_free(const google::protobuf::Map<int32_t, ::authority_api::v1::AuthorizeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_clear(google::protobuf::Map<int32_t, ::authority_api::v1::AuthorizeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_size(const google::protobuf::Map<int32_t, ::authority_api::v1::AuthorizeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_insert(google::protobuf::Map<int32_t, ::authority_api::v1::AuthorizeResponse> * m,
                          int32_t key, ::authority_api::v1::AuthorizeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_get(const google::protobuf::Map<int32_t, ::authority_api::v1::AuthorizeResponse>* m,
                       int32_t key, const ::authority_api::v1::AuthorizeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AuthorizeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_remove(google::protobuf::Map<int32_t, ::authority_api::v1::AuthorizeResponse> * m,
                          int32_t key, ::authority_api::v1::AuthorizeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_iter(
      const google::protobuf::Map<int32_t, ::authority_api::v1::AuthorizeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_AuthorizeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::authority_api::v1::AuthorizeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::authority_api::v1::AuthorizeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::authority_api::v1::AuthorizeResponse>* __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_new() {
    return new google::protobuf::Map<uint32_t, ::authority_api::v1::AuthorizeResponse>();
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_free(const google::protobuf::Map<uint32_t, ::authority_api::v1::AuthorizeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_clear(google::protobuf::Map<uint32_t, ::authority_api::v1::AuthorizeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_size(const google::protobuf::Map<uint32_t, ::authority_api::v1::AuthorizeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_insert(google::protobuf::Map<uint32_t, ::authority_api::v1::AuthorizeResponse> * m,
                          uint32_t key, ::authority_api::v1::AuthorizeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_get(const google::protobuf::Map<uint32_t, ::authority_api::v1::AuthorizeResponse>* m,
                       uint32_t key, const ::authority_api::v1::AuthorizeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AuthorizeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_remove(google::protobuf::Map<uint32_t, ::authority_api::v1::AuthorizeResponse> * m,
                          uint32_t key, ::authority_api::v1::AuthorizeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_iter(
      const google::protobuf::Map<uint32_t, ::authority_api::v1::AuthorizeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_AuthorizeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::authority_api::v1::AuthorizeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::authority_api::v1::AuthorizeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::authority_api::v1::AuthorizeResponse>* __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_new() {
    return new google::protobuf::Map<int64_t, ::authority_api::v1::AuthorizeResponse>();
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_free(const google::protobuf::Map<int64_t, ::authority_api::v1::AuthorizeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_clear(google::protobuf::Map<int64_t, ::authority_api::v1::AuthorizeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_size(const google::protobuf::Map<int64_t, ::authority_api::v1::AuthorizeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_insert(google::protobuf::Map<int64_t, ::authority_api::v1::AuthorizeResponse> * m,
                          int64_t key, ::authority_api::v1::AuthorizeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_get(const google::protobuf::Map<int64_t, ::authority_api::v1::AuthorizeResponse>* m,
                       int64_t key, const ::authority_api::v1::AuthorizeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AuthorizeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_remove(google::protobuf::Map<int64_t, ::authority_api::v1::AuthorizeResponse> * m,
                          int64_t key, ::authority_api::v1::AuthorizeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_iter(
      const google::protobuf::Map<int64_t, ::authority_api::v1::AuthorizeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_AuthorizeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::authority_api::v1::AuthorizeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::authority_api::v1::AuthorizeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::authority_api::v1::AuthorizeResponse>* __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_new() {
    return new google::protobuf::Map<uint64_t, ::authority_api::v1::AuthorizeResponse>();
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_free(const google::protobuf::Map<uint64_t, ::authority_api::v1::AuthorizeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_clear(google::protobuf::Map<uint64_t, ::authority_api::v1::AuthorizeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_size(const google::protobuf::Map<uint64_t, ::authority_api::v1::AuthorizeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_insert(google::protobuf::Map<uint64_t, ::authority_api::v1::AuthorizeResponse> * m,
                          uint64_t key, ::authority_api::v1::AuthorizeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_get(const google::protobuf::Map<uint64_t, ::authority_api::v1::AuthorizeResponse>* m,
                       uint64_t key, const ::authority_api::v1::AuthorizeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AuthorizeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_remove(google::protobuf::Map<uint64_t, ::authority_api::v1::AuthorizeResponse> * m,
                          uint64_t key, ::authority_api::v1::AuthorizeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_iter(
      const google::protobuf::Map<uint64_t, ::authority_api::v1::AuthorizeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_AuthorizeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::authority_api::v1::AuthorizeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::authority_api::v1::AuthorizeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::authority_api::v1::AuthorizeResponse>* __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_new() {
    return new google::protobuf::Map<bool, ::authority_api::v1::AuthorizeResponse>();
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_free(const google::protobuf::Map<bool, ::authority_api::v1::AuthorizeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_clear(google::protobuf::Map<bool, ::authority_api::v1::AuthorizeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_size(const google::protobuf::Map<bool, ::authority_api::v1::AuthorizeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_insert(google::protobuf::Map<bool, ::authority_api::v1::AuthorizeResponse> * m,
                          bool key, ::authority_api::v1::AuthorizeResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_get(const google::protobuf::Map<bool, ::authority_api::v1::AuthorizeResponse>* m,
                       bool key, const ::authority_api::v1::AuthorizeResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AuthorizeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_remove(google::protobuf::Map<bool, ::authority_api::v1::AuthorizeResponse> * m,
                          bool key, ::authority_api::v1::AuthorizeResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_iter(
      const google::protobuf::Map<bool, ::authority_api::v1::AuthorizeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_AuthorizeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::authority_api::v1::AuthorizeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::authority_api::v1::AuthorizeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::authority_api::v1::AuthorizeResponse>* __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_new() {
    return new google::protobuf::Map<std::string, ::authority_api::v1::AuthorizeResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_free(const google::protobuf::Map<std::string, ::authority_api::v1::AuthorizeResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_clear(google::protobuf::Map<std::string, ::authority_api::v1::AuthorizeResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_size(const google::protobuf::Map<std::string, ::authority_api::v1::AuthorizeResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_insert(google::protobuf::Map<std::string, ::authority_api::v1::AuthorizeResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::AuthorizeResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_get(const google::protobuf::Map<std::string, ::authority_api::v1::AuthorizeResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::authority_api::v1::AuthorizeResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AuthorizeResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_remove(google::protobuf::Map<std::string, ::authority_api::v1::AuthorizeResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::AuthorizeResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_iter(
      const google::protobuf::Map<std::string, ::authority_api::v1::AuthorizeResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AuthorizeResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::authority_api::v1::AuthorizeResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::authority_api::v1::AuthorizeResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// authority_api.v1.GetNetworkInfoRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_new() { return new ::authority_api::v1::GetNetworkInfoRequest(); }
void __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_delete(void* ptr) { delete static_cast<::authority_api::v1::GetNetworkInfoRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_serialize(::authority_api::v1::GetNetworkInfoRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_parse(::authority_api::v1::GetNetworkInfoRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_copy_from(::authority_api::v1::GetNetworkInfoRequest* dst, const ::authority_api::v1::GetNetworkInfoRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_repeated_len(google::protobuf::RepeatedPtrField<::authority_api::v1::GetNetworkInfoRequest>* field) {
  return field->size();
}
const ::authority_api::v1::GetNetworkInfoRequest& __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::authority_api::v1::GetNetworkInfoRequest>* field,
  size_t index) {
  return field->Get(index);
}
::authority_api::v1::GetNetworkInfoRequest* __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::authority_api::v1::GetNetworkInfoRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::authority_api::v1::GetNetworkInfoRequest* __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_repeated_add(google::protobuf::RepeatedPtrField<::authority_api::v1::GetNetworkInfoRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_repeated_clear(google::protobuf::RepeatedPtrField<::authority_api::v1::GetNetworkInfoRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__authority_api_v1_GetNetworkInfoRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::authority_api::v1::GetNetworkInfoRequest>& dst,
  const google::protobuf::RepeatedPtrField<::authority_api::v1::GetNetworkInfoRequest>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::authority_api::v1::GetNetworkInfoRequest>* __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_new() {
    return new google::protobuf::Map<int32_t, ::authority_api::v1::GetNetworkInfoRequest>();
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_free(const google::protobuf::Map<int32_t, ::authority_api::v1::GetNetworkInfoRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_clear(google::protobuf::Map<int32_t, ::authority_api::v1::GetNetworkInfoRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_size(const google::protobuf::Map<int32_t, ::authority_api::v1::GetNetworkInfoRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_insert(google::protobuf::Map<int32_t, ::authority_api::v1::GetNetworkInfoRequest> * m,
                          int32_t key, ::authority_api::v1::GetNetworkInfoRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_get(const google::protobuf::Map<int32_t, ::authority_api::v1::GetNetworkInfoRequest>* m,
                       int32_t key, const ::authority_api::v1::GetNetworkInfoRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNetworkInfoRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_remove(google::protobuf::Map<int32_t, ::authority_api::v1::GetNetworkInfoRequest> * m,
                          int32_t key, ::authority_api::v1::GetNetworkInfoRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_iter(
      const google::protobuf::Map<int32_t, ::authority_api::v1::GetNetworkInfoRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::authority_api::v1::GetNetworkInfoRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::authority_api::v1::GetNetworkInfoRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::authority_api::v1::GetNetworkInfoRequest>* __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_new() {
    return new google::protobuf::Map<uint32_t, ::authority_api::v1::GetNetworkInfoRequest>();
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_free(const google::protobuf::Map<uint32_t, ::authority_api::v1::GetNetworkInfoRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_clear(google::protobuf::Map<uint32_t, ::authority_api::v1::GetNetworkInfoRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_size(const google::protobuf::Map<uint32_t, ::authority_api::v1::GetNetworkInfoRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_insert(google::protobuf::Map<uint32_t, ::authority_api::v1::GetNetworkInfoRequest> * m,
                          uint32_t key, ::authority_api::v1::GetNetworkInfoRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_get(const google::protobuf::Map<uint32_t, ::authority_api::v1::GetNetworkInfoRequest>* m,
                       uint32_t key, const ::authority_api::v1::GetNetworkInfoRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNetworkInfoRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_remove(google::protobuf::Map<uint32_t, ::authority_api::v1::GetNetworkInfoRequest> * m,
                          uint32_t key, ::authority_api::v1::GetNetworkInfoRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_iter(
      const google::protobuf::Map<uint32_t, ::authority_api::v1::GetNetworkInfoRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::authority_api::v1::GetNetworkInfoRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::authority_api::v1::GetNetworkInfoRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::authority_api::v1::GetNetworkInfoRequest>* __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_new() {
    return new google::protobuf::Map<int64_t, ::authority_api::v1::GetNetworkInfoRequest>();
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_free(const google::protobuf::Map<int64_t, ::authority_api::v1::GetNetworkInfoRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_clear(google::protobuf::Map<int64_t, ::authority_api::v1::GetNetworkInfoRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_size(const google::protobuf::Map<int64_t, ::authority_api::v1::GetNetworkInfoRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_insert(google::protobuf::Map<int64_t, ::authority_api::v1::GetNetworkInfoRequest> * m,
                          int64_t key, ::authority_api::v1::GetNetworkInfoRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_get(const google::protobuf::Map<int64_t, ::authority_api::v1::GetNetworkInfoRequest>* m,
                       int64_t key, const ::authority_api::v1::GetNetworkInfoRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNetworkInfoRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_remove(google::protobuf::Map<int64_t, ::authority_api::v1::GetNetworkInfoRequest> * m,
                          int64_t key, ::authority_api::v1::GetNetworkInfoRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_iter(
      const google::protobuf::Map<int64_t, ::authority_api::v1::GetNetworkInfoRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::authority_api::v1::GetNetworkInfoRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::authority_api::v1::GetNetworkInfoRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::authority_api::v1::GetNetworkInfoRequest>* __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_new() {
    return new google::protobuf::Map<uint64_t, ::authority_api::v1::GetNetworkInfoRequest>();
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_free(const google::protobuf::Map<uint64_t, ::authority_api::v1::GetNetworkInfoRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_clear(google::protobuf::Map<uint64_t, ::authority_api::v1::GetNetworkInfoRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_size(const google::protobuf::Map<uint64_t, ::authority_api::v1::GetNetworkInfoRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_insert(google::protobuf::Map<uint64_t, ::authority_api::v1::GetNetworkInfoRequest> * m,
                          uint64_t key, ::authority_api::v1::GetNetworkInfoRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_get(const google::protobuf::Map<uint64_t, ::authority_api::v1::GetNetworkInfoRequest>* m,
                       uint64_t key, const ::authority_api::v1::GetNetworkInfoRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNetworkInfoRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_remove(google::protobuf::Map<uint64_t, ::authority_api::v1::GetNetworkInfoRequest> * m,
                          uint64_t key, ::authority_api::v1::GetNetworkInfoRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_iter(
      const google::protobuf::Map<uint64_t, ::authority_api::v1::GetNetworkInfoRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::authority_api::v1::GetNetworkInfoRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::authority_api::v1::GetNetworkInfoRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::authority_api::v1::GetNetworkInfoRequest>* __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_new() {
    return new google::protobuf::Map<bool, ::authority_api::v1::GetNetworkInfoRequest>();
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_free(const google::protobuf::Map<bool, ::authority_api::v1::GetNetworkInfoRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_clear(google::protobuf::Map<bool, ::authority_api::v1::GetNetworkInfoRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_size(const google::protobuf::Map<bool, ::authority_api::v1::GetNetworkInfoRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_insert(google::protobuf::Map<bool, ::authority_api::v1::GetNetworkInfoRequest> * m,
                          bool key, ::authority_api::v1::GetNetworkInfoRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_get(const google::protobuf::Map<bool, ::authority_api::v1::GetNetworkInfoRequest>* m,
                       bool key, const ::authority_api::v1::GetNetworkInfoRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNetworkInfoRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_remove(google::protobuf::Map<bool, ::authority_api::v1::GetNetworkInfoRequest> * m,
                          bool key, ::authority_api::v1::GetNetworkInfoRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_iter(
      const google::protobuf::Map<bool, ::authority_api::v1::GetNetworkInfoRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::authority_api::v1::GetNetworkInfoRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::authority_api::v1::GetNetworkInfoRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::authority_api::v1::GetNetworkInfoRequest>* __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_new() {
    return new google::protobuf::Map<std::string, ::authority_api::v1::GetNetworkInfoRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_free(const google::protobuf::Map<std::string, ::authority_api::v1::GetNetworkInfoRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_clear(google::protobuf::Map<std::string, ::authority_api::v1::GetNetworkInfoRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_size(const google::protobuf::Map<std::string, ::authority_api::v1::GetNetworkInfoRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_insert(google::protobuf::Map<std::string, ::authority_api::v1::GetNetworkInfoRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::GetNetworkInfoRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_get(const google::protobuf::Map<std::string, ::authority_api::v1::GetNetworkInfoRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::authority_api::v1::GetNetworkInfoRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNetworkInfoRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_remove(google::protobuf::Map<std::string, ::authority_api::v1::GetNetworkInfoRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::GetNetworkInfoRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_iter(
      const google::protobuf::Map<std::string, ::authority_api::v1::GetNetworkInfoRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::authority_api::v1::GetNetworkInfoRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::authority_api::v1::GetNetworkInfoRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// authority_api.v1.GetNetworkInfoResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_new() { return new ::authority_api::v1::GetNetworkInfoResponse(); }
void __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_delete(void* ptr) { delete static_cast<::authority_api::v1::GetNetworkInfoResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_serialize(::authority_api::v1::GetNetworkInfoResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_parse(::authority_api::v1::GetNetworkInfoResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_copy_from(::authority_api::v1::GetNetworkInfoResponse* dst, const ::authority_api::v1::GetNetworkInfoResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_repeated_len(google::protobuf::RepeatedPtrField<::authority_api::v1::GetNetworkInfoResponse>* field) {
  return field->size();
}
const ::authority_api::v1::GetNetworkInfoResponse& __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::authority_api::v1::GetNetworkInfoResponse>* field,
  size_t index) {
  return field->Get(index);
}
::authority_api::v1::GetNetworkInfoResponse* __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::authority_api::v1::GetNetworkInfoResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::authority_api::v1::GetNetworkInfoResponse* __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_repeated_add(google::protobuf::RepeatedPtrField<::authority_api::v1::GetNetworkInfoResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_repeated_clear(google::protobuf::RepeatedPtrField<::authority_api::v1::GetNetworkInfoResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::authority_api::v1::GetNetworkInfoResponse>& dst,
  const google::protobuf::RepeatedPtrField<::authority_api::v1::GetNetworkInfoResponse>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_get_emissary(::authority_api::v1::GetNetworkInfoResponse* msg) {
  absl::string_view val = msg->emissary();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_set_emissary(::authority_api::v1::GetNetworkInfoResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_emissary(absl::string_view(s.ptr, s.len));
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_get_transaction_manager(::authority_api::v1::GetNetworkInfoResponse* msg) {
  absl::string_view val = msg->transaction_manager();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__authority_api_v1_GetNetworkInfoResponse_set_transaction_manager(::authority_api::v1::GetNetworkInfoResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_transaction_manager(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::authority_api::v1::GetNetworkInfoResponse>* __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_new() {
    return new google::protobuf::Map<int32_t, ::authority_api::v1::GetNetworkInfoResponse>();
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_free(const google::protobuf::Map<int32_t, ::authority_api::v1::GetNetworkInfoResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_clear(google::protobuf::Map<int32_t, ::authority_api::v1::GetNetworkInfoResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_size(const google::protobuf::Map<int32_t, ::authority_api::v1::GetNetworkInfoResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_insert(google::protobuf::Map<int32_t, ::authority_api::v1::GetNetworkInfoResponse> * m,
                          int32_t key, ::authority_api::v1::GetNetworkInfoResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_get(const google::protobuf::Map<int32_t, ::authority_api::v1::GetNetworkInfoResponse>* m,
                       int32_t key, const ::authority_api::v1::GetNetworkInfoResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNetworkInfoResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_remove(google::protobuf::Map<int32_t, ::authority_api::v1::GetNetworkInfoResponse> * m,
                          int32_t key, ::authority_api::v1::GetNetworkInfoResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_iter(
      const google::protobuf::Map<int32_t, ::authority_api::v1::GetNetworkInfoResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_GetNetworkInfoResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::authority_api::v1::GetNetworkInfoResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::authority_api::v1::GetNetworkInfoResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::authority_api::v1::GetNetworkInfoResponse>* __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_new() {
    return new google::protobuf::Map<uint32_t, ::authority_api::v1::GetNetworkInfoResponse>();
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_free(const google::protobuf::Map<uint32_t, ::authority_api::v1::GetNetworkInfoResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_clear(google::protobuf::Map<uint32_t, ::authority_api::v1::GetNetworkInfoResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_size(const google::protobuf::Map<uint32_t, ::authority_api::v1::GetNetworkInfoResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_insert(google::protobuf::Map<uint32_t, ::authority_api::v1::GetNetworkInfoResponse> * m,
                          uint32_t key, ::authority_api::v1::GetNetworkInfoResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_get(const google::protobuf::Map<uint32_t, ::authority_api::v1::GetNetworkInfoResponse>* m,
                       uint32_t key, const ::authority_api::v1::GetNetworkInfoResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNetworkInfoResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_remove(google::protobuf::Map<uint32_t, ::authority_api::v1::GetNetworkInfoResponse> * m,
                          uint32_t key, ::authority_api::v1::GetNetworkInfoResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_iter(
      const google::protobuf::Map<uint32_t, ::authority_api::v1::GetNetworkInfoResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_GetNetworkInfoResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::authority_api::v1::GetNetworkInfoResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::authority_api::v1::GetNetworkInfoResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::authority_api::v1::GetNetworkInfoResponse>* __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_new() {
    return new google::protobuf::Map<int64_t, ::authority_api::v1::GetNetworkInfoResponse>();
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_free(const google::protobuf::Map<int64_t, ::authority_api::v1::GetNetworkInfoResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_clear(google::protobuf::Map<int64_t, ::authority_api::v1::GetNetworkInfoResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_size(const google::protobuf::Map<int64_t, ::authority_api::v1::GetNetworkInfoResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_insert(google::protobuf::Map<int64_t, ::authority_api::v1::GetNetworkInfoResponse> * m,
                          int64_t key, ::authority_api::v1::GetNetworkInfoResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_get(const google::protobuf::Map<int64_t, ::authority_api::v1::GetNetworkInfoResponse>* m,
                       int64_t key, const ::authority_api::v1::GetNetworkInfoResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNetworkInfoResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_remove(google::protobuf::Map<int64_t, ::authority_api::v1::GetNetworkInfoResponse> * m,
                          int64_t key, ::authority_api::v1::GetNetworkInfoResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_iter(
      const google::protobuf::Map<int64_t, ::authority_api::v1::GetNetworkInfoResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_GetNetworkInfoResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::authority_api::v1::GetNetworkInfoResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::authority_api::v1::GetNetworkInfoResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::authority_api::v1::GetNetworkInfoResponse>* __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_new() {
    return new google::protobuf::Map<uint64_t, ::authority_api::v1::GetNetworkInfoResponse>();
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_free(const google::protobuf::Map<uint64_t, ::authority_api::v1::GetNetworkInfoResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_clear(google::protobuf::Map<uint64_t, ::authority_api::v1::GetNetworkInfoResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_size(const google::protobuf::Map<uint64_t, ::authority_api::v1::GetNetworkInfoResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_insert(google::protobuf::Map<uint64_t, ::authority_api::v1::GetNetworkInfoResponse> * m,
                          uint64_t key, ::authority_api::v1::GetNetworkInfoResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_get(const google::protobuf::Map<uint64_t, ::authority_api::v1::GetNetworkInfoResponse>* m,
                       uint64_t key, const ::authority_api::v1::GetNetworkInfoResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNetworkInfoResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_remove(google::protobuf::Map<uint64_t, ::authority_api::v1::GetNetworkInfoResponse> * m,
                          uint64_t key, ::authority_api::v1::GetNetworkInfoResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_iter(
      const google::protobuf::Map<uint64_t, ::authority_api::v1::GetNetworkInfoResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_GetNetworkInfoResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::authority_api::v1::GetNetworkInfoResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::authority_api::v1::GetNetworkInfoResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::authority_api::v1::GetNetworkInfoResponse>* __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_new() {
    return new google::protobuf::Map<bool, ::authority_api::v1::GetNetworkInfoResponse>();
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_free(const google::protobuf::Map<bool, ::authority_api::v1::GetNetworkInfoResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_clear(google::protobuf::Map<bool, ::authority_api::v1::GetNetworkInfoResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_size(const google::protobuf::Map<bool, ::authority_api::v1::GetNetworkInfoResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_insert(google::protobuf::Map<bool, ::authority_api::v1::GetNetworkInfoResponse> * m,
                          bool key, ::authority_api::v1::GetNetworkInfoResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_get(const google::protobuf::Map<bool, ::authority_api::v1::GetNetworkInfoResponse>* m,
                       bool key, const ::authority_api::v1::GetNetworkInfoResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNetworkInfoResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_remove(google::protobuf::Map<bool, ::authority_api::v1::GetNetworkInfoResponse> * m,
                          bool key, ::authority_api::v1::GetNetworkInfoResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_iter(
      const google::protobuf::Map<bool, ::authority_api::v1::GetNetworkInfoResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_GetNetworkInfoResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::authority_api::v1::GetNetworkInfoResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::authority_api::v1::GetNetworkInfoResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::authority_api::v1::GetNetworkInfoResponse>* __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_new() {
    return new google::protobuf::Map<std::string, ::authority_api::v1::GetNetworkInfoResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_free(const google::protobuf::Map<std::string, ::authority_api::v1::GetNetworkInfoResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_clear(google::protobuf::Map<std::string, ::authority_api::v1::GetNetworkInfoResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_size(const google::protobuf::Map<std::string, ::authority_api::v1::GetNetworkInfoResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_insert(google::protobuf::Map<std::string, ::authority_api::v1::GetNetworkInfoResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::GetNetworkInfoResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_get(const google::protobuf::Map<std::string, ::authority_api::v1::GetNetworkInfoResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::authority_api::v1::GetNetworkInfoResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNetworkInfoResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_remove(google::protobuf::Map<std::string, ::authority_api::v1::GetNetworkInfoResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::GetNetworkInfoResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_iter(
      const google::protobuf::Map<std::string, ::authority_api::v1::GetNetworkInfoResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNetworkInfoResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::authority_api::v1::GetNetworkInfoResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::authority_api::v1::GetNetworkInfoResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// authority_api.v1.GetNotaryRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__authority_api_v1_GetNotaryRequest_new() { return new ::authority_api::v1::GetNotaryRequest(); }
void __rust_proto_thunk__authority_api_v1_GetNotaryRequest_delete(void* ptr) { delete static_cast<::authority_api::v1::GetNotaryRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__authority_api_v1_GetNotaryRequest_serialize(::authority_api::v1::GetNotaryRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__authority_api_v1_GetNotaryRequest_parse(::authority_api::v1::GetNotaryRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__authority_api_v1_GetNotaryRequest_copy_from(::authority_api::v1::GetNotaryRequest* dst, const ::authority_api::v1::GetNotaryRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__authority_api_v1_GetNotaryRequest_repeated_len(google::protobuf::RepeatedPtrField<::authority_api::v1::GetNotaryRequest>* field) {
  return field->size();
}
const ::authority_api::v1::GetNotaryRequest& __rust_proto_thunk__authority_api_v1_GetNotaryRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::authority_api::v1::GetNotaryRequest>* field,
  size_t index) {
  return field->Get(index);
}
::authority_api::v1::GetNotaryRequest* __rust_proto_thunk__authority_api_v1_GetNotaryRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::authority_api::v1::GetNotaryRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::authority_api::v1::GetNotaryRequest* __rust_proto_thunk__authority_api_v1_GetNotaryRequest_repeated_add(google::protobuf::RepeatedPtrField<::authority_api::v1::GetNotaryRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__authority_api_v1_GetNotaryRequest_repeated_clear(google::protobuf::RepeatedPtrField<::authority_api::v1::GetNotaryRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__authority_api_v1_GetNotaryRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::authority_api::v1::GetNotaryRequest>& dst,
  const google::protobuf::RepeatedPtrField<::authority_api::v1::GetNotaryRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__authority_api_v1_GetNotaryRequest_get_verifier(::authority_api::v1::GetNotaryRequest* msg) {
  absl::string_view val = msg->verifier();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__authority_api_v1_GetNotaryRequest_set_verifier(::authority_api::v1::GetNotaryRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_verifier(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::authority_api::v1::GetNotaryRequest>* __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_new() {
    return new google::protobuf::Map<int32_t, ::authority_api::v1::GetNotaryRequest>();
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_free(const google::protobuf::Map<int32_t, ::authority_api::v1::GetNotaryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_clear(google::protobuf::Map<int32_t, ::authority_api::v1::GetNotaryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_size(const google::protobuf::Map<int32_t, ::authority_api::v1::GetNotaryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_insert(google::protobuf::Map<int32_t, ::authority_api::v1::GetNotaryRequest> * m,
                          int32_t key, ::authority_api::v1::GetNotaryRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_get(const google::protobuf::Map<int32_t, ::authority_api::v1::GetNotaryRequest>* m,
                       int32_t key, const ::authority_api::v1::GetNotaryRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNotaryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_remove(google::protobuf::Map<int32_t, ::authority_api::v1::GetNotaryRequest> * m,
                          int32_t key, ::authority_api::v1::GetNotaryRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_iter(
      const google::protobuf::Map<int32_t, ::authority_api::v1::GetNotaryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::authority_api::v1::GetNotaryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::authority_api::v1::GetNotaryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::authority_api::v1::GetNotaryRequest>* __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_new() {
    return new google::protobuf::Map<uint32_t, ::authority_api::v1::GetNotaryRequest>();
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_free(const google::protobuf::Map<uint32_t, ::authority_api::v1::GetNotaryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_clear(google::protobuf::Map<uint32_t, ::authority_api::v1::GetNotaryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_size(const google::protobuf::Map<uint32_t, ::authority_api::v1::GetNotaryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_insert(google::protobuf::Map<uint32_t, ::authority_api::v1::GetNotaryRequest> * m,
                          uint32_t key, ::authority_api::v1::GetNotaryRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_get(const google::protobuf::Map<uint32_t, ::authority_api::v1::GetNotaryRequest>* m,
                       uint32_t key, const ::authority_api::v1::GetNotaryRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNotaryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_remove(google::protobuf::Map<uint32_t, ::authority_api::v1::GetNotaryRequest> * m,
                          uint32_t key, ::authority_api::v1::GetNotaryRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_iter(
      const google::protobuf::Map<uint32_t, ::authority_api::v1::GetNotaryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::authority_api::v1::GetNotaryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::authority_api::v1::GetNotaryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::authority_api::v1::GetNotaryRequest>* __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_new() {
    return new google::protobuf::Map<int64_t, ::authority_api::v1::GetNotaryRequest>();
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_free(const google::protobuf::Map<int64_t, ::authority_api::v1::GetNotaryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_clear(google::protobuf::Map<int64_t, ::authority_api::v1::GetNotaryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_size(const google::protobuf::Map<int64_t, ::authority_api::v1::GetNotaryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_insert(google::protobuf::Map<int64_t, ::authority_api::v1::GetNotaryRequest> * m,
                          int64_t key, ::authority_api::v1::GetNotaryRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_get(const google::protobuf::Map<int64_t, ::authority_api::v1::GetNotaryRequest>* m,
                       int64_t key, const ::authority_api::v1::GetNotaryRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNotaryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_remove(google::protobuf::Map<int64_t, ::authority_api::v1::GetNotaryRequest> * m,
                          int64_t key, ::authority_api::v1::GetNotaryRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_iter(
      const google::protobuf::Map<int64_t, ::authority_api::v1::GetNotaryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::authority_api::v1::GetNotaryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::authority_api::v1::GetNotaryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::authority_api::v1::GetNotaryRequest>* __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_new() {
    return new google::protobuf::Map<uint64_t, ::authority_api::v1::GetNotaryRequest>();
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_free(const google::protobuf::Map<uint64_t, ::authority_api::v1::GetNotaryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_clear(google::protobuf::Map<uint64_t, ::authority_api::v1::GetNotaryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_size(const google::protobuf::Map<uint64_t, ::authority_api::v1::GetNotaryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_insert(google::protobuf::Map<uint64_t, ::authority_api::v1::GetNotaryRequest> * m,
                          uint64_t key, ::authority_api::v1::GetNotaryRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_get(const google::protobuf::Map<uint64_t, ::authority_api::v1::GetNotaryRequest>* m,
                       uint64_t key, const ::authority_api::v1::GetNotaryRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNotaryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_remove(google::protobuf::Map<uint64_t, ::authority_api::v1::GetNotaryRequest> * m,
                          uint64_t key, ::authority_api::v1::GetNotaryRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_iter(
      const google::protobuf::Map<uint64_t, ::authority_api::v1::GetNotaryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::authority_api::v1::GetNotaryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::authority_api::v1::GetNotaryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::authority_api::v1::GetNotaryRequest>* __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_new() {
    return new google::protobuf::Map<bool, ::authority_api::v1::GetNotaryRequest>();
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_free(const google::protobuf::Map<bool, ::authority_api::v1::GetNotaryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_clear(google::protobuf::Map<bool, ::authority_api::v1::GetNotaryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_size(const google::protobuf::Map<bool, ::authority_api::v1::GetNotaryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_insert(google::protobuf::Map<bool, ::authority_api::v1::GetNotaryRequest> * m,
                          bool key, ::authority_api::v1::GetNotaryRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_get(const google::protobuf::Map<bool, ::authority_api::v1::GetNotaryRequest>* m,
                       bool key, const ::authority_api::v1::GetNotaryRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNotaryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_remove(google::protobuf::Map<bool, ::authority_api::v1::GetNotaryRequest> * m,
                          bool key, ::authority_api::v1::GetNotaryRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_iter(
      const google::protobuf::Map<bool, ::authority_api::v1::GetNotaryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::authority_api::v1::GetNotaryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::authority_api::v1::GetNotaryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::authority_api::v1::GetNotaryRequest>* __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_new() {
    return new google::protobuf::Map<std::string, ::authority_api::v1::GetNotaryRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_free(const google::protobuf::Map<std::string, ::authority_api::v1::GetNotaryRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_clear(google::protobuf::Map<std::string, ::authority_api::v1::GetNotaryRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_size(const google::protobuf::Map<std::string, ::authority_api::v1::GetNotaryRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_insert(google::protobuf::Map<std::string, ::authority_api::v1::GetNotaryRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::GetNotaryRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_get(const google::protobuf::Map<std::string, ::authority_api::v1::GetNotaryRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::authority_api::v1::GetNotaryRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNotaryRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_remove(google::protobuf::Map<std::string, ::authority_api::v1::GetNotaryRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::GetNotaryRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_iter(
      const google::protobuf::Map<std::string, ::authority_api::v1::GetNotaryRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::authority_api::v1::GetNotaryRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::authority_api::v1::GetNotaryRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// authority_api.v1.GetNotaryResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__authority_api_v1_GetNotaryResponse_new() { return new ::authority_api::v1::GetNotaryResponse(); }
void __rust_proto_thunk__authority_api_v1_GetNotaryResponse_delete(void* ptr) { delete static_cast<::authority_api::v1::GetNotaryResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__authority_api_v1_GetNotaryResponse_serialize(::authority_api::v1::GetNotaryResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__authority_api_v1_GetNotaryResponse_parse(::authority_api::v1::GetNotaryResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__authority_api_v1_GetNotaryResponse_copy_from(::authority_api::v1::GetNotaryResponse* dst, const ::authority_api::v1::GetNotaryResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__authority_api_v1_GetNotaryResponse_repeated_len(google::protobuf::RepeatedPtrField<::authority_api::v1::GetNotaryResponse>* field) {
  return field->size();
}
const ::authority_api::v1::GetNotaryResponse& __rust_proto_thunk__authority_api_v1_GetNotaryResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::authority_api::v1::GetNotaryResponse>* field,
  size_t index) {
  return field->Get(index);
}
::authority_api::v1::GetNotaryResponse* __rust_proto_thunk__authority_api_v1_GetNotaryResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::authority_api::v1::GetNotaryResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::authority_api::v1::GetNotaryResponse* __rust_proto_thunk__authority_api_v1_GetNotaryResponse_repeated_add(google::protobuf::RepeatedPtrField<::authority_api::v1::GetNotaryResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__authority_api_v1_GetNotaryResponse_repeated_clear(google::protobuf::RepeatedPtrField<::authority_api::v1::GetNotaryResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__authority_api_v1_GetNotaryResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::authority_api::v1::GetNotaryResponse>& dst,
  const google::protobuf::RepeatedPtrField<::authority_api::v1::GetNotaryResponse>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__authority_api_v1_GetNotaryResponse_get_signature(::authority_api::v1::GetNotaryResponse* msg) {
  absl::string_view val = msg->signature();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__authority_api_v1_GetNotaryResponse_set_signature(::authority_api::v1::GetNotaryResponse* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_signature(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::authority_api::v1::GetNotaryResponse>* __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_new() {
    return new google::protobuf::Map<int32_t, ::authority_api::v1::GetNotaryResponse>();
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_free(const google::protobuf::Map<int32_t, ::authority_api::v1::GetNotaryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_clear(google::protobuf::Map<int32_t, ::authority_api::v1::GetNotaryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_size(const google::protobuf::Map<int32_t, ::authority_api::v1::GetNotaryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_insert(google::protobuf::Map<int32_t, ::authority_api::v1::GetNotaryResponse> * m,
                          int32_t key, ::authority_api::v1::GetNotaryResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_get(const google::protobuf::Map<int32_t, ::authority_api::v1::GetNotaryResponse>* m,
                       int32_t key, const ::authority_api::v1::GetNotaryResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNotaryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_remove(google::protobuf::Map<int32_t, ::authority_api::v1::GetNotaryResponse> * m,
                          int32_t key, ::authority_api::v1::GetNotaryResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_iter(
      const google::protobuf::Map<int32_t, ::authority_api::v1::GetNotaryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_GetNotaryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::authority_api::v1::GetNotaryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::authority_api::v1::GetNotaryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::authority_api::v1::GetNotaryResponse>* __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_new() {
    return new google::protobuf::Map<uint32_t, ::authority_api::v1::GetNotaryResponse>();
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_free(const google::protobuf::Map<uint32_t, ::authority_api::v1::GetNotaryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_clear(google::protobuf::Map<uint32_t, ::authority_api::v1::GetNotaryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_size(const google::protobuf::Map<uint32_t, ::authority_api::v1::GetNotaryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_insert(google::protobuf::Map<uint32_t, ::authority_api::v1::GetNotaryResponse> * m,
                          uint32_t key, ::authority_api::v1::GetNotaryResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_get(const google::protobuf::Map<uint32_t, ::authority_api::v1::GetNotaryResponse>* m,
                       uint32_t key, const ::authority_api::v1::GetNotaryResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNotaryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_remove(google::protobuf::Map<uint32_t, ::authority_api::v1::GetNotaryResponse> * m,
                          uint32_t key, ::authority_api::v1::GetNotaryResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_iter(
      const google::protobuf::Map<uint32_t, ::authority_api::v1::GetNotaryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_GetNotaryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::authority_api::v1::GetNotaryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::authority_api::v1::GetNotaryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::authority_api::v1::GetNotaryResponse>* __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_new() {
    return new google::protobuf::Map<int64_t, ::authority_api::v1::GetNotaryResponse>();
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_free(const google::protobuf::Map<int64_t, ::authority_api::v1::GetNotaryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_clear(google::protobuf::Map<int64_t, ::authority_api::v1::GetNotaryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_size(const google::protobuf::Map<int64_t, ::authority_api::v1::GetNotaryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_insert(google::protobuf::Map<int64_t, ::authority_api::v1::GetNotaryResponse> * m,
                          int64_t key, ::authority_api::v1::GetNotaryResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_get(const google::protobuf::Map<int64_t, ::authority_api::v1::GetNotaryResponse>* m,
                       int64_t key, const ::authority_api::v1::GetNotaryResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNotaryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_remove(google::protobuf::Map<int64_t, ::authority_api::v1::GetNotaryResponse> * m,
                          int64_t key, ::authority_api::v1::GetNotaryResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_iter(
      const google::protobuf::Map<int64_t, ::authority_api::v1::GetNotaryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_GetNotaryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::authority_api::v1::GetNotaryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::authority_api::v1::GetNotaryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::authority_api::v1::GetNotaryResponse>* __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_new() {
    return new google::protobuf::Map<uint64_t, ::authority_api::v1::GetNotaryResponse>();
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_free(const google::protobuf::Map<uint64_t, ::authority_api::v1::GetNotaryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_clear(google::protobuf::Map<uint64_t, ::authority_api::v1::GetNotaryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_size(const google::protobuf::Map<uint64_t, ::authority_api::v1::GetNotaryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_insert(google::protobuf::Map<uint64_t, ::authority_api::v1::GetNotaryResponse> * m,
                          uint64_t key, ::authority_api::v1::GetNotaryResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_get(const google::protobuf::Map<uint64_t, ::authority_api::v1::GetNotaryResponse>* m,
                       uint64_t key, const ::authority_api::v1::GetNotaryResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNotaryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_remove(google::protobuf::Map<uint64_t, ::authority_api::v1::GetNotaryResponse> * m,
                          uint64_t key, ::authority_api::v1::GetNotaryResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_iter(
      const google::protobuf::Map<uint64_t, ::authority_api::v1::GetNotaryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_GetNotaryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::authority_api::v1::GetNotaryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::authority_api::v1::GetNotaryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::authority_api::v1::GetNotaryResponse>* __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_new() {
    return new google::protobuf::Map<bool, ::authority_api::v1::GetNotaryResponse>();
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_free(const google::protobuf::Map<bool, ::authority_api::v1::GetNotaryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_clear(google::protobuf::Map<bool, ::authority_api::v1::GetNotaryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_size(const google::protobuf::Map<bool, ::authority_api::v1::GetNotaryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_insert(google::protobuf::Map<bool, ::authority_api::v1::GetNotaryResponse> * m,
                          bool key, ::authority_api::v1::GetNotaryResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_get(const google::protobuf::Map<bool, ::authority_api::v1::GetNotaryResponse>* m,
                       bool key, const ::authority_api::v1::GetNotaryResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNotaryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_remove(google::protobuf::Map<bool, ::authority_api::v1::GetNotaryResponse> * m,
                          bool key, ::authority_api::v1::GetNotaryResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_iter(
      const google::protobuf::Map<bool, ::authority_api::v1::GetNotaryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_GetNotaryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::authority_api::v1::GetNotaryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::authority_api::v1::GetNotaryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::authority_api::v1::GetNotaryResponse>* __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_new() {
    return new google::protobuf::Map<std::string, ::authority_api::v1::GetNotaryResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_free(const google::protobuf::Map<std::string, ::authority_api::v1::GetNotaryResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_clear(google::protobuf::Map<std::string, ::authority_api::v1::GetNotaryResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_size(const google::protobuf::Map<std::string, ::authority_api::v1::GetNotaryResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_insert(google::protobuf::Map<std::string, ::authority_api::v1::GetNotaryResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::GetNotaryResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_get(const google::protobuf::Map<std::string, ::authority_api::v1::GetNotaryResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::authority_api::v1::GetNotaryResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::GetNotaryResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_remove(google::protobuf::Map<std::string, ::authority_api::v1::GetNotaryResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::GetNotaryResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_iter(
      const google::protobuf::Map<std::string, ::authority_api::v1::GetNotaryResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_GetNotaryResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::authority_api::v1::GetNotaryResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::authority_api::v1::GetNotaryResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// authority_api.v1.ListGovernedAssetsRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_new() { return new ::authority_api::v1::ListGovernedAssetsRequest(); }
void __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_delete(void* ptr) { delete static_cast<::authority_api::v1::ListGovernedAssetsRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_serialize(::authority_api::v1::ListGovernedAssetsRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_parse(::authority_api::v1::ListGovernedAssetsRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_copy_from(::authority_api::v1::ListGovernedAssetsRequest* dst, const ::authority_api::v1::ListGovernedAssetsRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_repeated_len(google::protobuf::RepeatedPtrField<::authority_api::v1::ListGovernedAssetsRequest>* field) {
  return field->size();
}
const ::authority_api::v1::ListGovernedAssetsRequest& __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::authority_api::v1::ListGovernedAssetsRequest>* field,
  size_t index) {
  return field->Get(index);
}
::authority_api::v1::ListGovernedAssetsRequest* __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::authority_api::v1::ListGovernedAssetsRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::authority_api::v1::ListGovernedAssetsRequest* __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_repeated_add(google::protobuf::RepeatedPtrField<::authority_api::v1::ListGovernedAssetsRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_repeated_clear(google::protobuf::RepeatedPtrField<::authority_api::v1::ListGovernedAssetsRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__authority_api_v1_ListGovernedAssetsRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::authority_api::v1::ListGovernedAssetsRequest>& dst,
  const google::protobuf::RepeatedPtrField<::authority_api::v1::ListGovernedAssetsRequest>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::authority_api::v1::ListGovernedAssetsRequest>* __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_new() {
    return new google::protobuf::Map<int32_t, ::authority_api::v1::ListGovernedAssetsRequest>();
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_free(const google::protobuf::Map<int32_t, ::authority_api::v1::ListGovernedAssetsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_clear(google::protobuf::Map<int32_t, ::authority_api::v1::ListGovernedAssetsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_size(const google::protobuf::Map<int32_t, ::authority_api::v1::ListGovernedAssetsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_insert(google::protobuf::Map<int32_t, ::authority_api::v1::ListGovernedAssetsRequest> * m,
                          int32_t key, ::authority_api::v1::ListGovernedAssetsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_get(const google::protobuf::Map<int32_t, ::authority_api::v1::ListGovernedAssetsRequest>* m,
                       int32_t key, const ::authority_api::v1::ListGovernedAssetsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::ListGovernedAssetsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_remove(google::protobuf::Map<int32_t, ::authority_api::v1::ListGovernedAssetsRequest> * m,
                          int32_t key, ::authority_api::v1::ListGovernedAssetsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_iter(
      const google::protobuf::Map<int32_t, ::authority_api::v1::ListGovernedAssetsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::authority_api::v1::ListGovernedAssetsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::authority_api::v1::ListGovernedAssetsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::authority_api::v1::ListGovernedAssetsRequest>* __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_new() {
    return new google::protobuf::Map<uint32_t, ::authority_api::v1::ListGovernedAssetsRequest>();
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_free(const google::protobuf::Map<uint32_t, ::authority_api::v1::ListGovernedAssetsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_clear(google::protobuf::Map<uint32_t, ::authority_api::v1::ListGovernedAssetsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_size(const google::protobuf::Map<uint32_t, ::authority_api::v1::ListGovernedAssetsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_insert(google::protobuf::Map<uint32_t, ::authority_api::v1::ListGovernedAssetsRequest> * m,
                          uint32_t key, ::authority_api::v1::ListGovernedAssetsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_get(const google::protobuf::Map<uint32_t, ::authority_api::v1::ListGovernedAssetsRequest>* m,
                       uint32_t key, const ::authority_api::v1::ListGovernedAssetsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::ListGovernedAssetsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_remove(google::protobuf::Map<uint32_t, ::authority_api::v1::ListGovernedAssetsRequest> * m,
                          uint32_t key, ::authority_api::v1::ListGovernedAssetsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_iter(
      const google::protobuf::Map<uint32_t, ::authority_api::v1::ListGovernedAssetsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::authority_api::v1::ListGovernedAssetsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::authority_api::v1::ListGovernedAssetsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::authority_api::v1::ListGovernedAssetsRequest>* __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_new() {
    return new google::protobuf::Map<int64_t, ::authority_api::v1::ListGovernedAssetsRequest>();
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_free(const google::protobuf::Map<int64_t, ::authority_api::v1::ListGovernedAssetsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_clear(google::protobuf::Map<int64_t, ::authority_api::v1::ListGovernedAssetsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_size(const google::protobuf::Map<int64_t, ::authority_api::v1::ListGovernedAssetsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_insert(google::protobuf::Map<int64_t, ::authority_api::v1::ListGovernedAssetsRequest> * m,
                          int64_t key, ::authority_api::v1::ListGovernedAssetsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_get(const google::protobuf::Map<int64_t, ::authority_api::v1::ListGovernedAssetsRequest>* m,
                       int64_t key, const ::authority_api::v1::ListGovernedAssetsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::ListGovernedAssetsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_remove(google::protobuf::Map<int64_t, ::authority_api::v1::ListGovernedAssetsRequest> * m,
                          int64_t key, ::authority_api::v1::ListGovernedAssetsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_iter(
      const google::protobuf::Map<int64_t, ::authority_api::v1::ListGovernedAssetsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::authority_api::v1::ListGovernedAssetsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::authority_api::v1::ListGovernedAssetsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::authority_api::v1::ListGovernedAssetsRequest>* __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_new() {
    return new google::protobuf::Map<uint64_t, ::authority_api::v1::ListGovernedAssetsRequest>();
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_free(const google::protobuf::Map<uint64_t, ::authority_api::v1::ListGovernedAssetsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_clear(google::protobuf::Map<uint64_t, ::authority_api::v1::ListGovernedAssetsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_size(const google::protobuf::Map<uint64_t, ::authority_api::v1::ListGovernedAssetsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_insert(google::protobuf::Map<uint64_t, ::authority_api::v1::ListGovernedAssetsRequest> * m,
                          uint64_t key, ::authority_api::v1::ListGovernedAssetsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_get(const google::protobuf::Map<uint64_t, ::authority_api::v1::ListGovernedAssetsRequest>* m,
                       uint64_t key, const ::authority_api::v1::ListGovernedAssetsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::ListGovernedAssetsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_remove(google::protobuf::Map<uint64_t, ::authority_api::v1::ListGovernedAssetsRequest> * m,
                          uint64_t key, ::authority_api::v1::ListGovernedAssetsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_iter(
      const google::protobuf::Map<uint64_t, ::authority_api::v1::ListGovernedAssetsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::authority_api::v1::ListGovernedAssetsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::authority_api::v1::ListGovernedAssetsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::authority_api::v1::ListGovernedAssetsRequest>* __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_new() {
    return new google::protobuf::Map<bool, ::authority_api::v1::ListGovernedAssetsRequest>();
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_free(const google::protobuf::Map<bool, ::authority_api::v1::ListGovernedAssetsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_clear(google::protobuf::Map<bool, ::authority_api::v1::ListGovernedAssetsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_size(const google::protobuf::Map<bool, ::authority_api::v1::ListGovernedAssetsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_insert(google::protobuf::Map<bool, ::authority_api::v1::ListGovernedAssetsRequest> * m,
                          bool key, ::authority_api::v1::ListGovernedAssetsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_get(const google::protobuf::Map<bool, ::authority_api::v1::ListGovernedAssetsRequest>* m,
                       bool key, const ::authority_api::v1::ListGovernedAssetsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::ListGovernedAssetsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_remove(google::protobuf::Map<bool, ::authority_api::v1::ListGovernedAssetsRequest> * m,
                          bool key, ::authority_api::v1::ListGovernedAssetsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_iter(
      const google::protobuf::Map<bool, ::authority_api::v1::ListGovernedAssetsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::authority_api::v1::ListGovernedAssetsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::authority_api::v1::ListGovernedAssetsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::authority_api::v1::ListGovernedAssetsRequest>* __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_new() {
    return new google::protobuf::Map<std::string, ::authority_api::v1::ListGovernedAssetsRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_free(const google::protobuf::Map<std::string, ::authority_api::v1::ListGovernedAssetsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_clear(google::protobuf::Map<std::string, ::authority_api::v1::ListGovernedAssetsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_size(const google::protobuf::Map<std::string, ::authority_api::v1::ListGovernedAssetsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_insert(google::protobuf::Map<std::string, ::authority_api::v1::ListGovernedAssetsRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::ListGovernedAssetsRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_get(const google::protobuf::Map<std::string, ::authority_api::v1::ListGovernedAssetsRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::authority_api::v1::ListGovernedAssetsRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::ListGovernedAssetsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_remove(google::protobuf::Map<std::string, ::authority_api::v1::ListGovernedAssetsRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::ListGovernedAssetsRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_iter(
      const google::protobuf::Map<std::string, ::authority_api::v1::ListGovernedAssetsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::authority_api::v1::ListGovernedAssetsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::authority_api::v1::ListGovernedAssetsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// authority_api.v1.ListGovernedAssetsResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_new() { return new ::authority_api::v1::ListGovernedAssetsResponse(); }
void __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_delete(void* ptr) { delete static_cast<::authority_api::v1::ListGovernedAssetsResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_serialize(::authority_api::v1::ListGovernedAssetsResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_parse(::authority_api::v1::ListGovernedAssetsResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_copy_from(::authority_api::v1::ListGovernedAssetsResponse* dst, const ::authority_api::v1::ListGovernedAssetsResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_repeated_len(google::protobuf::RepeatedPtrField<::authority_api::v1::ListGovernedAssetsResponse>* field) {
  return field->size();
}
const ::authority_api::v1::ListGovernedAssetsResponse& __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::authority_api::v1::ListGovernedAssetsResponse>* field,
  size_t index) {
  return field->Get(index);
}
::authority_api::v1::ListGovernedAssetsResponse* __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::authority_api::v1::ListGovernedAssetsResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::authority_api::v1::ListGovernedAssetsResponse* __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_repeated_add(google::protobuf::RepeatedPtrField<::authority_api::v1::ListGovernedAssetsResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_repeated_clear(google::protobuf::RepeatedPtrField<::authority_api::v1::ListGovernedAssetsResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::authority_api::v1::ListGovernedAssetsResponse>& dst,
  const google::protobuf::RepeatedPtrField<::authority_api::v1::ListGovernedAssetsResponse>& src) {
  dst = src;
}

const void* __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_get_asset_data(const ::authority_api::v1::ListGovernedAssetsResponse* msg) {
  return &msg->asset_data();
}
void* __rust_proto_thunk__authority_api_v1_ListGovernedAssetsResponse_get_mut_asset_data(::authority_api::v1::ListGovernedAssetsResponse* msg) { return msg->mutable_asset_data(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::authority_api::v1::ListGovernedAssetsResponse>* __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_new() {
    return new google::protobuf::Map<int32_t, ::authority_api::v1::ListGovernedAssetsResponse>();
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_free(const google::protobuf::Map<int32_t, ::authority_api::v1::ListGovernedAssetsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_clear(google::protobuf::Map<int32_t, ::authority_api::v1::ListGovernedAssetsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_size(const google::protobuf::Map<int32_t, ::authority_api::v1::ListGovernedAssetsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_insert(google::protobuf::Map<int32_t, ::authority_api::v1::ListGovernedAssetsResponse> * m,
                          int32_t key, ::authority_api::v1::ListGovernedAssetsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_get(const google::protobuf::Map<int32_t, ::authority_api::v1::ListGovernedAssetsResponse>* m,
                       int32_t key, const ::authority_api::v1::ListGovernedAssetsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::ListGovernedAssetsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_remove(google::protobuf::Map<int32_t, ::authority_api::v1::ListGovernedAssetsResponse> * m,
                          int32_t key, ::authority_api::v1::ListGovernedAssetsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_iter(
      const google::protobuf::Map<int32_t, ::authority_api::v1::ListGovernedAssetsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_ListGovernedAssetsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::authority_api::v1::ListGovernedAssetsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::authority_api::v1::ListGovernedAssetsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::authority_api::v1::ListGovernedAssetsResponse>* __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_new() {
    return new google::protobuf::Map<uint32_t, ::authority_api::v1::ListGovernedAssetsResponse>();
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_free(const google::protobuf::Map<uint32_t, ::authority_api::v1::ListGovernedAssetsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_clear(google::protobuf::Map<uint32_t, ::authority_api::v1::ListGovernedAssetsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_size(const google::protobuf::Map<uint32_t, ::authority_api::v1::ListGovernedAssetsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_insert(google::protobuf::Map<uint32_t, ::authority_api::v1::ListGovernedAssetsResponse> * m,
                          uint32_t key, ::authority_api::v1::ListGovernedAssetsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_get(const google::protobuf::Map<uint32_t, ::authority_api::v1::ListGovernedAssetsResponse>* m,
                       uint32_t key, const ::authority_api::v1::ListGovernedAssetsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::ListGovernedAssetsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_remove(google::protobuf::Map<uint32_t, ::authority_api::v1::ListGovernedAssetsResponse> * m,
                          uint32_t key, ::authority_api::v1::ListGovernedAssetsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_iter(
      const google::protobuf::Map<uint32_t, ::authority_api::v1::ListGovernedAssetsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_ListGovernedAssetsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::authority_api::v1::ListGovernedAssetsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::authority_api::v1::ListGovernedAssetsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::authority_api::v1::ListGovernedAssetsResponse>* __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_new() {
    return new google::protobuf::Map<int64_t, ::authority_api::v1::ListGovernedAssetsResponse>();
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_free(const google::protobuf::Map<int64_t, ::authority_api::v1::ListGovernedAssetsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_clear(google::protobuf::Map<int64_t, ::authority_api::v1::ListGovernedAssetsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_size(const google::protobuf::Map<int64_t, ::authority_api::v1::ListGovernedAssetsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_insert(google::protobuf::Map<int64_t, ::authority_api::v1::ListGovernedAssetsResponse> * m,
                          int64_t key, ::authority_api::v1::ListGovernedAssetsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_get(const google::protobuf::Map<int64_t, ::authority_api::v1::ListGovernedAssetsResponse>* m,
                       int64_t key, const ::authority_api::v1::ListGovernedAssetsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::ListGovernedAssetsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_remove(google::protobuf::Map<int64_t, ::authority_api::v1::ListGovernedAssetsResponse> * m,
                          int64_t key, ::authority_api::v1::ListGovernedAssetsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_iter(
      const google::protobuf::Map<int64_t, ::authority_api::v1::ListGovernedAssetsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_ListGovernedAssetsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::authority_api::v1::ListGovernedAssetsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::authority_api::v1::ListGovernedAssetsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::authority_api::v1::ListGovernedAssetsResponse>* __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_new() {
    return new google::protobuf::Map<uint64_t, ::authority_api::v1::ListGovernedAssetsResponse>();
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_free(const google::protobuf::Map<uint64_t, ::authority_api::v1::ListGovernedAssetsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_clear(google::protobuf::Map<uint64_t, ::authority_api::v1::ListGovernedAssetsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_size(const google::protobuf::Map<uint64_t, ::authority_api::v1::ListGovernedAssetsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_insert(google::protobuf::Map<uint64_t, ::authority_api::v1::ListGovernedAssetsResponse> * m,
                          uint64_t key, ::authority_api::v1::ListGovernedAssetsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_get(const google::protobuf::Map<uint64_t, ::authority_api::v1::ListGovernedAssetsResponse>* m,
                       uint64_t key, const ::authority_api::v1::ListGovernedAssetsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::ListGovernedAssetsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_remove(google::protobuf::Map<uint64_t, ::authority_api::v1::ListGovernedAssetsResponse> * m,
                          uint64_t key, ::authority_api::v1::ListGovernedAssetsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_iter(
      const google::protobuf::Map<uint64_t, ::authority_api::v1::ListGovernedAssetsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_ListGovernedAssetsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::authority_api::v1::ListGovernedAssetsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::authority_api::v1::ListGovernedAssetsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::authority_api::v1::ListGovernedAssetsResponse>* __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_new() {
    return new google::protobuf::Map<bool, ::authority_api::v1::ListGovernedAssetsResponse>();
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_free(const google::protobuf::Map<bool, ::authority_api::v1::ListGovernedAssetsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_clear(google::protobuf::Map<bool, ::authority_api::v1::ListGovernedAssetsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_size(const google::protobuf::Map<bool, ::authority_api::v1::ListGovernedAssetsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_insert(google::protobuf::Map<bool, ::authority_api::v1::ListGovernedAssetsResponse> * m,
                          bool key, ::authority_api::v1::ListGovernedAssetsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_get(const google::protobuf::Map<bool, ::authority_api::v1::ListGovernedAssetsResponse>* m,
                       bool key, const ::authority_api::v1::ListGovernedAssetsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::ListGovernedAssetsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_remove(google::protobuf::Map<bool, ::authority_api::v1::ListGovernedAssetsResponse> * m,
                          bool key, ::authority_api::v1::ListGovernedAssetsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_iter(
      const google::protobuf::Map<bool, ::authority_api::v1::ListGovernedAssetsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_ListGovernedAssetsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::authority_api::v1::ListGovernedAssetsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::authority_api::v1::ListGovernedAssetsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::authority_api::v1::ListGovernedAssetsResponse>* __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_new() {
    return new google::protobuf::Map<std::string, ::authority_api::v1::ListGovernedAssetsResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_free(const google::protobuf::Map<std::string, ::authority_api::v1::ListGovernedAssetsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_clear(google::protobuf::Map<std::string, ::authority_api::v1::ListGovernedAssetsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_size(const google::protobuf::Map<std::string, ::authority_api::v1::ListGovernedAssetsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_insert(google::protobuf::Map<std::string, ::authority_api::v1::ListGovernedAssetsResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::ListGovernedAssetsResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_get(const google::protobuf::Map<std::string, ::authority_api::v1::ListGovernedAssetsResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::authority_api::v1::ListGovernedAssetsResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::ListGovernedAssetsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_remove(google::protobuf::Map<std::string, ::authority_api::v1::ListGovernedAssetsResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::ListGovernedAssetsResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_iter(
      const google::protobuf::Map<std::string, ::authority_api::v1::ListGovernedAssetsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_ListGovernedAssetsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::authority_api::v1::ListGovernedAssetsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::authority_api::v1::ListGovernedAssetsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// authority_api.v1.SetGovernedAssetsRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_new() { return new ::authority_api::v1::SetGovernedAssetsRequest(); }
void __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_delete(void* ptr) { delete static_cast<::authority_api::v1::SetGovernedAssetsRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_serialize(::authority_api::v1::SetGovernedAssetsRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_parse(::authority_api::v1::SetGovernedAssetsRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_copy_from(::authority_api::v1::SetGovernedAssetsRequest* dst, const ::authority_api::v1::SetGovernedAssetsRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_repeated_len(google::protobuf::RepeatedPtrField<::authority_api::v1::SetGovernedAssetsRequest>* field) {
  return field->size();
}
const ::authority_api::v1::SetGovernedAssetsRequest& __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::authority_api::v1::SetGovernedAssetsRequest>* field,
  size_t index) {
  return field->Get(index);
}
::authority_api::v1::SetGovernedAssetsRequest* __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::authority_api::v1::SetGovernedAssetsRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::authority_api::v1::SetGovernedAssetsRequest* __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_repeated_add(google::protobuf::RepeatedPtrField<::authority_api::v1::SetGovernedAssetsRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_repeated_clear(google::protobuf::RepeatedPtrField<::authority_api::v1::SetGovernedAssetsRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::authority_api::v1::SetGovernedAssetsRequest>& dst,
  const google::protobuf::RepeatedPtrField<::authority_api::v1::SetGovernedAssetsRequest>& src) {
  dst = src;
}

const void* __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_get_asset_data(const ::authority_api::v1::SetGovernedAssetsRequest* msg) {
  return &msg->asset_data();
}
void* __rust_proto_thunk__authority_api_v1_SetGovernedAssetsRequest_get_mut_asset_data(::authority_api::v1::SetGovernedAssetsRequest* msg) { return msg->mutable_asset_data(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::authority_api::v1::SetGovernedAssetsRequest>* __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_new() {
    return new google::protobuf::Map<int32_t, ::authority_api::v1::SetGovernedAssetsRequest>();
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_free(const google::protobuf::Map<int32_t, ::authority_api::v1::SetGovernedAssetsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_clear(google::protobuf::Map<int32_t, ::authority_api::v1::SetGovernedAssetsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_size(const google::protobuf::Map<int32_t, ::authority_api::v1::SetGovernedAssetsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_insert(google::protobuf::Map<int32_t, ::authority_api::v1::SetGovernedAssetsRequest> * m,
                          int32_t key, ::authority_api::v1::SetGovernedAssetsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_get(const google::protobuf::Map<int32_t, ::authority_api::v1::SetGovernedAssetsRequest>* m,
                       int32_t key, const ::authority_api::v1::SetGovernedAssetsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetGovernedAssetsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_remove(google::protobuf::Map<int32_t, ::authority_api::v1::SetGovernedAssetsRequest> * m,
                          int32_t key, ::authority_api::v1::SetGovernedAssetsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_iter(
      const google::protobuf::Map<int32_t, ::authority_api::v1::SetGovernedAssetsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::authority_api::v1::SetGovernedAssetsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::authority_api::v1::SetGovernedAssetsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::authority_api::v1::SetGovernedAssetsRequest>* __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_new() {
    return new google::protobuf::Map<uint32_t, ::authority_api::v1::SetGovernedAssetsRequest>();
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_free(const google::protobuf::Map<uint32_t, ::authority_api::v1::SetGovernedAssetsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_clear(google::protobuf::Map<uint32_t, ::authority_api::v1::SetGovernedAssetsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_size(const google::protobuf::Map<uint32_t, ::authority_api::v1::SetGovernedAssetsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_insert(google::protobuf::Map<uint32_t, ::authority_api::v1::SetGovernedAssetsRequest> * m,
                          uint32_t key, ::authority_api::v1::SetGovernedAssetsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_get(const google::protobuf::Map<uint32_t, ::authority_api::v1::SetGovernedAssetsRequest>* m,
                       uint32_t key, const ::authority_api::v1::SetGovernedAssetsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetGovernedAssetsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_remove(google::protobuf::Map<uint32_t, ::authority_api::v1::SetGovernedAssetsRequest> * m,
                          uint32_t key, ::authority_api::v1::SetGovernedAssetsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_iter(
      const google::protobuf::Map<uint32_t, ::authority_api::v1::SetGovernedAssetsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::authority_api::v1::SetGovernedAssetsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::authority_api::v1::SetGovernedAssetsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::authority_api::v1::SetGovernedAssetsRequest>* __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_new() {
    return new google::protobuf::Map<int64_t, ::authority_api::v1::SetGovernedAssetsRequest>();
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_free(const google::protobuf::Map<int64_t, ::authority_api::v1::SetGovernedAssetsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_clear(google::protobuf::Map<int64_t, ::authority_api::v1::SetGovernedAssetsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_size(const google::protobuf::Map<int64_t, ::authority_api::v1::SetGovernedAssetsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_insert(google::protobuf::Map<int64_t, ::authority_api::v1::SetGovernedAssetsRequest> * m,
                          int64_t key, ::authority_api::v1::SetGovernedAssetsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_get(const google::protobuf::Map<int64_t, ::authority_api::v1::SetGovernedAssetsRequest>* m,
                       int64_t key, const ::authority_api::v1::SetGovernedAssetsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetGovernedAssetsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_remove(google::protobuf::Map<int64_t, ::authority_api::v1::SetGovernedAssetsRequest> * m,
                          int64_t key, ::authority_api::v1::SetGovernedAssetsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_iter(
      const google::protobuf::Map<int64_t, ::authority_api::v1::SetGovernedAssetsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::authority_api::v1::SetGovernedAssetsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::authority_api::v1::SetGovernedAssetsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::authority_api::v1::SetGovernedAssetsRequest>* __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_new() {
    return new google::protobuf::Map<uint64_t, ::authority_api::v1::SetGovernedAssetsRequest>();
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_free(const google::protobuf::Map<uint64_t, ::authority_api::v1::SetGovernedAssetsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_clear(google::protobuf::Map<uint64_t, ::authority_api::v1::SetGovernedAssetsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_size(const google::protobuf::Map<uint64_t, ::authority_api::v1::SetGovernedAssetsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_insert(google::protobuf::Map<uint64_t, ::authority_api::v1::SetGovernedAssetsRequest> * m,
                          uint64_t key, ::authority_api::v1::SetGovernedAssetsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_get(const google::protobuf::Map<uint64_t, ::authority_api::v1::SetGovernedAssetsRequest>* m,
                       uint64_t key, const ::authority_api::v1::SetGovernedAssetsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetGovernedAssetsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_remove(google::protobuf::Map<uint64_t, ::authority_api::v1::SetGovernedAssetsRequest> * m,
                          uint64_t key, ::authority_api::v1::SetGovernedAssetsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_iter(
      const google::protobuf::Map<uint64_t, ::authority_api::v1::SetGovernedAssetsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::authority_api::v1::SetGovernedAssetsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::authority_api::v1::SetGovernedAssetsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::authority_api::v1::SetGovernedAssetsRequest>* __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_new() {
    return new google::protobuf::Map<bool, ::authority_api::v1::SetGovernedAssetsRequest>();
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_free(const google::protobuf::Map<bool, ::authority_api::v1::SetGovernedAssetsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_clear(google::protobuf::Map<bool, ::authority_api::v1::SetGovernedAssetsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_size(const google::protobuf::Map<bool, ::authority_api::v1::SetGovernedAssetsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_insert(google::protobuf::Map<bool, ::authority_api::v1::SetGovernedAssetsRequest> * m,
                          bool key, ::authority_api::v1::SetGovernedAssetsRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_get(const google::protobuf::Map<bool, ::authority_api::v1::SetGovernedAssetsRequest>* m,
                       bool key, const ::authority_api::v1::SetGovernedAssetsRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetGovernedAssetsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_remove(google::protobuf::Map<bool, ::authority_api::v1::SetGovernedAssetsRequest> * m,
                          bool key, ::authority_api::v1::SetGovernedAssetsRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_iter(
      const google::protobuf::Map<bool, ::authority_api::v1::SetGovernedAssetsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::authority_api::v1::SetGovernedAssetsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::authority_api::v1::SetGovernedAssetsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::authority_api::v1::SetGovernedAssetsRequest>* __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_new() {
    return new google::protobuf::Map<std::string, ::authority_api::v1::SetGovernedAssetsRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_free(const google::protobuf::Map<std::string, ::authority_api::v1::SetGovernedAssetsRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_clear(google::protobuf::Map<std::string, ::authority_api::v1::SetGovernedAssetsRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_size(const google::protobuf::Map<std::string, ::authority_api::v1::SetGovernedAssetsRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_insert(google::protobuf::Map<std::string, ::authority_api::v1::SetGovernedAssetsRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::SetGovernedAssetsRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_get(const google::protobuf::Map<std::string, ::authority_api::v1::SetGovernedAssetsRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::authority_api::v1::SetGovernedAssetsRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetGovernedAssetsRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_remove(google::protobuf::Map<std::string, ::authority_api::v1::SetGovernedAssetsRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::SetGovernedAssetsRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_iter(
      const google::protobuf::Map<std::string, ::authority_api::v1::SetGovernedAssetsRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::authority_api::v1::SetGovernedAssetsRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::authority_api::v1::SetGovernedAssetsRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// authority_api.v1.SetGovernedAssetsResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_new() { return new ::authority_api::v1::SetGovernedAssetsResponse(); }
void __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_delete(void* ptr) { delete static_cast<::authority_api::v1::SetGovernedAssetsResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_serialize(::authority_api::v1::SetGovernedAssetsResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_parse(::authority_api::v1::SetGovernedAssetsResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_copy_from(::authority_api::v1::SetGovernedAssetsResponse* dst, const ::authority_api::v1::SetGovernedAssetsResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_repeated_len(google::protobuf::RepeatedPtrField<::authority_api::v1::SetGovernedAssetsResponse>* field) {
  return field->size();
}
const ::authority_api::v1::SetGovernedAssetsResponse& __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::authority_api::v1::SetGovernedAssetsResponse>* field,
  size_t index) {
  return field->Get(index);
}
::authority_api::v1::SetGovernedAssetsResponse* __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::authority_api::v1::SetGovernedAssetsResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::authority_api::v1::SetGovernedAssetsResponse* __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_repeated_add(google::protobuf::RepeatedPtrField<::authority_api::v1::SetGovernedAssetsResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_repeated_clear(google::protobuf::RepeatedPtrField<::authority_api::v1::SetGovernedAssetsResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__authority_api_v1_SetGovernedAssetsResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::authority_api::v1::SetGovernedAssetsResponse>& dst,
  const google::protobuf::RepeatedPtrField<::authority_api::v1::SetGovernedAssetsResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::authority_api::v1::SetGovernedAssetsResponse>* __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_new() {
    return new google::protobuf::Map<int32_t, ::authority_api::v1::SetGovernedAssetsResponse>();
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_free(const google::protobuf::Map<int32_t, ::authority_api::v1::SetGovernedAssetsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_clear(google::protobuf::Map<int32_t, ::authority_api::v1::SetGovernedAssetsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_size(const google::protobuf::Map<int32_t, ::authority_api::v1::SetGovernedAssetsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_insert(google::protobuf::Map<int32_t, ::authority_api::v1::SetGovernedAssetsResponse> * m,
                          int32_t key, ::authority_api::v1::SetGovernedAssetsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_get(const google::protobuf::Map<int32_t, ::authority_api::v1::SetGovernedAssetsResponse>* m,
                       int32_t key, const ::authority_api::v1::SetGovernedAssetsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetGovernedAssetsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_remove(google::protobuf::Map<int32_t, ::authority_api::v1::SetGovernedAssetsResponse> * m,
                          int32_t key, ::authority_api::v1::SetGovernedAssetsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_iter(
      const google::protobuf::Map<int32_t, ::authority_api::v1::SetGovernedAssetsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_SetGovernedAssetsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::authority_api::v1::SetGovernedAssetsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::authority_api::v1::SetGovernedAssetsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::authority_api::v1::SetGovernedAssetsResponse>* __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_new() {
    return new google::protobuf::Map<uint32_t, ::authority_api::v1::SetGovernedAssetsResponse>();
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_free(const google::protobuf::Map<uint32_t, ::authority_api::v1::SetGovernedAssetsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_clear(google::protobuf::Map<uint32_t, ::authority_api::v1::SetGovernedAssetsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_size(const google::protobuf::Map<uint32_t, ::authority_api::v1::SetGovernedAssetsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_insert(google::protobuf::Map<uint32_t, ::authority_api::v1::SetGovernedAssetsResponse> * m,
                          uint32_t key, ::authority_api::v1::SetGovernedAssetsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_get(const google::protobuf::Map<uint32_t, ::authority_api::v1::SetGovernedAssetsResponse>* m,
                       uint32_t key, const ::authority_api::v1::SetGovernedAssetsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetGovernedAssetsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_remove(google::protobuf::Map<uint32_t, ::authority_api::v1::SetGovernedAssetsResponse> * m,
                          uint32_t key, ::authority_api::v1::SetGovernedAssetsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_iter(
      const google::protobuf::Map<uint32_t, ::authority_api::v1::SetGovernedAssetsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_SetGovernedAssetsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::authority_api::v1::SetGovernedAssetsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::authority_api::v1::SetGovernedAssetsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::authority_api::v1::SetGovernedAssetsResponse>* __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_new() {
    return new google::protobuf::Map<int64_t, ::authority_api::v1::SetGovernedAssetsResponse>();
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_free(const google::protobuf::Map<int64_t, ::authority_api::v1::SetGovernedAssetsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_clear(google::protobuf::Map<int64_t, ::authority_api::v1::SetGovernedAssetsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_size(const google::protobuf::Map<int64_t, ::authority_api::v1::SetGovernedAssetsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_insert(google::protobuf::Map<int64_t, ::authority_api::v1::SetGovernedAssetsResponse> * m,
                          int64_t key, ::authority_api::v1::SetGovernedAssetsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_get(const google::protobuf::Map<int64_t, ::authority_api::v1::SetGovernedAssetsResponse>* m,
                       int64_t key, const ::authority_api::v1::SetGovernedAssetsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetGovernedAssetsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_remove(google::protobuf::Map<int64_t, ::authority_api::v1::SetGovernedAssetsResponse> * m,
                          int64_t key, ::authority_api::v1::SetGovernedAssetsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_iter(
      const google::protobuf::Map<int64_t, ::authority_api::v1::SetGovernedAssetsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_SetGovernedAssetsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::authority_api::v1::SetGovernedAssetsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::authority_api::v1::SetGovernedAssetsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::authority_api::v1::SetGovernedAssetsResponse>* __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_new() {
    return new google::protobuf::Map<uint64_t, ::authority_api::v1::SetGovernedAssetsResponse>();
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_free(const google::protobuf::Map<uint64_t, ::authority_api::v1::SetGovernedAssetsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_clear(google::protobuf::Map<uint64_t, ::authority_api::v1::SetGovernedAssetsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_size(const google::protobuf::Map<uint64_t, ::authority_api::v1::SetGovernedAssetsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_insert(google::protobuf::Map<uint64_t, ::authority_api::v1::SetGovernedAssetsResponse> * m,
                          uint64_t key, ::authority_api::v1::SetGovernedAssetsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_get(const google::protobuf::Map<uint64_t, ::authority_api::v1::SetGovernedAssetsResponse>* m,
                       uint64_t key, const ::authority_api::v1::SetGovernedAssetsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetGovernedAssetsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_remove(google::protobuf::Map<uint64_t, ::authority_api::v1::SetGovernedAssetsResponse> * m,
                          uint64_t key, ::authority_api::v1::SetGovernedAssetsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_iter(
      const google::protobuf::Map<uint64_t, ::authority_api::v1::SetGovernedAssetsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_SetGovernedAssetsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::authority_api::v1::SetGovernedAssetsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::authority_api::v1::SetGovernedAssetsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::authority_api::v1::SetGovernedAssetsResponse>* __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_new() {
    return new google::protobuf::Map<bool, ::authority_api::v1::SetGovernedAssetsResponse>();
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_free(const google::protobuf::Map<bool, ::authority_api::v1::SetGovernedAssetsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_clear(google::protobuf::Map<bool, ::authority_api::v1::SetGovernedAssetsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_size(const google::protobuf::Map<bool, ::authority_api::v1::SetGovernedAssetsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_insert(google::protobuf::Map<bool, ::authority_api::v1::SetGovernedAssetsResponse> * m,
                          bool key, ::authority_api::v1::SetGovernedAssetsResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_get(const google::protobuf::Map<bool, ::authority_api::v1::SetGovernedAssetsResponse>* m,
                       bool key, const ::authority_api::v1::SetGovernedAssetsResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetGovernedAssetsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_remove(google::protobuf::Map<bool, ::authority_api::v1::SetGovernedAssetsResponse> * m,
                          bool key, ::authority_api::v1::SetGovernedAssetsResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_iter(
      const google::protobuf::Map<bool, ::authority_api::v1::SetGovernedAssetsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_SetGovernedAssetsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::authority_api::v1::SetGovernedAssetsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::authority_api::v1::SetGovernedAssetsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::authority_api::v1::SetGovernedAssetsResponse>* __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_new() {
    return new google::protobuf::Map<std::string, ::authority_api::v1::SetGovernedAssetsResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_free(const google::protobuf::Map<std::string, ::authority_api::v1::SetGovernedAssetsResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_clear(google::protobuf::Map<std::string, ::authority_api::v1::SetGovernedAssetsResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_size(const google::protobuf::Map<std::string, ::authority_api::v1::SetGovernedAssetsResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_insert(google::protobuf::Map<std::string, ::authority_api::v1::SetGovernedAssetsResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::SetGovernedAssetsResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_get(const google::protobuf::Map<std::string, ::authority_api::v1::SetGovernedAssetsResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::authority_api::v1::SetGovernedAssetsResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::SetGovernedAssetsResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_remove(google::protobuf::Map<std::string, ::authority_api::v1::SetGovernedAssetsResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::SetGovernedAssetsResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_iter(
      const google::protobuf::Map<std::string, ::authority_api::v1::SetGovernedAssetsResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_SetGovernedAssetsResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::authority_api::v1::SetGovernedAssetsResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::authority_api::v1::SetGovernedAssetsResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// authority_api.v1.AddAssetRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__authority_api_v1_AddAssetRequest_new() { return new ::authority_api::v1::AddAssetRequest(); }
void __rust_proto_thunk__authority_api_v1_AddAssetRequest_delete(void* ptr) { delete static_cast<::authority_api::v1::AddAssetRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__authority_api_v1_AddAssetRequest_serialize(::authority_api::v1::AddAssetRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__authority_api_v1_AddAssetRequest_parse(::authority_api::v1::AddAssetRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__authority_api_v1_AddAssetRequest_copy_from(::authority_api::v1::AddAssetRequest* dst, const ::authority_api::v1::AddAssetRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__authority_api_v1_AddAssetRequest_repeated_len(google::protobuf::RepeatedPtrField<::authority_api::v1::AddAssetRequest>* field) {
  return field->size();
}
const ::authority_api::v1::AddAssetRequest& __rust_proto_thunk__authority_api_v1_AddAssetRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::authority_api::v1::AddAssetRequest>* field,
  size_t index) {
  return field->Get(index);
}
::authority_api::v1::AddAssetRequest* __rust_proto_thunk__authority_api_v1_AddAssetRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::authority_api::v1::AddAssetRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::authority_api::v1::AddAssetRequest* __rust_proto_thunk__authority_api_v1_AddAssetRequest_repeated_add(google::protobuf::RepeatedPtrField<::authority_api::v1::AddAssetRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__authority_api_v1_AddAssetRequest_repeated_clear(google::protobuf::RepeatedPtrField<::authority_api::v1::AddAssetRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__authority_api_v1_AddAssetRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::authority_api::v1::AddAssetRequest>& dst,
  const google::protobuf::RepeatedPtrField<::authority_api::v1::AddAssetRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__authority_api_v1_AddAssetRequest_get_currency_code(::authority_api::v1::AddAssetRequest* msg) {
  absl::string_view val = msg->currency_code();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__authority_api_v1_AddAssetRequest_set_currency_code(::authority_api::v1::AddAssetRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_currency_code(absl::string_view(s.ptr, s.len));
}

const void* __rust_proto_thunk__authority_api_v1_AddAssetRequest_get_asset_denomination(::authority_api::v1::AddAssetRequest* msg) {
  return static_cast<const void*>(&msg->asset_denomination());
}
void* __rust_proto_thunk__authority_api_v1_AddAssetRequest_get_mut_asset_denomination(::authority_api::v1::AddAssetRequest* msg) {
  return static_cast<void*>(msg->mutable_asset_denomination());
}
void __rust_proto_thunk__authority_api_v1_AddAssetRequest_clear_asset_denomination(::authority_api::v1::AddAssetRequest* msg) { msg->clear_asset_denomination(); }
bool __rust_proto_thunk__authority_api_v1_AddAssetRequest_has_asset_denomination(::authority_api::v1::AddAssetRequest* msg) { return msg->has_asset_denomination(); }


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::authority_api::v1::AddAssetRequest>* __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_new() {
    return new google::protobuf::Map<int32_t, ::authority_api::v1::AddAssetRequest>();
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_free(const google::protobuf::Map<int32_t, ::authority_api::v1::AddAssetRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_clear(google::protobuf::Map<int32_t, ::authority_api::v1::AddAssetRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_size(const google::protobuf::Map<int32_t, ::authority_api::v1::AddAssetRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_insert(google::protobuf::Map<int32_t, ::authority_api::v1::AddAssetRequest> * m,
                          int32_t key, ::authority_api::v1::AddAssetRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_get(const google::protobuf::Map<int32_t, ::authority_api::v1::AddAssetRequest>* m,
                       int32_t key, const ::authority_api::v1::AddAssetRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AddAssetRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_remove(google::protobuf::Map<int32_t, ::authority_api::v1::AddAssetRequest> * m,
                          int32_t key, ::authority_api::v1::AddAssetRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_iter(
      const google::protobuf::Map<int32_t, ::authority_api::v1::AddAssetRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::authority_api::v1::AddAssetRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::authority_api::v1::AddAssetRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::authority_api::v1::AddAssetRequest>* __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_new() {
    return new google::protobuf::Map<uint32_t, ::authority_api::v1::AddAssetRequest>();
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_free(const google::protobuf::Map<uint32_t, ::authority_api::v1::AddAssetRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_clear(google::protobuf::Map<uint32_t, ::authority_api::v1::AddAssetRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_size(const google::protobuf::Map<uint32_t, ::authority_api::v1::AddAssetRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_insert(google::protobuf::Map<uint32_t, ::authority_api::v1::AddAssetRequest> * m,
                          uint32_t key, ::authority_api::v1::AddAssetRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_get(const google::protobuf::Map<uint32_t, ::authority_api::v1::AddAssetRequest>* m,
                       uint32_t key, const ::authority_api::v1::AddAssetRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AddAssetRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_remove(google::protobuf::Map<uint32_t, ::authority_api::v1::AddAssetRequest> * m,
                          uint32_t key, ::authority_api::v1::AddAssetRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_iter(
      const google::protobuf::Map<uint32_t, ::authority_api::v1::AddAssetRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::authority_api::v1::AddAssetRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::authority_api::v1::AddAssetRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::authority_api::v1::AddAssetRequest>* __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_new() {
    return new google::protobuf::Map<int64_t, ::authority_api::v1::AddAssetRequest>();
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_free(const google::protobuf::Map<int64_t, ::authority_api::v1::AddAssetRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_clear(google::protobuf::Map<int64_t, ::authority_api::v1::AddAssetRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_size(const google::protobuf::Map<int64_t, ::authority_api::v1::AddAssetRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_insert(google::protobuf::Map<int64_t, ::authority_api::v1::AddAssetRequest> * m,
                          int64_t key, ::authority_api::v1::AddAssetRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_get(const google::protobuf::Map<int64_t, ::authority_api::v1::AddAssetRequest>* m,
                       int64_t key, const ::authority_api::v1::AddAssetRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AddAssetRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_remove(google::protobuf::Map<int64_t, ::authority_api::v1::AddAssetRequest> * m,
                          int64_t key, ::authority_api::v1::AddAssetRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_iter(
      const google::protobuf::Map<int64_t, ::authority_api::v1::AddAssetRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::authority_api::v1::AddAssetRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::authority_api::v1::AddAssetRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::authority_api::v1::AddAssetRequest>* __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_new() {
    return new google::protobuf::Map<uint64_t, ::authority_api::v1::AddAssetRequest>();
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_free(const google::protobuf::Map<uint64_t, ::authority_api::v1::AddAssetRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_clear(google::protobuf::Map<uint64_t, ::authority_api::v1::AddAssetRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_size(const google::protobuf::Map<uint64_t, ::authority_api::v1::AddAssetRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_insert(google::protobuf::Map<uint64_t, ::authority_api::v1::AddAssetRequest> * m,
                          uint64_t key, ::authority_api::v1::AddAssetRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_get(const google::protobuf::Map<uint64_t, ::authority_api::v1::AddAssetRequest>* m,
                       uint64_t key, const ::authority_api::v1::AddAssetRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AddAssetRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_remove(google::protobuf::Map<uint64_t, ::authority_api::v1::AddAssetRequest> * m,
                          uint64_t key, ::authority_api::v1::AddAssetRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_iter(
      const google::protobuf::Map<uint64_t, ::authority_api::v1::AddAssetRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::authority_api::v1::AddAssetRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::authority_api::v1::AddAssetRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::authority_api::v1::AddAssetRequest>* __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_new() {
    return new google::protobuf::Map<bool, ::authority_api::v1::AddAssetRequest>();
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_free(const google::protobuf::Map<bool, ::authority_api::v1::AddAssetRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_clear(google::protobuf::Map<bool, ::authority_api::v1::AddAssetRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_size(const google::protobuf::Map<bool, ::authority_api::v1::AddAssetRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_insert(google::protobuf::Map<bool, ::authority_api::v1::AddAssetRequest> * m,
                          bool key, ::authority_api::v1::AddAssetRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_get(const google::protobuf::Map<bool, ::authority_api::v1::AddAssetRequest>* m,
                       bool key, const ::authority_api::v1::AddAssetRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AddAssetRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_remove(google::protobuf::Map<bool, ::authority_api::v1::AddAssetRequest> * m,
                          bool key, ::authority_api::v1::AddAssetRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_iter(
      const google::protobuf::Map<bool, ::authority_api::v1::AddAssetRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::authority_api::v1::AddAssetRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::authority_api::v1::AddAssetRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::authority_api::v1::AddAssetRequest>* __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_new() {
    return new google::protobuf::Map<std::string, ::authority_api::v1::AddAssetRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_free(const google::protobuf::Map<std::string, ::authority_api::v1::AddAssetRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_clear(google::protobuf::Map<std::string, ::authority_api::v1::AddAssetRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_size(const google::protobuf::Map<std::string, ::authority_api::v1::AddAssetRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_insert(google::protobuf::Map<std::string, ::authority_api::v1::AddAssetRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::AddAssetRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_get(const google::protobuf::Map<std::string, ::authority_api::v1::AddAssetRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::authority_api::v1::AddAssetRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AddAssetRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_remove(google::protobuf::Map<std::string, ::authority_api::v1::AddAssetRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::AddAssetRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_iter(
      const google::protobuf::Map<std::string, ::authority_api::v1::AddAssetRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::authority_api::v1::AddAssetRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::authority_api::v1::AddAssetRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// authority_api.v1.AddAssetResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__authority_api_v1_AddAssetResponse_new() { return new ::authority_api::v1::AddAssetResponse(); }
void __rust_proto_thunk__authority_api_v1_AddAssetResponse_delete(void* ptr) { delete static_cast<::authority_api::v1::AddAssetResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__authority_api_v1_AddAssetResponse_serialize(::authority_api::v1::AddAssetResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__authority_api_v1_AddAssetResponse_parse(::authority_api::v1::AddAssetResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__authority_api_v1_AddAssetResponse_copy_from(::authority_api::v1::AddAssetResponse* dst, const ::authority_api::v1::AddAssetResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__authority_api_v1_AddAssetResponse_repeated_len(google::protobuf::RepeatedPtrField<::authority_api::v1::AddAssetResponse>* field) {
  return field->size();
}
const ::authority_api::v1::AddAssetResponse& __rust_proto_thunk__authority_api_v1_AddAssetResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::authority_api::v1::AddAssetResponse>* field,
  size_t index) {
  return field->Get(index);
}
::authority_api::v1::AddAssetResponse* __rust_proto_thunk__authority_api_v1_AddAssetResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::authority_api::v1::AddAssetResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::authority_api::v1::AddAssetResponse* __rust_proto_thunk__authority_api_v1_AddAssetResponse_repeated_add(google::protobuf::RepeatedPtrField<::authority_api::v1::AddAssetResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__authority_api_v1_AddAssetResponse_repeated_clear(google::protobuf::RepeatedPtrField<::authority_api::v1::AddAssetResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__authority_api_v1_AddAssetResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::authority_api::v1::AddAssetResponse>& dst,
  const google::protobuf::RepeatedPtrField<::authority_api::v1::AddAssetResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::authority_api::v1::AddAssetResponse>* __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_new() {
    return new google::protobuf::Map<int32_t, ::authority_api::v1::AddAssetResponse>();
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_free(const google::protobuf::Map<int32_t, ::authority_api::v1::AddAssetResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_clear(google::protobuf::Map<int32_t, ::authority_api::v1::AddAssetResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_size(const google::protobuf::Map<int32_t, ::authority_api::v1::AddAssetResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_insert(google::protobuf::Map<int32_t, ::authority_api::v1::AddAssetResponse> * m,
                          int32_t key, ::authority_api::v1::AddAssetResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_get(const google::protobuf::Map<int32_t, ::authority_api::v1::AddAssetResponse>* m,
                       int32_t key, const ::authority_api::v1::AddAssetResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AddAssetResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_remove(google::protobuf::Map<int32_t, ::authority_api::v1::AddAssetResponse> * m,
                          int32_t key, ::authority_api::v1::AddAssetResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_iter(
      const google::protobuf::Map<int32_t, ::authority_api::v1::AddAssetResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_AddAssetResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::authority_api::v1::AddAssetResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::authority_api::v1::AddAssetResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::authority_api::v1::AddAssetResponse>* __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_new() {
    return new google::protobuf::Map<uint32_t, ::authority_api::v1::AddAssetResponse>();
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_free(const google::protobuf::Map<uint32_t, ::authority_api::v1::AddAssetResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_clear(google::protobuf::Map<uint32_t, ::authority_api::v1::AddAssetResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_size(const google::protobuf::Map<uint32_t, ::authority_api::v1::AddAssetResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_insert(google::protobuf::Map<uint32_t, ::authority_api::v1::AddAssetResponse> * m,
                          uint32_t key, ::authority_api::v1::AddAssetResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_get(const google::protobuf::Map<uint32_t, ::authority_api::v1::AddAssetResponse>* m,
                       uint32_t key, const ::authority_api::v1::AddAssetResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AddAssetResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_remove(google::protobuf::Map<uint32_t, ::authority_api::v1::AddAssetResponse> * m,
                          uint32_t key, ::authority_api::v1::AddAssetResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_iter(
      const google::protobuf::Map<uint32_t, ::authority_api::v1::AddAssetResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_AddAssetResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::authority_api::v1::AddAssetResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::authority_api::v1::AddAssetResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::authority_api::v1::AddAssetResponse>* __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_new() {
    return new google::protobuf::Map<int64_t, ::authority_api::v1::AddAssetResponse>();
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_free(const google::protobuf::Map<int64_t, ::authority_api::v1::AddAssetResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_clear(google::protobuf::Map<int64_t, ::authority_api::v1::AddAssetResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_size(const google::protobuf::Map<int64_t, ::authority_api::v1::AddAssetResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_insert(google::protobuf::Map<int64_t, ::authority_api::v1::AddAssetResponse> * m,
                          int64_t key, ::authority_api::v1::AddAssetResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_get(const google::protobuf::Map<int64_t, ::authority_api::v1::AddAssetResponse>* m,
                       int64_t key, const ::authority_api::v1::AddAssetResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AddAssetResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_remove(google::protobuf::Map<int64_t, ::authority_api::v1::AddAssetResponse> * m,
                          int64_t key, ::authority_api::v1::AddAssetResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_iter(
      const google::protobuf::Map<int64_t, ::authority_api::v1::AddAssetResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_AddAssetResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::authority_api::v1::AddAssetResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::authority_api::v1::AddAssetResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::authority_api::v1::AddAssetResponse>* __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_new() {
    return new google::protobuf::Map<uint64_t, ::authority_api::v1::AddAssetResponse>();
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_free(const google::protobuf::Map<uint64_t, ::authority_api::v1::AddAssetResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_clear(google::protobuf::Map<uint64_t, ::authority_api::v1::AddAssetResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_size(const google::protobuf::Map<uint64_t, ::authority_api::v1::AddAssetResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_insert(google::protobuf::Map<uint64_t, ::authority_api::v1::AddAssetResponse> * m,
                          uint64_t key, ::authority_api::v1::AddAssetResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_get(const google::protobuf::Map<uint64_t, ::authority_api::v1::AddAssetResponse>* m,
                       uint64_t key, const ::authority_api::v1::AddAssetResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AddAssetResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_remove(google::protobuf::Map<uint64_t, ::authority_api::v1::AddAssetResponse> * m,
                          uint64_t key, ::authority_api::v1::AddAssetResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_iter(
      const google::protobuf::Map<uint64_t, ::authority_api::v1::AddAssetResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_AddAssetResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::authority_api::v1::AddAssetResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::authority_api::v1::AddAssetResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::authority_api::v1::AddAssetResponse>* __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_new() {
    return new google::protobuf::Map<bool, ::authority_api::v1::AddAssetResponse>();
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_free(const google::protobuf::Map<bool, ::authority_api::v1::AddAssetResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_clear(google::protobuf::Map<bool, ::authority_api::v1::AddAssetResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_size(const google::protobuf::Map<bool, ::authority_api::v1::AddAssetResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_insert(google::protobuf::Map<bool, ::authority_api::v1::AddAssetResponse> * m,
                          bool key, ::authority_api::v1::AddAssetResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_get(const google::protobuf::Map<bool, ::authority_api::v1::AddAssetResponse>* m,
                       bool key, const ::authority_api::v1::AddAssetResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AddAssetResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_remove(google::protobuf::Map<bool, ::authority_api::v1::AddAssetResponse> * m,
                          bool key, ::authority_api::v1::AddAssetResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_iter(
      const google::protobuf::Map<bool, ::authority_api::v1::AddAssetResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_AddAssetResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::authority_api::v1::AddAssetResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::authority_api::v1::AddAssetResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::authority_api::v1::AddAssetResponse>* __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_new() {
    return new google::protobuf::Map<std::string, ::authority_api::v1::AddAssetResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_free(const google::protobuf::Map<std::string, ::authority_api::v1::AddAssetResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_clear(google::protobuf::Map<std::string, ::authority_api::v1::AddAssetResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_size(const google::protobuf::Map<std::string, ::authority_api::v1::AddAssetResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_insert(google::protobuf::Map<std::string, ::authority_api::v1::AddAssetResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::AddAssetResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_get(const google::protobuf::Map<std::string, ::authority_api::v1::AddAssetResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::authority_api::v1::AddAssetResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::AddAssetResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_remove(google::protobuf::Map<std::string, ::authority_api::v1::AddAssetResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::AddAssetResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_iter(
      const google::protobuf::Map<std::string, ::authority_api::v1::AddAssetResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_AddAssetResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::authority_api::v1::AddAssetResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::authority_api::v1::AddAssetResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// authority_api.v1.RemoveAssetRequest
        // clang-format off
extern "C" {
void* __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_new() { return new ::authority_api::v1::RemoveAssetRequest(); }
void __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_delete(void* ptr) { delete static_cast<::authority_api::v1::RemoveAssetRequest*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_serialize(::authority_api::v1::RemoveAssetRequest* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_parse(::authority_api::v1::RemoveAssetRequest* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_copy_from(::authority_api::v1::RemoveAssetRequest* dst, const ::authority_api::v1::RemoveAssetRequest* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_repeated_len(google::protobuf::RepeatedPtrField<::authority_api::v1::RemoveAssetRequest>* field) {
  return field->size();
}
const ::authority_api::v1::RemoveAssetRequest& __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_repeated_get(
  google::protobuf::RepeatedPtrField<::authority_api::v1::RemoveAssetRequest>* field,
  size_t index) {
  return field->Get(index);
}
::authority_api::v1::RemoveAssetRequest* __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::authority_api::v1::RemoveAssetRequest>* field,
  size_t index) {
  return field->Mutable(index);
}
::authority_api::v1::RemoveAssetRequest* __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_repeated_add(google::protobuf::RepeatedPtrField<::authority_api::v1::RemoveAssetRequest>* field) {
  return field->Add();
}
void __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_repeated_clear(google::protobuf::RepeatedPtrField<::authority_api::v1::RemoveAssetRequest>* field) {
  field->Clear();
}
void __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::authority_api::v1::RemoveAssetRequest>& dst,
  const google::protobuf::RepeatedPtrField<::authority_api::v1::RemoveAssetRequest>& src) {
  dst = src;
}

::google::protobuf::rust_internal::PtrAndLen __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_get_currency_code(::authority_api::v1::RemoveAssetRequest* msg) {
  absl::string_view val = msg->currency_code();
  return ::google::protobuf::rust_internal::PtrAndLen(val.data(), val.size());
}
void __rust_proto_thunk__authority_api_v1_RemoveAssetRequest_set_currency_code(::authority_api::v1::RemoveAssetRequest* msg, ::google::protobuf::rust_internal::PtrAndLen s) {
  msg->set_currency_code(absl::string_view(s.ptr, s.len));
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::authority_api::v1::RemoveAssetRequest>* __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_new() {
    return new google::protobuf::Map<int32_t, ::authority_api::v1::RemoveAssetRequest>();
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_free(const google::protobuf::Map<int32_t, ::authority_api::v1::RemoveAssetRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_clear(google::protobuf::Map<int32_t, ::authority_api::v1::RemoveAssetRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_size(const google::protobuf::Map<int32_t, ::authority_api::v1::RemoveAssetRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_insert(google::protobuf::Map<int32_t, ::authority_api::v1::RemoveAssetRequest> * m,
                          int32_t key, ::authority_api::v1::RemoveAssetRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_get(const google::protobuf::Map<int32_t, ::authority_api::v1::RemoveAssetRequest>* m,
                       int32_t key, const ::authority_api::v1::RemoveAssetRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::RemoveAssetRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_remove(google::protobuf::Map<int32_t, ::authority_api::v1::RemoveAssetRequest> * m,
                          int32_t key, ::authority_api::v1::RemoveAssetRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_iter(
      const google::protobuf::Map<int32_t, ::authority_api::v1::RemoveAssetRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::authority_api::v1::RemoveAssetRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::authority_api::v1::RemoveAssetRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::authority_api::v1::RemoveAssetRequest>* __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_new() {
    return new google::protobuf::Map<uint32_t, ::authority_api::v1::RemoveAssetRequest>();
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_free(const google::protobuf::Map<uint32_t, ::authority_api::v1::RemoveAssetRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_clear(google::protobuf::Map<uint32_t, ::authority_api::v1::RemoveAssetRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_size(const google::protobuf::Map<uint32_t, ::authority_api::v1::RemoveAssetRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_insert(google::protobuf::Map<uint32_t, ::authority_api::v1::RemoveAssetRequest> * m,
                          uint32_t key, ::authority_api::v1::RemoveAssetRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_get(const google::protobuf::Map<uint32_t, ::authority_api::v1::RemoveAssetRequest>* m,
                       uint32_t key, const ::authority_api::v1::RemoveAssetRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::RemoveAssetRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_remove(google::protobuf::Map<uint32_t, ::authority_api::v1::RemoveAssetRequest> * m,
                          uint32_t key, ::authority_api::v1::RemoveAssetRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_iter(
      const google::protobuf::Map<uint32_t, ::authority_api::v1::RemoveAssetRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::authority_api::v1::RemoveAssetRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::authority_api::v1::RemoveAssetRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::authority_api::v1::RemoveAssetRequest>* __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_new() {
    return new google::protobuf::Map<int64_t, ::authority_api::v1::RemoveAssetRequest>();
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_free(const google::protobuf::Map<int64_t, ::authority_api::v1::RemoveAssetRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_clear(google::protobuf::Map<int64_t, ::authority_api::v1::RemoveAssetRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_size(const google::protobuf::Map<int64_t, ::authority_api::v1::RemoveAssetRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_insert(google::protobuf::Map<int64_t, ::authority_api::v1::RemoveAssetRequest> * m,
                          int64_t key, ::authority_api::v1::RemoveAssetRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_get(const google::protobuf::Map<int64_t, ::authority_api::v1::RemoveAssetRequest>* m,
                       int64_t key, const ::authority_api::v1::RemoveAssetRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::RemoveAssetRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_remove(google::protobuf::Map<int64_t, ::authority_api::v1::RemoveAssetRequest> * m,
                          int64_t key, ::authority_api::v1::RemoveAssetRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_iter(
      const google::protobuf::Map<int64_t, ::authority_api::v1::RemoveAssetRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::authority_api::v1::RemoveAssetRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::authority_api::v1::RemoveAssetRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::authority_api::v1::RemoveAssetRequest>* __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_new() {
    return new google::protobuf::Map<uint64_t, ::authority_api::v1::RemoveAssetRequest>();
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_free(const google::protobuf::Map<uint64_t, ::authority_api::v1::RemoveAssetRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_clear(google::protobuf::Map<uint64_t, ::authority_api::v1::RemoveAssetRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_size(const google::protobuf::Map<uint64_t, ::authority_api::v1::RemoveAssetRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_insert(google::protobuf::Map<uint64_t, ::authority_api::v1::RemoveAssetRequest> * m,
                          uint64_t key, ::authority_api::v1::RemoveAssetRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_get(const google::protobuf::Map<uint64_t, ::authority_api::v1::RemoveAssetRequest>* m,
                       uint64_t key, const ::authority_api::v1::RemoveAssetRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::RemoveAssetRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_remove(google::protobuf::Map<uint64_t, ::authority_api::v1::RemoveAssetRequest> * m,
                          uint64_t key, ::authority_api::v1::RemoveAssetRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_iter(
      const google::protobuf::Map<uint64_t, ::authority_api::v1::RemoveAssetRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::authority_api::v1::RemoveAssetRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::authority_api::v1::RemoveAssetRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::authority_api::v1::RemoveAssetRequest>* __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_new() {
    return new google::protobuf::Map<bool, ::authority_api::v1::RemoveAssetRequest>();
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_free(const google::protobuf::Map<bool, ::authority_api::v1::RemoveAssetRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_clear(google::protobuf::Map<bool, ::authority_api::v1::RemoveAssetRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_size(const google::protobuf::Map<bool, ::authority_api::v1::RemoveAssetRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_insert(google::protobuf::Map<bool, ::authority_api::v1::RemoveAssetRequest> * m,
                          bool key, ::authority_api::v1::RemoveAssetRequest value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_get(const google::protobuf::Map<bool, ::authority_api::v1::RemoveAssetRequest>* m,
                       bool key, const ::authority_api::v1::RemoveAssetRequest** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::RemoveAssetRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_remove(google::protobuf::Map<bool, ::authority_api::v1::RemoveAssetRequest> * m,
                          bool key, ::authority_api::v1::RemoveAssetRequest * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_iter(
      const google::protobuf::Map<bool, ::authority_api::v1::RemoveAssetRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::authority_api::v1::RemoveAssetRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::authority_api::v1::RemoveAssetRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::authority_api::v1::RemoveAssetRequest>* __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_new() {
    return new google::protobuf::Map<std::string, ::authority_api::v1::RemoveAssetRequest>();
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_free(const google::protobuf::Map<std::string, ::authority_api::v1::RemoveAssetRequest>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_clear(google::protobuf::Map<std::string, ::authority_api::v1::RemoveAssetRequest> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_size(const google::protobuf::Map<std::string, ::authority_api::v1::RemoveAssetRequest>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_insert(google::protobuf::Map<std::string, ::authority_api::v1::RemoveAssetRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::RemoveAssetRequest value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_get(const google::protobuf::Map<std::string, ::authority_api::v1::RemoveAssetRequest>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::authority_api::v1::RemoveAssetRequest** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::RemoveAssetRequest* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_remove(google::protobuf::Map<std::string, ::authority_api::v1::RemoveAssetRequest> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::RemoveAssetRequest * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_iter(
      const google::protobuf::Map<std::string, ::authority_api::v1::RemoveAssetRequest>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetRequest_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::authority_api::v1::RemoveAssetRequest** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::authority_api::v1::RemoveAssetRequest>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

// authority_api.v1.RemoveAssetResponse
        // clang-format off
extern "C" {
void* __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_new() { return new ::authority_api::v1::RemoveAssetResponse(); }
void __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_delete(void* ptr) { delete static_cast<::authority_api::v1::RemoveAssetResponse*>(ptr); }
google::protobuf::rust_internal::SerializedData __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_serialize(::authority_api::v1::RemoveAssetResponse* msg) {
  return google::protobuf::rust_internal::SerializeMsg(msg);
}
bool __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_parse(::authority_api::v1::RemoveAssetResponse* msg,
                         google::protobuf::rust_internal::SerializedData data) {
  return msg->ParseFromArray(data.data, data.len);
}

void __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_copy_from(::authority_api::v1::RemoveAssetResponse* dst, const ::authority_api::v1::RemoveAssetResponse* src) {
  dst->CopyFrom(*src);
}

size_t __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_repeated_len(google::protobuf::RepeatedPtrField<::authority_api::v1::RemoveAssetResponse>* field) {
  return field->size();
}
const ::authority_api::v1::RemoveAssetResponse& __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_repeated_get(
  google::protobuf::RepeatedPtrField<::authority_api::v1::RemoveAssetResponse>* field,
  size_t index) {
  return field->Get(index);
}
::authority_api::v1::RemoveAssetResponse* __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_repeated_get_mut(
  google::protobuf::RepeatedPtrField<::authority_api::v1::RemoveAssetResponse>* field,
  size_t index) {
  return field->Mutable(index);
}
::authority_api::v1::RemoveAssetResponse* __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_repeated_add(google::protobuf::RepeatedPtrField<::authority_api::v1::RemoveAssetResponse>* field) {
  return field->Add();
}
void __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_repeated_clear(google::protobuf::RepeatedPtrField<::authority_api::v1::RemoveAssetResponse>* field) {
  field->Clear();
}
void __rust_proto_thunk__authority_api_v1_RemoveAssetResponse_repeated_copy_from(
  google::protobuf::RepeatedPtrField<::authority_api::v1::RemoveAssetResponse>& dst,
  const google::protobuf::RepeatedPtrField<::authority_api::v1::RemoveAssetResponse>& src) {
  dst = src;
}


}  // extern "C"
// clang-format on

extern "C" {
  const google::protobuf::Map<int32_t, ::authority_api::v1::RemoveAssetResponse>* __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_new() {
    return new google::protobuf::Map<int32_t, ::authority_api::v1::RemoveAssetResponse>();
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_free(const google::protobuf::Map<int32_t, ::authority_api::v1::RemoveAssetResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_clear(google::protobuf::Map<int32_t, ::authority_api::v1::RemoveAssetResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_size(const google::protobuf::Map<int32_t, ::authority_api::v1::RemoveAssetResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_insert(google::protobuf::Map<int32_t, ::authority_api::v1::RemoveAssetResponse> * m,
                          int32_t key, ::authority_api::v1::RemoveAssetResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_get(const google::protobuf::Map<int32_t, ::authority_api::v1::RemoveAssetResponse>* m,
                       int32_t key, const ::authority_api::v1::RemoveAssetResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::RemoveAssetResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_remove(google::protobuf::Map<int32_t, ::authority_api::v1::RemoveAssetResponse> * m,
                          int32_t key, ::authority_api::v1::RemoveAssetResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_iter(
      const google::protobuf::Map<int32_t, ::authority_api::v1::RemoveAssetResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i32_authority_api_v1_RemoveAssetResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int32_t* key, const ::authority_api::v1::RemoveAssetResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int32_t, ::authority_api::v1::RemoveAssetResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint32_t, ::authority_api::v1::RemoveAssetResponse>* __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_new() {
    return new google::protobuf::Map<uint32_t, ::authority_api::v1::RemoveAssetResponse>();
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_free(const google::protobuf::Map<uint32_t, ::authority_api::v1::RemoveAssetResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_clear(google::protobuf::Map<uint32_t, ::authority_api::v1::RemoveAssetResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_size(const google::protobuf::Map<uint32_t, ::authority_api::v1::RemoveAssetResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_insert(google::protobuf::Map<uint32_t, ::authority_api::v1::RemoveAssetResponse> * m,
                          uint32_t key, ::authority_api::v1::RemoveAssetResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_get(const google::protobuf::Map<uint32_t, ::authority_api::v1::RemoveAssetResponse>* m,
                       uint32_t key, const ::authority_api::v1::RemoveAssetResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::RemoveAssetResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_remove(google::protobuf::Map<uint32_t, ::authority_api::v1::RemoveAssetResponse> * m,
                          uint32_t key, ::authority_api::v1::RemoveAssetResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_iter(
      const google::protobuf::Map<uint32_t, ::authority_api::v1::RemoveAssetResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u32_authority_api_v1_RemoveAssetResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint32_t* key, const ::authority_api::v1::RemoveAssetResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint32_t, ::authority_api::v1::RemoveAssetResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<int64_t, ::authority_api::v1::RemoveAssetResponse>* __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_new() {
    return new google::protobuf::Map<int64_t, ::authority_api::v1::RemoveAssetResponse>();
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_free(const google::protobuf::Map<int64_t, ::authority_api::v1::RemoveAssetResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_clear(google::protobuf::Map<int64_t, ::authority_api::v1::RemoveAssetResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_size(const google::protobuf::Map<int64_t, ::authority_api::v1::RemoveAssetResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_insert(google::protobuf::Map<int64_t, ::authority_api::v1::RemoveAssetResponse> * m,
                          int64_t key, ::authority_api::v1::RemoveAssetResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_get(const google::protobuf::Map<int64_t, ::authority_api::v1::RemoveAssetResponse>* m,
                       int64_t key, const ::authority_api::v1::RemoveAssetResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::RemoveAssetResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_remove(google::protobuf::Map<int64_t, ::authority_api::v1::RemoveAssetResponse> * m,
                          int64_t key, ::authority_api::v1::RemoveAssetResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_iter(
      const google::protobuf::Map<int64_t, ::authority_api::v1::RemoveAssetResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_i64_authority_api_v1_RemoveAssetResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      int64_t* key, const ::authority_api::v1::RemoveAssetResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<int64_t, ::authority_api::v1::RemoveAssetResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<uint64_t, ::authority_api::v1::RemoveAssetResponse>* __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_new() {
    return new google::protobuf::Map<uint64_t, ::authority_api::v1::RemoveAssetResponse>();
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_free(const google::protobuf::Map<uint64_t, ::authority_api::v1::RemoveAssetResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_clear(google::protobuf::Map<uint64_t, ::authority_api::v1::RemoveAssetResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_size(const google::protobuf::Map<uint64_t, ::authority_api::v1::RemoveAssetResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_insert(google::protobuf::Map<uint64_t, ::authority_api::v1::RemoveAssetResponse> * m,
                          uint64_t key, ::authority_api::v1::RemoveAssetResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_get(const google::protobuf::Map<uint64_t, ::authority_api::v1::RemoveAssetResponse>* m,
                       uint64_t key, const ::authority_api::v1::RemoveAssetResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::RemoveAssetResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_remove(google::protobuf::Map<uint64_t, ::authority_api::v1::RemoveAssetResponse> * m,
                          uint64_t key, ::authority_api::v1::RemoveAssetResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_iter(
      const google::protobuf::Map<uint64_t, ::authority_api::v1::RemoveAssetResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_u64_authority_api_v1_RemoveAssetResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      uint64_t* key, const ::authority_api::v1::RemoveAssetResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<uint64_t, ::authority_api::v1::RemoveAssetResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<bool, ::authority_api::v1::RemoveAssetResponse>* __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_new() {
    return new google::protobuf::Map<bool, ::authority_api::v1::RemoveAssetResponse>();
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_free(const google::protobuf::Map<bool, ::authority_api::v1::RemoveAssetResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_clear(google::protobuf::Map<bool, ::authority_api::v1::RemoveAssetResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_size(const google::protobuf::Map<bool, ::authority_api::v1::RemoveAssetResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_insert(google::protobuf::Map<bool, ::authority_api::v1::RemoveAssetResponse> * m,
                          bool key, ::authority_api::v1::RemoveAssetResponse value) {
    auto k = key;
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_get(const google::protobuf::Map<bool, ::authority_api::v1::RemoveAssetResponse>* m,
                       bool key, const ::authority_api::v1::RemoveAssetResponse** value) {
    auto it = m->find(key);
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::RemoveAssetResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_remove(google::protobuf::Map<bool, ::authority_api::v1::RemoveAssetResponse> * m,
                          bool key, ::authority_api::v1::RemoveAssetResponse * value) {
    auto num_removed = m->erase(key);
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_iter(
      const google::protobuf::Map<bool, ::authority_api::v1::RemoveAssetResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_bool_authority_api_v1_RemoveAssetResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      bool* key, const ::authority_api::v1::RemoveAssetResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<bool, ::authority_api::v1::RemoveAssetResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = cpp_key;
    *value = &cpp_value;
  }
}
extern "C" {
  const google::protobuf::Map<std::string, ::authority_api::v1::RemoveAssetResponse>* __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_new() {
    return new google::protobuf::Map<std::string, ::authority_api::v1::RemoveAssetResponse>();
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_free(const google::protobuf::Map<std::string, ::authority_api::v1::RemoveAssetResponse>* m) { delete m; }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_clear(google::protobuf::Map<std::string, ::authority_api::v1::RemoveAssetResponse> * m) { m->clear(); }
  size_t __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_size(const google::protobuf::Map<std::string, ::authority_api::v1::RemoveAssetResponse>* m) {
    return m->size();
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_insert(google::protobuf::Map<std::string, ::authority_api::v1::RemoveAssetResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::RemoveAssetResponse value) {
    auto k = std::string(key.ptr, key.len);
    auto it = m->find(k);
    if (it != m->end()) {
      return false;
    }
    (*m)[k] = value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_get(const google::protobuf::Map<std::string, ::authority_api::v1::RemoveAssetResponse>* m,
                       google::protobuf::rust_internal::PtrAndLen key, const ::authority_api::v1::RemoveAssetResponse** value) {
    auto it = m->find(std::string(key.ptr, key.len));
    if (it == m->end()) {
      return false;
    }
    const ::authority_api::v1::RemoveAssetResponse* cpp_value = &it->second;
    *value = cpp_value;
    return true;
  }
  bool __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_remove(google::protobuf::Map<std::string, ::authority_api::v1::RemoveAssetResponse> * m,
                          google::protobuf::rust_internal::PtrAndLen key, ::authority_api::v1::RemoveAssetResponse * value) {
    auto num_removed = m->erase(std::string(key.ptr, key.len));
    return num_removed > 0;
  }
  google::protobuf::internal::UntypedMapIterator __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_iter(
      const google::protobuf::Map<std::string, ::authority_api::v1::RemoveAssetResponse>* m) {
    return google::protobuf::internal::UntypedMapIterator::FromTyped(m->cbegin());
  }
  void __rust_proto_thunk__Map_ProtoStr_authority_api_v1_RemoveAssetResponse_iter_get(
      const google::protobuf::internal::UntypedMapIterator* iter,
      google::protobuf::rust_internal::PtrAndLen* key, const ::authority_api::v1::RemoveAssetResponse** value) {
    auto typed_iter = iter->ToTyped<
        google::protobuf::Map<std::string, ::authority_api::v1::RemoveAssetResponse>::const_iterator>();
    const auto& cpp_key = typed_iter->first;
    const auto& cpp_value = typed_iter->second;
    *key = google::protobuf::rust_internal::PtrAndLen(cpp_key.data(), cpp_key.size());
    *value = &cpp_value;
  }
}

